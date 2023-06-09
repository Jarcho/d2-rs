use crate::{module::Ordinal::Address, Addresses};

pub use crate::v114c::{DyPos, Entity, EntityTable, EntityTables, Room, StaticPos, BASE_ADDRESSES};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x3a6a70,
  env_effects: 0x3a89fc,
  game_type: 0x3a0610,
  active_entities: 0x3a5e70,
  client_loop_globals: 0x3a0480,
  // Signature: `__stdcall(DyPos* @ eax, FixedU16, FixedU16, Room*)`
  apply_pos_change: 0x24fb90,
  in_perspective: Address(0xf51d0),
  hwnd: Address(0xf59a0),
  server_update_time: 0x483d58,
  draw_menu: Address(0xf98e0),
};
