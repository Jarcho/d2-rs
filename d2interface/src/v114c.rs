use crate::{module::Ordinal::Address, Addresses};

pub use crate::v114b::{DyPos, Entity, EntityTable, EntityTables, Room, StaticPos, BASE_ADDRESSES};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x39daf8,
  env_splashes: 0x39fa84,
  env_bubbles: 0x39fa88,
  client_updates: 0x397520,
  game_type: 0x397698,
  active_entities: 0x39cef8,
  draw_game_fn: 0x39750c,
  client_fps_frames: 0x397534,
  client_total_frames: 0x39751c,
  // Signature: `__stdcall(DyPos* @ eax, FixedU16, FixedU16, Room*)`
  apply_pos_change: 0x250bc0,
  in_perspective: Address(0xf2770),
  hwnd: Address(0xf2f50),
  server_update_time: 0x47ace0,
  draw_menu: Address(0xf6f30),
};
