use crate::{module::Ordinal::Address, Addresses};

pub use crate::v114c::{DyPos, Entity, EntityTable, EntityTables, Room, StaticPos, BASE_ADDRESSES};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x3a6a70,
  env_splashes: 0x3a89fc,
  env_bubbles: 0x3a8a00,
  client_updates: 0x3a0498,
  game_type: 0x3a0610,
  active_entities: 0x3a5e70,
  draw_game_fn: 0x3a0484,
  client_fps_frames: 0x3a04ac,
  client_total_frames: 0x3a0494,
  // Signature: `__stdcall(DyPos* @ eax, FixedU16, FixedU16, Room*)`
  apply_pos_change: 0x24fb90,
  in_perspective: Address(0xf51d0),
  hwnd: Address(0xf59a0),
  server_update_time: 0x483d58,
  draw_menu: Address(0xf98e0),
};
