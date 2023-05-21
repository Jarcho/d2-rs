use crate::Addresses;

pub use crate::v111::{
  DyPos, Entity, EntityPos, EntityTable, EntityTables, Room, StaticPos, BASE_ADDRESSES,
};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x11c1e0,
  env_splashes: 0x11c518,
  env_bubbles: 0x11c51c,
  client_updates: 0x11a298,
  game_type: 0x11c2ac,
  active_entities: 0,
  draw_game_fn: 0x11a284,
  client_fps_frames: 0x11a2ac,
  client_total_frames: 0x11a294,
  // Signature: `__stdcall(DyPos* @ esi, FixedU16, FixedU16, Room*)`
  apply_pos_change: 0,
  in_perspective: 0xb2a0,
  hwnd: 0x14890,
  server_update_time: 0x111c00,
  draw_menu: 0x13d20,
};
