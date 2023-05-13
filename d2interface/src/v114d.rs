use crate::all_versions::{self, GameType};
use core::ptr::NonNull;
use windows_sys::Win32::Foundation::{HMODULE, HWND};

pub use crate::v110::{DyPos, Entity, EnvArray, EnvImage, Room, StaticPos};

pub type EntityTables = all_versions::EntityTables<Entity>;
pub type EntityTable = all_versions::EntityTable<Entity>;

decl_accessor! { GameAccessor {
  /// Pointer to the current player. May exist even when not in-game.
  player: NonNull<Option<NonNull<Entity>>> = 0x3a6a70,
  /// The array containing the active splash effects (Acts 1&3 rain).
  env_splashes: NonNull<Option<NonNull<EnvArray<EnvImage>>>> = 0x3a89fc,
  /// The array containing the active bubble effects.
  env_bubbles: NonNull<Option<NonNull<EnvArray<EnvImage>>>> = 0x3a8a00,
  /// The number of times the client has updated the game state.
  client_update_count: NonNull<u32> = 0x3a0498,
  /// The type of game the client is currently running. Only meaningful if a
  /// game is running.
  game_type: NonNull<GameType> = 0x3a0610,
  /// The table of active game entities.
  active_entity_tables: NonNull<EntityTables> = 0x3a5e70,
  /// The currently selected draw function.
  draw_game_fn: NonNull<unsafe extern "fastcall" fn(u32)> = 0x3a0484,
  /// Frame count used to calculate the client's current fps.
  client_fps_frame_count: NonNull<u32> = 0x3a04ac,
  /// The total number of frames drawn by the client.
  client_frame_count: NonNull<u32> = 0x3a0494,
  /// Whether the game is rendered in perspective mode.
  render_in_perspective: unsafe extern "stdcall" fn() -> u32 = 0xf51d0,
  /// The game's window handle
  hwnd: NonNull<HWND> = 0x3c8cbc,
  /// The time the game server most recently updated the game state.
  server_update_time: NonNull<u32> = 0x483d58,
  /// Draw the game's current menu.
  draw_menu: unsafe extern "stdcall" fn() = 0xf98e0,
}}
