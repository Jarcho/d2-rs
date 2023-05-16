use crate::{
  patch::{AppliedPatch, Patch},
  tracker::UnitId,
  util::Module,
  D2Fps, D2FPS, GAME_RATE,
};
use arrayvec::ArrayVec;
use core::{
  fmt,
  mem::{size_of, transmute},
  ptr::{null_mut, NonNull},
};
use d2interface::{
  all_versions::{EntityKind, EntityTables, GameType, LinkedList},
  v114d::{EnvArray, EnvImage},
  FixedI16, FixedU16, IsoPos, LinearPos,
};
use windows_sys::{
  w,
  Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    Media::timeGetTime,
    Storage::FileSystem::{
      GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW, VS_FIXEDFILEINFO,
    },
    System::{Performance::QueryPerformanceCounter, Threading::Sleep},
    UI::{
      Shell::{DefSubclassProc, RemoveWindowSubclass, SetWindowSubclass},
      WindowsAndMessaging::{SIZE_MINIMIZED, WM_ACTIVATE, WM_SIZE, WM_WINDOWPOSCHANGED},
    },
  },
};

macro_rules! call_target_patch {
  ($offset:literal, $value:literal, $target:expr) => {
    crate::patch::CallTargetPatch::new($offset, &$value, unsafe {
      core::mem::transmute($target as unsafe extern "stdcall" fn(_) -> _)
    })
  };
}

macro_rules! call_target_patchc {
  ($offset:literal, $value:literal, $target:expr) => {
    crate::patch::CallTargetPatch::new($offset, &$value, unsafe {
      core::mem::transmute($target as unsafe extern "C" fn() -> _)
    })
  };
}

macro_rules! reloc_byte {
  ($lit:literal) => {
    false
  };
  (reloc $lit1:literal) => {
    true
  };
}

macro_rules! call_patch {
  ($offset:literal, [$($($relocs:ident)? $bytes:literal),* $(,)?], $target:expr) => {{
    let original = &[$($bytes),*];
    let relocs = &[$(reloc_byte!($($relocs)? $bytes)),*];
    crate::patch::CallPatch::new($offset, original, relocs, unsafe {
      core::mem::transmute($target)
    })
  }};
}

macro_rules! apply_patches {
  ($self_:ident, $(($module:literal, $base:expr, $pref_base:literal, $patches:expr),)*) => {{
    let mut success = true;
    $(
      if $self_.apply_patches($module, $base, $pref_base, $patches).is_err() {
        success = false;
      }
    )*
    if success {
      return Ok(())
    } else {
      Err(())
    }
  }};
}

mod v109d;
mod v110;
mod v112;
mod v113c;
mod v113d;
mod v114d;

const GAME_EXE: *const u16 = w!("game.exe");
const D2CLIENT_DLL: *const u16 = w!("D2Client.dll");
const D2COMMON_DLL: *const u16 = w!("D2Common.dll");
const D2GAME_DLL: *const u16 = w!("D2Game.dll");
const D2GFX_DLL: *const u16 = w!("D2gfx.dll");
const D2WIN_DLL: *const u16 = w!("D2Win.dll");

