use crate::{module::Ordinal::Address, Addresses};

pub use crate::v114b::{
  dtbl, DyPos, Entity, EntityTable, EntityTables, GameCursor, Room, StaticPos, BASE_ADDRESSES,
};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x39daf8,
  env_effects: 0x39fa84,
  game_type: 0x397698,
  entity_table: 0x39cef8,
  entity_table2: 0x39c2f8,
  client_loop_globals: 0x397508,
  // Signature: `__stdcall(DyPos* @ eax, FixedU16, FixedU16, Room*)`
  apply_pos_change: 0x250bc0,
  in_perspective: Address(0xf2770),
  hwnd: Address(0xf2f50),
  server_update_time: 0x47ace0,
  draw_menu: Address(0xf6f30),
  cursor_table: 0x310990,
  game_cursor: 0x39db44,
  summit_cloud_x_pos: 0x3acc70,
  draw_line: Address(0xf3990),
  find_closest_color: Address(0xf8760),
  viewport_width: 0x39c2a8,
  viewport_height: 0x39c2a4,
  viewport_shift: 0x39c29c,
  max_weather_particles: 0x39fa68,
  weather_angle: 0x39fa4c,
  rain_speed: 0x39fa28,
  is_snowing: 0x39fa9c,
  sine_table: 0x3068b8,
  // Signature: (&mut Rng @ eax)
  gen_weather_particle: 0x6ecb0,
  env_array_remove: Address(0x2bdf20),
};
