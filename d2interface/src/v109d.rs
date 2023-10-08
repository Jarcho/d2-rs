use crate::{module::Ordinal::Ordinal, Addresses};

pub use crate::v109::{
  DyPos, Entity, EntityPos, EntityTable, EntityTables, GameCursor, Room, StaticPos, BASE_ADDRESSES,
};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x1263f8,
  env_effects: 0x11095c,
  game_type: 0x110bc0,
  active_entities: 0x124bf8,
  client_loop_globals: 0x1109b0,
  // Doesn't exist in this version
  apply_pos_change: 0,
  in_perspective: Ordinal(10010),
  hwnd: Ordinal(10027),
  server_update_time: 0xf4198,
  draw_menu: Ordinal(10019),
  cursor_table: 0xffae8,
  game_cursor: 0x12b128,
};
