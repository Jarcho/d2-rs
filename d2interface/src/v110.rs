use crate::{
  module::Ordinal::Ordinal, Addresses, EntityKind, FixedU16, FixedU8, InRoom, IsoPos, LinearPos,
  LinkedList, Rand, Size,
};
use core::ptr::NonNull;

pub use crate::v109d::BASE_ADDRESSES;

pub type EntityTables = crate::EntityTables<Entity>;
pub type EntityTable = crate::EntityTable<Entity>;
pub type GameCursor = crate::GameCursor<Entity>;

pub const ADDRESSES: Addresses = Addresses {
  player: 0x11c200,
  env_effects: 0x1076fc,
  game_type: 0x107960,
  entity_table: 0x11aa00,
  entity_table2: 0x11b600,
  client_loop_globals: 0x107750,
  // Signature: `__fastcall(DyPos*, Room*, FixedU16, FixedU16)`
  apply_pos_change: 0x6cc40,
  in_perspective: Ordinal(10010),
  hwnd: Ordinal(10027),
  server_update_time: 0x115844,
  draw_menu: Ordinal(10019),
  cursor_table: 0xf6b58,
  game_cursor: 0x121aa4,
  summit_cloud_x_pos: 0,
  draw_line: Ordinal(10057),
  find_closest_color: Ordinal(10034),
  viewport_width: 0xfa704,
  viewport_height: 0xfa700,
  viewport_shift: 0x10b9c8,
};

#[repr(C)]
pub struct Room {
  pub connected: Option<NonNull<NonNull<Room>>>,
  pub _padding1: [u32; 3],
  pub data: u32,
  pub _padding2: [u32; 4],
  pub connected_count: u32,
  pub _padding3: [u32; 9],
  pub pos: LinearPos<u32>,
  pub size: Size<u32>,
  pub _padding4: [u32; 9],
}

#[repr(C)]
pub struct StaticPos {
  pub room: Option<NonNull<Room>>,
  pub iso_pos: IsoPos<i32>,
  pub linear_pos: LinearPos<u32>,
  pub _padding1: [u32; 3],
}

#[repr(C)]
pub struct DyPos {
  pub linear_pos: LinearPos<FixedU16>,
  pub iso_pos: IsoPos<i32>,
  pub target_pos: [LinearPos<u16>; 3],
  pub room: Option<NonNull<Room>>,
  pub _padding1: [u32; 4],
  pub entity: NonNull<Entity>,
}

#[repr(C)]
pub union EntityPos {
  pub s: Option<NonNull<StaticPos>>,
  pub d: Option<NonNull<DyPos>>,
}

#[repr(C)]
pub struct Entity {
  pub kind: EntityKind,
  pub class_id: u32,
  pub mem_pool: *mut (),
  pub id: u32,
  pub state: u32,
  pub data: u32,
  pub act_id: u32,
  pub act: *mut (),
  pub rand: Rand,
  pub seed: u32,
  pub pos: EntityPos,
  pub _padding2: [u32; 5],
  pub frame: FixedU8,
  pub _padding3: [u32; 3],
  pub gfx_info: u32,
  pub _padding4: [u32; 3],
  pub light: u32,
  pub light_width: u32,
  pub _padding5: [u32; 30],
  pub next_entity: Option<NonNull<Entity>>,
  pub next_in_room: Option<NonNull<Entity>>,
  pub _padding6: [u32; 2],
}
impl LinkedList for Entity {
  fn next(&self) -> Option<NonNull<Self>> {
    self.next_entity
  }
}
impl LinkedList<InRoom> for Entity {
  fn next(&self) -> Option<NonNull<Self>> {
    self.next_in_room
  }
}
impl Entity {
  pub fn pos<T>(
    &self,
    static_fn: impl FnOnce(&StaticPos) -> T,
    dy_fn: impl FnOnce(&DyPos) -> T,
  ) -> Option<T> {
    unsafe {
      match self.kind {
        EntityKind::Pc | EntityKind::Npc | EntityKind::Missile => {
          self.pos.d.map(|pos| dy_fn(pos.as_ref()))
        }
        EntityKind::Object | EntityKind::Item | EntityKind::Tile => {
          self.pos.s.map(|pos| static_fn(pos.as_ref()))
        }
        _ => None,
      }
    }
  }

  pub fn has_room(&self) -> bool {
    self
      .pos(|pos| pos.room.is_some(), |pos| pos.room.is_some())
      .unwrap_or(false)
  }
}

pub mod dtbl {
  pub use crate::v109d::dtbl::*;
  use crate::{
    dtbl::{
      AccByLvl3, AccByLvl5, ByComponent, ByEqComponent, ByLvl, ByNgLvl, ByNpcMode, ByObjMode,
      CodeOffset, DropSet, Event, I32Code, Item, ItemCode, ItemStat, ItemTy, ItemTyCode, Lvl,
      MPrefix, MSuffix, MercDesc, Missile, Npc, NpcAi, NpcAnim, NpcEx, NpcPlace, NpcProp, NpcSound,
      NpcTy, Overlay, Pet, Prop, SItem, Set, SkDesc, Skill, Sound, StartItem, State, UItem, UMon,
    },
    ArmorTy, BodyLoc, Color, Component, ElTy, HitClass, Id16, Id8, NpcMode, Pc, PcMode, Range,
    RgbColor, ScreenRectS, Size, SkRange, StorePage, StrId,
  };

  use bitflags::bitflags;

  bitflags! {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ItemStatDefFlags: u32 {
      const SendOther = 0x1;
      const Signed = 0x2;
      const UpdateAnimRate = 0x200;
      const Fmin = 0x400;
      const DamageRelated = 0x4;
      const ItemSpecific = 0x8;
      const Direct = 0x10;
      const Fcallback = 0x800;
      const Saved = 0x1000;
      const CsvSigned = 0x2000;
    }
  }

  bitflags! {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct MissileDefFlags: u32 {
      const LastCollide = 0x1;
      const Explosion = 0x2;
      const Pierce = 0x4;
      const CanSlow = 0x8;
      const CanDestroy = 0x10;
      const NoMultishot = 0x1000;
      const NoUniqueMod = 0x2000;
      const ClientSend = 0x20;
      const GetHit = 0x40;
      const SoftHit = 0x80;
      const ApplyMastery = 0x100;
      const ReturnFire = 0x200;
      const Town = 0x400;
      const SrcTown = 0x800;
      const MissileSkill = 0x8000;
      const Half2hSrc = 0x4000;
    }
  }

  bitflags! {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct NpcExDefFlags: u32 {
      const NoGfxHitTest = 0x1;
      const NoMap = 0x2;
      const NoOverlay = 0x4;
      const IsSel = 0x8;
      const AlSel = 0x10;
      const NoSel = 0x20;
      const ShiftSel = 0x40;
      const CorpseSel = 0x80;
      const Revive = 0x100;
      const IsStt = 0x200;
      const Large = 0x800;
      const Small = 0x400;
      const Soft = 0x1000;
      const Critter = 0x2000;
      const Inert = 0x20000;
      const Objcol = 0x40000;
      const DeadCol = 0x80000;
      const UnflatDead = 0x100000;
      const Shadow = 0x4000;
      const NoUniqueShift = 0x8000;
      const CompositeDeath = 0x10000;
    }
  }

