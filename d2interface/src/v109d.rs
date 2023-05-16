use core::{ptr::NonNull, slice};

use windows_sys::Win32::Foundation::{HMODULE, HWND};

use crate::{
  all_versions::{self, EntityKind, GameType, LinkedList},
  FixedU16, IsoPos, LinearPos, UnknownPos,
};

pub type EntityTables = all_versions::EntityTables<Entity>;
pub type EntityTable = all_versions::EntityTable<Entity>;

decl_accessor! { D2ClientAccessor {
  /// Pointer to the current player. May exist even when not in-game.
  player: NonNull<Option<NonNull<Entity>>> = 0x1263f8,
  /// The array containing the active splash effects (Acts 1&3 rain).
  env_splashes: NonNull<Option<NonNull<EnvArray<EnvImage>>>> = 0x11095c,
  /// The array containing the active bubble effects.
  env_bubbles: NonNull<Option<NonNull<EnvArray<EnvImage>>>> = 0x110960,
  /// The number of times the client has updated the game state.
  client_update_count: NonNull<u32> = 0x1109c8,
  /// The type of game the client is currently running. Only meaningful if a
  /// game is running.
  game_type: NonNull<GameType> = 0x110bc0,
  /// The table of active game entities.
  active_entity_tables: NonNull<EntityTables> = 0x124bf8,
  /// The currently selected draw function.
  draw_game_fn: NonNull<unsafe extern "fastcall" fn(u32)> = 0x1109b4,
  /// Frame count used to calculate the client's current fps.
  client_fps_frame_count: NonNull<u32> = 0x1109dc,
  /// The total number of frames drawn by the client.
  client_frame_count: NonNull<u32> = 0x1109c4,
}}

decl_accessor! { D2CommonAccessor {
  /// Applies a position change to a `DyPos`.
  apply_pos_change: unsafe extern "fastcall" fn(NonNull<DyPos>, NonNull<Room>, FixedU16, FixedU16) = 0x5f180,
}}

decl_accessor! { D2GfxAccessor {
  /// Whether the game is being rendered in perspective mode.
  render_in_perspective: unsafe extern "stdcall" fn() -> u32 = 0x3b60,
  /// The game's window handle
  hwnd: NonNull<HWND> = 0x1d214,
}}

decl_accessor! { D2GameAccessor {
  /// The time the game server most recently updated the game state.
  server_update_time: NonNull<u32> = 0xf4198,
}}

decl_accessor! { D2WinAccessor {
  /// Draw the game's current menu.
  draw_menu: unsafe extern "stdcall" fn() = 0xf290,
}}

pub mod d2gfx_offsets {
  pub const PLAYER: u32 = 0x3a6a70;
  pub const GFX_FNS: u32 = 0x3c8cc0;
  pub const WINDOW_HANDLE: u32 = 0x3c8cbc;

  pub const DRAW_IMAGE: u32 = 0xf6480;
  pub const DRAW_IMAGE_SHIFTED: u32 = 0xf64b0;
  pub const DRAW_IMAGE_VCROPPED: u32 = 0xf64e0;
  pub const DRAW_IMAGE_CROPPED: u32 = 0xf6510;
}

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

#[repr(C)]
pub struct EnvArray<T> {
  pub name: [u8; 0x100],
  pub len: u32,
  pub element_size: u32,
  pub initialized: u32,
  pub next_free_idx: u32,
  pub last_active_idx: i32,
  pub active_count: u32,
  pub _padding: u16,
  pub data: NonNull<T>,
}
impl<T> EnvArray<T> {
  pub fn as_mut_slice(&mut self) -> &mut [T] {
    unsafe {
      slice::from_raw_parts_mut(
        self.data.as_ptr(),
        if self.initialized != 0 {
          (self.last_active_idx + 1) as usize
        } else {
          0
        },
      )
    }
  }
}

#[repr(C)]
pub struct EnvImage {
  pub active: u16,
  /// Linear space when rendering in perspective. Camera space when not.
  pub pos: UnknownPos<u32>,
  pub file_idx: u32,
  pub frame: u32,
  pub till_next_frame: u32,
}
