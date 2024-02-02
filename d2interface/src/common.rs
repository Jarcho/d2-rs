use crate::{ScreenM, ScreenM2d, FU8};
use core::{iter, marker::PhantomData, ops, ptr::NonNull, slice};
use num::M2d;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SId<T, Id>(T, PhantomData<Id>);

pub type Id16<Id> = SId<i16, Id>;
pub type Id8<Id> = SId<i8, Id>;

macro_rules! make_bool {
  ($name:ident($ty:ident)) => {
    #[derive(Clone, Copy, Eq)]
    #[repr(transparent)]
    pub struct $name($ty);
    impl $name {
      pub const fn bool(self) -> bool {
        self.0 != 0
      }
    }
    impl From<bool> for $name {
      fn from(x: bool) -> Self {
        Self(x.into())
      }
    }
    impl From<$name> for bool {
      fn from(x: $name) -> Self {
        x.bool()
      }
    }
    impl PartialEq for $name {
      fn eq(&self, other: &Self) -> bool {
        self.bool() == other.bool()
      }
    }
    impl PartialEq<bool> for $name {
      fn eq(&self, other: &bool) -> bool {
        self.bool() == *other
      }
    }
    impl PartialEq<$name> for bool {
      fn eq(&self, other: &$name) -> bool {
        *self == other.bool()
      }
    }
    impl ops::Not for $name {
      type Output = Self;
      fn not(self) -> Self::Output {
        Self((self.0 == 0).into())
      }
    }
  };
}

make_bool!(Bool32(u32));
make_bool!(Bool16(u16));
make_bool!(Bool8(u8));

decl_enum! { EntityKind(u32) {
  Pc = 0,
  Npc = 1,
  Object = 2,
  Missile = 3,
  Item = 4,
  Tile = 5,
}}

decl_id!(Act(i32));
decl_id!(StrId(i16));

decl_enum! { GameType(u32) {
  Sp = 0,
  Sp2 = 1,
  Bnet = 3,
  OpenBnetHost = 6,
  OpenBnet = 7,
  TcpHost = 8,
  Tcp = 9,
}}
impl GameType {
  #[inline]
  pub fn is_sp(self) -> bool {
    matches!(self, Self::Sp | Self::Sp2)
  }

  #[inline]
  pub fn is_host(self) -> bool {
    matches!(self, Self::OpenBnetHost | Self::TcpHost)
  }
}

pub struct InRoom;

pub trait LinkedList<T = Self>: Sized {
  fn next(&self) -> Option<NonNull<Self>>;
}

#[repr(transparent)]
pub struct EntityTables<T>([EntityTable<T>; 6]);
impl<T> ops::Index<EntityKind> for EntityTables<T> {
  type Output = EntityTable<T>;
  #[inline]
  fn index(&self, index: EntityKind) -> &Self::Output {
    &self.0[index.0 as usize]
  }
}
impl<T> ops::IndexMut<EntityKind> for EntityTables<T> {
  #[inline]
  fn index_mut(&mut self, index: EntityKind) -> &mut Self::Output {
    &mut self.0[index.0 as usize]
  }
}
impl<T: LinkedList> EntityTables<T> {
  pub fn for_each_dy(&self, mut f: impl FnMut(&T)) {
    unsafe { iter_lists(slice::from_raw_parts(self.0[0].0.as_ptr(), 256)) }.for_each(&mut f);
    self.0[3].iter().for_each(&mut f);
  }

  pub fn for_each_dy_mut(&mut self, mut f: impl FnMut(&mut T)) {
    unsafe { iter_mut_lists(slice::from_raw_parts_mut(self.0[0].0.as_mut_ptr(), 256)) }
      .for_each(&mut f);
    self.0[3].iter_mut().for_each(&mut f);
  }
}

#[repr(transparent)]
pub struct EntityTable<T>([Option<NonNull<T>>; 128]);
impl<T: LinkedList> EntityTable<T> {
  #[inline]
  pub fn iter(&self) -> impl Iterator<Item = &T> {
    unsafe { iter_lists(&self.0) }
  }

  #[inline]
  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
    unsafe { iter_mut_lists(&mut self.0) }
  }
}

