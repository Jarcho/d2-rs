use crate::{
  features::{FeaturePatches, Features, ModulePatches},
  util::{hash_module_file, read_file_version, FileVersion},
  InstanceSync, GAME_FPS, INSTANCE,
};
use core::{
  hash::Hash,
  mem::{replace, take, transmute},
  ptr::NonNull,
  sync::atomic::Ordering::Relaxed,
};
use d2interface as d2;
use std::collections::hash_map::Entry;
use windows_sys::{
  w,
  Win32::{
    Foundation::HWND,
    Media::timeGetTime,
    System::{
      LibraryLoader::GetModuleHandleW, Performance::QueryPerformanceCounter,
      SystemInformation::GetTickCount, Threading::Sleep,
    },
  },
};

mod v100;
mod v101;
mod v102;
mod v103;
mod v104b;
mod v104c;
mod v105;
mod v105b;
mod v106;
mod v106b;
mod v107;
mod v108;
mod v109;
mod v109b;
mod v109d;
mod v110;
mod v111;
mod v111b;
mod v112;
mod v113c;
mod v113d;
mod v114a;
mod v114b;
mod v114c;
mod v114d;

const GAME_EXE: *const u16 = w!("game.exe");

struct Hooks {
  version: &'static str,
  patches: FeaturePatches,
  addresses: d2::Addresses,
  base_addresses: d2::BaseAddresses,
  load_modules: fn() -> Option<d2::Modules>,
}
impl Hooks {
  const UNKNOWN: &Hooks = &Hooks {
    version: "unknown",
    patches: FeaturePatches::empty(),
    addresses: d2::Addresses::ZERO,
    base_addresses: d2::BaseAddresses::ZERO,
    load_modules: d2::Modules::load_split_modules,
  };

  fn from_game_file_version(version: FileVersion) -> &'static Hooks {
    match (version.ms, version.ls) {
      (0x0001_0000, 0x0000_0001) => match hash_module_file(unsafe { GetModuleHandleW(GAME_EXE) }) {
        Some(0x5215437ecc8b67b9) => &v100::HOOKS,
        Some(0x1b093efaa009e78b) => &v101::HOOKS,
        _ => Self::UNKNOWN,
      },
      (0x0001_0000, 0x0002_0000) => &v102::HOOKS,
      (0x0001_0000, 0x0003_0000) => &v103::HOOKS,
      (0x0001_0000, 0x0004_0001) => &v104b::HOOKS,
      (0x0001_0000, 0x0004_0002) => &v104c::HOOKS,
      (0x0001_0000, 0x0005_0000) => &v105::HOOKS,
      (0x0001_0000, 0x0005_0001) => &v105b::HOOKS,
      (0x0001_0000, 0x0006_0000) => match hash_module_file(unsafe { GetModuleHandleW(GAME_EXE) }) {
        Some(0x73645dbfe51df9ae) => &v106::HOOKS,
        Some(0x62fea87b064aec9e) => &v106b::HOOKS,
        _ => Self::UNKNOWN,
      },
      (0x0001_0000, 0x0007_0000) => &v107::HOOKS,
      (0x0001_0000, 0x0008_001c) => &v108::HOOKS,
      (0x0001_0000, 0x0009_0013) => &v109::HOOKS,
      (0x0001_0000, 0x0009_0014) => &v109b::HOOKS,
      (0x0001_0000, 0x0009_0016) => &v109d::HOOKS,
      // (0x0001_0000, 0x000a_0009) => "1.10b",
      // (0x0001_0000, 0x000a_000a) => "1.10s",
      (0x0001_0000, 0x000a_0027) => &v110::HOOKS,
      (0x0001_0000, 0x000b_002d) => &v111::HOOKS,
      (0x0001_0000, 0x000b_002e) => &v111b::HOOKS,
      (0x0001_0000, 0x000c_0031) => &v112::HOOKS,
      // (0x0001_0000, 0x000d_0037) => "1.13a",
      (0x0001_0000, 0x000d_003c) => &v113c::HOOKS,
      (0x0001_0000, 0x000d_0040) => &v113d::HOOKS,
      (0x0001_000e, 0x0000_0040) => &v114a::HOOKS,
      (0x0001_000e, 0x0001_0044) => &v114b::HOOKS,
      (0x0001_000e, 0x0002_0046) => &v114c::HOOKS,
      (0x0001_000e, 0x0003_0047) => &v114d::HOOKS,
      _ => Self::UNKNOWN,
    }
  }
}

