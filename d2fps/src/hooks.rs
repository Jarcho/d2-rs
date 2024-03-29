use crate::{
  features::{FeaturePatches, Features, ModulePatches},
  util::{hash_module_file, read_file_version, FileVersion},
  InstanceSync, GAME_FPS, INSTANCE,
};
use core::{
  hash::Hash,
  mem::{replace, take},
  ptr::{null, null_mut, NonNull},
  sync::atomic::Ordering::Relaxed,
};
use d2interface::{self as d2, IntoSys};
use fxhash::FxHashSet as HashSet;
use num::{M2d, WrappingAdd, WrappingFrom, WrappingInto, WrappingSub};
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
mod v105;
mod v106a;
mod v106b;
mod v107;
mod v108;
mod v109a;
mod v109d;
mod v110;
mod v111a;
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
  patches: FeaturePatches,
  addresses: d2::Addresses,
  base_addresses: d2::BaseAddresses,
  load_modules: fn() -> Option<d2::Modules>,
  helper_fns: HelperFns,
}
impl Hooks {
  const UNKNOWN: &'static Hooks = &Hooks {
    patches: FeaturePatches::empty(),
    addresses: d2::Addresses::ZERO,
    base_addresses: d2::BaseAddresses::ZERO,
    load_modules: d2::Modules::load_split_modules,
    helper_fns: HelperFns::INIT,
  };

  fn from_game_file_version(version: FileVersion) -> (&'static str, &'static Hooks, bool) {
    match (version.ms, version.ls) {
      (0x0001_0000, 0x0000_0001) => match hash_module_file(unsafe { GetModuleHandleW(GAME_EXE) }) {
        Some(0x5215437ecc8b67b9) => ("v1.00", &v100::HOOKS, false),
        Some(0x1b093efaa009e78b) => ("v1.01", &v101::HOOKS, false),
        _ => ("unknown", Self::UNKNOWN, false),
      },
      (0x0001_0000, 0x0002_0000) => ("v1.02", &v102::HOOKS, false),
      (0x0001_0000, 0x0003_0000) => ("v1.03", &v103::HOOKS, false),
      (0x0001_0000, 0x0004_0001) => ("v1.04b", &v104b::HOOKS, false),
      (0x0001_0000, 0x0004_0002) => ("v1.04c", &v104b::HOOKS, false),
      (0x0001_0000, 0x0005_0000) => ("v1.05a", &v105::HOOKS, false),
      (0x0001_0000, 0x0005_0001) => ("v1.05b", &v105::HOOKS, false),
      (0x0001_0000, 0x0006_0000) => match hash_module_file(unsafe { GetModuleHandleW(GAME_EXE) }) {
        Some(0x73645dbfe51df9ae) => ("v1.06a", &v106a::HOOKS, false),
        Some(0x62fea87b064aec9e) => ("v1.06b", &v106b::HOOKS, false),
        _ => ("unknown", Self::UNKNOWN, false),
      },
      (0x0001_0000, 0x0007_0000) => ("v1.07", &v107::HOOKS, true),
      (0x0001_0000, 0x0008_001c) => ("v1.08", &v108::HOOKS, true),
      (0x0001_0000, 0x0009_0013) => ("v1.09a", &v109a::HOOKS, true),
      (0x0001_0000, 0x0009_0014) => ("v1.09b", &v109a::HOOKS, true),
      (0x0001_0000, 0x0009_0016) => ("v1.09d", &v109d::HOOKS, true),
      // (0x0001_0000, 0x000a_0009) => ("1.10b", &v110b::HOOKS, true),
      // (0x0001_0000, 0x000a_000a) => ("1.10s", &v110s::HOOKS, true),
      (0x0001_0000, 0x000a_0027) => ("v1.10", &v110::HOOKS, true),
      (0x0001_0000, 0x000b_002d) => ("v1.11a", &v111a::HOOKS, true),
      (0x0001_0000, 0x000b_002e) => ("v1.11b", &v111b::HOOKS, true),
      (0x0001_0000, 0x000c_0031) => ("v1.12", &v112::HOOKS, true),
      // (0x0001_0000, 0x000d_0037) => ("1.13a", &v113a::HOOKS, true),
      (0x0001_0000, 0x000d_003c) => ("v1.13c", &v113c::HOOKS, true),
      (0x0001_0000, 0x000d_0040) => ("v1.13d", &v113d::HOOKS, true),
      (0x0001_000e, 0x0000_0040) => ("v1.14a", &v114a::HOOKS, true),
      (0x0001_000e, 0x0001_0044) => ("v1.14b", &v114b::HOOKS, true),
      (0x0001_000e, 0x0002_0046) => ("v1.14c", &v114c::HOOKS, true),
      (0x0001_000e, 0x0003_0047) => ("v1.14d", &v114d::HOOKS, true),
      _ => ("unknown", Self::UNKNOWN, false),
    }
  }
}

