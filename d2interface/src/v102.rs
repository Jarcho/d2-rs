use crate::{module::Ordinal::Ordinal, Addresses};

pub use crate::v101::{
  DyPos, Entity, EntityTable, EntityTables, GameCursor, Room, StaticPos, BASE_ADDRESSES,
};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x1450e0,
  env_effects: 0x12e8dc,
  game_type: 0x12eb68,
  active_entities: 0x1438e0,
  client_loop_globals: 0x12e958,
  // Doesn't exist in this version
  apply_pos_change: 0x0,
  in_perspective: Ordinal(10012),
  hwnd: Ordinal(10029),
  server_update_time: 0xbf864,
  draw_menu: Ordinal(10015),
  cursor_table: 0x11ad70,
  game_cursor: 0x168148,
  summit_cloud_x_pos: 0,
};
