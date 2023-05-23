use crate::{
  module::Ordinal::Ordinal, Addresses, BaseAddresses, EntityKind, FixedU16, IsoPos, LinearPos,
  LinkedList,
};
use core::ptr::NonNull;

pub type EntityTables = crate::EntityTables<Entity>;
pub type EntityTable = crate::EntityTable<Entity>;

pub const ADDRESSES: Addresses = Addresses {
  player: 0x1451b8,
  env_splashes: 0x12eb64,
  env_bubbles: 0x12eb68,
  client_updates: 0x12ec08,
  game_type: 0x12ede0,
  active_entities: 0x1439b8,
  draw_game_fn: 0x12ebf4,
  client_fps_frames: 0x12ec1c,
  client_total_frames: 0x12ec04,
  // Doesn't exist in this version
  apply_pos_change: 0x0,
  in_perspective: Ordinal(10012),
  hwnd: Ordinal(10029),
  server_update_time: 0xbe75c,
  draw_menu: Ordinal(10015),
};
pub const BASE_ADDRESSES: BaseAddresses = BaseAddresses {
  client: 0x10000000,
  common: 0x10000000,
  game: 0x10000000,
  gfx: 0x10000000,
  win: 0x10000000,
};

#[repr(C)]
pub struct Room {}

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
pub struct StaticPos {
  pub iso_pos: IsoPos<i32>,
  pub linear_pos: LinearPos<u32>,
  pub _padding1: [u32; 3],
  pub room: Option<NonNull<Room>>,
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
  pub _padding1: [u32; 10],
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
