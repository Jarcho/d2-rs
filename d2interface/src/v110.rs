use crate::{
  common::{self, EntityKind, InRoom, LinkedList},
  Addresses, FixedU16, FixedU8, IsoPos, LinearPos, Size,
};
use core::ptr::NonNull;

pub use crate::v109d::BASE_ADDRESSES;

pub type EntityTables = common::EntityTables<Entity>;
pub type EntityTable = common::EntityTable<Entity>;

pub const ADDRESSES: Addresses = Addresses {
  player: 0x11c200,
  env_splashes: 0x1076fc,
  env_bubbles: 0x107700,
  client_updates: 0x107768,
  game_type: 0x107960,
  active_entities: 0x11aa00,
  draw_game_fn: 0x107754,
  client_fps_frames: 0x10777c,
  client_total_frames: 0x107764,
  // Signature: `__fastcall(DyPos*, Room*, FixedU16, FixedU16)`
  apply_pos_change: 0x6cc40,
  in_perspective: 0x3b50,
  hwnd: 0x1d270,
  server_update_time: 0x115844,
  draw_menu: 0xd6f0,
};

#[repr(C)]
pub struct Room {
  pub connected: Option<NonNull<NonNull<Room>>>,
  pub _padding1: [u32; 3],
  pub data: u32,
  pub _padding2: [u32; 4],
  pub connected_count: u32,
  pub _padding3: [u32; 9],
  pub pos: LinearPos<u32>,
  pub size: Size<u32>,
  pub _padding4: [u32; 9],
}

#[repr(C)]
pub struct StaticPos {
  pub room: Option<NonNull<Room>>,
  pub iso_pos: IsoPos<i32>,
  pub linear_pos: LinearPos<u32>,
  pub _padding1: [u32; 3],
}

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
  pub mem_pool: u32,
  pub id: u32,
  pub mode: u32,
  pub data: u32,
  pub act_id: u32,
  pub _padding1: [u32; 4],
  pub pos: EntityPos,
  pub _padding2: [u32; 5],
  pub frame: FixedU8,
  pub _padding3: [u32; 3],
  pub gfx_info: u32,
  pub _padding4: [u32; 3],
  pub light: u32,
  pub light_width: u32,
  pub _padding5: [u32; 30],
  pub next_entity: Option<NonNull<Entity>>,
  pub next_in_room: Option<NonNull<Entity>>,
}
impl LinkedList for Entity {
  fn next(&self) -> Option<NonNull<Self>> {
    self.next_entity
  }
}
impl LinkedList<InRoom> for Entity {
  fn next(&self) -> Option<NonNull<Self>> {
    self.next_in_room
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
