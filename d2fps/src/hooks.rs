use crate::{
  config::Config,
  tracker::UnitId,
  util::{hash_module_file, read_file_version, FileVersion},
  D2Fps, D2FPS, GAME_FPS,
};
use bin_patch::Patch;
use core::{
  mem::{replace, take, transmute},
  ptr::NonNull,
};
use d2interface as d2;
use windows_sys::{
  w,
  Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    Media::{timeBeginPeriod, timeEndPeriod, timeGetTime},
    System::{
      LibraryLoader::GetModuleHandleW, Performance::QueryPerformanceCounter,
      SystemInformation::GetTickCount, Threading::Sleep,
    },
    UI::{
      Shell::{DefSubclassProc, SetWindowSubclass},
      WindowsAndMessaging::{SIZE_MINIMIZED, WM_ACTIVATE, WM_SIZE, WM_WINDOWPOSCHANGED},
    },
  },
};

mod v100;
mod v101;
mod v102;
mod v103;
mod v104b;
mod v105;
mod v106;
mod v106b;
mod v107;
mod v108;
mod v109;
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

struct ModulePatches {
  module: d2::Module,
  patches: &'static [Patch],
}
impl ModulePatches {
  const fn new(module: d2::Module, patches: &'static [Patch]) -> Self {
    Self { module, patches }
  }
}

struct PatchSets {
  menu_fps: &'static [ModulePatches],
  game_fps: &'static [ModulePatches],
  game_smoothing: &'static [ModulePatches],
}

struct HookSet {
  version: &'static str,
  patch_sets: PatchSets,
  addresses: d2::Addresses,
  base_addresses: d2::BaseAddresses,
  load_modules: fn() -> Option<d2::Modules>,
}
impl HookSet {
  const UNKNOWN: &HookSet = &HookSet {
    version: "unknown",
    patch_sets: PatchSets { menu_fps: &[], game_fps: &[], game_smoothing: &[] },
    addresses: d2::Addresses::ZERO,
    base_addresses: d2::BaseAddresses::ZERO,
    load_modules: d2::Modules::load_split_modules,
  };

