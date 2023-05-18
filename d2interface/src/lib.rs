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

pub mod all_versions;
mod util;
pub mod v109d;
pub mod v110;
pub mod v112;
pub mod v113c;
pub mod v113d;
pub mod v114a;
pub mod v114d;

pub use util::{
  FixedI16, FixedPoint, FixedU16, FixedU3, FixedU8, IsoPos, LinearPos, Size, UnknownPos,
};
