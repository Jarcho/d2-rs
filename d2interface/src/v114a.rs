use crate::all_versions::GameAddresses;

pub use crate::v113d::{DyPos, Entity, EntityTable, EntityTables, Room, StaticPos};

pub static ADDRESSES: GameAddresses = GameAddresses {
  player: 0x440df0,
  env_splashes: 0x432960,
  env_bubbles: 0x432964,
  client_update_count: 0x42e038,
  game_type: 0x42e1b0,
  active_entity_tables: 0x3a5e70,
  draw_game_fn: 0x42e024,
  client_fps_frame_count: 0x42e04c,
  client_total_frame_count: 0x42e034,
  // Signature: `__stdcall(DyPos* @ eax, FixedU16, FixedU16, Room*)`
  apply_pos_change: 0,
  render_in_perspective: 0x5370,
  hwnd: 0x348264,
  server_update_time: 0x497d38,
  draw_menu: 0x3cdd0,
};
