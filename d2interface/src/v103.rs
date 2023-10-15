use crate::{module::Ordinal::Ordinal, Addresses};

pub use crate::v102::{
  DyPos, Entity, EntityTable, EntityTables, GameCursor, Room, StaticPos, BASE_ADDRESSES,
};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x1450c0,
  env_effects: 0x12e83c,
  game_type: 0x12eac8,
  entity_table: 0x1438c0,
  entity_table2: 0x1444c0,
  client_loop_globals: 0x12e8b8,
  // Doesn't exist in this version
  apply_pos_change: 0x0,
  in_perspective: Ordinal(10012),
  hwnd: Ordinal(10029),
  server_update_time: 0xc17a4,
  draw_menu: Ordinal(10015),
  cursor_table: 0x11acd0,
  game_cursor: 0x168078,
  summit_cloud_x_pos: 0,
  draw_line: Ordinal(10061),
  find_closest_color: Ordinal(10030),
  viewport_width: 0,
  viewport_height: 0,
  viewport_shift: 0x134594,
};