  fn from_game_file_version(version: FileVersion) -> &'static HookSet {
    match (version.ms, version.ls) {
      (0x0001_0000, 0x0000_0001) => match hash_module_file(unsafe { GetModuleHandleW(GAME_EXE) }) {
        Some(0x5215437ecc8b67b9) => &HookSet {
          version: "1.00",
          patch_sets: v100::PATCHES,
          addresses: d2::v100::ADDRESSES,
          base_addresses: d2::v100::BASE_ADDRESSES,
          load_modules: d2::Modules::load_split_modules,
        },
        Some(0x1b093efaa009e78b) => &HookSet {
          version: "1.01",
          patch_sets: v101::PATCHES,
          addresses: d2::v101::ADDRESSES,
          base_addresses: d2::v101::BASE_ADDRESSES,
          load_modules: d2::Modules::load_split_modules,
        },
        _ => Self::UNKNOWN,
      },
      (0x0001_0000, 0x0002_0000) => &HookSet {
        version: "1.02",
        patch_sets: v102::PATCHES,
        addresses: d2::v102::ADDRESSES,
        base_addresses: d2::v102::BASE_ADDRESSES,
        load_modules: d2::Modules::load_split_modules,
      },
      (0x0001_0000, 0x0003_0000) => &HookSet {
        version: "1.03",
        patch_sets: v103::PATCHES,
        addresses: d2::v103::ADDRESSES,
        base_addresses: d2::v103::BASE_ADDRESSES,
        load_modules: d2::Modules::load_split_modules,
      },
      (0x0001_0000, 0x0004_0001) => &HookSet {
        version: "1.04b",
        patch_sets: v104b::PATCHES,
        addresses: d2::v104b::ADDRESSES,
        base_addresses: d2::v104b::BASE_ADDRESSES,
        load_modules: d2::Modules::load_split_modules,
      },
      (0x0001_0000, 0x0004_0002) => &HookSet {
        version: "1.04c",
        // Uses the same dll files a 1.04b
        patch_sets: v104b::PATCHES,
        addresses: d2::v104b::ADDRESSES,
        base_addresses: d2::v104b::BASE_ADDRESSES,
        load_modules: d2::Modules::load_split_modules,
      },
      (0x0001_0000, 0x0005_0000) => &HookSet {
        version: "1.05",
        patch_sets: v105::PATCHES,
        addresses: d2::v105::ADDRESSES,
        base_addresses: d2::v105::BASE_ADDRESSES,
        load_modules: d2::Modules::load_split_modules,
      },
      (0x0001_0000, 0x0005_0001) => &HookSet {
        version: "1.05b",
        // Uses the same dll files a 1.05
        patch_sets: v105::PATCHES,
        addresses: d2::v105::ADDRESSES,
        base_addresses: d2::v105::BASE_ADDRESSES,
        load_modules: d2::Modules::load_split_modules,
      },
      (0x0001_0000, 0x0006_0000) => match hash_module_file(unsafe { GetModuleHandleW(GAME_EXE) }) {
        Some(0x73645dbfe51df9ae) => &HookSet {
          version: "1.06",
          patch_sets: v106::PATCHES,
          addresses: d2::v106::ADDRESSES,
          base_addresses: d2::v106::BASE_ADDRESSES,
          load_modules: d2::Modules::load_split_modules,
        },
        Some(0x62fea87b064aec9e) => &HookSet {
          version: "1.06b",
          patch_sets: v106b::PATCHES,
          addresses: d2::v106b::ADDRESSES,
          base_addresses: d2::v106b::BASE_ADDRESSES,
          load_modules: d2::Modules::load_split_modules,
        },
        _ => Self::UNKNOWN,
      },
      (0x0001_0000, 0x0007_0000) => &HookSet {
        version: "1.07",
        patch_sets: v107::PATCHES,
        addresses: d2::v107::ADDRESSES,
        base_addresses: d2::v107::BASE_ADDRESSES,
        load_modules: d2::Modules::load_split_modules,
      },
      (0x0001_0000, 0x0008_001c) => &HookSet {
        version: "1.08",
        patch_sets: v108::PATCHES,
        addresses: d2::v108::ADDRESSES,
        base_addresses: d2::v108::BASE_ADDRESSES,
        load_modules: d2::Modules::load_split_modules,
      },
      (0x0001_0000, 0x0009_0013) => &HookSet {
        version: "1.09",
        patch_sets: v109::PATCHES,
        addresses: d2::v109::ADDRESSES,
        base_addresses: d2::v109::BASE_ADDRESSES,
        load_modules: d2::Modules::load_split_modules,
      },
      (0x0001_0000, 0x0009_0014) => &HookSet {
        version: "1.09b",
        // Uses the same dll files a 1.09
        patch_sets: v109::PATCHES,
        addresses: d2::v109::ADDRESSES,
        base_addresses: d2::v109::BASE_ADDRESSES,
        load_modules: d2::Modules::load_split_modules,
      },
      (0x0001_0000, 0x0009_0016) => &HookSet {
        version: "1.09d",
        patch_sets: v109d::PATCHES,
        addresses: d2::v109d::ADDRESSES,
        base_addresses: d2::v109d::BASE_ADDRESSES,
        load_modules: d2::Modules::load_split_modules,
      },
      // (0x0001_0000, 0x000a_0009) => "1.10b",
      // (0x0001_0000, 0x000a_000a) => "1.10s",
      (0x0001_0000, 0x000a_0027) => &HookSet {
        version: "1.10",
        patch_sets: v110::PATCHES,
        addresses: d2::v110::ADDRESSES,
        base_addresses: d2::v110::BASE_ADDRESSES,
        load_modules: d2::Modules::load_split_modules,
      },
      (0x0001_0000, 0x000b_002d) => &HookSet {
        version: "1.11",
        patch_sets: v111::PATCHES,
        addresses: d2::v111::ADDRESSES,
        base_addresses: d2::v111::BASE_ADDRESSES,
        load_modules: d2::Modules::load_split_modules,
      },
      (0x0001_0000, 0x000b_002e) => &HookSet {
        version: "1.11b",
        patch_sets: v111b::PATCHES,
        addresses: d2::v111b::ADDRESSES,
        base_addresses: d2::v111b::BASE_ADDRESSES,
        load_modules: d2::Modules::load_split_modules,
      },
      (0x0001_0000, 0x000c_0031) => &HookSet {
        version: "1.12",
        patch_sets: v112::PATCHES,
        addresses: d2::v112::ADDRESSES,
        base_addresses: d2::v112::BASE_ADDRESSES,
        load_modules: d2::Modules::load_split_modules,
      },
      // (0x0001_0000, 0x000d_0037) => "1.13a",
      (0x0001_0000, 0x000d_003c) => &HookSet {
        version: "1.13c",
        patch_sets: v113c::PATCHES,
        addresses: d2::v113c::ADDRESSES,
        base_addresses: d2::v113c::BASE_ADDRESSES,
        load_modules: d2::Modules::load_split_modules,
      },
      (0x0001_0000, 0x000d_0040) => &HookSet {
        version: "1.13d",
        patch_sets: v113d::PATCHES,
        addresses: d2::v113d::ADDRESSES,
        base_addresses: d2::v113d::BASE_ADDRESSES,
        load_modules: d2::Modules::load_split_modules,
      },
      (0x0001_000e, 0x0000_0040) => &HookSet {
        version: "1.14a",
        patch_sets: v114a::PATCHES,
        addresses: d2::v114a::ADDRESSES,
        base_addresses: d2::v114a::BASE_ADDRESSES,
        load_modules: d2::Modules::load_combined_module,
      },
      (0x0001_000e, 0x0001_0044) => &HookSet {
        version: "1.14b",
        patch_sets: v114b::PATCHES,
        addresses: d2::v114b::ADDRESSES,
        base_addresses: d2::v114b::BASE_ADDRESSES,
        load_modules: d2::Modules::load_combined_module,
      },
      (0x0001_000e, 0x0002_0046) => &HookSet {
        version: "1.14c",
        patch_sets: v114c::PATCHES,
        addresses: d2::v114c::ADDRESSES,
        base_addresses: d2::v114c::BASE_ADDRESSES,
        load_modules: d2::Modules::load_combined_module,
      },
      (0x0001_000e, 0x0003_0047) => &HookSet {
        version: "1.14d",
        patch_sets: v114d::PATCHES,
        addresses: d2::v114d::ADDRESSES,
        base_addresses: d2::v114d::BASE_ADDRESSES,
        load_modules: d2::Modules::load_combined_module,
      },
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
  const fn new() -> Self {
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
          timeBeginPeriod(1);
          instance
            .render_timer
            .switch_rate(&instance.perf_freq, instance.frame_rate);
        } else {
          timeEndPeriod(1);
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
      let hwnd = (accessor.get_hwnd)();
      if hwnd != 0 {
        self.0 = true;
        SetWindowSubclass(hwnd, Some(win_proc), Self::ID, 0);
        return true;
      }
    }
    false
  }
}

