use crate::{config::Config, tracker::UnitId, util::Module, D2Fps, D2FPS, GAME_FPS};
use arrayvec::ArrayVec;
use bin_patch::{AppliedPatch, Patch};
use core::{
  fmt,
  mem::{size_of, transmute},
  ptr::{null_mut, NonNull},
};
use d2interface::{
  all_versions::{
    D2Client, D2Common, D2Game, D2Gfx, D2Win, EntityKind, EntityTables, EnvArray, EnvImage,
    GameAddresses, GameType, LinkedList,
  },
  FixedI16, FixedU16, IsoPos, LinearPos,
};
use windows_sys::{
  w,
  Win32::{
    Foundation::{HMODULE, HWND, LPARAM, LRESULT, WPARAM},
    Media::timeGetTime,
    Storage::FileSystem::{
      GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW, VS_FIXEDFILEINFO,
    },
    System::{
      LibraryLoader::GetModuleHandleW, Performance::QueryPerformanceCounter, Threading::Sleep,
    },
    UI::{
      Shell::{DefSubclassProc, RemoveWindowSubclass, SetWindowSubclass},
      WindowsAndMessaging::{SIZE_MINIMIZED, WM_ACTIVATE, WM_SIZE, WM_WINDOWPOSCHANGED},
    },
  },
};

mod v109d;
mod v110;
mod v112;
mod v113c;
mod v113d;
mod v114d;

const GAME_EXE: *const u16 = w!("game.exe");

#[derive(Clone, Copy)]
enum D2Module {
  GameExe,
  Client,
  Common,
  #[allow(dead_code)]
  Game,
  #[allow(dead_code)]
  Gfx,
  Win,
}
impl fmt::Display for D2Module {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(match *self {
      Self::GameExe => "game.exe",
      Self::Client => "D2Client.dll",
      Self::Common => "D2Common.dll",
      Self::Game => "D2Game.dll",
      Self::Gfx => "D2gfx.dll",
      Self::Win => "D2Win.dll",
    })
  }
}
#[derive(Default)]
struct D2Modules {
  d2client: D2Client,
  d2common: D2Common,
  d2game: D2Game,
  d2gfx: D2Gfx,
  d2win: D2Win,
}
impl D2Modules {
  fn get(&self, module: D2Module) -> HMODULE {
    match module {
      D2Module::GameExe => self.d2client.0,
      D2Module::Client => self.d2client.0,
      D2Module::Common => self.d2common.0,
      D2Module::Game => self.d2game.0,
      D2Module::Gfx => self.d2gfx.0,
      D2Module::Win => self.d2win.0,
    }
  }

  fn from_game_exe() -> Result<Self, ()> {
    let module = unsafe { GetModuleHandleW(GAME_EXE) };
    if module == 0 {
      log!("Failed to find game.exe");
      Err(())
    } else {
      Ok(Self {
        d2client: D2Client(module),
        d2common: D2Common(module),
        d2game: D2Game(module),
        d2gfx: D2Gfx(module),
        d2win: D2Win(module),
      })
    }
  }

  fn from_loaded_modules(modules: &LoadedModules) -> Self {
    Self {
      d2client: D2Client(modules.d2client.0),
      d2common: D2Common(modules.d2common.0),
      d2game: D2Game(modules.d2game.0),
      d2gfx: D2Gfx(modules.d2gfx.0),
      d2win: D2Win(modules.d2win.0),
    }
  }
}

struct LoadedModules {
  d2client: Module,
  d2common: Module,
  d2game: Module,
  d2gfx: Module,
  d2win: Module,
}
impl LoadedModules {
  fn load() -> Result<Self, ()> {
    Ok(Self {
      d2client: match unsafe { Module::new(w!("D2Client.dll")) } {
        Ok(m) => m,
        Err(e) => {
          log!("Failed to load `D2Client.dll`");
          return Err(e);
        }
      },
      d2common: match unsafe { Module::new(w!("D2Common.dll")) } {
        Ok(m) => m,
        Err(e) => {
          log!("Failed to load `D2Common.dll`");
          return Err(e);
        }
      },
      d2game: match unsafe { Module::new(w!("D2Game.dll")) } {
        Ok(m) => m,
        Err(e) => {
          log!("Failed to load `D2Game.dll`");
          return Err(e);
        }
      },
      d2gfx: match unsafe { Module::new(w!("D2gfx.dll")) } {
        Ok(m) => m,
        Err(e) => {
          log!("Failed to load `D2gfx.dll`");
          return Err(e);
        }
      },
      d2win: match unsafe { Module::new(w!("D2Win.dll")) } {
        Ok(m) => m,
        Err(e) => {
          log!("Failed to load `D2Win.dll`");
          return Err(e);
        }
      },
    })
  }
}

