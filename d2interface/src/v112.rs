use crate::Addresses;

pub use crate::v111b::{
  DyPos, Entity, EntityPos, EntityTable, EntityTables, Room, StaticPos, BASE_ADDRESSES,
};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x11c3d0,
  env_splashes: 0x11c3e0,
  env_bubbles: 0x11c3e4,
  client_updates: 0x1032b0,
  game_type: 0x11bff8,
  active_entities: 0x11a960,
  draw_game_fn: 0x10329c,
  client_fps_frames: 0x1032c4,
  client_total_frames: 0x1032ac,
  // Signature: `__stdcall(DyPos* @ esi, FixedU16, FixedU16, Room*)`
  apply_pos_change: 0x4af00,
  in_perspective: 0x8de0,
  hwnd: 0x1d458,
  server_update_time: 0x111c34,
  draw_menu: 0xd710,
};
