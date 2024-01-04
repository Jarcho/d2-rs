use crate::{module::Ordinal::Address, Addresses};

pub use crate::v114a::{
  dtbl, DyPos, Entity, EntityTable, EntityTables, GameCursor, Room, StaticPos, BASE_ADDRESSES,
};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x39eaf8,
  env_effects: 0x3a0a84,
  game_type: 0x398698,
  entity_table: 0,
  entity_table2: 0,
  client_loop_globals: 0x398508,
  // Signature: `__stdcall(DyPos* @ eax, FixedU16, FixedU16, Room*)`
  apply_pos_change: 0,
  in_perspective: Address(0xf2770),
  hwnd: Address(0xf2f50),
  server_update_time: 0x47bd98,
  draw_menu: Address(0xf6f30),
  cursor_table: 0,
  game_cursor: 0,
  summit_cloud_x_pos: 0,
  draw_line: Address(0),
  find_closest_color: Address(0),
  viewport_width: 0,
  viewport_height: 0,
  viewport_shift: 0,
  max_weather_particles: 0x3a0a68,
  weather_angle: 0x3a0a4c,
  rain_speed: 0x3a0a28,
  is_snowing: 0x3a0a9c,
  sine_table: 0x3078b8,
  // Signature: (&mut Rng @ eax)
  gen_weather_particle: 0x6ecb0,
  env_array_remove: Address(0x2be340),
};
