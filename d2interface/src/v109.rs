use crate::{module::Ordinal::Ordinal, Addresses, BaseAddresses};

pub use crate::v108::{
  DyPos, Entity, EntityPos, EntityTable, EntityTables, GameCursor, Room, StaticPos,
};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x127578,
  env_effects: 0x111afc,
  game_type: 0x111d60,
  entity_table: 0x125d78,
  entity_table2: 0x126978,
  client_loop_globals: 0x111b50,
  // Doesn't exist in this version
  apply_pos_change: 0,
  in_perspective: Ordinal(10010),
  hwnd: Ordinal(10027),
  server_update_time: 0xf4300,
  draw_menu: Ordinal(10019),
  cursor_table: 0x100c78,
  game_cursor: 0x12c2a8,
  summit_cloud_x_pos: 0,
  draw_line: Ordinal(10057),
  find_closest_color: Ordinal(10034),
  viewport_width: 0x104b14,
  viewport_height: 0x104b10,
  viewport_shift: 0x116db4,
};
pub const BASE_ADDRESSES: BaseAddresses = BaseAddresses {
  client: 0x6faa0000,
  common: 0x6fd40000,
  game: 0x6fc30000,
  gfx: 0x6fa70000,
  win: 0x6f8a0000,
};