  bitflags! {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct EnabledComponents: u32 {
      const Hd = 0x1;
      const Tr = 0x2;
      const Lg = 0x4;
      const Ra = 0x8;
      const La = 0x10;
      const Rh = 0x20;
      const Lh = 0x40;
      const Sh = 0x80;
      const S1 = 0x100;
      const S2 = 0x200;
      const S3 = 0x400;
      const S4 = 0x800;
      const S5 = 0x1000;
      const S6 = 0x2000;
      const S7 = 0x4000;
      const S8 = 0x8000;
    }
  }

  bitflags! {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct NpcExDefFlags3: u32 {
      const Mdt = 0x1;
      const Mnu = 0x2;
      const Mwl = 0x4;
      const Mgh = 0x8;
      const Ma1 = 0x10;
      const Ma2 = 0x20;
      const Mbl = 0x40;
      const Msc = 0x80;
      const Ms1 = 0x100;
      const Ms2 = 0x200;
      const Ms3 = 0x400;
      const Ms4 = 0x800;
      const Mdd = 0x1000;
      const Mkb = 0x2000;
      const Msq = 0x4000;
      const Mrn = 0x8000;
    }
  }

  bitflags! {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct NpcExDefFlags4: u32 {
      const A1mv = 0x10;
      const A2mv = 0x20;
      const Scmv = 0x80;
      const S1mv = 0x100;
      const S2mv = 0x200;
      const S3mv = 0x400;
      const S4mv = 0x800;
    }
  }

  bitflags! {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct NpcDefFlags: u32 {
      const Enabled = 0x2000000;
      const RangedTy = 0x10000000;
      const PlaceSpawn = 0x800000;
      const IsSpawn = 0x1;
      const IsMelee = 0x2;
      const NoRatio = 0x4;
      const SetBoss = 0x10;
      const BossXfer = 0x20;
      const Boss = 0x40;
      const PrimeEvil = 0x80;
      const OpenDoors = 0x8;
      const Npc = 0x100;
      const Interact = 0x200;
      const Inventory = 0x1000000;
      const InTown = 0x400;
      const LowUndead = 0x800;
      const HighUndead = 0x1000;
      const Demon = 0x2000;
      const Flying = 0x4000;
      const Killable = 0x8000;
      const SwitchAi = 0x10000;
      const NoAura = 0x8000000;
      const NoMultishot = 0x20000;
      const NeverCount = 0x40000;
      const PetIgnore = 0x80000;
      const DeathDmg = 0x100000;
      const GenericSpawn = 0x200000;
      const Zoo = 0x400000;
      const NoShldBlock = 0x4000000;
    }
  }

  bitflags! {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct PetDefFlags: u32 {
      const Warp = 0x1;
      const Range = 0x2;
      const PartySend = 0x4;
      const Unsummon = 0x8;
      const AutoMap = 0x10;
      const DrawHp = 0x20;
    }
  }

  bitflags! {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct SkillDefFlags: u32 {
      const Lob = 0x2;
      const Decquant = 0x1;
      const Immediate = 0x8000;
      const StSuccessOnly = 0x1000;
      const StSoundDelay = 0x2000;
      const WeaponSnd = 0x4000;
      const Progressive = 0x4;
      const Finishing = 0x8;
      const Prgstack = 0x80;
      const Intown = 0x100;
      const Kick = 0x200;
      const Passive = 0x10;
      const Aura = 0x20;
      const Periodic = 0x40;
      const ItemTgtDo = 0x20000000;
      const InGame = 0x400;
      const NoAmmo = 0x10000;
      const Enhanceable = 0x20000;
      const Durability = 0x40000;
      const UseAttackRate = 0x80000;
      const TargetableOnly = 0x100000;
      const SearchEnemyXy = 0x200000;
      const SearchEnemyNear = 0x400000;
      const SearchOpenXy = 0x800000;
      const TargetCorpse = 0x1000000;
      const TargetPet = 0x2000000;
      const TargetAlly = 0x4000000;
      const TargetItem = 0x8000000;
      const AttackNoMana = 0x10000000;
      const LeftSkill = 0x40000000;
      const Interrupt = 0x80000000;
      const Repeat = 0x800;
    }
  }

  bitflags! {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct SkillDefFlags2: u32 {
      const Warp = 0x40;
      const General = 0x8;
      const Scroll = 0x10;
      const ItemCheckStart = 0x2;
      const ItemCltCheckStart = 0x4;
      const TgtPlaceCheck = 0x1;
      const UseManaOnDo = 0x20;
    }
  }

  bitflags! {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct StateDefFlags: u32 {
      const NoSend = 0x1;
      const Hide = 0x4;
      const Transform = 0x8;
      const Aura = 0x2;
      const Pgsv = 0x10;
      const Active = 0x20;
      const RemHit = 0x40;
      const DamBlue = 0x80;
      const DamRed = 0x100;
      const AttBlue = 0x200;
      const AttRed = 0x400;
      const Curse = 0x800;
      const Curable = 0x1000;
      const PlrStayDeath = 0x2000;
      const MonStayDeath = 0x4000;
      const BossStayDeath = 0x8000;
      const Disguise = 0x10000;
      const Restrict = 0x20000;
      const Blue = 0x40000;
      const ArmBlue = 0x80000;
      const RfBlue = 0x100000;
      const RcBlue = 0x200000;
      const RlBlue = 0x400000;
      const RpBlue = 0x800000;
      const StamBarBlue = 0x1000000;
      const ArmRed = 0x2000000;
      const RfRed = 0x4000000;
      const RcRed = 0x8000000;
      const RlRed = 0x10000000;
      const RpRed = 0x20000000;
      const Exp = 0x40000000;
      const Shatter = 0x80000000;
    }
  }

  bitflags! {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct StateDefFlags2: u32 {
      const BossInv = 0x20;
      const MeleeOnly = 0x40;
      const Life = 0x1;
      const Undead = 0x2;
      const Green = 0x4;
      const NoOverlays = 0x8;
      const NotOnDead = 0x80;
      const NoClear = 0x10;
    }
  }

  bitflags! {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct UItemDefFlags: u32 {
      const Enabled = 0x1;
      const Ladder = 0x8;
      const NoLimit = 0x2;
      const Carry1 = 0x4;
    }
  }

  #[derive(Clone, Copy)]
  #[repr(C)]
  pub union PropParam {
    pub value: i32,
    pub skill: Skill,
    pub npc_ty: NpcTy,
    pub state: State,
  }

  #[repr(C)]
  pub struct BookDef {
    pub name: StrId,
    pub spell_icon: u8,
    pub p_spell: i32,
    pub scroll_skill: Skill,
    pub book_skill: Skill,
    pub base_cost: i32,
    pub cost_per_charge: i32,
    pub scroll_spell_code: I32Code,
    pub book_spell_code: I32Code,
  }

  #[repr(C)]
  pub struct NgLvlDef {
    pub res_penalty: i32,
    pub xp_death_penalty: i32,
    pub uber_code_odds: i32,
    pub uber_code_odds_good: i32,
    pub npc_skill_bonus: i32,
    pub npc_freeze_div: i32,
    pub npc_cold_div: i32,
    pub ai_curse_div: i32,
    pub ultra_code_odds: i32,
    pub ultra_code_odds_good: i32,
    pub life_steal_div: i32,
    pub mana_steal_div: i32,
    pub unique_dmg_bonus: i32,
    pub champion_dmg_bonus: i32,
    pub hireable_boss_dmg_pct: i32,
    pub npc_ce_dmg_pct: i32,
    pub static_field_min: i32,
    pub gamble_rare: i32,
    pub gamble_set: i32,
    pub gamble_unique: i32,
    pub gamble_uber: i32,
    pub gamble_ultra: i32,
  }

