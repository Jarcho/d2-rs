use crate::{module::Ordinal::Ordinal, Addresses};

pub use crate::v109::{
  DyPos, Entity, EntityPos, EntityTable, EntityTables, Room, StaticPos, BASE_ADDRESSES,
};

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