pub struct GameAccessor {
  pub player: NonNull<Option<NonNull<()>>>,
  pub env_effects: NonNull<d2::ClientEnvEffects>,
  pub game_type: NonNull<d2::GameType>,
  pub active_entities: NonNull<()>,
  pub client_loop_globals: NonNull<d2::ClientLoopGlobals>,
  pub apply_pos_change: usize,
  pub server_update_time: NonNull<u32>,
  pub in_perspective: unsafe extern "stdcall" fn() -> u32,
  pub get_hwnd: unsafe extern "stdcall" fn() -> HWND,
  pub draw_menu: unsafe extern "stdcall" fn(),
}
unsafe impl Send for GameAccessor {}
impl GameAccessor {
  pub const fn new() -> Self {
    Self {
      player: NonNull::dangling(),
      env_effects: NonNull::dangling(),
      game_type: NonNull::dangling(),
      active_entities: NonNull::dangling(),
      client_loop_globals: NonNull::dangling(),
      apply_pos_change: 0,
      server_update_time: NonNull::dangling(),
      in_perspective: {
        extern "stdcall" fn f() -> u32 {
          panic!()
        }
        f
      },
      get_hwnd: {
        extern "stdcall" fn f() -> HWND {
          panic!()
        }
        f
      },
      draw_menu: {
        extern "stdcall" fn f() {
          panic!()
        }
        f
      },
    }
  }

  unsafe fn load(&mut self, modules: &d2::Modules, addresses: &d2::Addresses) -> Result<(), ()> {
    self.player = addresses.player(modules.client());
    self.env_effects = addresses.env_effects(modules.client());
    self.game_type = addresses.game_type(modules.client());
    self.active_entities = addresses.active_entities(modules.client());
    self.client_loop_globals = addresses.client_loop_globals(modules.client());
    self.apply_pos_change = addresses.apply_pos_change(modules.common());
    self.server_update_time = addresses.server_update_time(modules.game());
    self.in_perspective = addresses.in_perspective(modules.gfx()).ok_or(())?;
    self.get_hwnd = addresses.hwnd(modules.gfx()).ok_or(())?;
    self.draw_menu = addresses.draw_menu(modules.win()).ok_or(())?;
    Ok(())
  }

  pub unsafe fn player<T>(&self) -> Option<NonNull<T>> {
    *self.player.cast().as_ptr()
  }

  pub unsafe fn active_entities<T>(&self) -> NonNull<d2::EntityTables<T>> {
    self.active_entities.cast()
  }
}