const D2CLIENT_IDX: usize = 0;
const D2COMMON_IDX: usize = 1;
const D2GAME_IDX: usize = 2;
const D2GFX_IDX: usize = 3;
const D2WIN_IDX: usize = 4;

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
  pub render_in_perspective: unsafe extern "stdcall" fn() -> u32,
  pub hwnd: NonNull<HWND>,
  pub server_update_time: NonNull<u32>,
  pub client_update_count: NonNull<u32>,
  pub game_type: NonNull<GameType>,
  pub active_entity_tables: NonNull<()>,
  pub draw_game_fn: NonNull<unsafe extern "fastcall" fn(u32)>,
  pub client_fps_frame_count: NonNull<u32>,
  pub client_total_frame_count: NonNull<u32>,
  pub draw_menu: unsafe extern "stdcall" fn(),
  pub apply_pos_change: usize,
}
unsafe impl Send for GameAccessor {}
impl GameAccessor {
  const fn new() -> Self {
    Self {
      player: NonNull::dangling(),
      env_splashes: NonNull::dangling(),
      env_bubbles: NonNull::dangling(),
      render_in_perspective: {
        extern "stdcall" fn f() -> u32 {
          panic!()
        }
        f
      },
      hwnd: NonNull::dangling(),
      server_update_time: NonNull::dangling(),
      client_update_count: NonNull::dangling(),
      game_type: NonNull::dangling(),
      active_entity_tables: NonNull::dangling(),
      draw_game_fn: NonNull::dangling(),
      client_fps_frame_count: NonNull::dangling(),
      client_total_frame_count: NonNull::dangling(),
      draw_menu: {
        extern "stdcall" fn f() {
          panic!()
        }
        f
      },
      apply_pos_change: 0,
    }
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
  modules: ArrayVec<Module, 5>,
  accessor: GameAccessor,
  patches: ArrayVec<AppliedPatch, 59>,
  window_hook: WindowHook,
}
impl Drop for HookManager {
  fn drop(&mut self) {
    self.detach();
  }
}
impl HookManager {
  pub const fn new() -> Self {
    Self {
      version: None,
      modules: ArrayVec::new_const(),
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

  pub fn attach(&mut self) -> Result<(), ()> {
    let res = match self.version {
      Some(GameVersion::V109d) => unsafe { self.hook_v109d() },
      Some(GameVersion::V110) => unsafe { self.hook_v110() },
      Some(GameVersion::V112) => unsafe { self.hook_v112() },
      Some(GameVersion::V113c) => unsafe { self.hook_v113c() },
      Some(GameVersion::V113d) => unsafe { self.hook_v113d() },
      Some(GameVersion::V114d) => unsafe { self.hook_v114d() },
      _ => Ok(()),
    };

    if res.is_err() {
      self.patches.clear();
      self.modules.clear();
    }
    res
  }

  pub fn detach(&mut self) {
    unsafe {
      self.window_hook.detach(&self.accessor);
    }
    self.patches.clear();
    self.modules.clear();
  }

  unsafe fn apply_patches(
    &mut self,
    name: &str,
    base: usize,
    preferred_base: usize,
    patches: &[impl Patch],
  ) -> Result<(), ()> {
    let mut success = true;
    let reloc_dist = base.wrapping_sub(preferred_base);
    for patch in patches {
      match patch.apply(base, reloc_dist) {
        Ok(patch) => self.patches.push(patch),
        Err(_) => {
          success = false;
          log!("Failed to apply patch at: {}+{:#x}", name, patch.offset())
        }
      }
    }
    if success {
      Ok(())
    } else {
      Err(())
    }
  }

  unsafe fn load_dlls(&mut self) -> Result<&[Module; 5], ()> {
    assert!(self.modules.is_empty());
    self.modules.extend([
      Module::new(D2CLIENT_DLL)?,
      Module::new(D2COMMON_DLL)?,
      Module::new(D2GAME_DLL)?,
      Module::new(D2GFX_DLL)?,
      Module::new(D2WIN_DLL)?,
    ]);
    Ok(&*self.modules.as_ptr().cast())
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

unsafe extern "stdcall" fn draw_game<T: Entity>() {
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

    if instance.frame_rate != GAME_RATE {
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

unsafe extern "stdcall" fn draw_game_paused() {
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
) -> u32 {
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
  if instance.is_window_hidden {
    10
  } else {
    QueryPerformanceCounter(&mut time);
    (instance
      .perf_freq
      .sample_to_ms(instance.render_timer.next_update().saturating_sub(time as u64)) as u32)
      .saturating_sub(1)
  }
}

unsafe extern "fastcall" fn draw_menu_with_sleep(
  callback: Option<extern "stdcall" fn(u32)>,
  call_count: &mut u32,
) {
  Sleep(draw_menu(callback, call_count));
}

unsafe extern "stdcall" fn game_loop_sleep_hook(_: u32) {
  let instance = D2FPS.lock();
  let len = if instance.is_window_hidden {
    10
  } else {
    let mut time = 0;
    QueryPerformanceCounter(&mut time);
    (instance
      .perf_freq
      .sample_to_ms(instance.render_timer.next_update().saturating_sub(time as u64)) as u32)
      .saturating_sub(1)
      .min(10)
  };
  Sleep(len);
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
