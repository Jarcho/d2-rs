use crate::hooks::Hooks;
use d2interface::{
  self as d2,
  v109::{ADDRESSES, BASE_ADDRESSES},
};

#[rustfmt::skip]
pub(super) const HOOKS: Hooks = Hooks {
  version: "1.09b",
  addresses: ADDRESSES,
  base_addresses: BASE_ADDRESSES,
  load_modules: d2::Modules::load_split_modules,
  patches: super::v109::HOOKS.patches,
};
