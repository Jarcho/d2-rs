use crate::{
  module::Ordinal::Ordinal, Addresses, EntityKind, IsoP2d, LinearM2d, LinkedList, Rng, FU16,
};
use core::ptr::NonNull;

pub use crate::v105::{dtbl, Room, StaticPos, BASE_ADDRESSES};

pub type EntityTables = crate::EntityTables<Entity>;
pub type EntityTable = crate::EntityTable<Entity>;
pub type GameCursor = crate::GameCursor<Entity>;

pub const ADDRESSES: Addresses = Addresses {
  player: 0xf6768,
  env_effects: 0xe2dcc,
  game_type: 0xe3028,
  entity_table: 0xf4f68,
  entity_table2: 0xf5b68,
  client_loop_globals: 0xe2e18,
  // Doesn't exist in this version
  apply_pos_change: 0x0,
  in_perspective: Ordinal(10012),
  hwnd: Ordinal(10029),
  server_update_time: 0xc461c,
  draw_menu: Ordinal(10015),
  cursor_table: 0xd19d8,
  game_cursor: 0xfb4c8,
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
  sine_table: 0x201fc,
  // Signature: fastcall(&mut Rng)
  gen_weather_particle: 0x62a0,
  env_array_remove: Ordinal(10044),
};

#[repr(C)]
pub struct DyPos {
  pub linear_pos: LinearM2d<FU16>,
  pub iso_pos: IsoP2d<i32>,
  pub target_pos: [LinearM2d<u16>; 3],
  pub room: Option<NonNull<Room>>,
  pub _padding1: [u32; 4],
  pub entity: NonNull<Entity>,
}

#[repr(C)]
pub union EntityPos {
  pub s: Option<NonNull<StaticPos>>,
  pub d: Option<NonNull<DyPos>>,
}

#[repr(C)]
pub struct Entity {
  pub kind: EntityKind,
  pub class_id: u32,
  pub id: u32,
  pub _padding1: [u32; 6],
  pub rng: Rng,
  pub seed: u32,
  pub pos: EntityPos,
  pub _padding2: [u32; 51],
  pub next_entity: Option<NonNull<Entity>>,
}
impl LinkedList for Entity {
  fn next(&self) -> Option<NonNull<Self>> {
    self.next_entity
  }
}
impl Entity {
  pub fn pos<T>(
    &self,
    static_fn: impl FnOnce(&StaticPos) -> T,
    dy_fn: impl FnOnce(&DyPos) -> T,
  ) -> Option<T> {
    unsafe {
      match self.kind {
        EntityKind::Pc | EntityKind::Npc | EntityKind::Missile => {
          self.pos.d.map(|pos| dy_fn(pos.as_ref()))
        }
        EntityKind::Object | EntityKind::Item | EntityKind::Tile => {
          self.pos.s.map(|pos| static_fn(pos.as_ref()))
        }
        _ => None,
      }
    }
  }

  pub fn has_room(&self) -> bool {
    self
      .pos(|pos| pos.room.is_some(), |pos| pos.room.is_some())
      .unwrap_or(false)
  }
}
