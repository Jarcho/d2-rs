use crate::{
  hooks::HookManager,
  limiter::{MenuAniRateLimiter, VariableRateLimiter},
  util::{log_loaded_modules, PerfFreq, Ratio},
};
use config::Config;
use core::{
  ffi::c_void,
  num::NonZeroU32,
  sync::atomic::{AtomicBool, Ordering::Relaxed},
};
use d2interface::{FixedI16, IsoPos};
use parking_lot::Mutex;
use std::panic::set_hook;
use tracker::EntityTracker;
use util::{message_box_error, monitor_refresh_rate};
use windows_sys::Win32::{
  Foundation::{BOOL, FALSE, HMODULE, HWND, TRUE},
  Graphics::Gdi::{MonitorFromWindow, HMONITOR, MONITOR_DEFAULTTONEAREST},
  Media::timeBeginPeriod,
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
mod hooks;
mod limiter;
mod logger;
mod tracker;
mod util;

const GAME_FPS: Ratio = Ratio::new(25, unsafe { NonZeroU32::new_unchecked(1) });

struct D2Fps {
  hooks: HookManager,
  config: Config,
  entity_tracker: EntityTracker,
  render_timer: VariableRateLimiter,
  menu_timer: MenuAniRateLimiter,
  perf_freq: PerfFreq,
  frame_rate: Ratio,
  bg_frame_rate: Ratio,
  game_update_time_ms: u32,
  game_update_time: u64,
  client_update_count: u32,
  player_pos: IsoPos<i32>,
  unit_offset: FixedI16,
  menu_timer_updated: bool,
  is_window_hidden: bool,
  current_monitor: HMONITOR,
}
impl D2Fps {
  unsafe fn frame_rate_from_window(&mut self, hwnd: HWND) {
    if self.config.fps.is_none() {
      let mon: isize = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
      if mon != self.current_monitor {
        self.current_monitor = mon;
        if let Some(rate) = monitor_refresh_rate(mon) {
          log!("Detected monitor fps: {rate}");
          self.frame_rate = rate;
          self.render_timer.switch_rate(&self.perf_freq, rate);
        }
      }
    }
  }
}

static D2FPS: Mutex<D2Fps> = Mutex::new(D2Fps {
  hooks: HookManager::new(),
  config: Config::new(),
  entity_tracker: EntityTracker::new(),
  render_timer: VariableRateLimiter::new(),
  menu_timer: MenuAniRateLimiter::new(),
  perf_freq: PerfFreq::uninit(),
  frame_rate: GAME_FPS,
  bg_frame_rate: GAME_FPS,
  game_update_time_ms: 0,
  game_update_time: 0,
  client_update_count: 0,
  player_pos: IsoPos::new(0, 0),
  unit_offset: FixedI16::from_repr(0),
  menu_timer_updated: false,
  is_window_hidden: false,
  current_monitor: 0,
});
static IS_ATTACHED: AtomicBool = AtomicBool::new(false);

#[no_mangle]
pub extern "system" fn DllMain(module: HMODULE, reason: u32, _: *mut c_void) -> BOOL {
  match reason {
    DLL_PROCESS_ATTACH => {
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

      let mut instance = D2FPS.lock();
      if !instance.perf_freq.init() {
        return FALSE;
      };
      instance.hooks.init();
      instance.config.load_config();
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
    is_attached = IS_ATTACHED.load(Relaxed);
    if !is_attached {
      let mut instance_lock = D2FPS.lock();
      let instance = &mut *instance_lock;
      if IS_ATTACHED
        .compare_exchange_weak(is_attached, is_attached, Relaxed, Relaxed)
        .is_err()
      {
        continue;
      }

      log!("Attaching D2fps...");
      instance.hooks.attach(&mut instance.config);
      log_loaded_modules();

      unsafe {
        timeBeginPeriod(1);
      }

      instance.frame_rate = instance.config.fps.unwrap_or(GAME_FPS);
      instance.bg_frame_rate = instance.config.bg_fps.unwrap_or(GAME_FPS);
      instance
        .render_timer
        .switch_rate(&instance.perf_freq, instance.frame_rate);

      IS_ATTACHED.store(true, Relaxed);
    }

    return;
  }
}