impl InstanceSync {
  pub fn attach(&mut self) {
    let hooks = match unsafe { read_file_version(GAME_EXE) } {
      Ok(version) => Hooks::from_game_file_version(version),
      Err(_) => {
        log!("Error detecting game version");
        INSTANCE.config.features.store_relaxed(Features::empty());
        return;
      }
    };
    log!("Detected game version: {}", hooks.version);

    let Some(modules) = (hooks.load_modules)() else {
      log!("Disabling all features: failed to load game modules");
      INSTANCE.config.features.store_relaxed(Features::empty());
      return;
    };
    if unsafe { self.accessor.load(&modules, &hooks.addresses).is_err() } {
      log!("Disabling all features: failed to load game addresses");
      INSTANCE.config.features.store_relaxed(Features::empty());
      return;
    }

    for (feature, patches) in hooks
      .patches
      .iter()
      .filter(|(f, _)| INSTANCE.config.features.load_relaxed().intersects(f.as_flag()))
    {
      if !INSTANCE.config.features.load_relaxed().contains(feature.prereqs()) {
        log!(
          "Disabling feature `{feature}`: missing prerequisite features {}",
          feature.prereqs().difference(INSTANCE.config.features.load_relaxed()),
        );
      } else if patches.is_empty() {
        log!("Disabling feature `{feature}`: unsupported version");
      } else if unsafe { apply_patch_set(&modules, &hooks.base_addresses, patches).is_err() } {
        log!("Disabling feature `{feature}`: failed to apply patches");
      } else {
        log!("Applied feature `{feature}`");
        continue;
      }
      INSTANCE.config.features.remove_relaxed(feature.as_flag());
    }
  }
}

