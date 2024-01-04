use crate::{module::Ordinal::Ordinal, Addresses};

pub use crate::v101::{
  dtbl, DyPos, Entity, EntityTable, EntityTables, GameCursor, Room, StaticPos, BASE_ADDRESSES,
};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x1450e0,
  env_effects: 0x12e8dc,
  game_type: 0x12eb68,
  entity_table: 0x1438e0,
  entity_table2: 0x1444e0,
  client_loop_globals: 0x12e958,
  // Doesn't exist in this version
  apply_pos_change: 0x0,
  in_perspective: Ordinal(10012),
  hwnd: Ordinal(10029),
  server_update_time: 0xbf864,
  draw_menu: Ordinal(10015),
  cursor_table: 0x11ad70,
  game_cursor: 0x168148,
  summit_cloud_x_pos: 0,
  draw_line: Ordinal(10061),
  find_closest_color: Ordinal(10030),
  viewport_width: 0,
  viewport_height: 0,
  viewport_shift: 0x134634,
  max_weather_particles: 0x12e858,
  weather_angle: 0x12e844,
  rain_speed: 0x12e830,
  is_snowing: 0,
  sine_table: 0x28b6c,
  // Signature: fastcall(&mut Rng)
  gen_weather_particle: 0xd300,
  env_array_remove: Ordinal(10044),
};
