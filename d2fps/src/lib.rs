use crate::{
  hooks::HookManager,
  limiter::{MenuAniRateLimiter, VariableRateLimiter},
  util::{PerfFreq, Ratio},
};
use config::Config;
use core::{
  ffi::c_void,
  num::NonZeroU32,
  sync::atomic::{AtomicUsize, Ordering::Relaxed},
};
use d2interface::{FixedI16, IsoPos};
use parking_lot::Mutex;
use std::panic::set_hook;
use tracker::EntityTracker;
use util::{message_box, monitor_refresh_rate};
use windows_sys::{
  w,
  Win32::{
    Foundation::{BOOL, FALSE, HMODULE, HWND, TRUE},
    Graphics::Gdi::{MonitorFromWindow, HMONITOR, MONITOR_DEFAULTTONEAREST},
    Media::{timeBeginPeriod, timeEndPeriod},
    System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
    UI::WindowsAndMessaging::MB_ICONERROR,
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
mod patch;
mod tracker;
mod util;

const GAME_RATE: Ratio = Ratio::new(1, unsafe { NonZeroU32::new_unchecked(25) });

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
  game_update_count: u32,
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
  frame_rate: GAME_RATE,
  bg_frame_rate: GAME_RATE,
  game_update_time_ms: 0,
  game_update_time: 0,
  game_update_count: 0,
  player_pos: IsoPos::new(0, 0),
  unit_offset: FixedI16::from_repr(0),
  menu_timer_updated: false,
  is_window_hidden: true,
  current_monitor: 0,
});
static ATTACH_COUNT: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
pub extern "system" fn DllMain(_: HMODULE, reason: u32, _: *mut c_void) -> BOOL {
  match reason {
    DLL_PROCESS_ATTACH => {
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
        unsafe {
          message_box(w!("D2fps Error"), &msg, MB_ICONERROR);
        }
      }));

      let mut instance = D2FPS.lock();
      if instance.hooks.init().is_err() || !instance.perf_freq.init() {
        return FALSE;
      };
      instance.config.load_config();
    }
    DLL_PROCESS_DETACH => {
      if ATTACH_COUNT.swap(0, Relaxed) != 0 {
        unsafe {
          timeEndPeriod(1);
        }
      }
      crate::logger::shutdown();
    }
    _ => {}
  }

  TRUE
}

#[no_mangle]
pub extern "C" fn attach_hooks() -> bool {
  let mut expected;
  loop {
    expected = ATTACH_COUNT.load(Relaxed);
    if expected == 0 {
      let mut instance_lock = D2FPS.lock();
      let instance = &mut *instance_lock;
      if ATTACH_COUNT
        .compare_exchange_weak(expected, expected, Relaxed, Relaxed)
        .is_err()
      {
        continue;
      }

      if instance.hooks.attach().is_err() {
        return false;
      }

      unsafe {
        timeBeginPeriod(1);
      }

      instance.frame_rate = instance.config.fps.map_or(GAME_RATE, |r| r.inv());
      instance.bg_frame_rate = instance.config.bg_fps.map_or(GAME_RATE, |r| r.inv());
      instance
        .render_timer
        .switch_rate(&instance.perf_freq, instance.frame_rate);

      ATTACH_COUNT.store(1, Relaxed);
    } else if ATTACH_COUNT
      .compare_exchange_weak(expected, expected + 1, Relaxed, Relaxed)
      .is_ok()
    {
      return true;
    }
  }
}

#[no_mangle]
pub extern "C" fn detach_hooks() {
  let mut expected;
  loop {
    expected = ATTACH_COUNT.load(Relaxed);
    if expected == 0 {
      return;
    }
    if ATTACH_COUNT
      .compare_exchange_weak(expected, expected - 1, Relaxed, Relaxed)
      .is_ok()
    {
      break;
    }
  }

  if expected == 1 {
    let mut instance = D2FPS.lock();
    instance.hooks.detach();
    unsafe {
      timeEndPeriod(1);
    }
  }
}
