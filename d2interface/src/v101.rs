use crate::{module::Ordinal::Ordinal, Addresses};

pub use crate::v100::{DyPos, Entity, EntityTable, EntityTables, Room, StaticPos, BASE_ADDRESSES};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x145190,
  env_effects: 0x12e994,
  game_type: 0x12ec08,
  active_entities: 0x143990,
  client_loop_globals: 0x12ea18,
  // Doesn't exist in this version
  apply_pos_change: 0x0,
  in_perspective: Ordinal(10012),
  hwnd: Ordinal(10029),
  server_update_time: 0xbe6e4,
  draw_menu: Ordinal(10015),
};
