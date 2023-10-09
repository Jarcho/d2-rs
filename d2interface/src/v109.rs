use crate::{module::Ordinal::Ordinal, Addresses, BaseAddresses};

pub use crate::v108::{
  DyPos, Entity, EntityPos, EntityTable, EntityTables, GameCursor, Room, StaticPos,
};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x127578,
  env_effects: 0x111afc,
  game_type: 0x111d60,
  active_entities: 0x125d78,
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
};
pub const BASE_ADDRESSES: BaseAddresses = BaseAddresses {
  client: 0x6faa0000,
  common: 0x6fd40000,
  game: 0x6fc30000,
  gfx: 0x6fa70000,
  win: 0x6f8a0000,
};