  #[repr(C)]
  pub struct DropSetDef {
    pub name: [u8; 32],
    pub picks: i32,
    pub group: i16,
    pub level: i16,
    pub magic: i16,
    pub rare: i16,
    pub set: i16,
    pub unique: i16,
    pub _pad0: [u8; 4],
    pub nodrop: i32,
    pub items: [[u8; 64]; 10],
    pub weights: [i32; 10],
  }

  #[repr(C)]
  pub struct ItemMod {
    pub prop: Prop,
    pub param: PropParam,
    pub value: Range<i32>,
  }

  #[repr(C)]
  pub struct GemDef {
    pub name: [u8; 32],
    pub letter: [u8; 6],
    pub item: Item,
    pub display_name: StrId,
    pub mod_count: u8,
    pub transform: u8,
    pub weapon_mods: [ItemMod; 3],
    pub helm_mods: [ItemMod; 3],
    pub shield_mods: [ItemMod; 3],
  }

  #[repr(C)]
  pub struct ItemStatDef {
    pub id: Id16<ItemStat>,
    pub flags: ItemStatDefFlags,
    pub send_bits: u8,
    pub send_param_bits: u8,
    pub csv_bits: u8,
    pub csv_param: u8,
    pub div: i32,
    pub mul: i32,
    pub add: i32,
    pub val_shift: u8,
    pub save_bits: u8,
    pub save_bits_109: u8,
    pub save_add: i32,
    pub save_add_109: i32,
    pub save_param_bits: i32,
    pub _pad2: [u8; 4],
    pub min_accr: i32,
    pub encode: u8,
    pub max_stat: Id16<ItemStat>,
    pub desc_priority: i16,
    pub desc_func: u8,
    pub desc_val: u8,
    pub desc_str_pos: StrId,
    pub desc_str_neg: StrId,
    pub desc_str2: StrId,
    pub dgrp: i16,
    pub dgrp_func: u8,
    pub dgrp_val: u8,
    pub dgrp_str_pos: StrId,
    pub dgrp_str_neg: StrId,
    pub dgrp_str2: StrId,
    pub item_event1: Event,
    pub item_event2: Event,
    pub item_event_func1: i16,
    pub item_event_func2: i16,
    pub keep_zero: u8,
    pub _pad3: [u8; 3],
    pub op: u8,
    pub op_param: u8,
    pub op_base: Id16<ItemStat>,
    pub op_stats: [Id16<ItemStat>; 3],
    pub _pad4: [u8; 226],
    pub stuff: i32,
  }

  #[repr(C)]
  pub struct ItemTyDef {
    pub code: ItemTyCode,
    pub equiv: [Id16<ItemTy>; 2],
    pub can_repair: u8,
    pub body: u8,
    pub body_locs: [BodyLoc; 2],
    pub shoots: Id16<ItemTy>,
    pub quiver: Id16<ItemTy>,
    pub is_throwable: u8,
    pub is_reloadable: u8,
    pub is_reequipable: u8,
    pub auto_stack: u8,
    pub magic: u8,
    pub rare: u8,
    pub normal: u8,
    pub charm: u8,
    pub gem: u8,
    pub beltable: u8,
    pub max_socks: ByLvl<u8>,
    pub is_drop_set: u8,
    pub rarity: u8,
    pub staff_mods: Pc,
    pub cost_formula: u8,
    pub class: Pc,
    pub store_page: StorePage,
    pub var_inv_gfx: u8,
    pub inv_gfx: [[u8; 32]; 6],
  }

  #[repr(C)]
  pub struct ItemDef {
    pub flippy_file: [u8; 32],
    pub inv_file: [u8; 32],
    pub uinv_file: [u8; 32],
    pub sinv_file: [u8; 32],
    pub code: ItemCode,
    pub norm_code: I32Code,
    pub uber_code: I32Code,
    pub ultra_code: I32Code,
    pub alt_gfx: I32Code,
    pub p_spell: i32,
    pub state: Id16<State>,
    pub cstates: [Id16<State>; 2],
    pub stats: [Id16<ItemStat>; 3],
    pub calcs: [CodeOffset; 3],
    pub len: CodeOffset,
    pub spell_desc: u8,
    pub spell_desc_str: StrId,
    pub spell_desc_calc: CodeOffset,
    pub better_gem: I32Code,
    pub wclass: I32Code,
    pub wclass_2h: I32Code,
    pub tmog_ty: I32Code,
    pub armor: Range<i32>,
    pub gamble_cost: i32,
    pub speed: i32,
    pub bitfield1: i32,
    pub cost: i32,
    pub stack_size: Range<i32>,
    pub spawn_stack: i32,
    pub gem_offset: i32,
    pub name_str: StrId,
    pub version: i16,
    pub auto_prefix: i16,
    pub missile_ty: Id16<Missile>,
    pub rarity: u8,
    pub level: u8,
    pub dmg: Range<u8>,
    pub dmg_missile: Range<u8>,
    pub dmg_2h: Range<u8>,
    pub melee_range: u8,
    pub str_bonus: i16,
    pub dex_bonus: i16,
    pub req_str: i16,
    pub req_dex: i16,
    pub absorbs: u8,
    pub inv_size: Size<u8>,
    pub block: u8,
    pub durability: u8,
    pub indestructible: u8,
    pub missile: u8,
    pub component: Component,
    pub armor_gfx: ByEqComponent<ArmorTy>,
    pub two_handed: u8,
    pub useable: u8,
    pub tys: [Id16<ItemTy>; 2],
    pub sub_ty: u8,
    pub drop_sound: Id16<Sound>,
    pub use_sound: Id16<Sound>,
    pub drops_fx_frame: u8,
    pub unique: u8,
    pub quest: u8,
    pub quest_diff_check: u8,
    pub transparent: u8,
    pub trans_tbl: u8,
    pub _pad0: [u8; 1],
    pub light_size: u8,
    pub belt: u8,
    pub auto_belt: u8,
    pub is_stackable: u8,
    pub is_spawnable: u8,
    pub spell_icon: u8,
    pub dur_warning: u8,
    pub qnt_warning: u8,
    pub has_sockets: u8,
    pub socket_count: u8,
    pub transmogrify: u8,
    pub tmog_qnt: Range<u8>,
    pub hit_class: HitClass,
    pub multi_handed: u8,
    pub gem_apply_ty: u8,
    pub lvl_req: u8,
    pub mlvl: u8,
    pub transform: u8,
    pub inv_trans: u8,
    pub compact_save: u8,
    pub skip_name: u8,
    pub nameable: u8,
    pub vend_min: PerVendor,
    pub vend_max: PerVendor,
    pub vend_mmin: PerVendor,
    pub vend_mmax: PerVendor,
    pub vend_mlvl: PerVendor,
    pub nm_upg: ItemCode,
    pub hell_upg: ItemCode,
    pub can_sell_out: u8,
    pub can_multi_buy: u8,
  }

