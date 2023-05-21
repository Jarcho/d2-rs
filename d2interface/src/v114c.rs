use crate::Addresses;

pub use crate::v114b::{DyPos, Entity, EntityTable, EntityTables, Room, StaticPos, BASE_ADDRESSES};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x39daf8,
  env_splashes: 0x39fa84,
  env_bubbles: 0x39fa88,
  client_updates: 0x397520,
  game_type: 0x397698,
  active_entities: 0,
  draw_game_fn: 0x39750c,
  client_fps_frames: 0x397534,
  client_total_frames: 0x39751c,
  // Signature: `__stdcall(DyPos* @ eax, FixedU16, FixedU16, Room*)`
  apply_pos_change: 0,
  in_perspective: 0xf2770,
  hwnd: 0x3bfd44,
  server_update_time: 0x47ace0,
  draw_menu: 0xf6f30,
};
