use crate::{
  common, module::Ordinal::Ordinal, Addresses, BaseAddresses, EntityKind, FixedU16, IsoPos,
  LinearPos,
};
use core::ptr::NonNull;

pub use crate::v100::StaticPos;

pub type EntityTables = common::EntityTables<Entity>;
pub type EntityTable = common::EntityTable<Entity>;

pub const ADDRESSES: Addresses = Addresses {
  player: 0x1263f8,
  env_splashes: 0x11095c,
  env_bubbles: 0x110960,
  client_updates: 0x1109c8,
  game_type: 0x110bc0,
  active_entities: 0x124bf8,
  draw_game_fn: 0x1109b4,
  client_fps_frames: 0x1109dc,
  client_total_frames: 0x1109c4,
  // Signature: `__fastcall(DyPos*, Room*, FixedU16, FixedU16)`
  apply_pos_change: 0xf290,
  in_perspective: Ordinal(10010),
  hwnd: Ordinal(10027),
  server_update_time: 0xf4198,
  draw_menu: Ordinal(10019),
};
pub const BASE_ADDRESSES: BaseAddresses = BaseAddresses {
  client: 0x6faa0000,
  common: 0x6fd40000,
  game: 0x6fc30000,
  gfx: 0x6fa70000,
  win: 0x6f8a0000,
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
  pub _padding1: [u32; 11],
  pub pos: EntityPos,
  pub _padding2: [u32; 10],
  pub gfx_info: u32,
  pub _padding3: [u32; 8],
  pub light: u32,
  pub light_width: u32,
  pub _padding4: [u32; 30],
  pub next_entity: Option<NonNull<Entity>>,
}
impl common::LinkedList for Entity {
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
