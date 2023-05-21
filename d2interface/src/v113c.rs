use crate::Addresses;

pub use crate::v112::{
  DyPos, Entity, EntityPos, EntityTable, EntityTables, Room, StaticPos, BASE_ADDRESSES,
};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x11bbfc,
  env_splashes: 0x11bf60,
  env_bubbles: 0x11bf64,
  client_updates: 0x1197f8,
  game_type: 0x11c394,
  active_entities: 0x10a608,
  draw_game_fn: 0x1197e4,
  client_fps_frames: 0x11980c,
  client_total_frames: 0x1197f4,
  // Signature: `__stdcall(DyPos* @ esi, FixedU16, FixedU16, Room*)`
  apply_pos_change: 0xda40,
  in_perspective: 0xb290,
  hwnd: 0x11264,
  server_update_time: 0x111c44,
  draw_menu: 0x187e0,
};