struct ModulePatches {
  module: D2Module,
  pref_base: usize,
  patches: &'static [Patch],
}
impl ModulePatches {
  const fn new(module: D2Module, pref_base: usize, patches: &'static [Patch]) -> Self {
    Self { module, pref_base, patches }
  }
}

struct PatchSets {
  menu_fps: &'static [ModulePatches],
  game_fps: &'static [ModulePatches],
  game_smoothing: &'static [ModulePatches],
}

#[derive(Debug, Clone, Copy)]
enum GameVersion {
  // V100,
  // V101,
  V102,
  V103,
  V104b,
  V104c,
  V105,
  V105b,
  // V106,
  // V106b,
  V107,
  V108,
  V109,
  V109b,
  V109d,
  V110b,
  V110s,
  V110,
  V111,
  V111b,
  V112,
  V113a,
  V113c,
  V113d,
  V114a,
  V114b,
  V114c,
  V114d,
}
impl fmt::Display for GameVersion {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(match *self {
      Self::V102 => "1.02",
      Self::V103 => "1.03",
      Self::V104b => "1.04b",
      Self::V104c => "1.04c",
      Self::V105 => "1.05",
      Self::V105b => "1.05b",
      Self::V107 => "1.07",
      Self::V108 => "1.08",
      Self::V109 => "1.09",
      Self::V109b => "1.09b",
      Self::V109d => "1.09d",
      Self::V110b => "1.10b",
      Self::V110s => "1.10s",
      Self::V110 => "1.10",
      Self::V111 => "1.11",
      Self::V111b => "1.11b",
      Self::V112 => "1.12",
      Self::V113a => "1.13a",
      Self::V113c => "1.13c",
      Self::V113d => "1.13d",
      Self::V114a => "1.14a",
      Self::V114b => "1.14b",
      Self::V114c => "1.14c",
      Self::V114d => "1.14d",
    })
  }
}
impl GameVersion {
  fn from_file() -> Result<Option<GameVersion>, ()> {
    unsafe {
      let len = GetFileVersionInfoSizeW(GAME_EXE, null_mut());
      let mut buf = Vec::<u8>::with_capacity(len as usize);

      if GetFileVersionInfoW(GAME_EXE, 0, len, buf.as_mut_ptr().cast()) == 0 {
        return Err(());
      }
      buf.set_len(len as usize);

      let mut len = 0u32;
      let mut out = null_mut::<u8>();
      if VerQueryValueW(
        buf.as_mut_ptr().cast(),
        w!("\\"),
        (&mut out as *mut *mut u8).cast(),
        &mut len,
      ) == 0
        || (len as usize) < size_of::<VS_FIXEDFILEINFO>()
      {
        return Err(());
      }

      let info = &*out.cast::<VS_FIXEDFILEINFO>();
      match (info.dwFileVersionMS, info.dwFileVersionLS) {
        // (0x0001_0000, 0x0000_0001) => Some(GameVersion::v100),
        // (0x0001_0000, 0x0000_0001) => Some(GameVersion::v101),
        (0x0001_0000, 0x0002_0000) => Ok(Some(GameVersion::V102)),
        (0x0001_0000, 0x0003_0000) => Ok(Some(GameVersion::V103)),
        (0x0001_0000, 0x0004_0001) => Ok(Some(GameVersion::V104b)),
        (0x0001_0000, 0x0004_0002) => Ok(Some(GameVersion::V104c)),
        (0x0001_0000, 0x0005_0000) => Ok(Some(GameVersion::V105)),
        (0x0001_0000, 0x0005_0001) => Ok(Some(GameVersion::V105b)),
        // (0x0001_0000, 0x0006_0000) => Some(GameVersion::v106),
        // (0x0001_0000, 0x0006_0000) => Some(GameVersion::v106b),
        (0x0001_0000, 0x0007_0000) => Ok(Some(GameVersion::V107)),
        (0x0001_0000, 0x0008_001c) => Ok(Some(GameVersion::V108)),
        (0x0001_0000, 0x0009_0013) => Ok(Some(GameVersion::V109)),
        (0x0001_0000, 0x0009_0014) => Ok(Some(GameVersion::V109b)),
        (0x0001_0000, 0x0009_0016) => Ok(Some(GameVersion::V109d)),
        (0x0001_0000, 0x000a_0009) => Ok(Some(GameVersion::V110b)),
        (0x0001_0000, 0x000a_000a) => Ok(Some(GameVersion::V110s)),
        (0x0001_0000, 0x000a_0027) => Ok(Some(GameVersion::V110)),
        (0x0001_0000, 0x000b_002d) => Ok(Some(GameVersion::V111)),
        (0x0001_0000, 0x000b_002e) => Ok(Some(GameVersion::V111b)),
        (0x0001_0000, 0x000c_0031) => Ok(Some(GameVersion::V112)),
        (0x0001_0000, 0x000d_0037) => Ok(Some(GameVersion::V113a)),
        (0x0001_0000, 0x000d_003c) => Ok(Some(GameVersion::V113c)),
        (0x0001_0000, 0x000d_0040) => Ok(Some(GameVersion::V113d)),
        (0x0001_000e, 0x0000_0040) => Ok(Some(GameVersion::V114a)),
        (0x0001_000e, 0x0001_0044) => Ok(Some(GameVersion::V114b)),
        (0x0001_000e, 0x0002_0046) => Ok(Some(GameVersion::V114c)),
        (0x0001_000e, 0x0003_0047) => Ok(Some(GameVersion::V114d)),
        _ => Ok(None),
      }
    }
  }
}

