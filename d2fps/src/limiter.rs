use crate::{PerfFreq, Ratio, GAME_FPS};

pub(crate) struct MenuAniRateLimiter {
  next_update: u64,
  menu_fn: usize,
}
impl MenuAniRateLimiter {
  pub const fn new() -> Self {
    Self { next_update: 0, menu_fn: 0 }
  }

  pub fn update_time(&mut self, time: u64, freq: &PerfFreq, menu_fn: usize) -> bool {
    if menu_fn != self.menu_fn {
      self.menu_fn = menu_fn;
      self.next_update = 0;
    }

    if time >= self.next_update {
      let count = time * u64::from(GAME_FPS.num) / freq.s_to_sample(u64::from(GAME_FPS.den.get()));
      self.next_update =
        freq.s_to_sample(u64::from(GAME_FPS.den.get())) * (count + 1) / u64::from(GAME_FPS.num);
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

  pub fn update_time(&mut self, time: u64, freq: &PerfFreq) -> bool {
    if time >= self.next_update {
      let count = time * u64::from(self.fps.num) / freq.s_to_sample(u64::from(self.fps.den.get()));
      self.last_update =
        freq.s_to_sample(u64::from(self.fps.den.get())) * count / u64::from(self.fps.num);
      self.next_update =
        freq.s_to_sample(u64::from(self.fps.den.get())) * (count + 1) / u64::from(self.fps.num);
      true
    } else {
      false
    }
  }

  pub fn switch_rate(&mut self, freq: &PerfFreq, rate: Ratio) {
    if self.fps == rate {
      return;
    }

    self.fps = rate;
    let count =
      self.last_update * u64::from(self.fps.num) / freq.s_to_sample(u64::from(self.fps.den.get()));
    self.next_update =
      freq.s_to_sample(u64::from(self.fps.den.get())) * (count + 1) / u64::from(self.fps.num);
  }

  pub fn next_update(&self) -> u64 {
    self.next_update
  }

  pub fn last_update(&self) -> u64 {
    self.last_update
  }
}
