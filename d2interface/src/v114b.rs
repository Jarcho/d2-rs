use crate::all_versions::GameAddresses;

pub use crate::v114a::{DyPos, Entity, EntityTable, EntityTables, Room, StaticPos};

pub static ADDRESSES: GameAddresses = GameAddresses {
  player: 0x39eaf8,
  env_splashes: 0x3a0a84,
  env_bubbles: 0x3a0a88,
  client_update_count: 0x398520,
  game_type: 0x398698,
  active_entity_tables: 0,
  draw_game_fn: 0x39850c,
  client_fps_frame_count: 0x398534,
  client_total_frame_count: 0x39851c,
  // Signature: `__stdcall(DyPos* @ eax, FixedU16, FixedU16, Room*)`
  apply_pos_change: 0,
  render_in_perspective: 0xf2770,
  hwnd: 0x3c0d44,
  server_update_time: 0x47bd98,
  draw_menu: 0xf6f30,
};
