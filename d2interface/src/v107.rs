use crate::{
  module::Ordinal::Ordinal, Addresses, BaseAddresses, EntityKind, FixedU16, IsoPos, LinearPos, Rand,
};
use core::ptr::NonNull;

pub use crate::v100::StaticPos;

pub type EntityTables = crate::EntityTables<Entity>;
pub type EntityTable = crate::EntityTable<Entity>;
pub type GameCursor = crate::GameCursor<Entity>;

pub const ADDRESSES: Addresses = Addresses {
  player: 0x12f2a0,
  env_effects: 0x118ffc,
  game_type: 0x119260,
  active_entities: 0x12daa0,
  client_loop_globals: 0x119050,
  // Doesn't exist in this version
  apply_pos_change: 0,
  in_perspective: Ordinal(10010),
  hwnd: Ordinal(10027),
  server_update_time: 0xed75c,
  draw_menu: Ordinal(10019),
  cursor_table: 0x108160,
  game_cursor: 0x1340d8,
  summit_cloud_x_pos: 0,
};
pub const BASE_ADDRESSES: BaseAddresses = BaseAddresses {
  client: 0x6fad0000,
  common: 0x6fd60000,
  game: 0x6fc60000,
  gfx: 0x6faa0000,
  win: 0x6f900000,
};

#[repr(C)]
pub struct Room {
  pub linear_x: u32,
  pub width: u32,
  pub linear_y: u32,
  pub height: u32,
  pub _padding1: [u32; 5],
  pub connected: *mut *mut Room,
  pub connected_count: u32,
  pub _padding2: [u32; 2],
  pub collision_data: u32,
  pub data: u32,
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
  pub id: u32,
  pub _padding1: [u32; 8],
  pub rand: Rand,
  pub seed: u32,
  pub pos: EntityPos,
  pub _padding2: [u32; 10],
  pub gfx_info: u32,
  pub _padding3: [u32; 8],
  pub light: u32,
  pub light_width: u32,
  pub _padding4: [u32; 30],
  pub next_entity: Option<NonNull<Entity>>,
}
impl crate::LinkedList for Entity {
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
