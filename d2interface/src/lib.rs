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
mod coord;
mod module;

pub mod v100;
pub mod v101;
pub mod v102;
pub mod v103;
pub mod v104b;
pub mod v105;
pub mod v106;
pub mod v106b;
pub mod v107;
pub mod v108;
pub mod v109;
pub mod v109d;
pub mod v110;
pub mod v111;
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
    dtbl, Act, ArmorTy, BodyLoc, ClientEnvEffects, ClientFpsTimer, ClientLoopGlobals,
    ClientPingTimer, Color, Component, CubeMod, CubeTy, Cursor, CursorId, CursorState, ElTy,
    EntityKind, EntityTable, EntityTables, EnvImage, EnvImages, GameCursor, GameType, HitClass,
    Id16, Id8, InRoom, LinkedList, NgLvl, NpcMode, NpcSpawnTy, ObjMode, Pc, PcMode, Rand, RgbColor,
    SkRange, StorePage, StrId,
  },
  coord::{
    FixedI12, FixedI16, FixedI4, FixedI7, FixedPoint, FixedU16, FixedU3, FixedU4, FixedU8, IsoPos,
    LinearPos, Range, ScreenPos, ScreenRectLr, ScreenRectS, Size, TilePos, UnknownPos,
  },
  module::{Addresses, BaseAddresses, Client, Common, Game, Gfx, Module, Modules, Win},
};