pub struct HookManager {
  hook_set: &'static HookSet,
  accessor: GameAccessor,
  window_hook: WindowHook,
}
impl HookManager {
  pub const fn new() -> Self {
    Self {
      hook_set: HookSet::UNKNOWN,
      accessor: GameAccessor::new(),
      window_hook: WindowHook(false),
    }
  }

  pub fn init(&mut self) {
    match unsafe { read_file_version(GAME_EXE) } {
      Ok(version) => {
        self.hook_set = HookSet::from_game_file_version(version);
        log!("Detected game version: {}", self.hook_set.version);
      }
      Err(_) => {
        log!("Error detecting game version")
      }
    }
  }

  pub(crate) fn attach(&mut self, config: &mut Config) {
    let Some(modules) = (self.hook_set.load_modules)() else {
      log!("Disabling all features: failed to load game modules");
      config.enable_smoothing = false;
      return;
    };
    if unsafe { self.accessor.load(&modules, &self.hook_set.addresses).is_err() } {
      log!("Disabling all features: failed to load game addresses");
      config.enable_smoothing = false;
      return;
    }

    if self.hook_set.patch_sets.menu_fps.is_empty() {
      log!("Disabling menu frame rate unlock: unsupported version");
    } else if unsafe {
      self
        .apply_patch_set(&modules, self.hook_set.patch_sets.menu_fps)
        .is_err()
    } {
      log!("Disabling menu frame rate unlock: failed to apply patches");
    }

    let mut game_fps = true;
    if self.hook_set.patch_sets.game_fps.is_empty() {
      log!("Disabling game frame rate unlock: unsupported version");
      game_fps = false;
    } else if unsafe {
      self
        .apply_patch_set(&modules, self.hook_set.patch_sets.game_fps)
        .is_err()
    } {
      log!("Disabling game frame rate unlock: failed to apply patches");
      game_fps = false;
    }

    if config.enable_smoothing {
      if self.hook_set.patch_sets.game_smoothing.is_empty() {
        log!("Disabling game motion smoothing: unsupported version");
        config.enable_smoothing = false;
      } else if !game_fps {
        log!("Disabling game motion smoothing: game frame rate must be unlocked");
        config.enable_smoothing = false;
      } else if unsafe {
        self
          .apply_patch_set(&modules, self.hook_set.patch_sets.game_smoothing)
          .is_err()
      } {
        log!("Failed to apply game motion smoothing patches, disabling feature");
        config.enable_smoothing = false;
      }
    }
  }

