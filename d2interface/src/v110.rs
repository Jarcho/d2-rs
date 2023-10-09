use crate::{
  module::Ordinal::Ordinal, Addresses, EntityKind, FixedU16, FixedU8, InRoom, IsoPos, LinearPos,
  LinkedList, Rand, Size,
};
use core::ptr::NonNull;

pub use crate::v109d::BASE_ADDRESSES;

pub type EntityTables = crate::EntityTables<Entity>;
pub type EntityTable = crate::EntityTable<Entity>;
pub type GameCursor = crate::GameCursor<Entity>;

pub const ADDRESSES: Addresses = Addresses {
  player: 0x11c200,
  env_effects: 0x1076fc,
  game_type: 0x107960,
  active_entities: 0x11aa00,
  client_loop_globals: 0x107750,
  // Signature: `__fastcall(DyPos*, Room*, FixedU16, FixedU16)`
  apply_pos_change: 0x6cc40,
  in_perspective: Ordinal(10010),
  hwnd: Ordinal(10027),
  server_update_time: 0x115844,
  draw_menu: Ordinal(10019),
  cursor_table: 0xf6b58,
  game_cursor: 0x121aa4,
  summit_cloud_x_pos: 0,
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
  pub mem_pool: *mut (),
  pub id: u32,
  pub state: u32,
  pub data: u32,
  pub act_id: u32,
  pub act: *mut (),
  pub rand: Rand,
  pub seed: u32,
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
  pub _padding6: [u32; 2],
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
