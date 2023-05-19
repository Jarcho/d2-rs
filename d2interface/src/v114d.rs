use crate::all_versions::GameAddresses;

pub use crate::v114c::{DyPos, Entity, EntityTable, EntityTables, Room, StaticPos};

pub const ADDRESSES: GameAddresses = GameAddresses {
  player: 0x3a6a70,
  env_splashes: 0x3a89fc,
  env_bubbles: 0x3a8a00,
  client_update_count: 0x3a0498,
  game_type: 0x3a0610,
  active_entity_tables: 0x3a5e70,
  draw_game_fn: 0x3a0484,
  client_fps_frame_count: 0x3a04ac,
  client_total_frame_count: 0x3a0494,
  // Signature: `__stdcall(DyPos* @ eax, FixedU16, FixedU16, Room*)`
  apply_pos_change: 0x24fb90,
  render_in_perspective: 0xf51d0,
  hwnd: 0x3c8cbc,
  server_update_time: 0x483d58,
  draw_menu: 0xf98e0,
};
