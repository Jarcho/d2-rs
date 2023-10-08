use crate::{module::Ordinal::Ordinal, Addresses, BaseAddresses};

pub use crate::v110::{
  DyPos, Entity, EntityPos, EntityTable, EntityTables, GameCursor, Room, StaticPos,
};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x11c4f0,
  env_effects: 0x11c340,
  game_type: 0x11bfbc,
  active_entities: 0x10b290,
  client_loop_globals: 0xfb380,
  // Signature: `__stdcall(DyPos* @ esi, FixedU16, FixedU16, Room*)`
  apply_pos_change: 0x11010,
  in_perspective: Ordinal(10060),
  hwnd: Ordinal(10075),
  server_update_time: 0x111c04,
  draw_menu: Ordinal(10151),
  cursor_table: 0xd36f0,
  game_cursor: 0xea41c,
};
pub const BASE_ADDRESSES: BaseAddresses = BaseAddresses {
  client: 0x6fab0000,
  common: 0x6fd50000,
  game: 0x6fc20000,
  gfx: 0x6fa80000,
  win: 0x6f8e0000,
};
