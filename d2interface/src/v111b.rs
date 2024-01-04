use crate::{module::Ordinal::Ordinal, Addresses};

pub use crate::v111::{
  dtbl, DyPos, Entity, EntityPos, EntityTable, EntityTables, GameCursor, Room, StaticPos,
  BASE_ADDRESSES,
};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x11c1e0,
  env_effects: 0x11c518,
  game_type: 0x11c2ac,
  entity_table: 0x10b470,
  entity_table2: 0x10a870,
  client_loop_globals: 0x11a280,
  // Signature: `__stdcall(DyPos* @ esi, FixedU16, FixedU16, Room*)`
  apply_pos_change: 0x7c100,
  in_perspective: Ordinal(10046),
  hwnd: Ordinal(10022),
  server_update_time: 0x111c00,
  draw_menu: Ordinal(10129),
  cursor_table: 0xd4a30,
  game_cursor: 0xfb424,
  summit_cloud_x_pos: 0,
  draw_line: Ordinal(10001),
  find_closest_color: Ordinal(10070),
  viewport_width: 0x123394,
  viewport_height: 0x123390,
  viewport_shift: 0x11c3e8,
  max_weather_particles: 0x11b94c,
  weather_angle: 0x11b930,
  rain_speed: 0x11b90c,
  is_snowing: 0x11c530,
  sine_table: 0x2f518,
  // Signature: stdcall(&mut Rng)
  gen_weather_particle: 0x12d70,
  env_array_remove: Ordinal(10065),
};
