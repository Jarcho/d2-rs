use crate::{module::Ordinal::Ordinal, Addresses};

pub use crate::v104b::{DyPos, Entity, EntityTable, EntityTables, Room, StaticPos, BASE_ADDRESSES};

pub const ADDRESSES: Addresses = Addresses {
  player: 0xf6800,
  env_effects: 0xe2dcc,
  game_type: 0xe3028,
  active_entities: 0xf5000,
  client_loop_globals: 0xe2e18,
  // Doesn't exist in this version
  apply_pos_change: 0x0,
  in_perspective: Ordinal(10012),
  hwnd: Ordinal(10029),
  server_update_time: 0xc4674,
  draw_menu: Ordinal(10015),
};
