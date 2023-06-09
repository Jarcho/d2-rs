use crate::hooks::Hooks;
use d2interface::{
  self as d2,
  v105::{ADDRESSES, BASE_ADDRESSES},
};

#[rustfmt::skip]
pub(super) const HOOKS: Hooks = Hooks {
  version: "1.05b",
  addresses: ADDRESSES,
  base_addresses: BASE_ADDRESSES,
  load_modules: d2::Modules::load_split_modules,
  patches: super::v105::HOOKS.patches,
};
