use crate::{module::Ordinal::Ordinal, Addresses};

pub use crate::v113c::{
  DyPos, Entity, EntityPos, EntityTable, EntityTables, GameCursor, Room, StaticPos, BASE_ADDRESSES,
};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x11d050,
  env_effects: 0x11d080,
  game_type: 0x11d1dc,
  active_entities: 0x1047b8,
  client_loop_globals: 0x108740,
  // Signature: `__stdcall(DyPos* @ esi, FixedU16, FixedU16, Room*)`
  apply_pos_change: 0x6ee00,
  in_perspective: Ordinal(10037),
  hwnd: Ordinal(10007),
  server_update_time: 0x111c30,
  draw_menu: Ordinal(10127),
  cursor_table: 0xd9a30,
  game_cursor: 0xfc95c,
};
