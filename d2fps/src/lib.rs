#![feature(hash_drain_filter)]

use crate::{
  config::Config,
  hooks::{GameAccessor, Position, UnitId},
  limiter::{MenuAnimRateLimiter, VariableRateLimiter},
  util::{
    log_loaded_modules, message_box_error, monitor_refresh_rate, AtomicRatio, PerfFreq,
    PrecisionTimer, Ratio,
  },
  window::WindowHook,
};
use core::{
  ffi::c_void,
  num::NonZeroU32,
  sync::atomic::{AtomicBool, AtomicIsize, Ordering::Relaxed},
};
use d2interface::{FixedI16, IsoPos};
use fxhash::FxHashMap as HashMap;
use parking_lot::Mutex;
use std::panic::set_hook;
use windows_sys::Win32::{
  Foundation::{BOOL, FALSE, HMODULE, HWND, TRUE},
  Graphics::Gdi::{MonitorFromWindow, MONITOR_DEFAULTTONEAREST},
  System::{
    LibraryLoader::{
      GetModuleHandleExW, GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS, GET_MODULE_HANDLE_EX_FLAG_PIN,
    },
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

mod config;
mod features;
mod hooks;
mod limiter;
mod logger;
mod util;
mod window;

const GAME_FPS: Ratio = Ratio::new(25, unsafe { NonZeroU32::new_unchecked(1) });

struct InstanceSync {
  accessor: GameAccessor,
  entity_tracker: Option<HashMap<UnitId, Position>>,
  render_timer: VariableRateLimiter,
  menu_timer: MenuAnimRateLimiter,
  game_update_time_ms: u32,
  game_update_time: u64,
  client_update_count: u32,
  player_pos: IsoPos<i32>,
  unit_offset: FixedI16,
}
struct Instance {
  sync: Mutex<InstanceSync>,
  config: Config,
  game_fps: AtomicRatio,
  render_fps: AtomicRatio,
  current_monitor: AtomicIsize,
  is_attached: AtomicBool,
  precision_timer: PrecisionTimer,
  is_window_hidden: AtomicBool,
  perf_freq: PerfFreq,
  menu_timer_updated: AtomicBool,
  window_hook: WindowHook,
}
impl Instance {
  unsafe fn frame_rate_from_window(&self, hwnd: HWND) {
    if self.config.fps.load_relaxed().num == 0 {
      let mon: isize = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
      if self.current_monitor.swap(mon, Relaxed) != mon {
        if let Some(rate) = monitor_refresh_rate(mon) {
          log!("Detected monitor fps: {rate}");
          self.game_fps.store_relaxed(rate);
          self.render_fps.store_relaxed(rate);
        }
      }
    }
  }
}
static INSTANCE: Instance = Instance {
  sync: Mutex::new(InstanceSync {
    accessor: GameAccessor::new(),
    entity_tracker: None,
    render_timer: VariableRateLimiter::new(),
    menu_timer: MenuAnimRateLimiter::new(),
    game_update_time_ms: 0,
    game_update_time: 0,
    client_update_count: 0,
    player_pos: IsoPos::new(0, 0),
    unit_offset: FixedI16::from_repr(0),
  }),
  config: Config::new(),
  game_fps: AtomicRatio::new(GAME_FPS),
  render_fps: AtomicRatio::new(GAME_FPS),
  current_monitor: AtomicIsize::new(0),
  is_attached: AtomicBool::new(false),
  precision_timer: PrecisionTimer::new(),
  is_window_hidden: AtomicBool::new(false),
  perf_freq: PerfFreq::uninit(),
  menu_timer_updated: AtomicBool::new(false),
  window_hook: WindowHook::new(),
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
pub extern "C" fn attach_hooks() {
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
      INSTANCE.game_fps.store_relaxed(game_fps);
      INSTANCE.render_fps.store_relaxed(game_fps);
      sync_instance.entity_tracker = Some(HashMap::with_capacity_and_hasher(
        2048,
        fxhash::FxBuildHasher::default(),
      ));
      sync_instance.attach();

      if INSTANCE.config.features.fps() {
        INSTANCE.precision_timer.enable(true);
      }

      log_loaded_modules();

      INSTANCE.is_attached.store(true, Relaxed);
    }

    return;
  }
}
