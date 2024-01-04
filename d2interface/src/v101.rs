use crate::{module::Ordinal::Ordinal, Addresses};

pub use crate::v100::{
  DyPos, Entity, EntityTable, EntityTables, GameCursor, Room, StaticPos, BASE_ADDRESSES,
};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x145190,
  env_effects: 0x12e994,
  game_type: 0x12ec08,
  entity_table: 0x143990,
  entity_table2: 0x144590,
  client_loop_globals: 0x12ea18,
  // Doesn't exist in this version
  apply_pos_change: 0x0,
  in_perspective: Ordinal(10012),
  hwnd: Ordinal(10029),
  server_update_time: 0xbe6e4,
  draw_menu: Ordinal(10015),
  cursor_table: 0x11ae00,
  game_cursor: 0x168148,
  summit_cloud_x_pos: 0,
  draw_line: Ordinal(10061),
  find_closest_color: Ordinal(10030),
  viewport_width: 0,
  viewport_height: 0,
  viewport_shift: 0x1346d4,
  max_weather_particles: 0x12e910,
  weather_angle: 0x12e8fc,
  rain_speed: 0,
  is_snowing: 0,
  sine_table: 0x27b6c,
  // Signature: fastcall(&mut Rng)
  gen_weather_particle: 0xd5d0,
  env_array_remove: Ordinal(10044),
};

pub mod dtbl {
  pub use crate::v100::dtbl::*;
  use crate::StrId;

  #[repr(C)]
  pub struct LqItemDef {
    pub name: [u8; 32],
    pub display_name: StrId,
  }
}
