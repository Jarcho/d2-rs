use crate::{module::Ordinal::Ordinal, Addresses};

pub use crate::v104b::{DyPos, Entity, EntityTable, EntityTables, Room, StaticPos, BASE_ADDRESSES};

pub const ADDRESSES: Addresses = Addresses {
  player: 0xf6800,
  env_splashes: 0xe2dcc,
  env_bubbles: 0xe2dd0,
  client_updates: 0xe2e30,
  game_type: 0xe3028,
  active_entities: 0xf5000,
  draw_game_fn: 0xe2e1c,
  client_fps_frames: 0xe2e44,
  client_total_frames: 0xe2e2c,
  // Doesn't exist in this version
  apply_pos_change: 0x0,
  in_perspective: Ordinal(10012),
  hwnd: Ordinal(10029),
  server_update_time: 0xc4674,
  draw_menu: Ordinal(10015),
};
