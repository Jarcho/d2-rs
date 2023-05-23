use crate::Addresses;

pub use crate::v101::{DyPos, Entity, EntityTable, EntityTables, Room, StaticPos, BASE_ADDRESSES};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x1450e0,
  env_splashes: 0x12e8dc,
  env_bubbles: 0x12e8e0,
  client_updates: 0x12e970,
  game_type: 0x12eb68,
  active_entities: 0x1438e0,
  draw_game_fn: 0x12e95c,
  client_fps_frames: 0x12e984,
  client_total_frames: 0x12e96c,
  // Doesn't exist in this version
  apply_pos_change: 0x0,
  in_perspective: 0x10dc,
  hwnd: 0x2a994,
  server_update_time: 0xbf864,
  draw_menu: 0x121c,
};
