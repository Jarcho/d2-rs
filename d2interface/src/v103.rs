use crate::{module::Ordinal::Ordinal, Addresses};

pub use crate::v102::{DyPos, Entity, EntityTable, EntityTables, Room, StaticPos, BASE_ADDRESSES};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x1450c0,
  env_splashes: 0x12e83c,
  env_bubbles: 0x12e840,
  client_updates: 0x12e8d0,
  game_type: 0x12eac8,
  active_entities: 0x1438c0,
  draw_game_fn: 0x12e8bc,
  client_fps_frames: 0x12e8e4,
  client_total_frames: 0x12e8cc,
  // Doesn't exist in this version
  apply_pos_change: 0x0,
  in_perspective: Ordinal(10012),
  hwnd: Ordinal(10029),
  server_update_time: 0xc17a4,
  draw_menu: Ordinal(10015),
};
