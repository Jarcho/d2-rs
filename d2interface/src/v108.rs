use crate::{module::Ordinal::Ordinal, Addresses};

pub use crate::v107::{
  DyPos, Entity, EntityPos, EntityTable, EntityTables, GameCursor, Room, StaticPos, BASE_ADDRESSES,
};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x1245e0,
  env_effects: 0x10eb6c,
  game_type: 0x10edd0,
  entity_table: 0x122de0,
  entity_table2: 0x1239e0,
  client_loop_globals: 0x10ebc0,
  // Doesn't exist in this version
  apply_pos_change: 0,
  in_perspective: Ordinal(10010),
  hwnd: Ordinal(10027),
  server_update_time: 0xee68c,
  draw_menu: Ordinal(10019),
  cursor_table: 0xfdcd8,
  game_cursor: 0x129418,
  summit_cloud_x_pos: 0,
  draw_line: Ordinal(10057),
  find_closest_color: Ordinal(10034),
  viewport_width: 0x101b84,
  viewport_height: 0x101b80,
  viewport_shift: 0x113e24,
};