pub struct GameAccessor {
  pub player: NonNull<Option<NonNull<()>>>,
  pub env_splashes: NonNull<Option<NonNull<EnvArray<EnvImage>>>>,
  pub env_bubbles: NonNull<Option<NonNull<EnvArray<EnvImage>>>>,
  pub client_update_count: NonNull<u32>,
  pub game_type: NonNull<GameType>,
  pub active_entity_tables: NonNull<()>,
  pub draw_game_fn: NonNull<unsafe extern "fastcall" fn(u32)>,
  pub client_fps_frame_count: NonNull<u32>,
  pub client_total_frame_count: NonNull<u32>,
  pub apply_pos_change: usize,
  pub server_update_time: NonNull<u32>,
  pub render_in_perspective: unsafe extern "stdcall" fn() -> u32,
  pub hwnd: NonNull<HWND>,
  pub draw_menu: unsafe extern "stdcall" fn(),
}
unsafe impl Send for GameAccessor {}
impl GameAccessor {
  const fn new() -> Self {
    Self {
      player: NonNull::dangling(),
      env_splashes: NonNull::dangling(),
      env_bubbles: NonNull::dangling(),
      client_update_count: NonNull::dangling(),
      game_type: NonNull::dangling(),
      active_entity_tables: NonNull::dangling(),
      draw_game_fn: NonNull::dangling(),
      client_fps_frame_count: NonNull::dangling(),
      client_total_frame_count: NonNull::dangling(),
      apply_pos_change: 0,
      server_update_time: NonNull::dangling(),
      render_in_perspective: {
        extern "stdcall" fn f() -> u32 {
          panic!()
        }
        f
      },
      hwnd: NonNull::dangling(),
      draw_menu: {
        extern "stdcall" fn f() {
          panic!()
        }
        f
      },
    }
  }

  unsafe fn load(&mut self, modules: &D2Modules, addresses: &GameAddresses) {
    self.player = addresses.player(modules.d2client);
    self.env_splashes = addresses.env_splashes(modules.d2client);
    self.env_bubbles = addresses.env_bubbles(modules.d2client);
    self.client_update_count = addresses.client_update_count(modules.d2client);
    self.game_type = addresses.game_type(modules.d2client);
    self.active_entity_tables = addresses.active_entity_tables(modules.d2client);
    self.draw_game_fn = addresses.draw_game_fn(modules.d2client);
    self.client_fps_frame_count = addresses.client_fps_frame_count(modules.d2client);
    self.client_total_frame_count = addresses.client_total_frame_count(modules.d2client);
    self.apply_pos_change = addresses.apply_pos_change(modules.d2common);
    self.server_update_time = addresses.server_update_time(modules.d2game);
    self.render_in_perspective = addresses.render_in_perspective(modules.d2gfx);
    self.hwnd = addresses.hwnd(modules.d2gfx);
    self.draw_menu = addresses.draw_menu(modules.d2win);
  }

