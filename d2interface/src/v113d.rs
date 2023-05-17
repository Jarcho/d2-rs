use crate::all_versions::{GameAddresses};

pub use crate::v113c::{DyPos, Entity, EntityPos, Room, StaticPos, EntityTable, EntityTables};

pub static ADDRESSES: GameAddresses = GameAddresses {
  player: 0x11d050,
  env_splashes: 0x11d080,
  env_bubbles: 0x11d084,
  client_update_count: 0x108758,
  game_type: 0x11d1dc,
  active_entity_tables: 0x1047b8,
  draw_game_fn: 0x108744,
  client_fps_frame_count: 0x10876c,
  client_total_frame_count: 0x108754,
  // Signature: `__stdcall(DyPos* @ esi, FixedU16, FixedU16, Room*)`
  apply_pos_change: 0x6ee00,
  render_in_perspective: 0xa8b0,
  hwnd: 0x14a44,
  server_update_time: 0x111c30,
  draw_menu: 0xeb30,
};
