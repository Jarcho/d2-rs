use crate::all_versions::GameAddresses;

pub use crate::v110::{DyPos, Entity, EntityPos, EntityTable, EntityTables, Room, StaticPos};

pub const ADDRESSES: GameAddresses = GameAddresses {
  player: 0x11c4f0,
  env_splashes: 0x11c340,
  env_bubbles: 0x11c344,
  client_update_count: 0xfb398,
  game_type: 0x11bfbc,
  active_entity_tables: 0,
  draw_game_fn: 0xfb384,
  client_fps_frame_count: 0xfb3ac,
  client_total_frame_count: 0xfb394,
  // Signature: `__stdcall(DyPos* @ esi, FixedU16, FixedU16, Room*)`
  apply_pos_change: 0,
  render_in_perspective: 0x89a0,
  hwnd: 0x1a22c,
  server_update_time: 0x111c04,
  draw_menu: 0xc400,
};
