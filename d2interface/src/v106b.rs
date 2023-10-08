use crate::{module::Ordinal::Ordinal, Addresses};

pub use crate::v106::{
  DyPos, Entity, EntityTable, EntityTables, GameCursor, Room, StaticPos, BASE_ADDRESSES,
};

pub const ADDRESSES: Addresses = Addresses {
  player: 0xf6ac8,
  env_effects: 0xe312c,
  game_type: 0xe3388,
  active_entities: 0xf52c8,
  client_loop_globals: 0xe3178,
  // Doesn't exist in this version
  apply_pos_change: 0x0,
  in_perspective: Ordinal(10012),
  hwnd: Ordinal(10029),
  server_update_time: 0xc4abc,
  draw_menu: Ordinal(10015),
  cursor_table: 0xd1d08,
  game_cursor: 0xfb828,
};
