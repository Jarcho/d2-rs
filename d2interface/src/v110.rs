use crate::{
  all_versions::{self, EntityKind, GameType, InRoom, LinkedList},
  FixedU16, FixedU8, IsoPos, LinearPos, Size,
};
use core::ptr::NonNull;
use windows_sys::Win32::Foundation::{HMODULE, HWND};

pub use crate::v109d::{EnvArray, EnvImage};

pub type EntityTables = all_versions::EntityTables<Entity>;
pub type EntityTable = all_versions::EntityTable<Entity>;

decl_accessor! { D2ClientAccessor {
  /// Pointer to the current player. May exist even when not in-game.
  player: NonNull<Option<NonNull<Entity>>> = 0x11c200,
  /// The array containing the active splash effects (Acts 1&3 rain).
  env_splashes: NonNull<Option<NonNull<EnvArray<EnvImage>>>> = 0x1076fc,
  /// The array containing the active bubble effects.
  env_bubbles: NonNull<Option<NonNull<EnvArray<EnvImage>>>> = 0x107700,
  /// The number of times the client has updated the game state.
  client_update_count: NonNull<u32> = 0x107768,
  /// The type of game the client is currently running. Only meaningful if a
  /// game is running.
  game_type: NonNull<GameType> = 0x107960,
  /// The table of active game entities.
  active_entity_tables: NonNull<EntityTables> = 0x11aa00,
  /// The currently selected draw function.
  draw_game_fn: NonNull<unsafe extern "fastcall" fn(u32)> = 0x107754,
  /// Frame count used to calculate the client's current fps.
  client_fps_frame_count: NonNull<u32> = 0x10777c,
  /// The total number of frames drawn by the client.
  client_frame_count: NonNull<u32> = 0x107764,
}}

decl_accessor! { D2CommonAccessor {
  /// Applies a position change to a `DyPos`.
  apply_pos_change: unsafe extern "fastcall" fn(NonNull<DyPos>, NonNull<Room>, FixedU16, FixedU16) = 0x6cc40,
}}

decl_accessor! { D2GfxAccessor {
  /// Whether the game is being rendered in perspective mode.
  render_in_perspective: unsafe extern "stdcall" fn() -> u32 = 0x3b50,
  /// The game's window handle
  hwnd: NonNull<HWND> = 0x1d270,
}}

decl_accessor! { D2GameAccessor {
  /// The time the game server most recently updated the game state.
  server_update_time: NonNull<u32> = 0x115844,
}}

decl_accessor! { D2WinAccessor {
  /// Draw the game's current menu.
  draw_menu: unsafe extern "stdcall" fn() = 0xd6f0,
}}

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
