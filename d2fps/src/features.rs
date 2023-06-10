use bin_patch::Patch;
use bitflags::bitflags;
use core::{
  fmt,
  mem::transmute,
  sync::atomic::{AtomicU32, Ordering::Relaxed},
};
use d2interface as d2;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(dead_code)]
pub enum FeatureId {
  MenuFps = 0,
  GameFps = 1,
  MotionSmoothing = 2,
}
impl FeatureId {
  pub const fn name(self) -> &'static str {
    match self {
      Self::MenuFps => "menu fps",
      Self::GameFps => "game fps",
      Self::MotionSmoothing => "motion smoothing",
    }
  }

  pub fn iter() -> impl ExactSizeIterator<Item = FeatureId> {
    (0u8..3u8).map(|x| unsafe { transmute(x) })
  }

  pub fn as_flag(self) -> Features {
    Features::from_bits_retain(1 << self as u8)
  }

  pub fn prereqs(self) -> Features {
    match self {
      Self::MenuFps | Self::GameFps => Features::empty(),
      Self::MotionSmoothing => Features::GameFps,
    }
  }
}
impl fmt::Display for FeatureId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.name())
  }
}

bitflags! {
  #[repr(transparent)]
  #[derive(Default, Clone, Copy, PartialEq, Eq)]
  pub struct Features: u32 {
    const MenuFps = 1;
    const GameFps = 2;
    const Fps = 3;
    const MotionSmoothing = 4;
  }
}
impl fmt::Display for Features {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("[")?;
    let mut first = true;
    for feature in FeatureId::iter() {
      if self.intersects(Self::from_bits_retain(1u32 << feature as u8)) {
        if !first {
          f.write_str(", ")?;
        }
        write!(f, "`{feature}`")?;
        first = false;
      }
    }
    f.write_str("]")?;
    Ok(())
  }
}

pub struct AtomicFeatures(AtomicU32);
impl AtomicFeatures {
  #[allow(clippy::declare_interior_mutable_const)]
  pub const ALL: Self = Self(AtomicU32::new(Features::all().bits()));

  pub fn load_relaxed(&self) -> Features {
    Features::from_bits_retain(self.0.load(Relaxed))
  }

  pub fn store_relaxed(&self, x: Features) {
    self.0.store(x.bits(), Relaxed);
  }

  pub fn set_relaxed(&self, x: Features, enable: bool) {
    let load_mask = !(x.bits() & !(enable as u32).wrapping_sub(1));
    let store_bits = x.bits() & (!(enable as u32)).wrapping_add(1);

    loop {
      let x = self.0.load(Relaxed);
      if self
        .0
        .compare_exchange_weak(x, (x & load_mask) | store_bits, Relaxed, Relaxed)
        .is_ok()
      {
        break;
      }
    }
  }

  pub fn remove_relaxed(&self, x: Features) {
    loop {
      let old = self.0.load(Relaxed);
      if self
        .0
        .compare_exchange_weak(
          old,
          Features::from_bits_retain(old).difference(x).bits(),
          Relaxed,
          Relaxed,
        )
        .is_ok()
      {
        break;
      }
    }
  }

  pub fn motion_smoothing(&self) -> bool {
    self.load_relaxed().intersects(Features::MotionSmoothing)
  }

  pub fn fps(&self) -> bool {
    self.load_relaxed().intersects(Features::Fps)
  }
}

pub struct ModulePatches {
  pub module: d2::Module,
  pub patches: &'static [Patch],
}
impl ModulePatches {
  pub const fn new(module: d2::Module, patches: &'static [Patch]) -> Self {
    Self { module, patches }
  }
}

pub struct FeaturePatches([&'static [ModulePatches]; 3]);
impl FeaturePatches {
  pub const fn empty() -> Self {
    Self([&[]; 3])
  }

  pub const fn new(
    menu_fps: &'static [ModulePatches],
    game_fps: &'static [ModulePatches],
    motion_smoothing: &'static [ModulePatches],
  ) -> Self {
    Self([menu_fps, game_fps, motion_smoothing])
  }

  #[allow(clippy::needless_lifetimes)]
  pub fn iter<'a>(
    &'a self,
  ) -> impl 'a + ExactSizeIterator<Item = (FeatureId, &'static [ModulePatches])> {
    self
      .0
      .iter()
      .enumerate()
      .map(|(x, &y)| (unsafe { transmute(x as u8) }, y))
  }
}