unsafe fn apply_patch_set(
  modules: &d2::Modules,
  base_addresses: &d2::BaseAddresses,
  mod_patches: &[ModulePatches],
) -> Result<(), ()> {
  let mut success = true;

  for m in mod_patches {
    let d2mod = modules[m.module];
    let reloc_dist = d2mod.wrapping_sub(base_addresses[m.module] as isize);
    for p in m.patches {
      if !p.has_expected(d2mod, reloc_dist) {
        success = false;
        log!("Failed to apply patch at: {}+{:#x}", m.module, p.offset);
      }
    }
  }
  if !success {
    return Err(());
  }
  for m in mod_patches {
    let d2mod = modules[m.module];
    for p in m.patches {
      p.apply(d2mod)
    }
  }
  Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnitId {
  pub kind: d2::EntityKind,
  pub id: u32,
}
impl UnitId {
  pub const fn new(kind: d2::EntityKind, id: u32) -> Self {
    Self { kind, id }
  }
}

#[derive(Clone, Copy)]
pub struct Position {
  pub real: d2::LinearPos<d2::FixedU16>,
  pub delta: d2::LinearPos<d2::FixedI16>,
  pub teleport: bool,
  pub active: bool,
}
impl Position {
  pub fn for_time(&self, fract: d2::FixedI16) -> d2::LinearPos<d2::FixedU16> {
    let x = ((self.delta.x.repr() as i64 * fract.repr() as i64) >> 16) as u32;
    let y = ((self.delta.y.repr() as i64 * fract.repr() as i64) >> 16) as u32;
    let x = self.real.x.repr().wrapping_add(x);
    let y = self.real.y.repr().wrapping_add(y);
    d2::LinearPos::new(d2::FixedU16::from_repr(x), d2::FixedU16::from_repr(y))
  }

  fn update_pos(&mut self, pos: d2::LinearPos<d2::FixedU16>) {
    let dx = pos.x.repr().wrapping_sub(self.real.x.repr()) as i32;
    let dy = pos.y.repr().wrapping_sub(self.real.y.repr()) as i32;
    self.real = pos;
    self.delta = if self.teleport {
      d2::LinearPos::default()
    } else {
      d2::LinearPos::new(d2::FixedI16::from_repr(dx), d2::FixedI16::from_repr(dy))
    };
    self.teleport = false;
  }

  fn new(pos: d2::LinearPos<d2::FixedU16>) -> Self {
    Self {
      real: pos,
      delta: d2::LinearPos::default(),
      teleport: false,
      active: true,
    }
  }
}

trait Entity: d2::LinkedList {
  fn unit_id(&self) -> UnitId;
  fn has_room(&self) -> bool;
  fn linear_pos(&self) -> d2::LinearPos<d2::FixedU16>;
  fn iso_pos(&self) -> d2::IsoPos<i32>;
  fn set_pos(&mut self, pos: d2::LinearPos<d2::FixedU16>);
}

impl InstanceSync {
  unsafe fn hook_window(&self) {
    if INSTANCE.window_hook.attach(&self.accessor) && INSTANCE.config.fps.load_relaxed().num == 0 {
      INSTANCE.frame_rate_from_window((self.accessor.get_hwnd)());
    }
  }

  fn entity_adjusted_pos(&mut self, e: &impl Entity) -> Option<d2::LinearPos<d2::FixedU16>> {
    self
      .entity_tracker
      .as_mut()
      .unwrap()
      .get(&e.unit_id())
      .map(|pos| pos.for_time(self.unit_offset))
  }

  fn entity_linear_pos(&mut self, e: &impl Entity) -> d2::LinearPos<d2::FixedU16> {
    match self.entity_adjusted_pos(e) {
      Some(pos) => pos,
      None => e.linear_pos(),
    }
  }

  fn entity_iso_pos(&mut self, e: &impl Entity) -> d2::IsoPos<i32> {
    match self.entity_adjusted_pos(e) {
      Some(pos) => d2::IsoPos::from(pos),
      None => e.iso_pos(),
    }
  }

  unsafe fn update_game_time(&mut self, time: i64) -> bool {
    let is_sp = self.accessor.game_type.as_ref().is_sp();
    let prev_update_time = self.game_update_time_ms;
    self.game_update_time_ms = if is_sp {
      *self.accessor.server_update_time.as_ptr()
    } else {
      self.accessor.client_loop_globals.as_ref().last_update
    };
    if self.game_update_time_ms != prev_update_time {
      let cur_time_ms = if is_sp {
        timeGetTime() & 0x7FFFFFFF
      } else {
        GetTickCount()
      };
      if self.game_update_time_ms < cur_time_ms {
        self.game_update_time = INSTANCE.perf_freq.ms_to_sample(
          INSTANCE.perf_freq.sample_to_ms(time as u64)
            - (cur_time_ms - self.game_update_time_ms) as u64,
        );
      } else {
        // Fallback time for when the clock wraps around.
        // Will be corrected next frame.
        self.game_update_time = time as u64;
      }
      true
    } else {
      false
    }
  }

  unsafe fn update_entites_from_tables<T: Entity>(&mut self) {
    let tracker = self.entity_tracker.as_mut().unwrap();
    self.accessor.active_entities::<T>().as_mut().for_each_dy(|e| {
      match tracker.entry(e.unit_id()) {
        Entry::Occupied(mut x) => {
          x.get_mut().update_pos(e.linear_pos());
          x.get_mut().active = true;
        }
        Entry::Vacant(x) => {
          x.insert(Position::new(e.linear_pos()));
        }
      }
    });
    tracker.retain(|_, v| take(&mut v.active));
  }

  unsafe fn update_entites_from_tables_no_delta<T: Entity>(&mut self) {
    let tracker = self.entity_tracker.as_mut().unwrap();
    self.accessor.active_entities::<T>().as_mut().for_each_dy(|e| {
      if let Some(pos) = tracker.get_mut(&e.unit_id()) {
        let epos = e.linear_pos();
        if pos.real != epos && pos.teleport {
          pos.delta = d2::LinearPos::default();
          pos.teleport = false;
        }
        pos.real = e.linear_pos();
      }
    });
  }

  unsafe fn update_entity_positions<T: Entity>(&mut self) {
    let frame_len = INSTANCE.perf_freq.ms_to_sample(40) as i64;
    let since_update = self.render_timer.last_update().wrapping_sub(self.game_update_time) as i64;
    let since_update = since_update.min(frame_len);
    let offset = since_update - frame_len;
    let fract = (offset << 16) / frame_len;
    self.unit_offset = d2::FixedI16::from_repr(fract as i32);

    let tracker = self.entity_tracker.as_ref().unwrap();
    self.accessor.active_entities::<T>().as_mut().for_each_dy_mut(|e| {
      if let Some(pos) = tracker.get(&e.unit_id()) {
        e.set_pos(pos.for_time(self.unit_offset));
      }
    });
  }

  unsafe fn reset_entity_positions<T: Entity>(&mut self) {
    let tracker = self.entity_tracker.as_ref().unwrap();
    self.accessor.active_entities::<T>().as_mut().for_each_dy_mut(|e| {
      if let Some(pos) = tracker.get(&e.unit_id()) {
        e.set_pos(pos.real);
      }
    });
  }

  unsafe fn update_env_images(&mut self, prev_pos: d2::IsoPos<i32>) {
    if (self.accessor.in_perspective)() == 0 {
      let dx = prev_pos.x.wrapping_sub(self.player_pos.x) as u32;
      let dy = prev_pos.y.wrapping_sub(self.player_pos.y) as u32;

      if let Some(mut splashes) = self.accessor.env_effects.as_mut().splashes {
        for splash in splashes.as_mut().as_mut_slice() {
          splash.pos.x = splash.pos.x.wrapping_add(dx);
          splash.pos.y = splash.pos.y.wrapping_add(dy);
        }
      }
      if let Some(mut bubbles) = self.accessor.env_effects.as_mut().bubbles {
        for bubble in bubbles.as_mut().as_mut_slice() {
          bubble.pos.x = bubble.pos.x.wrapping_add(dx);
          bubble.pos.y = bubble.pos.y.wrapping_add(dy);
        }
      }
    }
  }
}

extern "stdcall" fn entity_iso_xpos<E: Entity>(e: &E) -> i32 {
  INSTANCE.sync.lock().entity_iso_pos(e).x
}
extern "stdcall" fn entity_iso_ypos<E: Entity>(e: &E) -> i32 {
  INSTANCE.sync.lock().entity_iso_pos(e).y
}

extern "stdcall" fn entity_linear_xpos<E: Entity>(e: &E) -> d2::FixedU16 {
  INSTANCE.sync.lock().entity_linear_pos(e).x
}
extern "stdcall" fn entity_linear_ypos<E: Entity>(e: &E) -> d2::FixedU16 {
  INSTANCE.sync.lock().entity_linear_pos(e).y
}

unsafe extern "C" fn draw_game<E: Entity>() {
  let mut lock = INSTANCE.sync.lock();
  let sync_instance = &mut *lock;
  sync_instance.hook_window();

  let Some(player) = sync_instance.accessor.player::<E>() else {
    return;
  };
  if !player.as_ref().has_room() {
    return;
  }

  let mut time = 0i64;
  QueryPerformanceCounter(&mut time);
  if sync_instance
    .render_timer
    .update_time(time as u64, INSTANCE.render_fps.load_relaxed())
  {
    let enable_smoothing =
      INSTANCE.render_fps.load_relaxed() != GAME_FPS && INSTANCE.config.features.motion_smoothing();
    let prev_update_count = replace(
      &mut sync_instance.client_update_count,
      sync_instance.accessor.client_loop_globals.as_ref().updates,
    );

    if enable_smoothing {
      if sync_instance.update_game_time(time) {
        sync_instance.update_entites_from_tables::<E>();
      } else {
        sync_instance.update_entites_from_tables_no_delta::<E>();
      }
      sync_instance.update_entity_positions::<E>();

      let prev_player_pos = sync_instance.player_pos;
      sync_instance.player_pos = sync_instance.entity_iso_pos(player.as_ref());

      if sync_instance.client_update_count == prev_update_count {
        sync_instance.update_env_images(prev_player_pos);
      }
    } else {
      sync_instance.unit_offset = d2::FixedI16::default();
      sync_instance.player_pos = player.as_ref().iso_pos();
    }

    let draw = sync_instance.accessor.client_loop_globals.as_ref().draw_fn;
    let unit_offset = take(&mut sync_instance.unit_offset);
    drop(lock);
    draw(0);

    let mut lock = INSTANCE.sync.lock();
    let sync_instance = &mut *lock;

    sync_instance.unit_offset = unit_offset;
    sync_instance.accessor.client_loop_globals.as_mut().frames_drawn += 1;
    sync_instance
      .accessor
      .client_loop_globals
      .as_mut()
      .fps_timer
      .frames_drawn += 1;

    if enable_smoothing {
      sync_instance.reset_entity_positions::<E>();
    }
  }
}

unsafe extern "C" fn draw_game_paused() {
  let mut lock = INSTANCE.sync.lock();
  let sync_instance = &mut *lock;

  let mut cur_time = 0i64;
  QueryPerformanceCounter(&mut cur_time);

  if sync_instance
    .render_timer
    .update_time(cur_time as u64, INSTANCE.render_fps.load_relaxed())
  {
    let draw = sync_instance.accessor.client_loop_globals.as_ref().draw_fn;
    drop(lock);
    draw(0);
  }
}

unsafe extern "fastcall" fn draw_menu(
  callback: Option<extern "stdcall" fn(u32)>,
  call_count: &mut u32,
) {
  let mut lock = INSTANCE.sync.lock();
  let sync_instance = &mut *lock;
  sync_instance.hook_window();

  let mut time = 0i64;
  QueryPerformanceCounter(&mut time);

  if sync_instance
    .render_timer
    .update_time(time as u64, INSTANCE.render_fps.load_relaxed())
  {
    if sync_instance.menu_timer.update_time(
      sync_instance.render_timer.last_update(),
      transmute(callback),
    ) {
      INSTANCE.menu_timer_updated.store(true, Relaxed);
      if let Some(callback) = callback {
        callback(*call_count);
        *call_count += 1;
      }
    } else {
      INSTANCE.menu_timer_updated.store(false, Relaxed);
    }

    let draw = sync_instance.accessor.draw_menu;
    drop(lock);
    draw();

    lock = INSTANCE.sync.lock();
  }

  let sync_instance = &mut *lock;
  QueryPerformanceCounter(&mut time);
  let sleep_len = (INSTANCE
    .perf_freq
    .sample_to_ms(sync_instance.render_timer.next_update().saturating_sub(time as u64))
    as u32)
    .saturating_sub(1)
    .min(10);
  let sleep_len = if INSTANCE.is_window_hidden.load(Relaxed) {
    10
  } else {
    sleep_len
  };
  Sleep(sleep_len);
}

unsafe extern "C" fn game_loop_sleep_hook() {
  let mut lock = INSTANCE.sync.lock();
  let sync_instance = &mut *lock;

  let mut time = 0;
  QueryPerformanceCounter(&mut time);
  let len = (INSTANCE
    .perf_freq
    .sample_to_ms(sync_instance.render_timer.next_update().saturating_sub(time as u64))
    as u32)
    .saturating_sub(1);
  let len = if INSTANCE.is_window_hidden.load(Relaxed) {
    10
  } else {
    len
  };
  let limit = if sync_instance.accessor.game_type.as_ref().is_host() {
    2
  } else {
    10
  };
  Sleep(len.min(limit));
}

unsafe extern "fastcall" fn update_menu_char_frame(rate: u32, frame: &mut u32) -> u32 {
  if INSTANCE.menu_timer_updated.load(Relaxed) {
    *frame += rate;
  }
  *frame
}

unsafe extern "fastcall" fn intercept_teleport(kind: d2::EntityKind, id: u32) -> usize {
  let mut lock = INSTANCE.sync.lock();
  let sync_instance = &mut *lock;

  if let Some(pos) = sync_instance
    .entity_tracker
    .as_mut()
    .unwrap()
    .get_mut(&UnitId { kind, id })
  {
    pos.teleport = true;
  }
  sync_instance.accessor.apply_pos_change
}
