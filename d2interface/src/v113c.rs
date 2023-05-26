use crate::{module::Ordinal::Ordinal, Addresses};

pub use crate::v112::{
  DyPos, Entity, EntityPos, EntityTable, EntityTables, Room, StaticPos, BASE_ADDRESSES,
};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x11bbfc,
  env_effects: 0x11bf60,
  game_type: 0x11c394,
  active_entities: 0x10a608,
  client_loop_globals: 0x1197e0,
  // Signature: `__stdcall(DyPos* @ esi, FixedU16, FixedU16, Room*)`
  apply_pos_change: 0xda40,
  in_perspective: Ordinal(10013),
  hwnd: Ordinal(10048),
  server_update_time: 0x111c44,
  draw_menu: Ordinal(10024),
};
