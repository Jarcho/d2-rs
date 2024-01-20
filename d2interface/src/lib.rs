#![no_std]

macro_rules! decl_enum {
  ($name:ident($ty:ty) { $($vname:ident = $value:expr),* $(,)? }) => {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(transparent)]
    pub struct $name(pub $ty);
    #[allow(non_upper_case_globals)]
    impl $name {
      $(pub const $vname: Self = Self($value);)*
    }
  }
}

macro_rules! decl_id {
  ($name:ident($ty:ty)) => {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(transparent)]
    pub struct $name(pub $ty);
    impl<T> From<$crate::common::SId<T, Self>> for $name
    where
      $ty: From<T>,
    {
      fn from(x: $crate::common::SId<T, Self>) -> Self {
        Self(x.0.into())
      }
    }
  };
}

mod common;
pub mod coord;
mod module;

pub mod v100;
pub mod v101;
pub mod v102;
pub mod v103;
pub mod v104b;
pub mod v105;
pub mod v106a;
pub mod v106b;
pub mod v107;
pub mod v108;
pub mod v109a;
pub mod v109d;
pub mod v110;
pub mod v111a;
pub mod v111b;
pub mod v112;
pub mod v113c;
pub mod v113d;
pub mod v114a;
pub mod v114b;
pub mod v114c;
pub mod v114d;

pub use crate::{
  common::{
    dtbl, Act, ArmorTy, BodyLoc, Bool32, ClientEnvEffects, ClientFpsTimer, ClientLoopGlobals,
    ClientPingTimer, Color, Component, CubeMod, CubeTy, Cursor, CursorId, CursorState, ElTy,
    EntityKind, EntityTable, EntityTables, EnvImage, EnvImages, EnvParticle, EnvParticles,
    GameCursor, GameType, Id16, Id8, InRoom, ItemHitClass, LinkedList, NgLvl, NpcSpawnTy, NpcState,
    ObjState, Pc, PcState, RgbColor, Rng, SkRange, StorePage, StrId,
  },
  coord::{IsoPos, LinearPos, Pos, ScreenPos, ScreenRectLr, ScreenRectS, Size, TilePos},
  module::{Addresses, BaseAddresses, Client, Common, Game, Gfx, Module, Modules, Win},
};

use common::dtbl::{AccByLvl3, AccByLvl5, ByNgLvl};
use num::Fixed;

pub type EnvArray = common::EnvArray<()>;

pub type FI16 = Fixed<i32, 16>;
pub type FI12 = Fixed<i32, 12>;
pub type FI7 = Fixed<i32, 7>;
pub type FI4 = Fixed<i32, 4>;

pub type FU16 = Fixed<u32, 16>;
pub type FU8 = Fixed<u32, 8>;
pub type FU4 = Fixed<u32, 4>;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Range<T> {
  pub min: T,
  pub max: T,
}
impl<T> Range<T> {
  pub const fn new(min: T, max: T) -> Self {
    Self { min, max }
  }

  pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> Range<U> {
    Range::new(f(self.min), f(self.max))
  }
}
impl<T: Copy> Range<ByNgLvl<T>> {
  pub fn at_ng_lvl(&self, lvl: NgLvl) -> Option<Range<T>> {
    Some(Range::new(
      self.min.at_ng_lvl(lvl)?,
      self.max.at_ng_lvl(lvl)?,
    ))
  }
}
impl<T: Copy + Into<i32>> Range<AccByLvl3<T>> {
  pub fn at_lvl(&self, lvl: u16) -> Range<i32> {
    Range::new(self.min.at_lvl(lvl), self.max.at_lvl(lvl))
  }
}
impl<T: Copy + Into<i32>> Range<AccByLvl5<T>> {
  pub fn at_lvl(&self, lvl: u16) -> Range<i32> {
    Range::new(self.min.at_lvl(lvl), self.max.at_lvl(lvl))
  }
}
impl<T: Copy, const N: usize> Range<[T; N]> {
  #[track_caller]
  pub fn index(&self, i: usize) -> Range<T> {
    Range::new(self.min[i], self.max[i])
  }
}
