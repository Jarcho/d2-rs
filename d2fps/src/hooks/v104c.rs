use crate::hooks::Hooks;
use d2interface::{
  self as d2,
  v104b::{ADDRESSES, BASE_ADDRESSES},
};

#[rustfmt::skip]
pub(super) const HOOKS: Hooks = Hooks {
  version: "1.04c",
  addresses: ADDRESSES,
  base_addresses: BASE_ADDRESSES,
  load_modules: d2::Modules::load_split_modules,
  patches: super::v104b::HOOKS.patches,
};
