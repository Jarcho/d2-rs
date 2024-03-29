use crate::{module::Ordinal::Ordinal, Addresses};

pub use crate::v111b::{
  dtbl, DyPos, Entity, EntityPos, EntityTable, EntityTables, GameCursor, Room, StaticPos,
  BASE_ADDRESSES,
};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x11c3d0,
  env_effects: 0x11c3e0,
  game_type: 0x11bff8,
  entity_table: 0x11a960,
  entity_table2: 0x119d60,
  client_loop_globals: 0x103298,
  // Signature: `__stdcall(DyPos* @ esi, FixedU16, FixedU16, Room*)`
  apply_pos_change: 0x4af00,
  in_perspective: Ordinal(10071),
  hwnd: Ordinal(10078),
  server_update_time: 0x111c34,
  draw_menu: Ordinal(10094),
  cursor_table: 0xbd7b8,
  game_cursor: 0xe1644,
  summit_cloud_x_pos: 0,
  draw_line: Ordinal(10031),
  find_closest_color: Ordinal(10097),
  viewport_width: 0x11d528,
  viewport_height: 0x11d524,
  viewport_shift: 0x11c1d4,
  max_weather_particles: 0x11b7a8,
  weather_angle: 0x11b78c,
  rain_speed: 0x11b768,
  is_snowing: 0x11c3f8,
  sine_table: 0x2f788,
  // Signature: stdcall(&mut Rng)
  gen_weather_particle: 0x147d0,
  env_array_remove: Ordinal(10065),
};