  pub unsafe fn player<T>(&self) -> Option<NonNull<T>> {
    *self.player.cast().as_ptr()
  }

  pub unsafe fn active_entity_tables<T>(&self) -> NonNull<EntityTables<T>> {
    self.active_entity_tables.cast()
  }
}

unsafe extern "system" fn win_proc(
  hwnd: HWND,
  msg: u32,
  wparam: WPARAM,
  lparam: LPARAM,
  _: usize,
  _: usize,
) -> LRESULT {
  match msg {
    WM_ACTIVATE => {
      if let Some(mut instance_lock) = D2FPS.try_lock() {
        let instance = &mut *instance_lock;
        if wparam != 0 {
          instance
            .render_timer
            .switch_rate(&instance.perf_freq, instance.frame_rate);
        } else {
          instance
            .render_timer
            .switch_rate(&instance.perf_freq, instance.bg_frame_rate);
        }
      }
    }
    WM_SIZE => {
      if let Some(mut instance) = D2FPS.try_lock() {
        instance.is_window_hidden = wparam == SIZE_MINIMIZED as usize;
      }
    }
    WM_WINDOWPOSCHANGED => {
      if let Some(mut instance) = D2FPS.try_lock() {
        instance.frame_rate_from_window(hwnd);
      }
    }
    _ => {}
  }

  DefSubclassProc(hwnd, msg, wparam, lparam)
}

struct WindowHook(bool);
impl WindowHook {
  const ID: usize = 59384;

  pub unsafe fn attach(&mut self, accessor: &GameAccessor) -> bool {
    if !self.0 {
      let hwnd = *accessor.hwnd.as_ptr();
      if hwnd != 0 {
        self.0 = true;
        SetWindowSubclass(hwnd, Some(win_proc), Self::ID, 0);
        return true;
      }
    }
    false
  }

  pub unsafe fn detach(&mut self, accessor: &GameAccessor) {
    if self.0 {
      let hwnd = *accessor.hwnd.as_ptr();
      RemoveWindowSubclass(hwnd, Some(win_proc), Self::ID);
      self.0 = false;
    }
  }
}

pub struct HookManager {
  version: Option<GameVersion>,
  modules: Option<LoadedModules>,
  accessor: GameAccessor,
  patches: ArrayVec<AppliedPatch, 60>,
  window_hook: WindowHook,
}
impl HookManager {
  pub const fn new() -> Self {
    Self {
      version: None,
      modules: None,
      accessor: GameAccessor::new(),
      patches: ArrayVec::new_const(),
      window_hook: WindowHook(false),
    }
  }

  pub fn init(&mut self) -> Result<(), ()> {
    assert!(self.version.is_none());
    match GameVersion::from_file() {
      Ok(Some(version)) => {
        self.version = Some(version);
        log!("Detected game version: v{version}");
        Ok(())
      }
      Ok(None) => {
        log!("Unknown game version");
        Err(())
      }
      Err(_) => {
        log!("Error detecting game version");
        Err(())
      }
    }
  }

  pub(crate) fn attach(&mut self, config: &mut Config) -> Result<(), ()> {
    let (sets, modules) = match self.version {
      Some(GameVersion::V109d) => {
        let modules = D2Modules::from_loaded_modules(self.loaded_modules()?);
        unsafe { self.accessor.load(&modules, &d2interface::v109d::ADDRESSES) };
        (&v109d::PATCHES, modules)
      }
      Some(GameVersion::V110) => {
        let modules = D2Modules::from_loaded_modules(self.loaded_modules()?);
        unsafe { self.accessor.load(&modules, &d2interface::v110::ADDRESSES) };
        (&v110::PATCHES, modules)
      }
      Some(GameVersion::V112) => {
        let modules = D2Modules::from_loaded_modules(self.loaded_modules()?);
        unsafe { self.accessor.load(&modules, &d2interface::v112::ADDRESSES) };
        (&v112::PATCHES, modules)
      }
      Some(GameVersion::V113c) => {
        let modules = D2Modules::from_loaded_modules(self.loaded_modules()?);
        unsafe { self.accessor.load(&modules, &d2interface::v113c::ADDRESSES) };
        (&v113c::PATCHES, modules)
      }
      Some(GameVersion::V113d) => {
        let modules = D2Modules::from_loaded_modules(self.loaded_modules()?);
        unsafe { self.accessor.load(&modules, &d2interface::v113d::ADDRESSES) };
        (&v113d::PATCHES, modules)
      }
      Some(GameVersion::V114d) => {
        let modules = D2Modules::from_game_exe()?;
        unsafe { self.accessor.load(&modules, &d2interface::v114d::ADDRESSES) };
        (&v114d::PATCHES, modules)
      }
      _ => return Err(()),
    };

    let mut count = 2;
    if unsafe { self.apply_patch_set(&modules, sets.menu_fps).is_err() } {
      log!("Failed to apply menu fps patches");
      count -= 1;
    }
    if unsafe { self.apply_patch_set(&modules, sets.game_fps).is_err() } {
      log!("Failed to apply game fps patches");
      count -= 2;
      config.enable_smoothing = false;
    } else if config.enable_smoothing
      && unsafe { self.apply_patch_set(&modules, sets.game_smoothing).is_err() }
    {
      log!("Failed to apply game smoothing patches");
      config.enable_smoothing = false;
    }

    if count == 0 {
      self.modules = None;
      Err(())
    } else {
      Ok(())
    }
  }