macro_rules! decl_fns {
  (
    $fname:ident: $sname:ident:
    $(unsafe $(extern $abi:tt)? fn $name:ident(
      $($arg:ident: $arg_ty:ty),*
      $(, @$target:ident)?
      $(,)?
    ) $(-> $ret:ty)?),*$(,)?
  ) => {
    pub(crate) struct $sname {$(
      $name: unsafe $(extern $abi)? fn($($arg: $arg_ty,)* $($target: usize)?) $(-> $ret)?
    ),*}
    impl $sname {
      pub const INIT: Self = Self {$(
        $name: {
          #[allow(unused)]
          $(extern $abi)? fn f($(_: $arg_ty,)* $($target: usize)?) $(-> $ret)? {
            panic!()
          }
          f
        }
      ),*};
    }
    impl GameAccessor {$(
      #[allow(unused)]
      pub(crate) unsafe fn $name(&self, $($arg: $arg_ty),*) $(-> $ret)? {
        (self.$fname.$name)($($arg,)* $(self.$target)?)
      }
    )*}
  };
}

decl_fns! {
  fns: GameFns:
  unsafe extern "stdcall" fn in_per() -> d2::Bool32,
  unsafe extern "stdcall" fn get_hwnd() -> HWND,
  unsafe extern "stdcall" fn draw_menu(),
  unsafe extern "stdcall" fn find_closest_color(r: u8, g: u8, b: u8) -> u8,
  unsafe extern "stdcall" fn draw_line(x1: i32, y1: i32, x2: i32, y2: i32, color: u8, alpha: u8),
  unsafe extern "fastcall" fn env_array_remove(array: *mut d2::EnvArray, id: u32),
}

decl_fns! {
  helper_fns: HelperFns:
  unsafe extern "fastcall" fn gen_weather_particle(rng: *mut d2::Rng, @gen_weather_particle),
}

pub struct GameAccessor {
  helper_fns: &'static HelperFns,
  fns: GameFns,
  pub is_expansion: bool,
  pub player: *mut Option<NonNull<()>>,
  pub env_effects: *mut d2::ClientEnvEffects,
  pub game_type: *mut d2::GameType,
  pub entity_table: *mut (),
  pub entity_table2: *mut (),
  pub client_loop_globals: *mut d2::ClientLoopGlobals,
  pub apply_pos_change: usize,
  pub server_update_time: *mut u32,
  pub cursor_table: *const [d2::Cursor; 7],
  pub summit_cloud_x_pos: *mut [d2::FI4; 10],
  pub viewport_width: *mut u32,
  pub viewport_height: *mut u32,
  pub viewport_shift: *mut i32,
  pub max_weather_particles: *mut u32,
  pub weather_angle: *mut d2::FU8,
  pub rain_speed: *mut f32,
  pub is_snowing: *mut d2::Bool32,
  pub sine_table: *const [f32; 0x200],
  pub gen_weather_particle: usize,
}
unsafe impl Send for GameAccessor {}
impl GameAccessor {
  pub const fn new() -> Self {
    Self {
      helper_fns: &Hooks::UNKNOWN.helper_fns,
      fns: GameFns::INIT,
      is_expansion: false,
      player: null_mut(),
      env_effects: null_mut(),
      game_type: null_mut(),
      entity_table: null_mut(),
      entity_table2: null_mut(),
      client_loop_globals: null_mut(),
      apply_pos_change: 0,
      server_update_time: null_mut(),
      cursor_table: null(),
      summit_cloud_x_pos: null_mut(),
      viewport_height: null_mut(),
      viewport_width: null_mut(),
      viewport_shift: null_mut(),
      max_weather_particles: null_mut(),
      weather_angle: null_mut(),
      rain_speed: null_mut(),
      is_snowing: null_mut(),
      sine_table: null(),
      gen_weather_particle: 0,
    }
  }

