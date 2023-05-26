use crate::{module::Ordinal::Address, Addresses};

pub use crate::v114b::{DyPos, Entity, EntityTable, EntityTables, Room, StaticPos, BASE_ADDRESSES};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x39daf8,
  env_effects: 0x39fa84,
  game_type: 0x397698,
  active_entities: 0x39cef8,
  client_loop_globals: 0x397508,
  // Signature: `__stdcall(DyPos* @ eax, FixedU16, FixedU16, Room*)`
  apply_pos_change: 0x250bc0,
  in_perspective: Address(0xf2770),
  hwnd: Address(0xf2f50),
  server_update_time: 0x47ace0,
  draw_menu: Address(0xf6f30),
};