  pub fn detach(&mut self) {
    unsafe {
      self.window_hook.detach(&self.accessor);
    }
    self.patches.clear();
    self.modules = None;
  }

  unsafe fn apply_patch_set(
    &mut self,
    modules: &D2Modules,
    patches: &[ModulePatches],
  ) -> Result<(), ()> {
    let start_idx = self.patches.len();
    let mut success = true;

    for m in patches {
      let d2mod = modules.get(m.module);
      let reloc_dist = d2mod.wrapping_sub(m.pref_base as isize);
      for patch in m.patches {
        match patch.apply(d2mod, reloc_dist) {
          Ok(p) => self.patches.push(p),
          Err(_) => {
            success = false;
            log!("Failed to apply patch at: {}+{:#x}", m.module, patch.offset);
          }
        }
      }
    }

    if success {
      Ok(())
    } else {
      self.patches.truncate(start_idx);
      Err(())
    }
  }

  fn loaded_modules(&mut self) -> Result<&LoadedModules, ()> {
    match &mut self.modules {
      Some(modules) => Ok(modules),
      modules @ None => {
        *modules = Some(LoadedModules::load()?);
        modules.as_ref().ok_or(())
      }
    }
  }
}

trait Entity: LinkedList {
  fn unit_id(&self) -> UnitId;
  fn has_room(&self) -> bool;
  fn linear_pos(&self) -> LinearPos<FixedU16>;
  fn iso_pos(&self) -> IsoPos<i32>;
  unsafe fn tracker_pos(&self) -> (LinearPos<FixedU16>, LinearPos<u16>);
}
trait DyPos {
  type Entity: Entity;
  fn entity(&self) -> NonNull<Self::Entity>;
  fn linear_pos(&self) -> LinearPos<FixedU16>;
}

impl D2Fps {
  fn entity_adjusted_pos(&mut self, e: &impl Entity) -> Option<LinearPos<FixedU16>> {
    self
      .entity_tracker
      .get(e.unit_id())
      .map(|pos| pos.for_time(self.unit_offset))
  }

  fn entity_linear_pos(&mut self, e: &impl Entity) -> LinearPos<FixedU16> {
    match self.entity_adjusted_pos(e) {
      Some(pos) => pos,
      None => e.linear_pos(),
    }
  }

  fn entity_iso_pos(&mut self, e: &impl Entity) -> IsoPos<i32> {
    match self.entity_adjusted_pos(e) {
      Some(pos) => IsoPos::from(pos),
      None => e.iso_pos(),
    }
  }

  fn dypos_linear_pos(&mut self, pos: &impl DyPos) -> LinearPos<FixedU16> {
    self
      .entity_adjusted_pos(unsafe { pos.entity().as_ref() })
      .unwrap_or(pos.linear_pos())
  }

