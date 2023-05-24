use crate::{module::Ordinal::Ordinal, Addresses};

pub use crate::v106::{DyPos, Entity, EntityTable, EntityTables, Room, StaticPos, BASE_ADDRESSES};

pub const ADDRESSES: Addresses = Addresses {
  player: 0xf6ac8,
  env_splashes: 0xe312c,
  env_bubbles: 0xe3130,
  client_updates: 0xe3190,
  game_type: 0xe3388,
  active_entities: 0xf52c8,
  draw_game_fn: 0xe317c,
  client_fps_frames: 0xe31a4,
  client_total_frames: 0xe318c,
  // Doesn't exist in this version
  apply_pos_change: 0x0,
  in_perspective: Ordinal(10012),
  hwnd: Ordinal(10029),
  server_update_time: 0xc4abc,
  draw_menu: Ordinal(10015),
};