  #[repr(C)]
  pub struct LvlDef {
    pub id: Id8<Lvl>,
    pub _pad0: [u8; 1],
    pub pal: u8,
    pub act: u8,
    pub teleport: u8,
    pub rain: u8,
    pub mud: u8,
    pub no_per: u8,
    pub is_inside: u8,
    pub draw_edges: u8,
    pub warp_dist: i32,
    pub mlvls: ByNgLvl<u16>,
    pub mlvls_ex: ByNgLvl<u16>,
    pub mon_density: ByNgLvl<i32>,
    pub umon_min: ByNgLvl<u8>,
    pub umon_max: ByNgLvl<u8>,
    pub mon_wndr: u8,
    pub mon_spc_walk: u8,
    pub quest: u8,
    pub ranged_spawn: u8,
    pub mon_count: u8,
    pub _pad1: [u8; 3],
    pub mons: [Id16<Npc>; 25],
    pub nm_mons: [Id16<Npc>; 25],
    pub umons: [Id16<Npc>; 25],
    pub critters: [Id16<Npc>; 4],
    pub critter_weights: [i16; 4],
    pub camt: [i16; 4],
    pub waypoint: u8,
    pub obj_groups: [u8; 8],
    pub obj_weights: [u8; 8],
    pub lvl_name: [u8; 40],
    pub lvl_warp: [u8; 40],
    pub entry_file: [u8; 40],
    pub wlvl_name: [u16; 40],
    pub wlvl_warp: [u16; 40],
    pub themes: i32,
    pub floor_filter: i32,
    pub blank_screen: i32,
    pub sound_env: u8,
  }

  #[repr(C)]
  pub struct LvlExDef {
    pub quest_flag: i32,
    pub quest_flag_ex: i32,
    pub layer: i32,
    pub width: ByNgLvl<i32>,
    pub height: ByNgLvl<i32>,
    pub offset_x: i32,
    pub offset_y: i32,
    pub depend: i32,
    pub drlg_ty: i32,
    pub lvl_ty: i32,
    pub sub_ty: i32,
    pub sub_theme: i32,
    pub sub_waypoint: i32,
    pub sub_shrine: i32,
    pub vis: [i32; 8],
    pub warps: [i32; 8],
    pub light_intensity: u8,
    pub light_color: RgbColor,
    pub portal: i32,
    pub position: i32,
    pub save_npcs: i32,
    pub los_draw: i32,
  }

  #[repr(C)]
  pub struct MAffixDef {
    pub name: [u8; 32],
    pub display_name: StrId,
    pub version: i16,
    pub mods: [ItemMod; 3],
    pub spawnable: u8,
    pub _pad1: [u8; 1],
    pub transform_color: Color,
    pub lvl: i32,
    pub group: i32,
    pub max_lvl: i32,
    pub rare: u8,
    pub lvl_req: u8,
    pub class_req: Pc,
    pub class: Pc,
    pub class_lvl_req: u8,
    pub item_tys: [Id16<ItemTy>; 7],
    pub not_item_tys: [Id16<ItemTy>; 5],
    pub freq: u8,
    pub div: i32,
    pub mul: i32,
    pub add: i32,
  }

  #[repr(C)]
  pub struct MazeLvlDef {
    pub lvl: Lvl,
    pub rooms: ByNgLvl<i32>,
    pub size: Size<i32>,
    pub merge: i32,
  }

  #[repr(C)]
  pub struct MercDef {
    pub version: i16,
    pub id: i32,
    pub class: i32,
    pub act: i32,
    pub ng_lvl: i32,
    pub seller: i32,
    pub gold: i32,
    pub lvl: i32,
    pub xp_lvl: i32,
    pub hp: i32,
    pub hp_lvl: i32,
    pub armor: i32,
    pub armor_lvl: i32,
    pub str: i32,
    pub str_lvl: i32,
    pub dex: i32,
    pub dex_lvl: i32,
    pub ar: i32,
    pub ar_lvl: i32,
    pub share: i32,
    pub dmg: Range<i32>,
    pub dmg_lvl: i32,
    pub resist: i32,
    pub resist_lvl: i32,
    pub default_chance: i32,
    pub head: i32,
    pub torso: i32,
    pub weapon: i32,
    pub shield: i32,
    pub skills: [Skill; 6],
    pub sk_chance: [i32; 6],
    pub sk_chance_lvl: [i32; 6],
    pub sk_modes: [NpcMode; 6],
    pub sk_lvl: [u8; 6],
    pub sk_lvl_per_lvl: [u8; 6],
    pub hire_desc: MercDesc,
    pub name_first: [u8; 32],
    pub name_last: [u8; 32],
    pub display_name_first: StrId,
    pub display_name_last: StrId,
  }

  #[repr(C)]
  pub struct MissileDef {
    pub missile: Id16<Missile>,
    pub flags: MissileDefFlags,
    pub client_do_fn: i16,
    pub client_hit_fn: i16,
    pub server_do_fn: i16,
    pub server_hit_fn: i16,
    pub server_dmg_fn: i16,
    pub travel_sound: Id16<Sound>,
    pub hit_sound: Id16<Sound>,
    pub explosion_missile: Id16<Missile>,
    pub server_sub_missiles: [Id16<Missile>; 3],
    pub client_sub_missiles: [Id16<Missile>; 3],
    pub server_sub_missiles_on_hit: [Id16<Missile>; 4],
    pub client_sub_missiles_on_hit: [Id16<Missile>; 4],
    pub prog_sound: Id16<Sound>,
    pub prog_overlay: Overlay,
    pub server_params: [i32; 5],
    pub server_hit_params: [i32; 3],
    pub client_params: [i32; 5],
    pub client_hit_params: [i32; 3],
    pub server_dmg_params: [i32; 2],
    pub server_calc: CodeOffset,
    pub client_calc: CodeOffset,
    pub server_hit_calc: CodeOffset,
    pub client_hit_calc: CodeOffset,
    pub server_dmg_calc: CodeOffset,
    pub hit_class: i8,
    pub range: i16,
    pub range_lvl: i16,
    pub vel: i8,
    pub vel_lvl: i8,
    pub max_vel: i8,
    pub accel: i16,
    pub anim_rate: i16,
    pub xoffset: i16,
    pub yoffset: i16,
    pub zoffset: i16,
    pub hit_flags: i32,
    pub result_flags: i16,
    pub knock_back: i8,
    pub dmg: Range<i32>,
    pub min_dmg_lvl: AccByLvl5<i32>,
    pub max_dmg_lvl: AccByLvl5<i32>,
    pub dmg_sym_per_calc: CodeOffset,
    pub el_ty: ElTy,
    pub el_dmg: Range<i32>,
    pub el_min_dmg_lvl: AccByLvl5<i32>,
    pub el_max_dmg_lvl: AccByLvl5<i32>,
    pub el_dmg_sym_per_calc: CodeOffset,
    pub el_len: i32,
    pub el_len_lvl: AccByLvl3<i32>,
    pub client_src_town: i8,
    pub src_dmg: i8,
    pub src_dmg_missile: i8,
    pub holy: i8,
    pub light_size: i8,
    pub flicker_size: i8,
    pub light_color: RgbColor,
    pub pre_vis_frames: u8,
    pub pre_collide_frames: u8,
    pub loop_anim: u8,
    pub cel_file: [u8; 64],
    pub anim_len: u8,
    pub start_frame: i32,
    pub sub_loop: u8,
    pub sub_start: u8,
    pub sub_stop: u8,
    pub collide_ty: u8,
    pub collision: u8,
    pub client_col: u8,
    pub collide_kill: u8,
    pub collide_friend: u8,
    pub collision_rate_limit: u8,
    pub collision_rate_frames: u8,
    pub size: i8,
    pub use_ar: i8,
    pub always_explode: i8,
    pub trans: i8,
    pub qty: i8,
    pub special_setup: i32,
    pub skill: Id16<Skill>,
    pub hit_shift: i8,
    pub _pad0: [u8; 5],
    pub damage_rate: i32,
    pub direction_count: i8,
    pub anim_speed: i8,
    pub local_blood: i8,
  }

