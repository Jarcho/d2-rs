use crate::{module::Ordinal::Ordinal, Addresses};

pub use crate::v107::{
  DyPos, Entity, EntityPos, EntityTable, EntityTables, Room, StaticPos, BASE_ADDRESSES,
};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x1245e0,
  env_splashes: 0x10eb6c,
  env_bubbles: 0x10eb70,
  client_updates: 0x10ebd8,
  game_type: 0x10edd0,
  active_entities: 0x122de0,
  draw_game_fn: 0x10ebc4,
  client_fps_frames: 0x10ebec,
  client_total_frames: 0x10ebd4,
  // Doesn't exist in this version
  apply_pos_change: 0,
  in_perspective: Ordinal(10010),
  hwnd: Ordinal(10027),
  server_update_time: 0xee68c,
  draw_menu: Ordinal(10019),
};
