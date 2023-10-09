use crate::{Ratio, GAME_FPS, INSTANCE};
use core::mem::replace;

pub(crate) struct FixedRateLimiter {
  next_update: u64,
}
impl FixedRateLimiter {
  pub const fn new() -> Self {
    Self { next_update: 0 }
  }

  pub fn update_time(&mut self, time: u64) -> bool {
    if time >= self.next_update {
      let count = u128::from(time) * u128::from(GAME_FPS.num)
        / u128::from(INSTANCE.perf_freq.s_to_ticks(u64::from(GAME_FPS.den.get())));
      self.next_update = (u128::from(INSTANCE.perf_freq.s_to_ticks(u64::from(GAME_FPS.den.get())))
        * (count + 1)
        / u128::from(GAME_FPS.num)) as u64;
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
      let count = u128::from(self.last_update) * u128::from(self.fps.num)
        / u128::from(INSTANCE.perf_freq.s_to_ticks(u64::from(self.fps.den.get())));
      self.next_update = (u128::from(INSTANCE.perf_freq.s_to_ticks(u64::from(self.fps.den.get())))
        * (count + 1)
        / u128::from(self.fps.num)) as u64;
    }

    if time >= self.next_update {
      let count = u128::from(time) * u128::from(self.fps.num)
        / u128::from(INSTANCE.perf_freq.s_to_ticks(u64::from(self.fps.den.get())));
      self.last_update = (u128::from(INSTANCE.perf_freq.s_to_ticks(u64::from(self.fps.den.get())))
        * count
        / u128::from(self.fps.num)) as u64;
      self.next_update = (u128::from(INSTANCE.perf_freq.s_to_ticks(u64::from(self.fps.den.get())))
        * (count + 1)
        / u128::from(self.fps.num)) as u64;
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
