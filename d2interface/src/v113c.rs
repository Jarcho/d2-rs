use crate::all_versions::{self, GameType};
use core::ptr::NonNull;
use windows_sys::Win32::Foundation::{HMODULE, HWND};

pub use crate::v110::{DyPos, Entity, EntityPos, EnvArray, EnvImage, Room, StaticPos};

pub type EntityTables = all_versions::EntityTables<Entity>;
pub type EntityTable = all_versions::EntityTable<Entity>;

decl_accessor! { D2ClientAccessor {
  /// Pointer to the current player. May exist even when not in-game.
  player: NonNull<Option<NonNull<Entity>>> = 0x11bbfc,
  /// The array containing the active splash effects (Acts 1&3 rain).
  env_splashes: NonNull<Option<NonNull<EnvArray<EnvImage>>>> = 0x11bf60,
  /// The array containing the active bubble effects.
  env_bubbles: NonNull<Option<NonNull<EnvArray<EnvImage>>>> = 0x11bf64,
  /// The number of times the client has updated the game state.
  client_update_count: NonNull<u32> = 0x1197f8,
  /// The type of game the client is currently running. Only meaningful if a
  /// game is running.
  game_type: NonNull<GameType> = 0x11c394,
  /// The table of active game entities.
  active_entity_tables: NonNull<EntityTables> = 0x10a608,
  /// The currently selected draw function.
  draw_game_fn: NonNull<unsafe extern "fastcall" fn(u32)> = 0x1197e4,
  /// Frame count used to calculate the client's current fps.
  client_fps_frame_count: NonNull<u32> = 0x11980c,
  /// The total number of frames drawn by the client.
  client_frame_count: NonNull<u32> = 0x1197f4,
}}

decl_accessor! { D2CommonAccessor {
  /// Applies a position change to a `DyPos`.
  /// Signature: `__stdcall(DyPos* @ esi, FixedU16, FixedU16, Room*)`
  apply_pos_change: usize = 0xda40,
}}

decl_accessor! { D2GfxAccessor {
  /// Whether the game is being rendered in perspective mode.
  render_in_perspective: unsafe extern "stdcall" fn() -> u32 = 0xb290,
  /// The game's window handle
  hwnd: NonNull<HWND> = 0x11264,
}}

decl_accessor! { D2GameAccessor {
  /// The time the game server most recently updated the game state.
  server_update_time: NonNull<u32> = 0x111c44,
}}

decl_accessor! { D2WinAccessor {
  /// Draw the game's current menu.
  draw_menu: unsafe extern "stdcall" fn() = 0x187e0,
}}
