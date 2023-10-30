use crate::{module::Ordinal::Ordinal, Addresses};

pub use crate::v106::{
  dtbl, DyPos, Entity, EntityTable, EntityTables, GameCursor, Room, StaticPos, BASE_ADDRESSES,
};

pub const ADDRESSES: Addresses = Addresses {
  player: 0xf6ac8,
  env_effects: 0xe312c,
  game_type: 0xe3388,
  entity_table: 0xf52c8,
  entity_table2: 0xf5ec8,
  client_loop_globals: 0xe3178,
  // Doesn't exist in this version
  apply_pos_change: 0x0,
  in_perspective: Ordinal(10012),
  hwnd: Ordinal(10029),
  server_update_time: 0xc4abc,
  draw_menu: Ordinal(10015),
  cursor_table: 0xd1d08,
  game_cursor: 0xfb828,
  summit_cloud_x_pos: 0,
  draw_line: Ordinal(10061),
  find_closest_color: Ordinal(10030),
  viewport_width: 0,
  viewport_height: 0,
  viewport_shift: 0xe7eac,
};
