use core::ptr::NonNull;

use crate::{module::Ordinal::Address, Addresses, BaseAddresses, CursorId, CursorState, FU8};

pub use crate::v113d::{dtbl, DyPos, Entity, EntityTable, EntityTables, Room, StaticPos};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x440df0,
  env_effects: 0x432960,
  game_type: 0x42e1b0,
  entity_table: 0,
  entity_table2: 0,
  client_loop_globals: 0x42e020,
  // Signature: `__stdcall(DyPos* @ eax, FixedU16, FixedU16, Room*)`
  apply_pos_change: 0,
  in_perspective: Address(0x5370),
  hwnd: Address(0x5b50),
  server_update_time: 0x497d38,
  draw_menu: Address(0x3cdd0),
  cursor_table: 0,
  game_cursor: 0,
  summit_cloud_x_pos: 0,
  draw_line: Address(0),
  find_closest_color: Address(0),
  viewport_width: 0,
  viewport_height: 0,
  viewport_shift: 0,
  max_weather_particles: 0x432944,
  weather_angle: 0x432928,
  rain_speed: 0x432904,
  is_snowing: 0x432978,
  sine_table: 0x307af8,
  // Signature: (&mut Rng @ eax)
  gen_weather_particle: 0x59120,
  env_array_remove: Address(0x2c0b20),
};
pub const BASE_ADDRESSES: BaseAddresses = BaseAddresses {
  client: 0x00400000,
  common: 0x00400000,
  fog: 0x00400000,
  game: 0x00400000,
  gfx: 0x00400000,
  win: 0x00400000,
};

#[repr(C)]
pub struct GameCursor {
  pub item: Option<NonNull<Entity>>,
  pub dc6_files: [usize; 7],
  pub id: CursorId,
  pub frame: FU8,
  pub _padding: u32,
  pub last_move_time: u32,
  pub last_update_time: u32,
  pub state: CursorState,
}
