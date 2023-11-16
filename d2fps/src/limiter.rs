use crate::{Ratio, GAME_FPS, INSTANCE};
use core::{mem::replace, num::NonZeroU32};

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
  den_ticks: u64,
  next_update: u64,
  last_update: u64,
  last_count: u128,
}
impl VariableRateLimiter {
  pub const fn new() -> Self {
    Self {
      fps: Ratio::new(0, unsafe { NonZeroU32::new_unchecked(1) }),
      den_ticks: 0,
      next_update: 0,
      last_update: 0,
      last_count: 0,
    }
  }

  pub fn update_time(&mut self, time: u64, fps: Ratio) -> bool {
    if replace(&mut self.fps, fps) != fps {
      self.den_ticks = INSTANCE.perf_freq.s_to_ticks(u64::from(self.fps.den.get()));
      self.last_count = u128::from(self.last_update) * u128::from(self.fps.num)
        / u128::from(INSTANCE.perf_freq.s_to_ticks(u64::from(self.fps.den.get())));
      self.next_update =
        (u128::from(self.den_ticks) * (self.last_count + 1) / u128::from(self.fps.num)) as u64;
    }

    if time >= self.next_update {
      let count = u128::from(time) * u128::from(self.fps.num) / u128::from(self.den_ticks);
      if count == self.last_count {
        self.last_count += 1;
      } else {
        self.last_count = count;
      }

      self.last_update =
        (u128::from(self.den_ticks) * self.last_count / u128::from(self.fps.num)) as u64;
      self.next_update =
        (u128::from(self.den_ticks) * (self.last_count + 1) / u128::from(self.fps.num)) as u64;

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
