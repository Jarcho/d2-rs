use crate::{module::Ordinal::Ordinal, Addresses, BaseAddresses};

pub use crate::v107::{DyPos, Entity, EntityPos, EntityTable, EntityTables, Room, StaticPos};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x1263f8,
  env_splashes: 0x11095c,
  env_bubbles: 0x110960,
  client_updates: 0x1109c8,
  game_type: 0x110bc0,
  active_entities: 0x124bf8,
  draw_game_fn: 0x1109b4,
  client_fps_frames: 0x1109dc,
  client_total_frames: 0x1109c4,
  // Doesn't exist in this version
  apply_pos_change: 0,
  in_perspective: Ordinal(10010),
  hwnd: Ordinal(10027),
  server_update_time: 0xf4198,
  draw_menu: Ordinal(10019),
};
pub const BASE_ADDRESSES: BaseAddresses = BaseAddresses {
  client: 0x6faa0000,
  common: 0x6fd40000,
  game: 0x6fc30000,
  gfx: 0x6fa70000,
  win: 0x6f8a0000,
};
