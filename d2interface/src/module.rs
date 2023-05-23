use crate::{EnvImages, GameType};
use core::{fmt, mem::transmute, ops::Index, ptr::NonNull};
use windows_sys::Win32::{
  Foundation::{HMODULE, HWND},
  System::LibraryLoader::GetProcAddress,
};

#[derive(Default, Clone, Copy)]
#[repr(transparent)]
pub struct Client(pub HMODULE);
#[derive(Default, Clone, Copy)]
#[repr(transparent)]
pub struct Common(pub HMODULE);
#[derive(Default, Clone, Copy)]
#[repr(transparent)]
pub struct Game(pub HMODULE);
#[derive(Default, Clone, Copy)]
#[repr(transparent)]
pub struct Gfx(pub HMODULE);
#[derive(Default, Clone, Copy)]
#[repr(transparent)]
pub struct Win(pub HMODULE);

pub(crate) enum Ordinal {
  Ordinal(u16),
  Address(usize),
}

macro_rules! decl_addresses_ty {
  () => {
    usize
  };
  (ordinal) => {
    Ordinal
  };
}
macro_rules! decl_addresses_init {
  () => {
    0usize
  };
  (ordinal) => {
    Ordinal::Address(0usize)
  };
}
macro_rules! decl_addresses_impl {
  ($(#[$meta:meta])* $module:ident::$item:ident: $ty:ty) => {
    $(#[$meta])*
    #[allow(clippy::missing_safety_doc, clippy::useless_transmute)]
    pub unsafe fn $item(&self, m: $module) -> $ty {
      transmute(self.$item.wrapping_add(m.0 as usize))
    }
  };
  ($(#[$meta:meta])* ordinal $module:ident::$item:ident: $ty:ty) => {
    $(#[$meta])*
    #[allow(clippy::missing_safety_doc, clippy::useless_transmute)]
    pub unsafe fn $item(&self, m: $module) -> Option<$ty> {
      match self.$item {
        Ordinal::Ordinal(o) => GetProcAddress(m.0, transmute(o as usize)).map(|x| transmute(x)),
        Ordinal::Address(a) => transmute(a.wrapping_add(m.0 as usize)),
      }
    }
  };
}

macro_rules! decl_addresses {
  ($($(#[$meta:meta])* $(#$ordinal:ident)? $module:ident::$item:ident: $ty:ty),* $(,)?) => {
    pub struct Addresses {$(
      $(#[$meta])*
      pub(crate) $item: decl_addresses_ty!($($ordinal)?)
    ),*}
    impl Addresses {
      pub const ZERO: Self = Self {
        $($item: decl_addresses_init!($($ordinal)?),)*
      };
      $(decl_addresses_impl! {
        $(#[$meta])* $($ordinal)? $module::$item: $ty
      })*
    }
  };
}
decl_addresses! {
  /// Pointer to the current player. May exist even when not in-game.
  Client::player: NonNull<Option<NonNull<()>>>,
  /// The array containing the active splash effects (Acts 1&3 rain).
  Client::env_splashes: NonNull<Option<NonNull<EnvImages>>>,
  /// The array containing the active bubble effects (Act 3 water).
  Client::env_bubbles: NonNull<Option<NonNull<EnvImages>>>,
  /// The number of times the client has updated the game state.
  Client::client_updates: NonNull<u32>,
  /// The type of game the client is currently running. Only meaningful if a
  /// game is running.
  Client::game_type: NonNull<GameType>,
  /// The table of active game entities.
  Client::active_entities: NonNull<()>,
  /// The currently selected draw function.
  Client::draw_game_fn: NonNull<unsafe extern "fastcall" fn(u32)>,
  /// Frame count used to calculate the client's current fps.
  Client::client_fps_frames: NonNull<u32>,
  /// The total number of frames drawn by the client.
  Client::client_total_frames: NonNull<u32>,
  /// Applies a position change to a `DyPos`. Signature depends on game version.
  Common::apply_pos_change: usize,
  /// Whether the game is rendered in perspective mode.
  #ordinal Gfx::in_perspective: unsafe extern "stdcall" fn() -> u32,
  /// The game's window handle
  #ordinal Gfx::hwnd: unsafe extern "stdcall" fn() -> HWND,
  /// The time the game server most recently updated the game state.
  Game::server_update_time: NonNull<u32>,
  /// Draw the game's current menu.
  #ordinal Win::draw_menu: unsafe extern "stdcall" fn(),
}

#[derive(Clone, Copy)]
pub enum Module {
  /// The v1.14* monolithic game.exe
  GameExe,
  Client,
  Common,
  #[allow(dead_code)]
  Game,
  #[allow(dead_code)]
  Gfx,
  Win,
}
impl Module {
  pub fn as_str(&self) -> &'static str {
    match *self {
      Self::GameExe => "game.exe",
      Self::Client => "D2Client.dll",
      Self::Common => "D2Common.dll",
      Self::Game => "D2Game.dll",
      Self::Gfx => "D2gfx.dll",
      Self::Win => "D2Win.dll",
    }
  }
}
impl fmt::Display for Module {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

pub struct BaseAddresses {
  pub client: usize,
  pub common: usize,
  pub game: usize,
  pub gfx: usize,
  pub win: usize,
}
impl BaseAddresses {
  pub const ZERO: Self = Self { client: 0, common: 0, game: 0, gfx: 0, win: 0 };
}
impl Index<Module> for BaseAddresses {
  type Output = usize;
  fn index(&self, index: Module) -> &Self::Output {
    match index {
      Module::GameExe | Module::Client => &self.client,
      Module::Common => &self.common,
      Module::Game => &self.game,
      Module::Gfx => &self.gfx,
      Module::Win => &self.win,
    }
  }
}