  unsafe fn load(
    &mut self,
    modules: &d2::Modules,
    addresses: &d2::Addresses,
    is_expansion: bool,
    stubs: &'static HelperFns,
  ) -> Result<(), ()> {
    self.is_expansion = is_expansion;
    self.helper_fns = stubs;
    self.player = addresses.player(modules.client()).as_ptr();
    self.env_effects = addresses.env_effects(modules.client()).as_ptr();
    self.game_type = addresses.game_type(modules.client()).as_ptr();
    self.entity_table = addresses.entity_table(modules.client()).as_ptr();
    self.entity_table2 = addresses.entity_table2(modules.client()).as_ptr();
    self.client_loop_globals = addresses.client_loop_globals(modules.client()).as_ptr();
    self.apply_pos_change = addresses.apply_pos_change(modules.common());
    self.server_update_time = addresses.server_update_time(modules.game()).as_ptr();
    self.fns.in_per = addresses.in_perspective(modules.gfx()).ok_or(())?;
    self.fns.get_hwnd = addresses.hwnd(modules.gfx()).ok_or(())?;
    self.fns.draw_menu = addresses.draw_menu(modules.win()).ok_or(())?;
    self.cursor_table = addresses.cursor_table(modules.client());
    self.summit_cloud_x_pos = addresses.summit_cloud_x_pos(modules.client()).as_ptr();
    self.viewport_width = addresses.viewport_width(modules.client()).as_ptr();
    self.viewport_height = addresses.viewport_height(modules.client()).as_ptr();
    self.viewport_shift = addresses.viewport_shift(modules.client()).as_ptr();
    self.fns.find_closest_color = addresses.find_closest_color(modules.win()).ok_or(())?;
    self.fns.draw_line = addresses.draw_line(modules.gfx()).ok_or(())?;
    self.max_weather_particles = addresses.max_weather_particles(modules.client()).as_ptr();
    self.weather_angle = addresses.weather_angle(modules.client()).as_ptr();
    self.rain_speed = addresses.rain_speed(modules.client()).as_ptr();
    self.is_snowing = addresses.is_snowing(modules.client()).as_ptr();
    self.sine_table = addresses.sine_table(modules.fog()).as_ptr();
    self.gen_weather_particle = addresses.gen_weather_particle(modules.client());
    self.fns.env_array_remove = addresses.env_array_remove(modules.fog()).ok_or(())?;
    Ok(())
  }

  pub unsafe fn player<'a, T>(&self) -> Option<&'a mut T> {
    (*self.player).map(|x| &mut *x.cast().as_ptr())
  }

  pub unsafe fn entity_table<'a, T>(&self) -> &'a mut d2::EntityTables<T> {
    &mut *self.entity_table.cast()
  }

  pub unsafe fn entity_table2<'a, T>(&self) -> &'a mut d2::EntityTables<T> {
    &mut *self.entity_table2.cast()
  }

  unsafe fn for_each_dy_entity<T: Entity>(
    &self,
    ids: &mut HashSet<UnitId>,
    mut f: impl FnMut(&T, bool),
  ) {
    ids.clear();
    self.entity_table::<T>().for_each_dy(|e| {
      ids.insert(e.unit_id());
      f(e, true);
    });
    self.entity_table2::<T>().for_each_dy(|e| {
      if !ids.contains(&e.unit_id()) {
        f(e, false);
      }
    });
  }

  unsafe fn for_each_dy_entity_mut<T: Entity>(
    &self,
    ids: &mut HashSet<UnitId>,
    mut f: impl FnMut(&mut T),
  ) {
    ids.clear();
    self.entity_table::<T>().for_each_dy_mut(|e| {
      ids.insert(e.unit_id());
      f(e);
    });
    self.entity_table2::<T>().for_each_dy_mut(|e| {
      if !ids.contains(&e.unit_id()) {
        f(e);
      }
    });
  }

  pub unsafe fn game_type(&self) -> d2::GameType {
    *self.game_type
  }

