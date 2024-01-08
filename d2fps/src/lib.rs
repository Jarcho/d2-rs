use crate::{
  config::Config,
  features::FeaturePatches,
  hooks::{GameAccessor, Position, UnitId},
  limiter::{FixedRateLimiter, VariableRateLimiter},
  util::{
    log_loaded_modules, message_box_error, monitor_refresh_rate, AtomicRatio, PerfFreq,
    PrecisionTimer, Ratio,
  },
  window::WindowHook,
};
use arcane::ArcaneBg;
use atomic_float::AtomicF64;
use core::{
  ffi::c_void,
  num::NonZeroU32,
  sync::atomic::{AtomicBool, AtomicIsize, Ordering::Relaxed},
};
use d2interface as d2;
use fxhash::{FxBuildHasher, FxHashMap as HashMap, FxHashSet as HashSet};
use parking_lot::Mutex;
use rand::SeedableRng;
use std::{panic::set_hook, sync::atomic::AtomicU64};
use windows_sys::Win32::{
  Foundation::{BOOL, FALSE, HMODULE, HWND, TRUE},
  Graphics::Gdi::{MonitorFromWindow, MONITOR_DEFAULTTONEAREST},
  System::{
    LibraryLoader::{
      GetModuleHandleExW, GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS, GET_MODULE_HANDLE_EX_FLAG_PIN,
    },
    Performance::QueryPerformanceCounter,
    SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
  },
};

macro_rules! log {
  ($($args:tt)*) => {
    crate::logger::log(|x| {
      use std::fmt::Write;
      let _ = write!(x, $($args)*);
    })
  }
}

mod arcane;
mod config;
mod features;
mod hooks;
mod limiter;
mod logger;
mod util;
mod weather;
mod window;

const GAME_FPS: Ratio = Ratio::new(25, unsafe { NonZeroU32::new_unchecked(1) });

type Rng = rand_xoshiro::Xoshiro128Plus;

