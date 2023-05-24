use crate::{
  module::Ordinal::Ordinal, Addresses, EntityKind, FixedU16, IsoPos, LinearPos, LinkedList,
};
use core::ptr::NonNull;

pub use crate::v105::{Room, StaticPos, BASE_ADDRESSES};

pub const ADDRESSES: Addresses = Addresses {
  player: 0xf6768,
  env_splashes: 0xe2dcc,
  env_bubbles: 0xe2dd0,
  client_updates: 0xe2e30,
  game_type: 0xe3028,
  active_entities: 0xf4f68,
  draw_game_fn: 0xe2e1c,
  client_fps_frames: 0xe2e44,
  client_total_frames: 0xe2e2c,
  // Doesn't exist in this version
  apply_pos_change: 0x0,
  in_perspective: Ordinal(10012),
  hwnd: Ordinal(10029),
  server_update_time: 0xc461c,
  draw_menu: Ordinal(10015),
};

#[repr(C)]
pub struct DyPos {
  pub linear_pos: LinearPos<FixedU16>,
  pub iso_pos: IsoPos<i32>,
  pub target_pos: [LinearPos<u16>; 3],
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
  pub _padding1: [u32; 9],
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