  pub unsafe fn cursor_table(&self) -> &'static [d2::Cursor; 7] {
    &*self.cursor_table
  }

  pub unsafe fn viewport_size(&self) -> M2d<u32> {
    if self.is_expansion {
      M2d::new(*self.viewport_width, *self.viewport_width)
    } else {
      M2d::new(640, 440)
    }
  }

  pub unsafe fn is_snowing(&self) -> bool {
    self.is_expansion && (*self.is_snowing).bool()
  }

  pub unsafe fn sin(&self, x: d2::FU8) -> f32 {
    (*self.sine_table)[x.repr() as usize & 0x1ff]
  }

  pub unsafe fn cos(&self, x: d2::FU8) -> f32 {
    (*self.sine_table)[x.repr().wrapping_add(0x80) as usize & 0x1ff]
  }
}

impl InstanceSync {
  pub fn attach(&mut self) {
    let (version, hooks, is_expansion) = match unsafe { read_file_version(GAME_EXE) } {
      Ok(version) => Hooks::from_game_file_version(version),
      Err(_) => {
        log!("Error detecting game version");
        INSTANCE.config.features.store_relaxed(Features::empty());
        return;
      }
    };
    log!("Detected game version: {}", version);

    let Some(modules) = (hooks.load_modules)() else {
      log!("Disabling all features: failed to load game modules");
      INSTANCE.config.features.store_relaxed(Features::empty());
      return;
    };
    if unsafe {
      self
        .accessor
        .load(&modules, &hooks.addresses, is_expansion, &hooks.helper_fns)
        .is_err()
    } {
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
      } else if unsafe { try_apply_patch_set(&modules, &hooks.base_addresses, patches).is_err() } {
        log!("Disabling feature `{feature}`: failed to apply patches");
      } else {
        log!("Applied feature `{feature}`");
        continue;
      }
      INSTANCE.config.features.remove_relaxed(feature.as_flag());
    }

    if INSTANCE.config.reapply_patches.load(Relaxed) {
      self.reapply_patches = Some((&hooks.patches, modules));
    }
  }

  fn reapply_patches(&mut self) {
    if let Some((patches, modules)) = self.reapply_patches.take() {
      log!("Reapplying all patches");
      let features = INSTANCE.config.features.load_relaxed();
      for (_, patches) in patches.iter().filter(|(f, _)| features.intersects(f.as_flag())) {
        unsafe {
          apply_patch_set(&modules, patches);
        }
      }
    }
  }
}

unsafe fn try_apply_patch_set(
  modules: &d2::Modules,
  base_addresses: &d2::BaseAddresses,
  mod_patches: &[ModulePatches],
) -> Result<(), ()> {
  if INSTANCE.config.integrity_checks.load(Relaxed) {
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
  }
  apply_patch_set(modules, mod_patches);
  Ok(())
}

unsafe fn apply_patch_set(modules: &d2::Modules, mod_patches: &[ModulePatches]) {
  for m in mod_patches {
    let d2mod = modules[m.module];
    for p in m.patches {
      p.apply(d2mod)
    }
  }
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
  pub real: d2::LinearM2d<d2::FU16>,
  pub delta: d2::LinearM2d<d2::FI16>,
  pub teleport: bool,
  pub active: bool,
  pub from_first_table: bool,
}
impl Position {
  pub fn for_time(&self, fract: d2::FI16) -> d2::LinearM2d<d2::FU16> {
    self.real.wadd(d2::LinearM2d::wfrom(self.delta * fract))
  }

  fn update_pos(&mut self, pos: d2::LinearM2d<d2::FU16>, from_first_table: bool) {
    self.delta = if self.teleport || self.from_first_table != from_first_table {
      d2::LinearM2d::default()
    } else {
      pos.wsub(self.real).winto()
    };
    self.real = pos;
    self.from_first_table = from_first_table;
    self.teleport = false;
  }

  fn new(pos: d2::LinearM2d<d2::FU16>, from_first_table: bool) -> Self {
    Self {
      real: pos,
      delta: d2::LinearM2d::default(),
      teleport: false,
      active: true,
      from_first_table,
    }
  }
}

trait Entity: d2::LinkedList {
  fn unit_id(&self) -> UnitId;
  fn has_room(&self) -> bool;
  fn linear_pos(&self) -> d2::LinearM2d<d2::FU16>;
  fn iso_pos(&self) -> d2::IsoP2d<i32>;
  fn set_pos(&mut self, pos: d2::LinearM2d<d2::FU16>);
  fn rng(&mut self) -> &mut d2::Rng;
}