  unsafe fn update_server_time(&mut self, time: i64) -> bool {
    let game_type = *self.hooks.accessor.game_type.as_ptr();
    if game_type.is_sp() && self.config.enable_smoothing {
      let prev_update_time = self.game_update_time_ms;
      self.game_update_time_ms = *self.hooks.accessor.server_update_time.as_ptr();

      if self.game_update_time_ms != prev_update_time {
        let cur_time_ms = timeGetTime() & 0x7FFFFFFF;
        if self.game_update_time_ms < cur_time_ms {
          self.game_update_time = self.perf_freq.ms_to_sample(
            self.perf_freq.sample_to_ms(time as u64)
              - (cur_time_ms - self.game_update_time_ms) as u64,
          );
        } else {
          // Fallback time for when the clock wraps around.
          // Will be corrected next frame.
          self.game_update_time = time as u64;
        }
        return true;
      }
    }
    false
  }

  unsafe fn update_entites_from_server<T: Entity>(&mut self) {
    let mut f = |e: &T| {
      let (pos, target_pos) = e.tracker_pos();
      self.entity_tracker.insert_or_update(e.unit_id(), pos, target_pos);
    };
    let tables = self.hooks.accessor.active_entity_tables::<T>().as_ref();
    tables[EntityKind::Pc].iter().for_each(&mut f);
    tables[EntityKind::Npc].iter().for_each(&mut f);
    tables[EntityKind::Missile].iter().for_each(&mut f);
    self.entity_tracker.clear_unused();
  }

  fn update_unit_offset(&mut self) {
    let frame_len = self.perf_freq.ms_to_sample(40) as i64;
    let since_update = self.render_timer.last_update().wrapping_sub(self.game_update_time) as i64;
    let since_update = since_update.min(frame_len);
    let offset = since_update - frame_len;
    let fract = (offset << 16) / frame_len;
    self.unit_offset = FixedI16::from_repr(fract as i32);
  }

  unsafe fn update_env_images(&mut self, prev_pos: IsoPos<i32>) {
    if (self.hooks.accessor.render_in_perspective)() == 0 {
      let dx = prev_pos.x.wrapping_sub(self.player_pos.x) as u32;
      let dy = prev_pos.y.wrapping_sub(self.player_pos.y) as u32;

      if let Some(mut splashes) = *self.hooks.accessor.env_splashes.as_ptr() {
        for splash in splashes.as_mut().as_mut_slice() {
          splash.pos.x = splash.pos.x.wrapping_add(dx);
          splash.pos.y = splash.pos.y.wrapping_add(dy);
        }
      }
      if let Some(mut bubbles) = *self.hooks.accessor.env_bubbles.as_ptr() {
        for bubble in bubbles.as_mut().as_mut_slice() {
          bubble.pos.x = bubble.pos.x.wrapping_add(dx);
          bubble.pos.y = bubble.pos.y.wrapping_add(dy);
        }
      }
    }
  }
}

extern "stdcall" fn entity_iso_xpos<E: Entity>(e: &E) -> i32 {
  D2FPS.lock().entity_iso_pos(e).x
}
extern "stdcall" fn entity_iso_ypos<E: Entity>(e: &E) -> i32 {
  D2FPS.lock().entity_iso_pos(e).y
}

extern "stdcall" fn entity_linear_xpos<E: Entity>(e: &E) -> FixedU16 {
  D2FPS.lock().entity_linear_pos(e).x
}
extern "stdcall" fn entity_linear_ypos<E: Entity>(e: &E) -> FixedU16 {
  D2FPS.lock().entity_linear_pos(e).y
}

extern "stdcall" fn entity_linear_whole_xpos<E: Entity>(e: &E) -> u32 {
  D2FPS.lock().entity_linear_pos(e).x.truncate()
}
extern "stdcall" fn entity_linear_whole_ypos<E: Entity>(e: &E) -> u32 {
  D2FPS.lock().entity_linear_pos(e).y.truncate()
}

extern "stdcall" fn dypos_linear_whole_xpos<P: DyPos>(pos: &P) -> u32 {
  D2FPS.lock().dypos_linear_pos(pos).x.truncate()
}
extern "stdcall" fn dypos_linear_whole_ypos<P: DyPos>(pos: &P) -> u32 {
  D2FPS.lock().dypos_linear_pos(pos).y.truncate()
}

