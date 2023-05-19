use crate::all_versions::GameAddresses;

pub use crate::v111::{DyPos, Entity, EntityPos, EntityTable, EntityTables, Room, StaticPos};

pub const ADDRESSES: GameAddresses = GameAddresses {
  player: 0x11c1e0,
  env_splashes: 0x11c518,
  env_bubbles: 0x11c51c,
  client_update_count: 0x11a298,
  game_type: 0x11c2ac,
  active_entity_tables: 0,
  draw_game_fn: 0x11a284,
  client_fps_frame_count: 0x11a2ac,
  client_total_frame_count: 0x11a294,
  // Signature: `__stdcall(DyPos* @ esi, FixedU16, FixedU16, Room*)`
  apply_pos_change: 0,
  render_in_perspective: 0xb2a0,
  hwnd: 0x14890,
  server_update_time: 0x111c00,
  draw_menu: 0x13d20,
};