  #[repr(C)]
  pub struct NpcDef {
    pub id: Id16<Npc>,
    pub base_id: Id16<Npc>,
    pub next_in_class: Id16<Npc>,
    pub name_str: StrId,
    pub desc_str: StrId,
    pub flags: NpcDefFlags,
    pub code: I32Code,
    pub sound: Id16<NpcSound>,
    pub usound: Id16<NpcSound>,
    pub npc_stats_ex: NpcEx,
    pub prop: Id16<NpcProp>,
    pub ty: Id16<NpcTy>,
    pub ai: NpcAi,
    pub spawn: Id16<Npc>,
    pub spawnx: i8,
    pub spawny: i8,
    pub spawn_mode: NpcMode,
    pub minions: [Id16<Npc>; 2],
    pub _pad0: [u8; 2],
    pub minion_count: Range<u8>,
    pub rarity: i8,
    pub group_size: Range<u8>,
    pub sparse_populate: i8,
    pub walk_speed: i16,
    pub run_speed: i16,
    pub _pad1: [u8; 4],
    pub miss_a1: Id16<Missile>,
    pub miss_a2: Id16<Missile>,
    pub miss_s1: Id16<Missile>,
    pub miss_s2: Id16<Missile>,
    pub miss_s3: Id16<Missile>,
    pub miss_s4: Id16<Missile>,
    pub miss_c: Id16<Missile>,
    pub miss_sq: Id16<Missile>,
    pub _pad2: [u8; 2],
    pub align: i8,
    pub trans_lvl: i8,
    pub threat: i8,
    pub ai_delay: ByNgLvl<u8>,
    pub ai_dist: ByNgLvl<u8>,
    pub ai_params: [ByNgLvl<i16>; 8],
    pub drop_sets: ByNgLvl<[Id16<DropSet>; 4]>,
    pub drop_set_quest_id: i8,
    pub drop_set_quest_cp: i8,
    pub leach_pct: ByNgLvl<i8>,
    pub block_pct: ByNgLvl<i8>,
    pub crit: i8,
    pub skill_dmg: Id16<Skill>,
    pub lvl: ByNgLvl<i16>,
    pub min_hp: ByNgLvl<i16>,
    pub max_hp: ByNgLvl<i16>,
    pub armor: ByNgLvl<i16>,
    pub a1_ar: ByNgLvl<i16>,
    pub a2_ar: ByNgLvl<i16>,
    pub s1_ar: ByNgLvl<i16>,
    pub xp: ByNgLvl<i16>,
    pub a1_min_dmg: ByNgLvl<i16>,
    pub a1_max_dmg: ByNgLvl<i16>,
    pub a2_min_dmg: ByNgLvl<i16>,
    pub a2_max_dmg: ByNgLvl<i16>,
    pub s1_min_dmg: ByNgLvl<i16>,
    pub s1_max_dmg: ByNgLvl<i16>,
    pub el_modes: [NpcMode; 3],
    pub el_tys: [ElTy; 3],
    pub el_pct: [ByNgLvl<i8>; 3],
    pub el_min_dmg: [ByNgLvl<i16>; 3],
    pub el_max_dmg: [ByNgLvl<i16>; 3],
    pub el_length_frames: [ByNgLvl<i16>; 3],
    pub res_dmg: ByNgLvl<i16>,
    pub res_magic: ByNgLvl<i16>,
    pub res_fire: ByNgLvl<i16>,
    pub res_lightning: ByNgLvl<i16>,
    pub res_cold: ByNgLvl<i16>,
    pub res_poison: ByNgLvl<i16>,
    pub cold_effect: ByNgLvl<i8>,
    pub send_skills: i32,
    pub skills: [Id16<Skill>; 8],
    pub sk_modes: [NpcMode; 8],
    pub sk_anims: [NpcAnim; 8],
    pub sk_lvls: [i8; 8],
    pub damage_regen: i32,
    pub spl_end_death: i8,
    pub spl_get_mode_chart: i8,
    pub spl_end_generic: i8,
    pub spl_client_end: i8,
  }

  #[repr(C)]
  pub struct NpcAnimDef {
    pub seq: NpcAnim,
    pub mode: NpcMode,
    pub frame: i8,
    pub dir: i8,
    pub event: i8,
  }

  #[repr(C)]
  pub struct NpcEquipDef {
    pub npc: Id16<Npc>,
    pub lvl: i16,
    pub on_init: i8,
    pub items: [ItemCode; 3],
    pub body_locs: [BodyLoc; 3],
    pub mods: [i8; 3],
  }

  #[repr(C)]
  pub struct NpcExDef {
    pub id: NpcEx,
    pub flags: NpcExDefFlags,
    pub size: Size<i8>,
    pub spawn_col: i8,
    pub height: i8,
    pub overlay_height: i8,
    pub pix_height: i8,
    pub melee_range: i8,
    pub base_w: I32Code,
    pub hit_class: HitClass,
    pub component_variant_count: ByComponent<u8>,
    pub _pad0: [u8; 1],
    pub component_tys: ByComponent<[i8; 12]>,
    pub enabled_components: EnabledComponents,
    pub component_count: u8,
    pub flags3: NpcExDefFlags3,
    pub enabled_modes: ByNpcMode<u8>,
    pub flags4: NpcExDefFlags4,
    pub inferno_len: i8,
    pub inferno_anim: i8,
    pub inferno_rollback: i8,
    pub res_mode: NpcMode,
    pub res_skill: Id16<Skill>,
    pub hit_test_rect: ScreenRectS<i16, i16>,
    pub automap_cel: i32,
    pub local_blood: i8,
    pub bleed: i8,
    pub light_size: i8,
    pub light_color: RgbColor,
    pub utrans: ByNgLvl<i8>,
    pub heart: I32Code,
    pub body_part: I32Code,
    pub restore: i8,
  }

  #[repr(C)]
  pub struct NpcLvlDef {
    pub armor_pct: ByNgLvl<i32>,
    pub l_armor_pct: ByNgLvl<i32>,
    pub ar_pct: ByNgLvl<i32>,
    pub l_ar_pct: ByNgLvl<i32>,
    pub hp_pct: ByNgLvl<i32>,
    pub l_hp_pct: ByNgLvl<i32>,
    pub dmg_pct: ByNgLvl<i32>,
    pub l_dmg_pct: ByNgLvl<i32>,
    pub xp_pct: ByNgLvl<i32>,
    pub l_xp_pct: ByNgLvl<i32>,
  }

