use crate::{module::Ordinal::Ordinal, Addresses};

pub use crate::v112::{
  DyPos, Entity, EntityPos, EntityTable, EntityTables, GameCursor, Room, StaticPos, BASE_ADDRESSES,
};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x11bbfc,
  env_effects: 0x11bf60,
  game_type: 0x11c394,
  active_entities: 0x10a608,
  client_loop_globals: 0x1197e0,
  // Signature: `__stdcall(DyPos* @ esi, FixedU16, FixedU16, Room*)`
  apply_pos_change: 0xda40,
  in_perspective: Ordinal(10013),
  hwnd: Ordinal(10048),
  server_update_time: 0x111c44,
  draw_menu: Ordinal(10024),
  cursor_table: 0xd8558,
  game_cursor: 0xfb834,
  summit_cloud_x_pos: 0,
  draw_line: Ordinal(10010),
  find_closest_color: Ordinal(10190),
  viewport_width: 0xf9e14,
  viewport_height: 0xf9e18,
  viewport_shift: 0x11c418,
};