unsafe extern "C" fn draw_game<T: Entity>() {
  let mut instance_lock = D2FPS.lock();
  let instance = &mut *instance_lock;

  let Some(player) = instance.hooks.accessor.player::<T>() else {
    return;
  };
  if !player.as_ref().has_room() {
    return;
  }

  let mut time = 0i64;
  QueryPerformanceCounter(&mut time);
  if instance.update_server_time(time) {
    instance.update_entites_from_server::<T>();
  }

  QueryPerformanceCounter(&mut time);
  if instance.render_timer.update_time(time as u64, &instance.perf_freq) {
    let prev_update_count = instance.game_update_count;
    instance.game_update_count = *instance.hooks.accessor.client_update_count.as_ptr();

    if instance.frame_rate != GAME_FPS {
      instance.update_unit_offset();

      let prev_player_pos = instance.player_pos;
      instance.player_pos = instance.entity_iso_pos(player.as_ref());

      if instance.game_update_count == prev_update_count {
        instance.update_env_images(prev_player_pos);
      }
    } else {
      instance.unit_offset = FixedI16::from_repr(0);
      instance.player_pos = instance.entity_iso_pos(player.as_ref());
    }

    let draw = instance.hooks.accessor.draw_game_fn;
    let frame_count = instance.hooks.accessor.client_fps_frame_count;
    let total_frame_count = instance.hooks.accessor.client_total_frame_count;
    drop(instance_lock);
    (*draw.as_ptr())(0);
    *frame_count.as_ptr() += 1;
    *total_frame_count.as_ptr() += 1;
  }
}

unsafe extern "C" fn draw_game_paused() {
  let mut instance_lock = crate::D2FPS.lock();
  let instance = &mut *instance_lock;

  let mut cur_time = 0i64;
  QueryPerformanceCounter(&mut cur_time);

  if instance
    .render_timer
    .update_time(cur_time as u64, &instance.perf_freq)
  {
    let f = *instance.hooks.accessor.draw_game_fn.as_ptr();
    drop(instance_lock);
    f(0);
  }
}

unsafe extern "fastcall" fn draw_menu(
  callback: Option<extern "stdcall" fn(u32)>,
  call_count: &mut u32,
) {
  let mut instance_lock = D2FPS.lock();
  let instance = &mut *instance_lock;
  if instance.hooks.window_hook.attach(&instance.hooks.accessor) && instance.config.fps.is_none() {
    let hwnd = *instance.hooks.accessor.hwnd.as_ptr();
    instance.frame_rate_from_window(hwnd);
  }

  let mut time = 0i64;
  QueryPerformanceCounter(&mut time);

  if instance.render_timer.update_time(time as u64, &instance.perf_freq) {
    if instance.menu_timer.update_time(
      instance.render_timer.last_update(),
      &instance.perf_freq,
      transmute(callback),
    ) {
      instance.menu_timer_updated = true;
      if let Some(callback) = callback {
        callback(*call_count);
        *call_count += 1;
      }
    }

    let draw = instance.hooks.accessor.draw_menu;
    drop(instance_lock);
    draw();

    instance_lock = D2FPS.lock();
    instance_lock.menu_timer_updated = false;
  }

  let instance = &mut *instance_lock;
  QueryPerformanceCounter(&mut time);
  let sleep_len = (instance
    .perf_freq
    .sample_to_ms(instance.render_timer.next_update().saturating_sub(time as u64))
    as u32)
    .saturating_sub(1)
    .min(10);
  let sleep_len = if instance.is_window_hidden {
    10
  } else {
    sleep_len
  };
  Sleep(sleep_len);
}

unsafe extern "C" fn game_loop_sleep_hook() {
  let instance = D2FPS.lock();
  let mut time = 0;
  QueryPerformanceCounter(&mut time);
  let len = (instance
    .perf_freq
    .sample_to_ms(instance.render_timer.next_update().saturating_sub(time as u64))
    as u32)
    .saturating_sub(1);
  let len = if instance.is_window_hidden { 10 } else { len };
  let limit = if instance.hooks.accessor.game_type.as_ref().is_host() {
    2
  } else {
    10
  };
  Sleep(len.min(limit));
}

unsafe extern "fastcall" fn update_menu_char_frame(rate: u32, frame: &mut u32) -> u32 {
  if D2FPS.lock().menu_timer_updated {
    *frame += rate;
  }
  *frame
}

unsafe extern "fastcall" fn intercept_teleport<E: Entity>(
  entity: &E,
  x: FixedU16,
  y: FixedU16,
) -> usize {
  let mut instance = D2FPS.lock();
  if let Some(pos) = instance.entity_tracker.get(entity.unit_id()) {
    pos.real = LinearPos::new(x, y);
    pos.delta = LinearPos::default();
  }
  instance.hooks.accessor.apply_pos_change
}