  #[repr(C)]
  pub struct NpcModDef {
    pub id: Id16<NpcMod>,
    pub _pad0: [u8; 2],
    pub version: i16,
    pub enabled: i8,
    pub xfer: i8,
    pub champion: i8,
    pub fpick: i8,
    pub not_tys: [Id16<NpcTy>; 2],
    pub champ_weight: ByNgLvl<i16>,
    pub unique_weight: ByNgLvl<i16>,
    pub constants: i32,
  }

  #[repr(C)]
  pub struct NpcMod {
    pub prop: Prop,
    pub param: i32,
    pub value: Range<i32>,
  }

  #[repr(C)]
  pub struct NpcPropDef {
    pub id: NpcProp,
    pub mods: ByNgLvl<[NpcMod; 6]>,
    pub chance_pcts: ByNgLvl<[i8; 6]>,
  }

  #[repr(C)]
  pub struct NpcAttackSound {
    pub sound: Sound,
    pub delay: i32,
    pub chance_pct: i32,
  }

  #[repr(C)]
  pub struct NpcWeaponSound {
    pub sound: Sound,
    pub delay: i32,
    pub volume: i32,
  }

  #[repr(C)]
  pub struct NpcBasicSound {
    pub sound: Sound,
    pub delay: i32,
  }

  #[repr(C)]
  pub struct NpcModeSound {
    pub initial: NpcMode,
    pub target: NpcMode,
    pub skill: Skill,
  }

  #[repr(C)]
  pub struct NpcSoundDef {
    pub id: Id16<NpcSound>,
    pub on_attack1: NpcAttackSound,
    pub weapon1: NpcWeaponSound,
    pub on_attack2: NpcAttackSound,
    pub weapon2: NpcWeaponSound,
    pub on_hit: NpcBasicSound,
    pub on_death: NpcBasicSound,
    pub skills: [Sound; 4],
    pub footstep: Sound,
    pub footstep_layer: Sound,
    pub footstep_count: i32,
    pub footstep_offset: i32,
    pub footstep_chance_pct: i32,
    pub neutral: Sound,
    pub neutral_delay: i32,
    pub init: Sound,
    pub taunt: Sound,
    pub flee: Sound,
    pub on_mode_cvt: [NpcModeSound; 3],
  }

  #[repr(C)]
  pub struct NpcTyDef {
    pub id: Id16<NpcTy>,
    pub equiv: [Id16<NpcTy>; 3],
    pub str_single: StrId,
    pub str_plural: StrId,
  }

  #[repr(C)]
  pub struct ObjDef {
    pub name: [u8; 64],
    pub wname: [u16; 64],
    pub token: [u8; 3],
    pub spawn_max: u8,
    pub is_selectable: ByObjMode<u8>,
    pub trap_prob: u8,
    pub size: Size<i32>,
    pub frame_count: ByObjMode<i32>,
    pub frame_rate: ByObjMode<i16>,
    pub loop_anim: ByObjMode<i8>,
    pub light_size: ByObjMode<i8>,
    pub blocks_light: ByObjMode<i8>,
    pub has_collision: ByObjMode<i8>,
    pub is_attackable: i8,
    pub start_frame: ByObjMode<i8>,
    pub draw_order: ByObjMode<i8>,
    pub env_effect: i8,
    pub is_door: i8,
    pub blocks_vis: i8,
    pub orientation: i8,
    pub pre_operate: i8,
    pub trans: i8,
    pub has_mode: ByObjMode<i8>,
    pub xoffset: i32,
    pub yoffset: i32,
    pub draw: i8,
    pub has_components: ByComponent<u8>,
    pub component_count: u8,
    pub xspace: i8,
    pub yspace: i8,
    pub light_color: RgbColor,
    pub sub_class: i8,
    pub name_offset: i32,
    pub _pad1: [u8; 1],
    pub monster_ok: i8,
    pub operate_range: i8,
    pub shrine_fn: i8,
    pub act: i8,
    pub lockable: i8,
    pub gore: i8,
    pub restore: i8,
    pub only_restore_unused: i8,
    pub sync: i8,
    pub param: ByObjMode<i32>,
    pub n_tgt_fx: i8,
    pub n_tgt_fy: i8,
    pub n_tgt_bx: i8,
    pub n_tgt_by: i8,
    pub damage: i8,
    pub collision_subst: i8,
    pub left: i32,
    pub top: i32,
    pub width: i32,
    pub height: i32,
    pub beta: i8,
    pub init_fn: i8,
    pub populate_fn: i8,
    pub operate_fn: i8,
    pub client_fn: i8,
    pub overlay: i8,
    pub block_missile: i8,
    pub draw_under: i8,
    pub open_warp: i8,
    pub auto_map: i32,
  }

  #[repr(C)]
  pub struct OverlayDef {
    pub id: Overlay,
    pub file_name: [u8; 64],
    pub version: i16,
    pub frames: i32,
    pub pre_draw: i8,
    pub of_n: i32,
    pub dir: i8,
    pub open: i8,
    pub beta: i8,
    pub xoffset: i32,
    pub yoffset: i32,
    pub heights: [i32; 4],
    pub anim_rate: i32,
    pub init_radius: i32,
    pub radius: i32,
    pub loop_wait_time: i32,
    pub trans: i8,
    pub color: RgbColor,
    pub direction_count: i8,
    pub local_blood: i8,
  }

  #[repr(C)]
  pub struct PcDef {
    pub wclass: [u16; 16],
    pub class: [u8; 16],
    pub str: i8,
    pub dex: i8,
    pub int: i8,
    pub vit: i8,
    pub stamina: i8,
    pub hp_add: i8,
    pub pct_str: i8,
    pub pct_int: i8,
    pub pct_dex: i8,
    pub pct_vit: i8,
    pub mana_regen: i8,
    pub to_hit_factor: i32,
    pub walk_speed: i8,
    pub run_speed: i8,
    pub run_drain: i8,
    pub life_per_lvl: i8,
    pub stamina_per_lvl: i8,
    pub mana_per_lvl: i8,
    pub life_per_vit: i8,
    pub stamina_per_vit: i8,
    pub mana_per_magic: i8,
    pub block_factor: i8,
    pub base_wclass: I32Code,
    pub stat_per_level: i8,
    pub all_skills: StrId,
    pub skill_tabs: [StrId; 3],
    pub class_only: StrId,
    pub start_items: [StartItem; 10],
    pub _pad1: [u8; 2],
    pub start_skill: Id16<Skill>,
    pub skills: [Id16<Skill>; 10],
  }

  #[repr(C)]
  pub struct PetDef {
    pub id: Pet,
    pub flags: PetDefFlags,
    pub group: i16,
    pub base_max: i16,
    pub name: StrId,
    pub icon_type: i8,
    pub base_icon: [u8; 32],
    pub micons: [[u8; 32]; 4],
    pub _pad0: [u8; 2],
    pub mclass: [i16; 4],
    pub _pad1: [u8; 38],
  }

  decl_enum! { PresetNpcPlaceKind(u8) {
    Place = 0,
    Npc = 1,
    UMon = 2,
  }}

  #[derive(Clone, Copy)]
  pub union PresetNpcPlace {
    pub place: NpcPlace,
    pub npc: Id16<Npc>,
    pub umon: UMon,
  }

  #[repr(C)]
  pub struct PresetNpcDef {
    pub act: i8,
    pub place_kind: PresetNpcPlaceKind,
    pub place: PresetNpcPlace,
  }

