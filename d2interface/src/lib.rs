#![no_std]

macro_rules! decl_enum {
  ($name:ident($ty:ty) { $($vname:ident = $value:literal),* $(,)? }) => {
    #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(transparent)]
    pub struct $name(pub $ty);
    #[allow(non_upper_case_globals)]
    impl $name {
      $(pub const $vname: Self = Self($value);)*
    }
  }
}

mod common;
mod coord;
mod module;

pub mod v100;
pub mod v101;
pub mod v102;
pub mod v103;
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
    ActId, ActIdS, EntityKind, EntityTable, EntityTables, EnvImage, EnvImages, GameType, InRoom,
    LinkedList,
  },
  coord::{FixedI16, FixedPoint, FixedU16, FixedU3, FixedU8, IsoPos, LinearPos, Size, UnknownPos},
  module::{Addresses, BaseAddresses, Client, Common, Game, Gfx, Module, Win},
};