impl InstanceSync {
  unsafe fn hook_window(&self) {
    if INSTANCE.window_hook.attach(&self.accessor) && INSTANCE.config.fps.load_relaxed().num == 0 {
      INSTANCE.frame_rate_from_window(self.accessor.get_hwnd());
    }
  }

  fn entity_adjusted_pos(&mut self, e: &impl Entity) -> Option<d2::LinearM2d<d2::FU16>> {
    self
      .delayed
      .as_mut()
      .unwrap()
      .entity_tracker
      .get(&e.unit_id())
      .map(|pos| pos.for_time(self.unit_movement_fract))
  }

  fn entity_adjusted_linear_pos(&mut self, e: &impl Entity) -> d2::LinearM2d<d2::FU16> {
    match self.entity_adjusted_pos(e) {
      Some(pos) => pos,
      None => e.linear_pos(),
    }
  }

  fn entity_adjusted_iso_pos(&mut self, e: &impl Entity) -> d2::IsoP2d<i32> {
    match self.entity_adjusted_pos(e) {
      Some(pos) => pos.into_sys(),
      None => e.iso_pos(),
    }
  }

  unsafe fn update_game_time(&mut self, time: i64) -> bool {
    let is_sp = self.accessor.game_type().is_sp();
    let prev_update_time = replace(
      &mut self.game_update_time_ms,
      if is_sp {
        *self.accessor.server_update_time
      } else {
        (*self.accessor.client_loop_globals).last_update
      },
    );
    if self.game_update_time_ms != prev_update_time {
      let cur_time_ms = if is_sp {
        timeGetTime() & 0x7FFFFFFF
      } else {
        GetTickCount()
      };
      if self.game_update_time_ms < cur_time_ms {
        self.game_update_time = INSTANCE.perf_freq.ms_to_ticks(
          INSTANCE.perf_freq.ticks_to_ms(time as u64)
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
    let instance = self.delayed.as_mut().unwrap();

    self
      .accessor
      .for_each_dy_entity::<T>(
        &mut instance.visited_entities,
        |e, first_table| match instance.entity_tracker.entry(e.unit_id()) {
          Entry::Occupied(mut x) => {
            x.get_mut().update_pos(e.linear_pos(), first_table);
            x.get_mut().active = true;
          }
          Entry::Vacant(x) => {
            x.insert(Position::new(e.linear_pos(), first_table));
          }
        },
      );
    instance.entity_tracker.retain(|_, v| take(&mut v.active));
  }

  unsafe fn update_entites_from_tables_no_delta<T: Entity>(&mut self) {
    let instance = &mut self.delayed.as_mut().unwrap();
    self
      .accessor
      .for_each_dy_entity::<T>(&mut instance.visited_entities, |e, first_table| {
        if let Some(pos) = instance.entity_tracker.get_mut(&e.unit_id()) {
          let epos = e.linear_pos();
          if pos.real != epos && (pos.teleport || pos.from_first_table != first_table) {
            pos.delta = d2::LinearM2d::default();
            pos.teleport = false;
            pos.from_first_table = first_table
          }
          pos.real = epos;
        }
      });
  }

  unsafe fn update_entity_positions<T: Entity>(&mut self) {
    let frame_len = INSTANCE.perf_freq.game_frame_time() as i64;
    let since_update = self.render_timer.last_update().wrapping_sub(self.game_update_time) as i64;
    let offset = since_update.clamp(0, frame_len) - frame_len;
    self.unit_movement_fract = d2::FI16::from_repr(((offset << 16) / frame_len) as i32);

    let instance = &mut self.delayed.as_mut().unwrap();
    self
      .accessor
      .for_each_dy_entity_mut::<T>(&mut instance.visited_entities, |e| {
        if let Some(pos) = instance.entity_tracker.get(&e.unit_id()) {
          e.set_pos(pos.for_time(self.unit_movement_fract));
        }
      });
  }

  unsafe fn reset_entity_positions<T: Entity>(&mut self) {
    let instance = &mut self.delayed.as_mut().unwrap();
    self
      .accessor
      .for_each_dy_entity_mut::<T>(&mut instance.visited_entities, |e| {
        if let Some(pos) = instance.entity_tracker.get(&e.unit_id()) {
          e.set_pos(pos.real);
        }
      });
  }

  unsafe fn update_env_images(&mut self, env_shift: d2::ScreenM2d<i32>) {
    let env_shift = env_shift.map(|x| x.0);
    if !self.accessor.in_per().bool() {
      for splash in (*(*self.accessor.env_effects).splashes).as_mut_slice() {
        splash.pos = splash.pos.wadd(env_shift);
      }
      for bubble in (*(*self.accessor.env_effects).bubbles).as_mut_slice() {
        bubble.pos = bubble.pos.wadd(env_shift);
      }
    }
  }
}

extern "stdcall" fn entity_iso_xpos<E: Entity>(e: &E) -> i32 {
  INSTANCE.sync.lock().entity_adjusted_iso_pos(e).x.0
}
extern "stdcall" fn entity_iso_ypos<E: Entity>(e: &E) -> i32 {
  INSTANCE.sync.lock().entity_adjusted_iso_pos(e).y.0
}

extern "stdcall" fn entity_linear_xpos<E: Entity>(e: &E) -> d2::FU16 {
  INSTANCE.sync.lock().entity_adjusted_linear_pos(e).x.0
}
extern "stdcall" fn entity_linear_ypos<E: Entity>(e: &E) -> d2::FU16 {
  INSTANCE.sync.lock().entity_adjusted_linear_pos(e).y.0
}

unsafe extern "C" fn draw_game<E: Entity>() {
  let mut lock = INSTANCE.sync.lock();
  let sync_instance = &mut *lock;
  sync_instance.hook_window();

  // Don't draw anything if the player doesn't exist.
  // Shouldn't happen, but the game also has the same checks.
  let Some(player) = sync_instance.accessor.player::<E>() else {
    return;
  };
  if !player.has_room() {
    return;
  }

  let mut time = 0i64;
  QueryPerformanceCounter(&mut time);
  let last_update = sync_instance.render_timer.last_update();
  if sync_instance
    .render_timer
    .update_time(time as u64, INSTANCE.render_fps.load_relaxed())
  {
    let frame_len = INSTANCE.perf_freq.game_frame_time() as i64;
    let time_delta = (sync_instance.render_timer.last_update() - last_update) as i64;
    INSTANCE
      .update_time_fract
      .store(time_delta as f64 / frame_len as f64, Relaxed);
    INSTANCE.update_ticks.store(
      sync_instance.render_timer.last_update() - last_update,
      Relaxed,
    );

    let enable_smoothing = INSTANCE.config.features.motion_smoothing();
    let smooth_frame = INSTANCE.render_fps.load_relaxed() != GAME_FPS && enable_smoothing;

    let prev_update_count = replace(
      &mut sync_instance.client_update_count,
      (*sync_instance.accessor.client_loop_globals).updates,
    );
    let client_updated = sync_instance.client_update_count != prev_update_count;
    INSTANCE.client_updated.store(client_updated, Relaxed);

    if enable_smoothing {
      if sync_instance.update_game_time(time) {
        sync_instance.update_entites_from_tables::<E>();
      } else {
        sync_instance.update_entites_from_tables_no_delta::<E>();
      }
    }

    let env_shift = if smooth_frame {
      sync_instance.update_entity_positions::<E>();
      let prev_player_pos = sync_instance.player_pos;
      sync_instance.player_pos = sync_instance.entity_adjusted_iso_pos(player);
      let env_shift = prev_player_pos
        .wsub(sync_instance.player_pos)
        .map(|x| x.with_sys::<d2::ScreenSys>());

      if !client_updated {
        // Environment particles are positioned in screen space and updated only
        // when the client state has been updated. For every other frame we need
        // to adjust their positions.
        sync_instance.update_env_images(env_shift);
      }
      env_shift
    } else {
      sync_instance.unit_movement_fract = d2::FI16::default();
      let prev_player_pos = sync_instance.player_pos;
      sync_instance.player_pos = player.iso_pos();
      prev_player_pos
        .wsub(sync_instance.player_pos)
        .map(|x| x.with_sys::<d2::ScreenSys>())
    };

    if INSTANCE.config.features.weather_smoothing() {
      if client_updated {
        crate::weather::update_weather(player.rng(), env_shift, sync_instance);
      } else {
        crate::weather::apply_weather_delta(env_shift, sync_instance);
      }
    }

    let draw = (*sync_instance.accessor.client_loop_globals).draw_fn;
    // Set the movement fraction to zero for rendering. Otherwise, a unit's
    // position will be double adjusted for cursor detection. Once from the
    // edited positions earlier, and once when accessing the position in the
    // cursor detection code.
    let unit_movement_fract = take(&mut sync_instance.unit_movement_fract);
    drop(lock);
    draw(0);

    let mut lock = INSTANCE.sync.lock();
    let sync_instance = &mut *lock;

    sync_instance.unit_movement_fract = unit_movement_fract;
    (*sync_instance.accessor.client_loop_globals).frames_drawn =
      (*sync_instance.accessor.client_loop_globals)
        .frames_drawn
        .wrapping_add(1);
    (*sync_instance.accessor.client_loop_globals).fps_timer.frames_drawn =
      (*sync_instance.accessor.client_loop_globals)
        .fps_timer
        .frames_drawn
        .wrapping_add(1);

    if smooth_frame {
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
    let draw = (*sync_instance.accessor.client_loop_globals).draw_fn;
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
  sync_instance.reapply_patches();

  let mut time = 0i64;
  QueryPerformanceCounter(&mut time);

  if sync_instance
    .render_timer
    .update_time(time as u64, INSTANCE.render_fps.load_relaxed())
  {
    if sync_instance
      .menu_anim_timer
      .update_time(sync_instance.render_timer.last_update())
    {
      INSTANCE.update_menu_char_anim.store(true, Relaxed);
      if let Some(callback) = callback {
        callback(*call_count);
        *call_count = (*call_count).wrapping_add(1);
      }
    } else {
      INSTANCE.update_menu_char_anim.store(false, Relaxed);
    }

    let draw = sync_instance.accessor.fns.draw_menu;
    drop(lock);
    draw();

    lock = INSTANCE.sync.lock();
  }

  let sync_instance = &mut *lock;
  QueryPerformanceCounter(&mut time);
  let sleep_len = (INSTANCE
    .perf_freq
    .ticks_to_ms(sync_instance.render_timer.next_update().saturating_sub(time as u64))
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
    .ticks_to_ms(sync_instance.render_timer.next_update().saturating_sub(time as u64))
    as u32)
    .saturating_sub(1);
  let len = if INSTANCE.is_window_hidden.load(Relaxed) {
    10
  } else {
    len
  };
  let limit = if sync_instance.accessor.game_type().is_host() {
    2
  } else {
    10
  };
  Sleep(len.min(limit));
}

unsafe extern "fastcall" fn update_menu_char_frame(rate: u32, frame: &mut u32) -> u32 {
  if INSTANCE.update_menu_char_anim.load(Relaxed) {
    *frame += rate;
  }
  *frame
}

unsafe extern "fastcall" fn intercept_teleport(kind: d2::EntityKind, id: u32) -> usize {
  let mut lock = INSTANCE.sync.lock();
  let sync_instance = &mut *lock;

  if let Some(pos) = sync_instance
    .delayed
    .as_mut()
    .unwrap()
    .entity_tracker
    .get_mut(&UnitId { kind, id })
  {
    pos.teleport = true;
  }
  sync_instance.accessor.apply_pos_change
}

unsafe extern "fastcall" fn should_update_cursor(cursor: d2::CursorId) -> bool {
  INSTANCE.client_updated.load(Relaxed)
    && INSTANCE.sync.lock().accessor.cursor_table()[cursor.0 as usize].is_anim != 0
}

extern "fastcall" fn summit_cloud_move_amount(x: d2::FU4) -> d2::FU4 {
  (f64::from(x) * INSTANCE.update_time_fract.load(Relaxed)).into()
}

unsafe extern "C" fn draw_arcane_bg() {
  let mut lock = INSTANCE.sync.lock();
  let sync_instance = &mut *lock;
  let delayed_instace = sync_instance.delayed.as_mut().unwrap();

  delayed_instace.arcane_bg.draw(
    &mut delayed_instace.rng,
    &sync_instance.accessor,
    INSTANCE.update_ticks.load(Relaxed),
  );
}
