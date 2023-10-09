use crate::{module::Ordinal::Ordinal, Addresses};

pub use crate::v111b::{
  DyPos, Entity, EntityPos, EntityTable, EntityTables, GameCursor, Room, StaticPos, BASE_ADDRESSES,
};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x11c3d0,
  env_effects: 0x11c3e0,
  game_type: 0x11bff8,
  active_entities: 0x11a960,
  client_loop_globals: 0x103298,
  // Signature: `__stdcall(DyPos* @ esi, FixedU16, FixedU16, Room*)`
  apply_pos_change: 0x4af00,
  in_perspective: Ordinal(10071),
  hwnd: Ordinal(10078),
  server_update_time: 0x111c34,
  draw_menu: Ordinal(10094),
  cursor_table: 0xbd7b8,
  game_cursor: 0xe1644,
  summit_cloud_x_pos: 0,
};
