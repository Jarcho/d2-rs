use crate::{module::Ordinal::Ordinal, Addresses};

pub use crate::v113c::{
  dtbl, DyPos, Entity, EntityPos, EntityTable, EntityTables, GameCursor, Room, StaticPos,
  BASE_ADDRESSES,
};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x11d050,
  env_effects: 0x11d080,
  game_type: 0x11d1dc,
  entity_table: 0x1047b8,
  entity_table2: 0x103bb8,
  client_loop_globals: 0x108740,
  // Signature: `__stdcall(DyPos* @ esi, FixedU16, FixedU16, Room*)`
  apply_pos_change: 0x6ee00,
  in_perspective: Ordinal(10037),
  hwnd: Ordinal(10007),
  server_update_time: 0x111c30,
  draw_menu: Ordinal(10127),
  cursor_table: 0xd9a30,
  game_cursor: 0xfc95c,
  summit_cloud_x_pos: 0,
  draw_line: Ordinal(10013),
  find_closest_color: Ordinal(10069),
  viewport_width: 0x123d64,
  viewport_height: 0x123d60,
  viewport_shift: 0x11d074,
  max_weather_particles: 0x10870c,
  weather_angle: 0x1086f0,
  rain_speed: 0x1086cc,
  is_snowing: 0x11d098,
  sine_table: 0x2f078,
  // Signature: stdcall(&mut Rng)
  gen_weather_particle: 0x4b350,
  env_array_remove: Ordinal(10065),
};