  unsafe fn apply_patch_set(
    &mut self,
    modules: &d2::Modules,
    mod_patches: &[ModulePatches],
  ) -> Result<(), ()> {
    let mut success = true;

    for m in mod_patches {
      let d2mod = modules[m.module];
      let reloc_dist = d2mod.wrapping_sub(self.hook_set.base_addresses[m.module] as isize);
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
}

trait Entity: d2::LinkedList {
  fn unit_id(&self) -> UnitId;
  fn has_room(&self) -> bool;
  fn linear_pos(&self) -> d2::LinearPos<d2::FixedU16>;
  fn iso_pos(&self) -> d2::IsoPos<i32>;
  fn set_pos(&mut self, pos: d2::LinearPos<d2::FixedU16>);
}

impl D2Fps {
  fn entity_adjusted_pos(&mut self, e: &impl Entity) -> Option<d2::LinearPos<d2::FixedU16>> {
    self
      .entity_tracker
      .get(e.unit_id())
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
    let is_sp = self.hooks.accessor.game_type.as_ref().is_sp();
    let prev_update_time = self.game_update_time_ms;
    self.game_update_time_ms = if is_sp {
      *self.hooks.accessor.server_update_time.as_ptr()
    } else {
      self.hooks.accessor.client_loop_globals.as_ref().last_update
    };
    if self.game_update_time_ms != prev_update_time {
      let cur_time_ms = if is_sp {
        timeGetTime() & 0x7FFFFFFF
      } else {
        GetTickCount()
      };
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
      true
    } else {
      false
    }
  }

  unsafe fn update_entites_from_tables<T: Entity>(&mut self) {
    self.hooks.accessor.active_entities::<T>().as_mut().for_each_dy(|e| {
      self.entity_tracker.insert_or_update(e.unit_id(), e.linear_pos());
    });
    self.entity_tracker.clear_unused();
  }

  unsafe fn update_entites_from_tables_no_delta<T: Entity>(&mut self) {
    self.hooks.accessor.active_entities::<T>().as_mut().for_each_dy(|e| {
      if let Some(pos) = self.entity_tracker.get(e.unit_id()) {
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
    let frame_len = self.perf_freq.ms_to_sample(40) as i64;
    let since_update = self.render_timer.last_update().wrapping_sub(self.game_update_time) as i64;
    let since_update = since_update.min(frame_len);
    let offset = since_update - frame_len;
    let fract = (offset << 16) / frame_len;
    self.unit_offset = d2::FixedI16::from_repr(fract as i32);

    self
      .hooks
      .accessor
      .active_entities::<T>()
      .as_mut()
      .for_each_dy_mut(|e| {
        if let Some(pos) = self.entity_tracker.get(e.unit_id()) {
          e.set_pos(pos.for_time(self.unit_offset));
        }
      });
  }

  unsafe fn reset_entity_positions<T: Entity>(&mut self) {
    self
      .hooks
      .accessor
      .active_entities::<T>()
      .as_mut()
      .for_each_dy_mut(|e| {
        if let Some(pos) = self.entity_tracker.get(e.unit_id()) {
          e.set_pos(pos.real);
        }
      });
  }

  unsafe fn update_env_images(&mut self, prev_pos: d2::IsoPos<i32>) {
    if (self.hooks.accessor.in_perspective)() == 0 {
      let dx = prev_pos.x.wrapping_sub(self.player_pos.x) as u32;
      let dy = prev_pos.y.wrapping_sub(self.player_pos.y) as u32;

      if let Some(mut splashes) = self.hooks.accessor.env_effects.as_mut().splashes {
        for splash in splashes.as_mut().as_mut_slice() {
          splash.pos.x = splash.pos.x.wrapping_add(dx);
          splash.pos.y = splash.pos.y.wrapping_add(dy);
        }
      }
      if let Some(mut bubbles) = self.hooks.accessor.env_effects.as_mut().bubbles {
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

extern "stdcall" fn entity_linear_xpos<E: Entity>(e: &E) -> d2::FixedU16 {
  D2FPS.lock().entity_linear_pos(e).x
}
extern "stdcall" fn entity_linear_ypos<E: Entity>(e: &E) -> d2::FixedU16 {
  D2FPS.lock().entity_linear_pos(e).y
}

unsafe extern "C" fn draw_game<E: Entity>() {
  let mut instance_lock = D2FPS.lock();
  let instance = &mut *instance_lock;

  let Some(player) = instance.hooks.accessor.player::<E>() else {
    return;
  };
  if !player.as_ref().has_room() {
    return;
  }

  let mut time = 0i64;
  QueryPerformanceCounter(&mut time);
  if instance.render_timer.update_time(time as u64, &instance.perf_freq) {
    let enable_smoothing = instance.frame_rate != GAME_FPS && instance.config.enable_smoothing;
    let prev_update_count = replace(
      &mut instance.client_update_count,
      instance.hooks.accessor.client_loop_globals.as_ref().updates,
    );

    if enable_smoothing {
      if instance.update_game_time(time) {
        instance.update_entites_from_tables::<E>();
      } else {
        instance.update_entites_from_tables_no_delta::<E>();
      }
      instance.update_entity_positions::<E>();

      let prev_player_pos = instance.player_pos;
      instance.player_pos = instance.entity_iso_pos(player.as_ref());

      if instance.client_update_count == prev_update_count {
        instance.update_env_images(prev_player_pos);
      }
    } else {
      instance.unit_offset = d2::FixedI16::default();
      instance.player_pos = player.as_ref().iso_pos();
    }

    let draw = instance.hooks.accessor.client_loop_globals.as_ref().draw_fn;
    let unit_offset = take(&mut instance.unit_offset);
    drop(instance_lock);
    draw(0);

    let mut instance_lock = D2FPS.lock();
    let instance = &mut *instance_lock;

    instance.unit_offset = unit_offset;
    instance.hooks.accessor.client_loop_globals.as_mut().frames_drawn += 1;
    instance
      .hooks
      .accessor
      .client_loop_globals
      .as_mut()
      .fps_timer
      .frames_drawn += 1;

    if enable_smoothing {
      instance.reset_entity_positions::<E>();
    }
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
    let draw = instance.hooks.accessor.client_loop_globals.as_ref().draw_fn;
    drop(instance_lock);
    draw(0);
  }
}

unsafe extern "fastcall" fn draw_menu(
  callback: Option<extern "stdcall" fn(u32)>,
  call_count: &mut u32,
) {
  let mut instance_lock = D2FPS.lock();
  let instance = &mut *instance_lock;
  if instance.hooks.window_hook.attach(&instance.hooks.accessor) && instance.config.fps.is_none() {
    let hwnd = (instance.hooks.accessor.get_hwnd)();
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

unsafe extern "fastcall" fn intercept_teleport(kind: d2::EntityKind, id: u32) -> usize {
  let mut instance = D2FPS.lock();
  if let Some(pos) = instance.entity_tracker.get(UnitId { kind, id }) {
    pos.teleport = true;
  }
  instance.hooks.accessor.apply_pos_change
}
