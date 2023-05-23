use crate::{module::Ordinal::Address, Addresses};

pub use crate::v114a::{DyPos, Entity, EntityTable, EntityTables, Room, StaticPos, BASE_ADDRESSES};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x39eaf8,
  env_splashes: 0x3a0a84,
  env_bubbles: 0x3a0a88,
  client_updates: 0x398520,
  game_type: 0x398698,
  active_entities: 0,
  draw_game_fn: 0x39850c,
  client_fps_frames: 0x398534,
  client_total_frames: 0x39851c,
  // Signature: `__stdcall(DyPos* @ eax, FixedU16, FixedU16, Room*)`
  apply_pos_change: 0,
  in_perspective: Address(0xf2770),
  hwnd: Address(0xf2f50),
  server_update_time: 0x47bd98,
  draw_menu: Address(0xf6f30),
};
