use crate::{Ratio, GAME_FPS, INSTANCE};
use core::mem::replace;

pub(crate) struct MenuAnimRateLimiter {
  next_update: u64,
  menu_fn: usize,
}
impl MenuAnimRateLimiter {
  pub const fn new() -> Self {
    Self { next_update: 0, menu_fn: 0 }
  }

  pub fn update_time(&mut self, time: u64, menu_fn: usize) -> bool {
    if replace(&mut self.menu_fn, menu_fn) != menu_fn {
      self.next_update = 0;
    }

    if time >= self.next_update {
      let count = time * u64::from(GAME_FPS.num)
        / INSTANCE.perf_freq.s_to_sample(u64::from(GAME_FPS.den.get()));
      self.next_update = INSTANCE.perf_freq.s_to_sample(u64::from(GAME_FPS.den.get()))
        * (count + 1)
        / u64::from(GAME_FPS.num);
      true
    } else {
      false
    }
  }
}

pub(crate) struct VariableRateLimiter {
  fps: Ratio,
  next_update: u64,
  last_update: u64,
}
impl VariableRateLimiter {
  pub const fn new() -> Self {
    Self { fps: GAME_FPS, next_update: 0, last_update: 0 }
  }

  pub fn update_time(&mut self, time: u64, fps: Ratio) -> bool {
    if replace(&mut self.fps, fps) != fps {
      let count = self.last_update * u64::from(self.fps.num)
        / INSTANCE.perf_freq.s_to_sample(u64::from(self.fps.den.get()));
      self.next_update = INSTANCE.perf_freq.s_to_sample(u64::from(self.fps.den.get()))
        * (count + 1)
        / u64::from(self.fps.num);
    }

    if time >= self.next_update {
      let count = time * u64::from(self.fps.num)
        / INSTANCE.perf_freq.s_to_sample(u64::from(self.fps.den.get()));
      self.last_update = INSTANCE.perf_freq.s_to_sample(u64::from(self.fps.den.get())) * count
        / u64::from(self.fps.num);
      self.next_update = INSTANCE.perf_freq.s_to_sample(u64::from(self.fps.den.get()))
        * (count + 1)
        / u64::from(self.fps.num);
      true
    } else {
      false
    }
  }

  pub fn next_update(&self) -> u64 {
    self.next_update
  }

  pub fn last_update(&self) -> u64 {
    self.last_update
  }
}
