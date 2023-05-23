use crate::{module::Ordinal::Ordinal, Addresses};

pub use crate::v100::{DyPos, Entity, EntityTable, EntityTables, Room, StaticPos, BASE_ADDRESSES};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x145190,
  env_splashes: 0x12e994,
  env_bubbles: 0x12e998,
  client_updates: 0x12ea30,
  game_type: 0x12ec08,
  active_entities: 0x143990,
  draw_game_fn: 0x12ea1c,
  client_fps_frames: 0x12ea44,
  client_total_frames: 0x12ea2c,
  // Doesn't exist in this version
  apply_pos_change: 0x0,
  in_perspective: Ordinal(10012),
  hwnd: Ordinal(10029),
  server_update_time: 0xbe6e4,
  draw_menu: Ordinal(10015),
};