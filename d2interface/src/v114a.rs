use crate::{module::Ordinal::Address, Addresses, BaseAddresses};

pub use crate::v113d::{DyPos, Entity, EntityTable, EntityTables, Room, StaticPos};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x440df0,
  env_splashes: 0x432960,
  env_bubbles: 0x432964,
  client_updates: 0x42e038,
  game_type: 0x42e1b0,
  active_entities: 0,
  draw_game_fn: 0x42e024,
  client_fps_frames: 0x42e04c,
  client_total_frames: 0x42e034,
  // Signature: `__stdcall(DyPos* @ eax, FixedU16, FixedU16, Room*)`
  apply_pos_change: 0,
  in_perspective: Address(0x5370),
  hwnd: Address(0x5b50),
  server_update_time: 0x497d38,
  draw_menu: Address(0x3cdd0),
};
pub const BASE_ADDRESSES: BaseAddresses = BaseAddresses {
  client: 0x00400000,
  common: 0x00400000,
  game: 0x00400000,
  gfx: 0x00400000,
  win: 0x00400000,
};