  #[repr(C)]
  pub struct PropDef {
    pub id: Id16<Prop>,
    pub sets: [i8; 7],
    pub vals: [i16; 7],
    pub fns: [i8; 7],
    pub stats: [Id16<ItemStat>; 7],
  }

  #[repr(C)]
  pub struct RAffixDef {
    pub _pad0: [u8; 14],
    pub version: i16,
    pub item_tys: [Id16<ItemTy>; 7],
    pub not_item_tys: [Id16<ItemTy>; 4],
    pub name: [u8; 32],
    pub display_name: StrId,
  }

  #[repr(C)]
  pub struct RecipeMod {
    pub prop: Prop,
    pub param: i16,
    pub value: Range<i16>,
    pub chance: i8,
  }

  bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct RecipeInFlags: u16 {
      const Item = 0x1;
      const ItemTy = 0x2;
      const NoSocket = 0x4;
      const Socketed = 0x8;
      const Etheral = 0x10;
      const NoEtheral = 0x20;
      const UniqueSet = 0x40;
      const Upgraded = 0x80;
      const Base = 0x100;
      const Exceptional = 0x200;
      const Elite = 0x400;
      const Nru = 0x800;
    }
  }

  decl_enum! { ItemQuality(u8) {
    Low = 1,
    Normal = 2,
    High = 3,
    Magic = 4,
    Set = 5,
    Rare = 6,
    Unique = 7,
    Crafted = 8,
    Tempered = 9,
  }}

  #[derive(Clone, Copy)]
  #[repr(C)]
  pub union ItemOrTy {
    pub item: Id16<Item>,
    pub item_ty: Id16<ItemTy>,
  }

  #[derive(Clone, Copy)]
  #[repr(C)]
  pub union UniqueOrSet {
    pub unique: Id16<UItem>,
    pub set: Id16<SItem>,
  }

  #[repr(C)]
  pub struct RecipeIn {
    pub flags: RecipeInFlags,
    pub ty: ItemOrTy,
    pub unique_or_set: UniqueOrSet,
    pub item_quality: ItemQuality,
  }

  bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct RecipeOutFlags: u16 {
      const WithMods = 0x1;
      const AddSockets = 0x2;
      const Etheral = 0x4;
      const UniqueSet = 0x8;
      const DestroySocketed = 0x10;
      const RecoverSocketed = 0x20;
      const Regenerate = 0x40;
      const Exceptional = 0x80;
      const Elite = 0x100;
      const Repaired = 0x200;
      const Recharged = 0x400;
    }
  }

  decl_enum! { RecipeOutTy(u8) {
    CowPortal = 1,
    PandemoniumPortal = 2,
    PandemoniumPortalFinale = 3,
    Item = 0xfc,
    ItemTy = 0xfd,
    UseItem = 0xfe,
    UseType = 0xff,
  }}

  #[repr(C)]
  pub struct RecipeOut {
    pub flags: RecipeOutFlags,
    pub item: ItemOrTy,
    pub uitem: UniqueOrSet,
    pub quality: ItemQuality,
    pub quantity: u8,
    pub ty: RecipeOutTy,
    pub lvl: i8,
    pub plvl: i8,
    pub ilvl: i8,
    pub prefixes: [MPrefix; 3],
    pub suffixes: [MSuffix; 3],
    pub mods: [RecipeMod; 5],
  }

  #[derive(Clone, Copy)]
  #[repr(C)]
  pub union RecipeParam {
    pub value: i32,
    pub item_stat: ItemStat,
  }

  #[repr(C)]
  pub struct RecipeDef {
    pub enabled: i8,
    pub ladder: i8,
    pub min_diff: i8,
    pub class: Pc,
    pub op: i8,
    pub param: RecipeParam,
    pub value: i32,
    pub input_count: i8,
    pub version: i16,
    pub inputs: [RecipeIn; 7],
    pub output: [RecipeOut; 3],
  }

  #[repr(C)]
  pub struct RuneWordDef {
    pub name: [u8; 64],
    pub rune_name: [u8; 64],
    pub complete: i8,
    pub server: i8,
    pub _pad0: [u8; 4],
    pub item_tys: [Id16<ItemTy>; 6],
    pub not_item_tys: [Id16<ItemTy>; 3],
    pub runes: [Item; 6],
    pub mods: [ItemMod; 7],
  }

  #[repr(C)]
  pub struct SItemDef {
    pub _pad0: [u8; 2],
    pub index: [u8; 32],
    pub _pad1: [u8; 6],
    pub item: I32Code,
    pub set: Set,
    pub _pad2: [u8; 2],
    pub lvl: i16,
    pub lvl_req: i16,
    pub rarity: i32,
    pub cost_mult: i32,
    pub cost_add: i32,
    pub chr_transform: Color,
    pub inv_transform: Color,
    pub flippy_file: [u8; 32],
    pub inv_file: [u8; 32],
    pub drop_sound: Id16<Sound>,
    pub use_sound: Id16<Sound>,
    pub drops_fx_frame: i8,
    pub add_fn: i8,
    pub mods: [ItemMod; 9],
    pub set_mods: [[ItemMod; 2]; 5],
  }

  #[repr(C)]
  pub struct SetDef {
    pub id: Set,
    pub name: StrId,
    pub version: i16,
    pub _pad0: [u8; 10],
    pub partial_mods: [ItemMod; 8],
    pub full_mods: [ItemMod; 8],
    pub _pad1: [u8; 24],
  }

  #[repr(C)]
  pub struct SkDescDef {
    pub id: SkDesc,
    pub skill_page: i8,
    pub skill_row: i8,
    pub skill_column: i8,
    pub list_row: i8,
    pub list_pool: i8,
    pub icon_cel: i8,
    pub str_name: StrId,
    pub str_short: StrId,
    pub str_long: StrId,
    pub str_alt: StrId,
    pub str_mana: StrId,
    pub desc_dam: i16,
    pub desc_att: i16,
    pub dmg_calcs: [CodeOffset; 2],
    pub el_ty_by_charge: [ElTy; 3],
    pub dmg_min_by_charge: [CodeOffset; 3],
    pub dmg_max_by_charge: [CodeOffset; 3],
    pub desc_missiles: [Id16<Missile>; 3],
    pub desc_lines: [i8; 6],
    pub desc2_lines: [i8; 4],
    pub desc3_lines: [i8; 7],
    pub desc_texts: [StrId; 6],
    pub desc2_texts: [StrId; 4],
    pub desc3_texts: [StrId; 7],
    pub desc_texts2: [StrId; 6],
    pub desc2_texts2: [StrId; 4],
    pub desc3_texts2: [StrId; 7],
    pub desc_calcs: [CodeOffset; 6],
    pub desc2_calcs: [CodeOffset; 4],
    pub desc3_calcs: [CodeOffset; 7],
    pub desc_calcs2: [CodeOffset; 6],
    pub desc2_calcs2: [CodeOffset; 4],
    pub desc3_calcs2: [CodeOffset; 7],
  }

  #[repr(C)]
  pub struct SkillDef {
    pub skill: Id16<Skill>,
    pub flags: SkillDefFlags,
    pub flags2: SkillDefFlags2,
    pub char_class: Pc,
    pub _pad0: [u8; 3],
    pub anim: PcMode,
    pub mon_anim: NpcMode,
    pub seq_trans: PcMode,
    pub seq_num: i8,
    pub range: SkRange,
    pub select_proc: i8,
    pub seq_input: i8,
    pub item_tys: [[Id16<ItemTy>; 3]; 2],
    pub not_item_tys: [[Id16<ItemTy>; 2]; 2],
    pub server_st_fn: i16,
    pub server_do_fn: i16,
    pub server_prg_fns: [i16; 3],
    pub prg_calcs: [CodeOffset; 3],
    pub prg_dmg: i8,
    pub server_missile: Id16<Missile>,
    pub server_missile_secondary: [Id16<Missile>; 3],
    pub server_overlay: Overlay,
    pub aura_filter: i32,
    pub aura_stats: [Id16<ItemStat>; 6],
    pub aura_len_calc: CodeOffset,
    pub aura_range_calc: CodeOffset,
    pub aura_stat_calcs: [CodeOffset; 6],
    pub aura_state: Id16<State>,
    pub aura_target_state: Id16<State>,
    pub aura_events: [Event; 3],
    pub aura_event_fns: [i16; 3],
    pub aura_target_event: Event,
    pub aura_target_event_fn: i16,
    pub passive_state: Id16<State>,
    pub passive_item_ty: Id16<ItemTy>,
    pub passive_stats: [Id16<ItemStat>; 5],
    pub passive_calcs: [CodeOffset; 5],
    pub passive_event: Event,
    pub passive_event_fn: i16,
    pub summon: Id16<Npc>,
    pub pet_ty: Id8<Pet>,
    pub summon_mode: NpcMode,
    pub max_pets: CodeOffset,
    pub summon_skills: [Id16<Skill>; 5],
    pub summon_skill_calcs: [CodeOffset; 5],
    pub summon_mod: Id16<NpcMod>,
    pub summon_overlay: Overlay,
    pub client_missile: Id16<Missile>,
    pub client_sub_missiles: [Id16<Missile>; 4],
    pub client_st_fn: i16,
    pub client_do_fn: i16,
    pub client_prg_fns: [i16; 3],
    pub st_sound: Id16<Sound>,
    pub st_sound_class: Id16<Sound>,
    pub do_sound: Id16<Sound>,
    pub do_sub_sound: [Id16<Sound>; 2],
    pub cast_overlay: Overlay,
    pub target_overlay: Overlay,
    pub target_sound: Id16<Sound>,
    pub prg_overlay: Overlay,
    pub prg_sound: Id16<Sound>,
    pub client_overlays: [Overlay; 2],
    pub client_calcs: [CodeOffset; 3],
    pub item_target: i8,
    pub item_cast_sound: Id16<Sound>,
    pub item_cast_overlay: Overlay,
    pub per_delay: CodeOffset,
    pub max_lvl: i16,
    pub result_flags: i16,
    pub hit_flags: i32,
    pub hit_class: i32,
    pub calcs: [CodeOffset; 4],
    pub params: [i32; 8],
    pub weapon_select: i8,
    pub item_effect: i16,
    pub item_client_effect: i16,
    pub req_points: CodeOffset,
    pub req_lvl: i16,
    pub req_str: i16,
    pub req_dex: i16,
    pub req_int: i16,
    pub req_vit: i16,
    pub req_skills: [Id16<Skill>; 3],
    pub start_mana: i16,
    pub min_mana: i16,
    pub mana_shift: i16,
    pub mana: i16,
    pub mana_lvl: i16,
    pub attack_rank: i8,
    pub los: i8,
    pub delay: CodeOffset,
    pub skill_desc: SkDesc,
    pub ar: i32,
    pub ar_lvl: i32,
    pub ar_calc: CodeOffset,
    pub hit_shift: i8,
    pub use_src_dam: i8,
    pub dmg: Range<i32>,
    pub dmg_min_lvl: AccByLvl5<i32>,
    pub dmg_max_lvl: AccByLvl5<i32>,
    pub dmg_sym_per_calc: CodeOffset,
    pub el_ty: ElTy,
    pub el_dmg: Range<i32>,
    pub el_dmg_min_lvl: AccByLvl5<i32>,
    pub el_dmg_max_lvl: AccByLvl5<i32>,
    pub el_dmg_sym_per_calc: CodeOffset,
    pub el_length_frames: i32,
    pub el_length_lvl: AccByLvl3<i32>,
    pub el_len_sym_per_calc: CodeOffset,
    pub restrict: i8,
    pub states: [Id16<State>; 3],
    pub ai_ty: i8,
    pub ai_bonus: i16,
    pub cost_mult: i32,
    pub cost_add: i32,
  }

  #[repr(C)]
  pub struct StateDef {
    pub state: Id16<State>,
    pub overlays: [Overlay; 4],
    pub cast_overlay: Overlay,
    pub remove_overlay: Overlay,
    pub pgsv_overlay: Overlay,
    pub flags: StateDefFlags,
    pub flags2: StateDefFlags2,
    pub stat: Id16<ItemStat>,
    pub set_fn: i16,
    pub remove_fn: i16,
    pub group: i16,
    pub color_pri: i8,
    pub color_shift: i8,
    pub light_color: RgbColor,
    pub on_sound: Id16<Sound>,
    pub off_sound: Id16<Sound>,
    pub item_ty: Id16<ItemTy>,
    pub item_trans: Color,
    pub gfx_type: i8,
    pub gfx_class: i16,
    pub client_event: Event,
    pub client_event_fn: i16,
    pub client_active_fn: i16,
    pub server_active_fn: i16,
    pub skill: Id16<Skill>,
    pub missile: Id16<Missile>,
  }

  #[repr(C)]
  pub struct UItemDef {
    pub _pad0: [u8; 2],
    pub index: [u8; 32],
    pub _pad1: [u8; 2],
    pub version: i16,
    pub code: I32Code,
    pub flags: UItemDefFlags,
    pub rarity: i16,
    pub _pad2: [u8; 2],
    pub lvl: i16,
    pub lvl_req: i16,
    pub chr_transform: Color,
    pub inv_transform: Color,
    pub flippy_file: [u8; 32],
    pub inv_file: [u8; 32],
    pub cost_mult: i32,
    pub cost_add: i32,
    pub drop_sound: Id16<Sound>,
    pub use_sound: Id16<Sound>,
    pub drop_sfx_frame: i8,
    pub mods: [ItemMod; 12],
  }

  #[repr(C)]
  pub struct UMonDef {
    pub id: UMon,
    pub name: StrId,
    pub class: Npc,
    pub hc_idx: i32,
    pub mods: [i32; 3],
    pub sound: NpcSound,
    pub pack_size: Range<i32>,
    pub auto_pos: i8,
    pub eclass: i8,
    pub stacks: i8,
    pub replaceable: i8,
    pub utrans: ByNgLvl<i8>,
    pub drop_set: ByNgLvl<Id16<DropSet>>,
  }

  #[repr(C)]
  pub struct UNameDef {
    pub name: StrId,
  }

  #[repr(C)]
  pub struct VendorDef {
    pub npc: Npc,
    pub sell_mult: i32,
    pub buy_mult: i32,
    pub rep_mult: i32,
    pub quest_flags: [i32; 3],
    pub quest_sell_mult: [i32; 3],
    pub quest_buy_mult: [i32; 3],
    pub quest_rep_mult: [i32; 3],
    pub max_buy: ByNgLvl<i32>,
  }

  #[repr(C)]
  pub struct XpReqDef {
    pub by_pc: [i32; 7],
    pub exp_ratio: i32,
  }
}
