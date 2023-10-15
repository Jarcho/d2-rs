use crate::{module::Ordinal::Ordinal, Addresses, BaseAddresses};

pub use crate::v103::{DyPos, Entity, EntityTable, EntityTables, GameCursor, Room, StaticPos};

pub const ADDRESSES: Addresses = Addresses {
  player: 0xf6788,
  env_effects: 0xe2dec,
  game_type: 0xe3048,
  active_entities: 0xf4f88,
  client_loop_globals: 0xe2e38,
  // Doesn't exist in this version
  apply_pos_change: 0x0,
  in_perspective: Ordinal(10012),
  hwnd: Ordinal(10029),
  server_update_time: 0xc4774,
  draw_menu: Ordinal(10015),
  cursor_table: 0xd19f8,
  game_cursor: 0xfb4e8,
  summit_cloud_x_pos: 0,
  draw_line: Ordinal(10061),
  find_closest_color: Ordinal(10030),
  viewport_width: 0,
  viewport_height: 0,
  viewport_shift: 0xe7b6c,
};
pub const BASE_ADDRESSES: BaseAddresses = BaseAddresses {
  client: 0x6fb60000,
  common: 0x6fd80000,
  game: 0x6fcb0000,
  gfx: 0x6fb30000,
  win: 0x6f9a0000,
};
