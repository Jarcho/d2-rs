use crate::{module::Ordinal::Ordinal, Addresses};

pub use crate::v104b::{
  dtbl, DyPos, Entity, EntityTable, EntityTables, GameCursor, Room, StaticPos, BASE_ADDRESSES,
};

pub const ADDRESSES: Addresses = Addresses {
  player: 0xf6800,
  env_effects: 0xe2dcc,
  game_type: 0xe3028,
  entity_table: 0xf5000,
  entity_table2: 0xf5c00,
  client_loop_globals: 0xe2e18,
  // Doesn't exist in this version
  apply_pos_change: 0x0,
  in_perspective: Ordinal(10012),
  hwnd: Ordinal(10029),
  server_update_time: 0xc4674,
  draw_menu: Ordinal(10015),
  cursor_table: 0xd19d8,
  game_cursor: 0xfb560,
  summit_cloud_x_pos: 0,
  draw_line: Ordinal(10061),
  find_closest_color: Ordinal(10030),
  viewport_width: 0,
  viewport_height: 0,
  viewport_shift: 0xe7b4c,
  max_weather_particles: 0xe2d48,
  weather_angle: 0xe2d34,
  rain_speed: 0xe2d20,
  is_snowing: 0,
  sine_table: 0x1f1e4,
  // Signature: fastcall(&mut Rng)
  gen_weather_particle: 0x62a0,
  env_array_remove: Ordinal(10044),
};