/// Gets an iterator over all elements in the slice of linked lists.
///
/// # Safety
/// Creating a reference to each element in the lists must be valid.
pub unsafe fn iter_lists<T: LinkedList<U>, U>(
  lists: &[Option<NonNull<T>>],
) -> impl Iterator<Item = &T> {
  lists.iter().flat_map(|&(mut p)| {
    iter::from_fn(move || {
      p.map(|i| unsafe {
        p = i.as_ref().next();
        &*i.as_ptr()
      })
    })
  })
}

/// Gets an iterator over all elements in the slice of linked lists.
///
/// # Safety
/// Creating a mutable reference to each element in the lists must be valid, and
/// each item may appear only once amongst all the lists.
pub unsafe fn iter_mut_lists<T: LinkedList<U>, U>(
  lists: &mut [Option<NonNull<T>>],
) -> impl Iterator<Item = &mut T> {
  lists.iter_mut().flat_map(|&mut mut p| {
    iter::from_fn(move || {
      p.map(|i| unsafe {
        p = i.as_ref().next();
        &mut *i.as_ptr()
      })
    })
  })
}

#[repr(C)]
pub struct EnvArray<T> {
  pub name: [u8; 0x100],
  pub len: u32,
  pub element_size: u32,
  pub initialized: Bool32,
  pub next_free_idx: u32,
  pub last_active_idx: i32,
  pub active_count: u32,
  pub _padding: u16,
  pub data: NonNull<T>,
}
impl<T> EnvArray<T> {
  pub fn as_slice(&self) -> &[T] {
    unsafe {
      slice::from_raw_parts(
        self.data.as_ptr(),
        if self.initialized.bool() {
          (self.last_active_idx + 1) as usize
        } else {
          0
        },
      )
    }
  }

