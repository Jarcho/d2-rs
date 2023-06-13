use crate::{hooks::GameAccessor, INSTANCE};
use core::sync::atomic::{AtomicBool, Ordering::Relaxed};
use windows_sys::Win32::{
  Foundation::{HWND, LPARAM, LRESULT, WPARAM},
  UI::{
    Shell::{DefSubclassProc, SetWindowSubclass},
    WindowsAndMessaging::{SIZE_MINIMIZED, WM_ACTIVATE, WM_SIZE, WM_WINDOWPOSCHANGED},
  },
};

unsafe extern "system" fn win_proc(
  hwnd: HWND,
  msg: u32,
  wparam: WPARAM,
  lparam: LPARAM,
  _: usize,
  _: usize,
) -> LRESULT {
  match msg {
    WM_ACTIVATE if INSTANCE.config.bg_fps.load_relaxed().num != 0 => {
      INSTANCE.precision_timer.enable(wparam != 0);
      INSTANCE.render_fps.copy_from_relaxed(if wparam != 0 {
        &INSTANCE.game_fps
      } else {
        &INSTANCE.config.bg_fps
      });
    }
    WM_SIZE => INSTANCE
      .is_window_hidden
      .store(wparam == SIZE_MINIMIZED as usize, Relaxed),
    WM_WINDOWPOSCHANGED => INSTANCE.frame_rate_from_window(hwnd),
    _ => {}
  }

  DefSubclassProc(hwnd, msg, wparam, lparam)
}

pub struct WindowHook(AtomicBool);
impl WindowHook {
  const ID: usize = 59384;

  pub const fn new() -> Self {
    Self(AtomicBool::new(false))
  }

  pub unsafe fn attach(&self, accessor: &GameAccessor) -> bool {
    loop {
      if self.0.load(Relaxed) {
        return false;
      }
      let hwnd = (accessor.get_hwnd)();
      if hwnd == 0 {
        return false;
      }
      if self.0.compare_exchange_weak(false, true, Relaxed, Relaxed).is_err() {
        continue;
      }
      if !INSTANCE.config.features.fps() {
        return false;
      }
      SetWindowSubclass(hwnd, Some(win_proc), Self::ID, 0);
      return true;
    }
  }
}
