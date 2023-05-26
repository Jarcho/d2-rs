use crate::{module::Ordinal::Ordinal, Addresses};

pub use crate::v111::{
  DyPos, Entity, EntityPos, EntityTable, EntityTables, Room, StaticPos, BASE_ADDRESSES,
};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x11c1e0,
  env_effects: 0x11c518,
  game_type: 0x11c2ac,
  active_entities: 0x10b470,
  client_loop_globals: 0x11a280,
  // Signature: `__stdcall(DyPos* @ esi, FixedU16, FixedU16, Room*)`
  apply_pos_change: 0x7c100,
  in_perspective: Ordinal(10046),
  hwnd: Ordinal(10022),
  server_update_time: 0x111c00,
  draw_menu: Ordinal(10129),
};