  pub fn as_mut_slice(&mut self) -> &mut [T] {
    unsafe {
      slice::from_raw_parts_mut(
        self.data.as_ptr(),
        if self.initialized.bool() {
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
  pub active: Bool16,
  /// Linear space when rendering in perspective. Camera space when not.
  pub pos: M2d<i32>,
  pub file_idx: u32,
  pub frame: u32,
  pub till_next_frame: u32,
}
pub type EnvImages = EnvArray<EnvImage>;

#[repr(C)]
pub struct EnvParticle {
  pub active: Bool16,
  pub pos: ScreenM2d<i32>,
  pub end_y_pos: ScreenM<i32>,
  pub orientation: u32,
  pub speed: i32,
  pub angle: FU8,
  pub at_end: Bool32,
  pub frames_remaining: u32,
  pub color: u8,
  pub alpha: u8,
}
pub type EnvParticles = EnvArray<EnvParticle>;

#[repr(C)]
pub struct ClientEnvEffects {
  /// Water splashes when raining (Act 1&3).
  pub splashes: *mut EnvImages,
  /// Water bubbles (Act 3).
  pub bubbles: *mut EnvImages,
  /// Weather particles (Rain & Snow).
  pub particles: *mut EnvParticles,
}

#[repr(C)]
pub struct ClientFpsTimer {
  /// The most recently calculated fps
  pub fps: u32,
  /// The number of frames drawn since the last update
  pub frames_drawn: u32,
  /// The number of frames skipped since the last update
  pub frames_skipped: u32,
  /// The most recently calculated frames skipped per second.
  pub fps_skip: u32,
  /// The time of the last fps update.
  pub last_update: u32,
}

#[repr(C)]
pub struct ClientPingTimer {
  /// The time of the next ping update.
  pub next_update: u32,
  /// The time of the previous ping update.
  pub last_update: u32,
  /// The last measured ping time.
  pub ping: u32,
}

#[repr(C)]
pub struct ClientLoopGlobals {
  /// Which draw function is active.
  pub draw_fn_id: u32,
  /// The current active draw function.
  pub draw_fn: unsafe extern "fastcall" fn(u32),
  pub mem_pool: *mut (),
  /// The time the client was last stepped while the game was not paused.
  pub last_step: u32,
  /// The time the client state was last updated.
  pub last_update: u32,
  /// The number of frames drawn this game session.
  pub frames_drawn: u32,
  /// The number of times the client state was updated this game session.
  pub updates: u32,
  /// The client's ping timer.
  pub ping_timer: ClientPingTimer,
  /// The client's fps timer.
  pub fps_timer: ClientFpsTimer,
  /// The time of the loading screen update.
  pub last_loading_update: u32,
}

#[repr(C)]
pub struct Rng([u32; 2]);
impl Default for Rng {
  fn default() -> Self {
    Self::new()
  }
}
impl Rng {
  pub const fn new() -> Self {
    Self([1, 0x29a])
  }

  pub const fn with_seed(seed: u32) -> Self {
    Self([seed, 0x29a])
  }

  #[allow(clippy::should_implement_trait)]
  pub fn next(&mut self) -> u32 {
    let (x, o) = self.0[0].overflowing_mul(0x6ac690c5);
    self.0[1] = self.0[1].wrapping_add(o.into());
    self.0[0] = x.wrapping_add(self.0[1]);
    self.0[0]
  }
}

#[repr(C)]
pub struct Cursor {
  pub is_anim: u32,
  pub repeat_anim: u32,
  pub frame_count: u32,
  pub anim_speed: FU8,
  /// Should this cursor use the mouse down animation.
  pub use_mouse_down_anim: u32,
  pub draw_fn: extern "C" fn(),
  /// Name of the file without the path or extension.
  pub file_name: &'static u8,
}

decl_enum! { CursorId(u32) {
  Menu = 0,
  Grasp = 1,
  ToIdleActive = 2,
  Idle = 3,
  MouseDown = 4,
  Active = 5,
  Static = 6,
}}

decl_enum! { CursorState(u32) {
  Active = 1,
  ToIdle = 2,
  ToActive = 3,
  Idle = 4,
  MouseDown = 5,
  Static = 6,
}}

#[repr(C)]
pub struct GameCursor<E = ()> {
  pub item: Option<NonNull<E>>,
  pub dc6_files: [usize; 7],
  pub id: CursorId,
  pub frame: FU8,
  pub _padding: u32,
  pub last_move_time: u32,
  pub state: CursorState,
}

decl_enum! { ElTy(u8) {
  None = 0,
  Fire = 1,
  Light = 2,
  Magic = 3,
  Cold = 4,
  Poison = 5,
  HpSteal = 6,
  MpSteal = 7,
  StSteal = 8,
  Stun = 9,
  Spectral = 10,
  Burn = 11,
  Freeze = 12,
}}

decl_enum! { ArmorTy(i8) {
  None = -1,
  Light = 0,
  Med = 1,
  Heavy = 2,
}}

decl_enum! { Color(u8) {
    White = 0,
    LGrey = 1,
    DGrey = 2,
    Black = 3,
    LBlue = 4,
    DBlue = 5,
    CBlue = 6,
    LRed = 7,
    DRed = 8,
    CRed = 9,
    LGreen = 10,
    DGreen = 11,
    CGreen = 12,
    LYellow = 13,
    DYellow = 14,
    LGold = 15,
    DGold = 16,
    LPurple = 17,
    DPurple = 18,
    Orange = 19,
    BWhite = 20,
}}

decl_enum! { Component(u8) {
    Head = 0,
    Torso = 1,
    Legs = 2,
    RArm = 3,
    LArm = 4,
    RHand = 5,
    LHand = 6,
    Shield = 7,
    LShoulder = 8,
    RShoulder = 9,
    Sp3 = 10,
    Sp4 = 11,
    Sp5 = 12,
    Sp6 = 13,
    Sp7 = 14,
    Sp8 = 15,
}}

decl_enum! { NgLvl(u8) {
    Norm = 0,
    Nm = 1,
    Hell = 2,
}}

decl_enum! { BodyLoc(u8) {
    None = 0,
    Head = 1,
    Neck = 2,
    Torso = 3,
    RArm = 4,
    LArm = 5,
    RRing = 6,
    LRing = 7,
    Belt = 8,
    Feet = 9,
    Gloves = 10,
}}

decl_enum! { StorePage(u8) {
    Armor = 0,
    Weapons = 1,
    Magic = 2,
    Misc = 3,
}}

decl_enum! { CubeMod(u8) {
    None = 0,
    Amethyst = 1,
    Ruby = 2,
    Sapphire = 3,
    Topaz = 4,
    Emerald = 5,
    Diamond = 6,
    Skill = 7,
    Magic = 8,
    Rare = 9,
    Unique = 10,
    Crafted = 11,
}}

decl_enum! { CubeTy(u8) {
    None = 0,
    HpPot = 1,
    MpPot = 2,
    Item = 3,
    Axe = 4,
    Sword = 5,
    Spear = 6,
    Gem = 7,
    Staff = 8,
    Belt = 9,
    Dagger = 10,
    Weapon = 11,
    Armor = 12,
    Ring = 13,
    Amulet = 14,
}}

decl_enum! { NpcState(u8) {
    Death = 0,
    Neutral = 1,
    Walk = 2,
    Recovery = 3,
    Att1 = 4,
    Att2 = 5,
    Block = 6,
    Cast = 7,
    Sk1 = 8,
    Sk2 = 9,
    Sk3 = 10,
    Sk4 = 11,
    Dead = 12,
    KnockBack = 13,
    Seq = 14,
    Run = 15,
}}

decl_enum! { ObjState(u8) {
    Neutral = 0,
    Operating = 1,
    Opened = 2,
    Sp1 = 3,
    Sp2 = 4,
    Sp3 = 5,
    Sp4 = 6,
    Sp5 = 7,
}}

decl_enum! { PcState(u8) {
    Death = 0,
    Neutral = 1,
    Walk = 2,
    Run = 3,
    Recover = 4,
    TownNeutral = 5,
    TownWalk = 6,
    Att1 = 7,
    Att2 = 8,
    Block = 9,
    Cast = 10,
    Throw = 11,
    Kick = 12,
    Sk1 = 13,
    Sk2 = 14,
    Sk3 = 15,
    Sk4 = 16,
    Dead = 17,
    Seq = 18,
    KnockBack = 19,
}}

#[derive(Clone, Copy)]
#[repr(C)]
pub union PcOrNpcState {
  pub pc: PcState,
  pub npc: NpcState,
}

decl_enum! { SkRange(u8) {
    None = 0,
    H2h = 1,
    Range = 2,
    Both = 3,
    Location = 4,
}}

decl_enum! { Pc(u8) {
    Zon = 0,
    Sorc = 1,
    Necro = 2,
    Pal = 3,
    Barb = 4,
    Druid = 5,
    Sin = 6,
}}

decl_enum! { NpcSpawnTy(u8) {
    Normal = 0,
    Map = 1,
    Special = 2,
}}

decl_enum! { ItemHitClass(u8) {
  None = 0,
  H2h = 1,
  Swing1H = 2,
  SwingBig1H = 3,
  Swing2H = 4,
  SwingBig2H = 5,
  Thrust1H = 6,
  Thrust2H = 7,
  Club = 8,
  Staff = 9,
  Bow = 10,
  XBow = 11,
  Claw = 12,
  Overlay = 13,
}}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct RgbColor {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

pub mod dtbl {
  use super::{Component, NgLvl, NpcState, ObjState};

  use crate::BodyLoc;

  decl_id!(I32Code(u32));
  decl_id!(ItemCode(u32));
  decl_id!(ItemTyCode(u32));

  decl_id!(CodeOffset(u32));

  decl_id!(DropSet(i32));
  decl_id!(Event(i16));
  decl_id!(Gem(i32));
  decl_id!(Item(i32));
  decl_id!(ItemStat(i32));
  decl_id!(ItemTy(i32));
  decl_id!(Lvl(i32));
  decl_id!(MercDesc(i8));
  decl_id!(Missile(i32));
  decl_id!(MPrefix(i16));
  decl_id!(MSuffix(i16));
  decl_id!(Npc(i32));
  decl_id!(NpcAi(i16));
  decl_id!(NpcAnim(i16));
  decl_id!(NpcEquip(i16));
  decl_id!(NpcEx(i16));
  decl_id!(NpcPlace(i16));
  decl_id!(NpcTy(i32));
  decl_id!(NpcMod(i16));
  decl_id!(NpcProp(i32));
  decl_id!(NpcSound(i32));
  decl_id!(Overlay(i16));
  decl_id!(Pet(i32));
  decl_id!(Prop(i32));
  decl_id!(Set(i16));
  decl_id!(SItem(i16));
  decl_id!(SkDesc(i32));
  decl_id!(Skill(i32));
  decl_id!(Sound(i32));
  decl_id!(Effect(i32));
  decl_id!(UItem(i16));
  decl_id!(UMon(i16));

  #[derive(Clone, Copy)]
  #[repr(C)]
  pub struct ByLvl<T> {
    pub lvl1: T,
    pub lvl25: T,
    pub lvl40: T,
  }
  impl<T: Copy> ByLvl<T> {
    pub fn at_lvl(&self, lvl: u32) -> T {
      match lvl {
        0..=24 => self.lvl1,
        25..=39 => self.lvl25,
        40..=u32::MAX => self.lvl40,
      }
    }
  }

  #[derive(Clone, Copy)]
  #[repr(C)]
  pub struct AccByLvl3<T> {
    pub lvl2: T,
    pub lvl10: T,
    pub lvl15: T,
  }
  impl<T: Copy + Into<i32>> AccByLvl3<T> {
    pub fn at_lvl(&self, lvl: u16) -> i32 {
      let b1 = i32::from(lvl.max(9).saturating_sub(1));
      let b2 = i32::from(lvl.max(14).saturating_sub(9));
      let b3 = i32::from(lvl.saturating_sub(14));
      b1 * self.lvl2.into() + b2 * self.lvl10.into() + b3 * self.lvl15.into()
    }
  }

  #[derive(Clone, Copy)]
  #[repr(C)]
  pub struct AccByLvl5<T> {
    pub lvl2: T,
    pub lvl9: T,
    pub lvl17: T,
    pub lvl23: T,
    pub lvl29: T,
  }
  impl<T: Copy + Into<i32>> AccByLvl5<T> {
    pub fn at_lvl(&self, lvl: u16) -> i32 {
      let b1 = i32::from(lvl.max(8).saturating_sub(1));
      let b2 = i32::from(lvl.max(16).saturating_sub(8));
      let b3 = i32::from(lvl.max(22).saturating_sub(16));
      let b4 = i32::from(lvl.max(28).saturating_sub(22));
      let b5 = i32::from(lvl.saturating_sub(28));
      b1 * self.lvl2.into()
        + b2 * self.lvl9.into()
        + b3 * self.lvl17.into()
        + b4 * self.lvl23.into()
        + b5 * self.lvl29.into()
    }
  }

  #[derive(Clone, Copy)]
  #[repr(C)]
  pub struct ByNgLvl<T> {
    pub values: [T; 3],
  }
  impl<T: Copy> ByNgLvl<T> {
    pub fn at_ng_lvl(&self, x: NgLvl) -> Option<T> {
      self.values.get(x.0 as usize).copied()
    }
  }

  #[derive(Clone, Copy)]
  #[repr(C)]
  pub struct ByObjState<T> {
    pub values: [T; 8],
  }
  impl<T: Copy> ByObjState<T> {
    pub fn for_state(&self, x: ObjState) -> Option<T> {
      self.values.get(x.0 as usize).copied()
    }
  }

  #[derive(Clone, Copy)]
  #[repr(C)]
  pub struct ByNpcState<T> {
    pub values: [T; 16],
  }
  impl<T: Copy> ByNpcState<T> {
    pub fn for_state(&self, x: NpcState) -> Option<T> {
      self.values.get(x.0 as usize).copied()
    }
  }

  #[derive(Clone, Copy)]
  #[repr(C)]
  pub struct ByComponent<T> {
    pub values: [T; 16],
  }
  impl<T: Copy> ByComponent<T> {
    pub fn for_component(&self, x: Component) -> Option<T> {
      self.values.get(x.0 as usize).copied()
    }
  }

  #[derive(Clone, Copy)]
  #[repr(C)]
  pub struct ByEqComponent<T> {
    pub rarm: T,
    pub larm: T,
    pub torso: T,
    pub legs: T,
    pub rshoulder: T,
    pub lshoulder: T,
  }

  #[repr(C)]
  pub struct StartItem {
    pub item: ItemCode,
    pub loc: BodyLoc,
    pub count: u8,
  }
}
