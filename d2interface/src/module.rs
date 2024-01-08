use crate::{
  Bool32, ClientEnvEffects, ClientLoopGlobals, Cursor, EnvArray, FixedI4, FixedU8, GameType,
};
use core::{fmt, mem::transmute, ops::Index, ptr::NonNull};
use windows_sys::{
  w,
  Win32::{
    Foundation::{HMODULE, HWND},
    System::LibraryLoader::{GetModuleHandleW, GetProcAddress, LoadLibraryW},
  },
};

#[derive(Default, Clone, Copy)]
#[repr(transparent)]
pub struct Client(HMODULE);
#[derive(Default, Clone, Copy)]
#[repr(transparent)]
pub struct Common(HMODULE);
#[derive(Default, Clone, Copy)]
#[repr(transparent)]
pub struct Fog(HMODULE);
#[derive(Default, Clone, Copy)]
#[repr(transparent)]
pub struct Game(HMODULE);
#[derive(Default, Clone, Copy)]
#[repr(transparent)]
pub struct Gfx(HMODULE);
#[derive(Default, Clone, Copy)]
#[repr(transparent)]
pub struct Win(HMODULE);

pub struct Modules {
  modules: [HMODULE; 6],
}
impl Modules {
  pub fn load_split_modules() -> Option<Self> {
    const MODULE_NAMES: [*const u16; 6] = [
      w!("D2Client.dll"),
      w!("D2Common.dll"),
      w!("Fog.dll"),
      w!("D2Game.dll"),
      w!("D2gfx.dll"),
      w!("D2Win.dll"),
    ];

    let modules = MODULE_NAMES.map(|name| unsafe { LoadLibraryW(name) });
    modules.iter().all(|&x| x != 0).then_some(Self { modules })
  }

  pub fn load_combined_module() -> Option<Self> {
    let module = unsafe { GetModuleHandleW(w!("game.exe")) };
    (module != 0).then_some(Self { modules: [module; 6] })
  }

  #[inline]
  pub fn client(&self) -> Client {
    Client(self.modules[0])
  }

  #[inline]
  pub fn common(&self) -> Common {
    Common(self.modules[1])
  }

  #[inline]
  pub fn fog(&self) -> Fog {
    Fog(self.modules[2])
  }

  #[inline]
  pub fn game(&self) -> Game {
    Game(self.modules[3])
  }

  #[inline]
  pub fn gfx(&self) -> Gfx {
    Gfx(self.modules[4])
  }

  #[inline]
  pub fn win(&self) -> Win {
    Win(self.modules[5])
  }
}
impl Index<Module> for Modules {
  type Output = HMODULE;

  fn index(&self, index: Module) -> &Self::Output {
    &self.modules[match index {
      Module::GameExe | Module::Client => 0,
      Module::Common => 1,
      Module::Fog => 2,
      Module::Game => 3,
      Module::Gfx => 4,
      Module::Win => 5,
    }]
  }
}

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
  /// The arrays containing the environment effects data.
  Client::env_effects: NonNull<ClientEnvEffects>,
  /// The type of game the client is currently running. Only meaningful if a
  /// game is running.
  Client::game_type: NonNull<GameType>,
  /// The table of active game entities.
  Client::entity_table: NonNull<()>,
  Client::entity_table2: NonNull<()>,
  /// Globals controlling the main client loop.
  Client::client_loop_globals: NonNull<ClientLoopGlobals>,
  /// Applies a position change to a `DyPos`. Signature depends on game version.
  Common::apply_pos_change: usize,
  /// Whether the game is rendered in perspective mode.
  #ordinal Gfx::in_perspective: unsafe extern "stdcall" fn() -> Bool32,
  /// The game's window handle
  #ordinal Gfx::hwnd: unsafe extern "stdcall" fn() -> HWND,
  /// The time the game server most recently updated the game state.
  Game::server_update_time: NonNull<u32>,
  /// Draw the game's current menu.
  #ordinal Win::draw_menu: unsafe extern "stdcall" fn(),
  /// The cursor definition table.
  Client::cursor_table: &'static [Cursor; 7],
  /// The in-game cursor's state
  Client::game_cursor: NonNull<()>,
  /// The x-positions of the clouds in the Arreat Summit.
  Client::summit_cloud_x_pos: NonNull<[FixedI4; 10]>,
  /// Draw a one pixel thick line
  #ordinal Gfx::draw_line: unsafe extern "stdcall" fn(i32, i32, i32, i32, u8, u8),
  /// Gets the closest available color in the current palette
  #ordinal Win::find_closest_color: unsafe extern "stdcall" fn(u8, u8, u8) -> u8,
  ///The width of the game's viewport.
  Client::viewport_width: NonNull<u32>,
  ///The height of the game's viewport.
  Client::viewport_height: NonNull<u32>,
  /// How far the viewport is shifted from the center.
  Client::viewport_shift: NonNull<i32>,
  /// The maximum number of weather particles for the current frame.
  Client::max_weather_particles: NonNull<u32>,
  /// The angle the weather is currently moving
  Client::weather_angle: NonNull<FixedU8>,
  // The speed the rain is currently moving at.
  Client::rain_speed: NonNull<f32>,
  /// Whether the current weather is snow
  Client::is_snowing: NonNull<Bool32>,
  /// Sine table for 8-bit fixed point numbers. Input in x*pi radians.
  Fog::sine_table: NonNull<[f32; 0x200]>,
  /// Generates a weather particle. Signature depends on game version.
  Client::gen_weather_particle: usize,
  /// Removes an item from the env array.
  #ordinal Fog::env_array_remove: unsafe extern "fastcall" fn(*mut EnvArray, id: u32),
}

#[derive(Clone, Copy)]
pub enum Module {
  /// The v1.14* monolithic game.exe
  GameExe,
  Client,
  Common,
  Fog,
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
      Self::Fog => "Fog.dll",
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
  pub fog: usize,
  pub game: usize,
  pub gfx: usize,
  pub win: usize,
}
impl BaseAddresses {
  pub const ZERO: Self = Self {
    client: 0,
    common: 0,
    fog: 0,
    game: 0,
    gfx: 0,
    win: 0,
  };
}
impl Index<Module> for BaseAddresses {
  type Output = usize;
  fn index(&self, index: Module) -> &Self::Output {
    match index {
      Module::GameExe | Module::Client => &self.client,
      Module::Common => &self.common,
      Module::Fog => &self.fog,
      Module::Game => &self.game,
      Module::Gfx => &self.gfx,
      Module::Win => &self.win,
    }
  }
}
