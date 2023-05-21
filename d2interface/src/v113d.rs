use crate::Addresses;

pub use crate::v113c::{
  DyPos, Entity, EntityPos, EntityTable, EntityTables, Room, StaticPos, BASE_ADDRESSES,
};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x11d050,
  env_splashes: 0x11d080,
  env_bubbles: 0x11d084,
  client_updates: 0x108758,
  game_type: 0x11d1dc,
  active_entities: 0x1047b8,
  draw_game_fn: 0x108744,
  client_fps_frames: 0x10876c,
  client_total_frames: 0x108754,
  // Signature: `__stdcall(DyPos* @ esi, FixedU16, FixedU16, Room*)`
  apply_pos_change: 0x6ee00,
  in_perspective: 0xa8b0,
  hwnd: 0x14a44,
  server_update_time: 0x111c30,
  draw_menu: 0xeb30,
};