struct InstanceDelayed {
  arcane_bg: ArcaneBg,
  rng: Rng,
  entity_tracker: HashMap<UnitId, Position>,
  /// Used when visiting both entity tables, to avoid visiting an entity in the
  /// secondary table when it exists in the primary table.
  /// Stored as a global to avoid reallocating every frame.
  visited_entities: HashSet<UnitId>,
}
struct InstanceSync {
  accessor: GameAccessor,
  /// Timer for the renderer, both in-game and in-menu.
  render_timer: VariableRateLimiter,
  /// Timer for the character animations in the menu screens.
  menu_anim_timer: FixedRateLimiter,
  /// The last time the game state was updated. Used to detect game state changes.
  game_update_time_ms: u32,
  /// The last time the game state was update. Used as the base time to determine
  /// how much movement the renderer should apply. Different from the previous
  /// field as `GetTickCount` and `QueryPerformanceCounter` are different clocks.
  game_update_time: u64,
  /// The number of times the client state has been updated. Used to detect when
  /// environment effects are updated.
  client_update_count: u32,
  /// The player's last rendered position. Used to adjust the screen position of
  /// certain environment effects.
  player_pos: d2::IsoPos<i32>,
  /// The amount of a unit's detected movement to apply. Used to adjust a unit's
  /// position for cursor detection outside the rendering code.
  unit_movement_fract: d2::FixedI16,
  weather_particles: Vec<weather::Particle>,
  /// Patches to reapply once the menu is loaded. Helps compatibility with other
  /// mods that patch code without validating the patch location's data.
  reapply_patches: Option<(&'static FeaturePatches, d2::Modules)>,
  delayed: Option<InstanceDelayed>,
}
struct Instance {
  sync: Mutex<InstanceSync>,
  config: Config,
  /// The fps to render at while the game window is active.
  active_fps: AtomicRatio,
  /// The fps we are currently rendering at.
  render_fps: AtomicRatio,
  /// Handle to the monitor on which the window was last seen.
  current_monitor: AtomicIsize,
  /// Whether we've attempted to attached to the game.
  is_attached: AtomicBool,
  /// Manages whether sleep calls should use a high precision timer.
  precision_timer: PrecisionTimer,
  /// Whether the game's window is currently hidden. Sleep lengths will be
  /// longer if it is.
  is_window_hidden: AtomicBool,
  /// Performance counter related constants.
  perf_freq: PerfFreq,
  /// Whether the menu character's animation should advance this frame.
  update_menu_char_anim: AtomicBool,
  /// Manages hooking into the game's window procedure. This has to be delayed
  /// as the window may not exist when the library is attached.
  window_hook: WindowHook,
  /// Whether the client state has been updated immediately before the current
  /// frame. Used to know when to step the cursor's animation.
  client_updated: AtomicBool,
  /// The ratio between the time since the previous frame draw, and the game's
  /// update rate.
  update_time_fract: AtomicF64,
  /// The number of QPC ticks that have occurred since the previous drawn frame.
  update_ticks: AtomicU64,
}
impl Instance {
  unsafe fn frame_rate_from_window(&self, hwnd: HWND) {
    if self.config.fps.load_relaxed().num == 0 {
      let mon: isize = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
      if self.current_monitor.swap(mon, Relaxed) != mon {
        if let Some(rate) = monitor_refresh_rate(mon) {
          log!("Detected monitor fps: {rate}");
          self.active_fps.store_relaxed(rate);
          self.render_fps.store_relaxed(rate);
        }
      }
    }
  }
}
static INSTANCE: Instance = Instance {
  sync: Mutex::new(InstanceSync {
    accessor: GameAccessor::new(),
    render_timer: VariableRateLimiter::new(),
    menu_anim_timer: FixedRateLimiter::new(),
    game_update_time_ms: 0,
    game_update_time: 0,
    client_update_count: 0,
    player_pos: d2::IsoPos::new(0, 0),
    unit_movement_fract: d2::FixedI16::from_repr(0),
    weather_particles: Vec::new(),
    reapply_patches: None,
    delayed: None,
  }),
  config: Config::new(),
  active_fps: AtomicRatio::new(GAME_FPS),
  render_fps: AtomicRatio::new(GAME_FPS),
  current_monitor: AtomicIsize::new(0),
  is_attached: AtomicBool::new(false),
  precision_timer: PrecisionTimer::new(),
  is_window_hidden: AtomicBool::new(false),
  perf_freq: PerfFreq::uninit(),
  update_menu_char_anim: AtomicBool::new(false),
  window_hook: WindowHook::new(),
  client_updated: AtomicBool::new(true),
  update_time_fract: AtomicF64::new(0.0),
  update_ticks: AtomicU64::new(0),
};

#[no_mangle]
pub extern "system" fn DllMain(module: HMODULE, reason: u32, _: *mut c_void) -> BOOL {
  match reason {
    DLL_PROCESS_ATTACH => {
      // Should never fail starting with Windows XP
      if !INSTANCE.perf_freq.init() {
        return FALSE;
      };

      // Prevent the library from unloading. Patched game code and the logging
      // thread would error otherwise.
      unsafe {
        GetModuleHandleExW(
          GET_MODULE_HANDLE_EX_FLAG_PIN | GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS,
          module as *const u16,
          &mut 0,
        );
      }

      set_hook(Box::new(|info| {
        let msg: &str = if let Some(s) = info.payload().downcast_ref::<&str>() {
          s
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
          s
        } else {
          "Unknown D2fps error"
        };
        let msg = if let Some(l) = info.location() {
          let mut msg = format!("Error at {}\n{}", l, msg);
          if let Some(p) = msg.find('\0') {
            msg.truncate(p)
          }
          msg
        } else {
          msg.into()
        };
        log!("D2fps Error: {msg}");
        message_box_error(&msg);
      }));
    }
    DLL_PROCESS_DETACH => {
      crate::logger::shutdown();
    }
    _ => {}
  }

  TRUE
}

#[no_mangle]
unsafe extern "stdcall" fn Init() {
  let mut is_attached;
  loop {
    is_attached = INSTANCE.is_attached.load(Relaxed);
    if !is_attached {
      let mut instance_lock = INSTANCE.sync.lock();
      let sync_instance = &mut *instance_lock;
      if INSTANCE
        .is_attached
        .compare_exchange_weak(is_attached, is_attached, Relaxed, Relaxed)
        .is_err()
      {
        continue;
      }

      log!("Attaching D2fps...");
      INSTANCE.config.load_config();
      let config_fps = INSTANCE.config.fps.load_relaxed();
      let game_fps = if config_fps.num == 0 {
        GAME_FPS
      } else {
        config_fps
      };
      INSTANCE.active_fps.store_relaxed(game_fps);
      INSTANCE.render_fps.store_relaxed(game_fps);
      sync_instance.attach();

      let mut time = 0;
      QueryPerformanceCounter(&mut time);
      sync_instance.delayed = Some(InstanceDelayed {
        arcane_bg: ArcaneBg::new(),
        rng: Rng::seed_from_u64(time as u64),
        entity_tracker: HashMap::with_capacity_and_hasher(
          2048, // Should be more than enough to store active units
          FxBuildHasher::default(),
        ),
        visited_entities: HashSet::with_capacity_and_hasher(2048, FxBuildHasher::default()),
      });

      if INSTANCE.config.features.fps() {
        INSTANCE.precision_timer.enable(true);
      }

      log_loaded_modules();

      INSTANCE.is_attached.store(true, Relaxed);
    }

    return;
  }
}

#[no_mangle]
unsafe extern "stdcall" fn InitD2Mod(_: u32) -> u32 {
  Init();
  0
}
