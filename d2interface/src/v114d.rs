use crate::{module::Ordinal::Address, Addresses};

pub use crate::v114c::{
  DyPos, Entity, EntityTable, EntityTables, GameCursor, Room, StaticPos, BASE_ADDRESSES,
};

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
  cursor_table: 0x312010,
  game_cursor: 0x3a6abc,
  summit_cloud_x_pos: 0x3b5be8,
  draw_line: Address(0xf6380),
  find_closest_color: Address(0xfb180),
  viewport_width: 0x3a5220,
  viewport_height: 0x3a521c,
  viewport_shift: 0x3a5214,
};
