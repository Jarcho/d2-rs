use crate::{
  module::Ordinal::Ordinal, Addresses, BaseAddresses, EntityKind, FixedU16, IsoPos, LinearPos,
  LinkedList, Rand,
};
use core::ptr::NonNull;

pub type EntityTables = crate::EntityTables<Entity>;
pub type EntityTable = crate::EntityTable<Entity>;
pub type GameCursor = crate::GameCursor<Entity>;

pub const ADDRESSES: Addresses = Addresses {
  player: 0x1451b8,
  env_effects: 0x12eb64,
  game_type: 0x12ede0,
  entity_table: 0x1439b8,
  entity_table2: 0x1445b8,
  client_loop_globals: 0x12ebf0,
  // Doesn't exist in this version
  apply_pos_change: 0x0,
  in_perspective: Ordinal(10012),
  hwnd: Ordinal(10029),
  server_update_time: 0xbe75c,
  draw_menu: Ordinal(10015),
  cursor_table: 0x11b028,
  game_cursor: 0x168170,
  summit_cloud_x_pos: 0,
  draw_line: Ordinal(10061),
  find_closest_color: Ordinal(10030),
  viewport_width: 0,
  viewport_height: 0,
  viewport_shift: 0x1348ac,
};
pub const BASE_ADDRESSES: BaseAddresses = BaseAddresses {
  client: 0x10000000,
  common: 0x10000000,
  game: 0x10000000,
  gfx: 0x10000000,
  win: 0x10000000,
};

#[repr(C)]
pub struct Room {}

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
pub struct StaticPos {
  pub iso_pos: IsoPos<i32>,
  pub linear_pos: LinearPos<u32>,
  pub _padding1: [u32; 3],
  pub room: Option<NonNull<Room>>,
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
  pub id: u32,
  pub _padding1: [u32; 7],
  pub rand: Rand,
  pub seed: u32,
  pub pos: EntityPos,
  pub _padding2: [u32; 51],
  pub next_entity: Option<NonNull<Entity>>,
}
impl LinkedList for Entity {
  fn next(&self) -> Option<NonNull<Self>> {
    self.next_entity
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
  use crate::{
    dtbl::{
      ByComponent, ByEqComponent, ByNgLvl, ByNpcMode, ByObjMode, I32Code, ItemCode, ItemTyCode,
      Lvl, Missile, Npc, NpcTy, Skill, StartItem,
    },
    ArmorTy, BodyLoc, Component, ElTy, FixedI12, FixedI7, HitClass, Id16, Id8, NpcMode, NpcSpawnTy,
    Range, RgbColor, ScreenPos, ScreenRectLr, ScreenRectS, Size, StrId, TilePos,
  };

  #[repr(C)]
  pub struct BeltLayoutDef {
    pub _pad0: [u8; 4],
    pub box_count: u8,
    pub boxes: [ScreenRectLr<u32>; 16],
  }

  #[repr(C)]
  pub struct BookDef {
    pub completed: u8,
    pub spell_icon: u8,
    pub spell_offset: i32,
    pub skill_scroll_offset: i32,
    pub skill_book_offset: i32,
    pub base_cost: i32,
    pub cost_per_charge: i32,
    pub scroll_spell_code: I32Code,
    pub book_spell_code: I32Code,
    pub name: [u8; 32],
    pub _pad0: [u8; 4],
  }

  #[repr(C)]
  pub struct NgLvlDef {
    pub res_penalty: i32,
    pub xp_death_penalty: i32,
    pub uber_code_odds_normal: i32,
    pub uber_code_odds_good: i32,
    pub npc_skill_bonus: i32,
    pub npc_freeze_div: i32,
    pub npc_cold_div: i32,
    pub ai_curse_div: i32,
  }

  #[repr(C)]
  pub struct DropSetDef {
    pub item_count: u8,
    pub items: [ItemCode; 30],
  }

  #[repr(C)]
  pub struct EnvSoundDef {
    pub song: i32,
    pub day_ambience: i32,
    pub night_ambience: i32,
    pub day_event: i32,
    pub night_event: i32,
    pub event_delay: i32,
    pub indoors: u8,
    pub material_1: i32,
    pub material_2: i32,
    pub eax_environ: i32,
    pub eax_env_size: i32,
    pub eax_env_diff: i32,
    pub eax_room_vol: i32,
    pub eax_room_hf: i32,
    pub eax_decay_time: i32,
    pub eax_decay_hf: i32,
    pub eax_reflect: i32,
    pub eax_reflect_delay: i32,
    pub eax_reverb: i32,
    pub eax_rev_delay: i32,
    pub eax_room_roll: i32,
    pub eax_air_absorb: i32,
  }

  #[repr(C)]
  pub struct ItemMod {
    pub prop: [u8; 5],
    pub offset: i32,
    pub min: i32,
    pub max: i32,
  }

  #[repr(C)]
  pub struct GemDef {
    pub name: [u8; 32],
    pub item: ItemCode,
    pub display_name: StrId,
    pub mod_count: u8,
    pub transform: u8,
    pub weapon_mods: [ItemMod; 3],
    pub helm_mods: [ItemMod; 3],
    pub shield_mods: [ItemMod; 3],
  }

  #[repr(C)]
  pub struct ItemRatioDef {
    pub unique: i32,
    pub unique_div: i32,
    pub rare: i32,
    pub rare_div: i32,
    pub set: i32,
    pub set_div: i32,
    pub magic: i32,
    pub magic_div: i32,
    pub hq: i32,
    pub hq_div: i32,
    pub normal: i32,
    pub normal_div: i32,
  }

  #[repr(C)]
  pub struct ItemStatDef {
    pub div: i32,
    pub mul: i32,
    pub add: i32,
    pub item_specific: i32,
  }

  #[repr(C)]
  pub struct InvLayoutDef {
    pub pos: ScreenRectLr<u32>,
    pub grid_size: Size<u8>,
    pub grid_pos: ScreenRectLr<u32>,
    pub grid_box_size: Size<u8>,
    pub rarm_pos: ScreenRectLr<u32>,
    pub rarm_size: Size<u8>,
    pub torso_pos: ScreenRectLr<u32>,
    pub torso_size: Size<u8>,
    pub larm_pos: ScreenRectLr<u32>,
    pub larm_width: Size<u8>,
    pub head_pos: ScreenRectLr<u32>,
    pub head_size: Size<u8>,
    pub neck_pos: ScreenRectLr<u32>,
    pub neck_size: Size<u8>,
    pub rhand_pos: ScreenRectLr<u32>,
    pub rhand_size: Size<u8>,
    pub lhand_pos: ScreenRectLr<u32>,
    pub lhand_size: Size<u8>,
    pub belt_pos: ScreenRectLr<u32>,
    pub belt_size: Size<u8>,
    pub feet_pos: ScreenRectLr<u32>,
    pub feet_size: Size<u8>,
    pub gloves_pos: ScreenRectLr<u32>,
    pub gloves_size: Size<u8>,
  }

  #[repr(C)]
  pub struct PerVendor {
    pub akara: u8,
    pub gheed: u8,
    pub charsi: u8,
    pub fara: u8,
    pub lysander: u8,
    pub drognan: u8,
    pub hralti: u8,
    pub alkor: u8,
    pub ormus: u8,
    pub elzix: u8,
    pub asheara: u8,
    pub cain: u8,
    pub halbu: u8,
    pub jamella: u8,
  }

  #[repr(C)]
  pub struct ItemDef {
    pub completed: u8,
    pub rarity: u8,
    pub lvl: u8,
    pub dmg: Range<u8>,
    pub dmg_throw: Range<u8>,
    pub dmg_2h: Range<u8>,
    pub melee_range: u8,
    pub str_bonus: u8,
    pub dex_bonus: u8,
    pub armor: Range<i32>,
    pub req_str: u8,
    pub req_dex: u8,
    pub absorbs: u8,
    pub inv_size: Size<u8>,
    pub block: u8,
    pub durability: u8,
    pub indestructible: u8,
    pub missile: u8,
    pub replenish: u8,
    pub special: u8,
    pub component: Component,
    pub body_locs: [BodyLoc; 2],
    pub armor_gfx: ByEqComponent<ArmorTy>,
    pub two_handed: u8,
    pub useable: u8,
    pub ty: u8,
    pub sub_ty: u8,
    pub sound: u8,
    pub unique: u8,
    pub quest: u8,
    pub transparent: u8,
    pub trans_tbl: u8,
    pub throwable: u8,
    pub _pad0: [u8; 1],
    pub light_size: u8,
    pub belt: u8,
    pub auto_belt: u8,
    pub quivered: u8,
    pub stackable: u8,
    pub spawnable: u8,
    pub missile_ty: Id16<Missile>,
    pub spell_icon: u8,
    pub dur_warning: u8,
    pub qnt_warning: u8,
    pub has_sockets: u8,
    pub socket_count: u8,
    pub transmogrify: u8,
    pub tmog_qnt: Range<u8>,
    pub _pad1: [u8; 1],
    pub hit_class: HitClass,
    pub multi_handed: u8,
    pub transform: u8,
    pub inv_trans: u8,
    pub weapon_group_enum: u8,
    pub weapon_speed_enum: u8,
    pub speed: i32,
    pub bitfield1: i32,
    pub cost: i32,
    pub stack_size: Range<i32>,
    pub spell_offset: i32,
    pub gem_offset: i32,
    pub code: ItemCode,
    pub alt_gfx: I32Code,
    pub uber_code: ItemCode,
    pub wclass: I32Code,
    pub wclass_2h: I32Code,
    pub group: [u8; 15],
    pub tmog_ty: ItemCode,
    pub name: [u8; 64],
    pub wname: [u16; 64],
    pub flippy_file: [u8; 32],
    pub inv_file: [u8; 32],
    pub uinv_file: [u8; 32],
    pub better_gem: ItemCode,
    pub skip_name: u8,
    pub vend_qnt: Range<PerVendor>,
    pub vend_mqnt: Range<PerVendor>,
    pub vend_mlvl: PerVendor,
    pub nm_upg: u8,
    pub hell_upg: u8,
  }

  #[repr(C)]
  pub struct LqItemDef {
    pub name: [u8; 32],
    pub wname: [u16; 32],
    pub _pad0: [u8; 64],
  }

  #[repr(C)]
  pub struct LvlDef {
    pub id: u8,
    pub pal: u8,
    pub act: u8,
    pub rain: u8,
    pub mud: u8,
    pub no_per: u8,
    pub is_inside: u8,
    pub warp_dist: i32,
    pub area_lvl: ByNgLvl<u8>,
    pub mon_density: i32,
    pub umon_spawn_count: Range<u8>,
    pub mon_wndr: u8,
    pub mon_spc_walk: u8,
    pub quest: u8,
    pub mon_count: u8,
    pub mons: [Npc; 25],
    pub smons: [Npc; 25],
    pub umon_count: u8,
    pub umons: [Npc; 25],
    pub critters: [Npc; 5],
    pub ca: [i32; 5],
    pub cd: [i32; 5],
    pub waypoint: u8,
    pub obj_groups: [u8; 8],
    pub obj_weights: [u8; 8],
    pub lvl_name: [u8; 40],
    pub lvl_warp: [u8; 40],
    pub entry_file: [u8; 40],
    pub wlvl_name: [u16; 40],
    pub wlvl_warp: [u16; 40],
    pub themes: i32,
    pub sound_env: u8,
  }

  #[repr(C)]
  pub struct LvlExDef {
    pub layer: i32,
    pub size_x: i32,
    pub size_y: i32,
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
    pub warp: [i32; 8],
    pub light_intensity: u8,
    pub light_color: RgbColor,
    pub portal: i32,
    pub position: i32,
    pub save_npcs: i32,
    pub los_draw: i32,
  }

  #[repr(C)]
  pub struct LvlTyDef {
    pub files: [[u8; 60]; 32],
    pub act: u8,
  }

  #[repr(C)]
  pub struct LvlWarpDef {
    pub select: ScreenRectS<i32, i32>,
    pub exit_walk: TilePos<i32>,
    pub offset: TilePos<i32>,
    pub lit_version: i32,
  }

  #[repr(C)]
  pub struct MAffixDef {
    pub mod_count: u8,
    pub mods: [ItemMod; 3],
    pub armor: u8,
    pub shield: u8,
    pub weapon: u8,
    pub scepter: u8,
    pub wand: u8,
    pub staff: u8,
    pub bow: u8,
    pub boots: u8,
    pub gloves: u8,
    pub belt: u8,
    pub ring: u8,
    pub amulet: u8,
    pub spawnable: u8,
    pub transform: u8,
    pub transform_color: u8,
    pub lvl: i32,
    pub group: i32,
    pub div: i32,
    pub mul: i32,
    pub add: i32,
    pub name: [u8; 32],
    pub display_name: StrId,
  }

  #[repr(C)]
  pub struct MapTileDef {
    pub lvl_name: [u8; 16],
    pub tile_name: [u8; 8],
    pub style: u8,
    pub seq: Range<u8>,
    pub cel: [i32; 4],
  }

  #[repr(C)]
  pub struct MazeLvlDef {
    pub lvl: Lvl,
    pub rooms: i32,
    pub room_size: Size<u32>,
    pub merge: i32,
  }

  #[repr(C)]
  pub struct MissileDef {
    pub vel: u8,
    pub max_vel: u8,
    pub accel: i32,
    pub range: i32,
    pub range_lvl: i32,
    pub light_size: u8,
    pub flicker_size: u8,
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
    pub collide_kill: u8,
    pub collide_friend: u8,
    pub last_collide: u8,
    pub can_destroy: u8,
    pub client_send: u8,
    pub collision_rate_limit: u8,
    pub collision_rate_frames: u8,
    pub size: u8,
    pub use_ar: u8,
    pub always_explode: u8,
    pub is_explosion: u8,
    pub never_dup: u8,
    pub trigger_target_effects: u8,
    pub trigger_recovery: u8,
    pub knock_back_pct: u8,
    pub blend_mode: u8,
    pub use_qty: u8,
    pub inherit_pierce_chance: u8,
    pub params: [i32; 2],
    pub open: u8,
    pub beta: u8,
    pub special_setup: i32,
    pub skill: Skill,
    pub dmg_shift: u8,
    pub use_src_dmg: FixedI7,
    pub dmg: Range<i32>,
    pub lvl_dmg: i32,
    pub el_ty: ElTy,
    pub edmg: Range<i32>,
    pub edmg_lvl: i32,
    pub elen_frames: i32,
    pub elen_frames_lvl: i32,
    pub _pad0: [u8; 4],
    pub hit_class: i32,
    pub direction_count: u8,
    pub local_blood: u8,
  }

  #[repr(C)]
  pub struct NpcDef {
    pub name: [u8; 60],
    pub wname: [u16; 60],
    pub descriptor: [u8; 60],
    pub wdescriptor: [u16; 60],
    pub base_id: i32,
    pub spawned: u8,
    pub code: [u8; 5],
    pub hp: Range<ByNgLvl<i32>>,
    pub no_map: u8,
    pub size: Size<i32>,
    pub height: u8,
    pub overlay_height: u8,
    pub walk_speed: i32,
    pub run_speed: i32,
    pub _pad2: [u8; 4],
    pub armor: ByNgLvl<i32>,
    pub xp: ByNgLvl<i32>,
    pub lvl: ByNgLvl<u8>,
    pub melee_range: u8,
    pub rarity: u8,
    pub group_size: Range<u8>,
    pub used_components: ByComponent<u8>,
    pub component_count: u8,
    pub base_w: I32Code,
    pub ai_params: [u8; 5],
    pub used_modes: ByNpcMode<u8>,
    pub el_mode: NpcMode,
    pub el_ty: ElTy,
    pub el_chance_pct: u8,
    pub el_dmg: Range<u8>,
    pub el_len_frames: u8,
    pub missile_a1: Missile,
    pub missile_a2: Missile,
    pub missile_s1: Missile,
    pub missile_s2: Missile,
    pub missile_s3: Missile,
    pub missile_s4: Missile,
    pub missile_c: Missile,
    pub missile_sq: Missile,
    pub a1_move: u8,
    pub a1_dmg: Range<ByNgLvl<i32>>,
    pub a1_ar: ByNgLvl<i32>,
    pub a2_move: u8,
    pub a2_dmg: Range<ByNgLvl<i32>>,
    pub a2_ar: ByNgLvl<i32>,
    pub s1_move: u8,
    pub s1_mg: Range<ByNgLvl<i32>>,
    pub s1_ar: ByNgLvl<i32>,
    pub s2_move: u8,
    pub s3_move: u8,
    pub s4_move: u8,
    pub block_chance: u8,
    pub cmove: u8,
    pub is_ally: u8,
    pub is_melee: u8,
    pub has_hover_life: u8,
    pub has_hover_name: u8,
    pub never_select: u8,
    pub can_select_corpse: u8,
    pub is_attackable: u8,
    pub ignore_pets: u8,
    pub is_npc: u8,
    pub is_critter: u8,
    pub in_town: u8,
    pub blood_ty: u8,
    pub has_shadow: u8,
    pub light_size: u8,
    pub no_unique_shift: u8,
    pub composite_death: u8,
    pub skills: [Skill; 5],
    pub skill_seqs: [u8; 5],
    pub skill_lvls: [u8; 5],
    pub light_color: RgbColor,
    pub dmg_resist: ByNgLvl<u8>,
    pub mdmg_resist: ByNgLvl<u8>,
    pub fire_resist: ByNgLvl<u8>,
    pub light_resist: ByNgLvl<u8>,
    pub cold_resist: ByNgLvl<u8>,
    pub poison_resist: ByNgLvl<u8>,
    pub hp_regen: FixedI12,
    pub is_low_undead: u8,
    pub is_high_undead: u8,
    pub is_demon: u8,
    pub is_magic_using: u8,
    pub is_large: u8,
    pub is_small: u8,
    pub is_flying: u8,
    pub can_open_doors: u8,
    pub is_boss: u8,
    pub spawn_ty: NpcSpawnTy,
    pub pix_height: u8,
    pub can_interact: u8,
    pub spawn_components: u8,
    pub is_soft: u8,
    pub heart: I32Code,
    pub body_part: I32Code,
    pub drop_sets: ByNgLvl<[u8; 4]>,
    pub spawn_pct_bonus: u8,
    pub can_die: u8,
    pub can_change_align: u8,
    pub is_saved: u8,
    pub no_quest_count: u8,
    pub hit_class: HitClass,
    pub spl_end_death: u8,
    pub spl_get_mode_chart: u8,
    pub spl_end_generic: u8,
    pub spl_client_end: u8,
    pub corpse_collision: u8,
    pub corpse_unwalkable: u8,
    pub blood_local: u8,
    pub does_dmg_on_death: u8,
    pub no_gfx_hit_test: u8,
    pub hit_test_ul_pos: ScreenPos<i32>,
    pub hit_test_size: Size<u8>,
  }

  #[repr(C)]
  pub struct NpcAnimDef {
    pub name: [u8; 32],
    pub token: I32Code,
    pub direction_count: ByNpcMode<u8>,
  }

  #[repr(C)]
  pub struct NpcItemPctDef {
    pub heart_pct: u8,
    pub body_part_pct: u8,
    pub drop_set_pct: u8,
    pub component_pct: u8,
  }

  #[repr(C)]
  pub struct ObjDef {
    pub name: [u8; 64],
    pub wname: [u16; 64],
    pub token: [u8; 3],
    pub spawn_max: u8,
    pub selectable: ByObjMode<u8>,
    pub trap_prob: u8,
    pub size: Size<i32>,
    pub frame_count: ByObjMode<i32>,
    pub frame_rate: ByObjMode<i32>,
    pub loop_anim: ByObjMode<u8>,
    pub light_size: ByObjMode<u8>,
    pub block_light: ByObjMode<u8>,
    pub can_collide: ByObjMode<u8>,
    pub is_attackable: u8,
    pub start_frame: ByObjMode<u8>,
    pub draw_order: ByObjMode<u8>,
    pub env_effect: u8,
    pub is_door: u8,
    pub blocks_vis: u8,
    pub orientation: u8,
    pub pre_operate: u8,
    pub trans: u8,
    pub has_mode: ByObjMode<u8>,
    pub offset: ScreenPos<i32>,
    pub draw: u8,
    pub has_components: ByComponent<u8>,
    pub component_count: u8,
    pub xspace: u8,
    pub yspace: u8,
    pub light_color: RgbColor,
    pub sub_class: u8,
    pub name_offset: i32,
    pub _pad1: [u8; 1],
    pub monster_ok: u8,
    pub operate_range: u8,
    pub shrine_function: u8,
    pub act: u8,
    pub lockable: u8,
    pub gore: u8,
    pub restore: u8,
    pub restore_virgins: u8,
    pub sync: u8,
    pub flicker: u8,
    pub parms: [i32; 8],
    pub n_tgt_fx: u8,
    pub n_tgt_fy: u8,
    pub n_tgt_bx: u8,
    pub n_tgt_by: u8,
    pub damage: u8,
    pub collision_subst: u8,
    pub left: i32,
    pub top: i32,
    pub width: i32,
    pub height: i32,
    pub beta: u8,
    pub init_fn: u8,
    pub populate_fn: u8,
    pub operate_fn: u8,
    pub client_fn: u8,
    pub overlay: u8,
    pub block_missile: u8,
    pub draw_under: u8,
    pub open_warp: u8,
    pub auto_map: i32,
  }

  #[repr(C)]
  pub struct ObjGroupDef {
    pub ids: [i32; 8],
    pub density: [u8; 8],
    pub prob: [u8; 8],
    pub shrines: u8,
    pub wells: u8,
  }

  #[repr(C)]
  pub struct OverlayDef {
    pub filename: [u8; 64],
    pub frames: i32,
    pub pre_draw: u8,
    pub of_n: i32,
    pub dir: u8,
    pub open: u8,
    pub beta: u8,
    pub offset: ScreenPos<i32>,
    pub height: [i32; 4],
    pub anim_rate: i32,
    pub init_radius: i32,
    pub radius: i32,
    pub loop_wait_time: i32,
    pub trans: u8,
    pub color: RgbColor,
    pub direction_count: u8,
    pub local_blood: u8,
  }

  #[repr(C)]
  pub struct PcDef {
    pub wclass: [u16; 16],
    pub class: [u8; 16],
    pub str: u8,
    pub dex: u8,
    pub int: u8,
    pub vit: u8,
    pub hp_add: u8,
    pub pct_str: u8,
    pub pct_int: u8,
    pub pct_dex: u8,
    pub pct_vit: u8,
    pub mana_regen: u8,
    pub to_hit_factor: i32,
    pub walk_speed: u8,
    pub run_speed: u8,
    pub run_drain: u8,
    pub life_per_lvl: u8,
    pub stamina_per_lvl: u8,
    pub mana_per_lvl: u8,
    pub life_per_vit: u8,
    pub stamina_per_vit: u8,
    pub mana_per_magic: u8,
    pub block_factor: u8,
    pub base_wclass: I32Code,
    pub start_skill: Skill,
    pub start_items: [StartItem; 10],
  }

  #[repr(C)]
  pub struct PresetLvlDef {
    pub def: i32,
    pub lvl: Lvl,
    pub populate: i32,
    pub logicals: i32,
    pub is_outdoors: i32,
    pub has_animated_tiles: i32,
    pub kill_edge: i32,
    pub fill_blanks: i32,
    pub _pad0: [u8; 4],
    pub size: Size<u32>,
    pub revealed_map: i32,
    pub scan: i32,
    pub pops: i32,
    pub pop_pad: i32,
    pub file_count: i32,
    pub files: [[u8; 60]; 6],
    pub dt1_mask: i32,
  }

  #[repr(C)]
  pub struct QItemDef {
    pub armor: u8,
    pub weapon: u8,
    pub shield: u8,
    pub scepter: u8,
    pub wand: u8,
    pub staff: u8,
    pub bow: u8,
    pub boots: u8,
    pub gloves: u8,
    pub belt: u8,
    pub mod_count: u8,
    pub mods: [ItemMod; 2],
    pub effects: [[u8; 32]; 2],
    pub display_effects: [StrId; 2],
  }

  #[repr(C)]
  pub struct RAffixDef {
    pub armor: u8,
    pub helm: u8,
    pub shield: u8,
    pub sword: u8,
    pub axe: u8,
    pub mace: u8,
    pub spear: u8,
    pub scepter: u8,
    pub wand: u8,
    pub staff: u8,
    pub bow: u8,
    pub boots: u8,
    pub gloves: u8,
    pub belt: u8,
    pub ring: u8,
    pub amulet: u8,
    pub add: i32,
    pub multiply: i32,
    pub divide: i32,
    pub name: [u8; 32],
    pub display_name: StrId,
  }

  #[repr(C)]
  pub struct SetItemDef {
    pub item: ItemCode,
    pub display_suffix: StrId,
    pub suffix: [u8; 32],
  }

  #[repr(C)]
  pub struct SetDef {
    pub name: [u8; 96],
    pub display_name: StrId,
    pub prefix: [u8; 32],
    pub display_prefix: StrId,
    pub item_count: u8,
    pub prop_count: u8,
    pub transform: u8,
    pub transform_color: u8,
    pub lvl: i32,
    pub items: [SetItemDef; 6],
    pub mods: [ItemMod; 25],
  }

  #[repr(C)]
  pub struct ShrineDef {
    pub code: u8,
    pub arg0: i32,
    pub arg1: i32,
    pub duration_in_frames: i32,
    pub reset_time_in_minutes: u8,
    pub rarity: u8,
    pub view_name: [u8; 32],
    pub nifty_phrase: [u8; 128],
    pub effect_class: u8,
    pub lvl_min: i32,
  }

  #[repr(C)]
  pub struct SkillDef {
    pub _pad0: [u8; 4],
    pub name: [u8; 31],
    pub pc_class: [u8; 31],
    pub class_req: i32,
    pub attack_rank: u8,
    pub item_tys: [ItemTyCode; 6],
    pub anim: [u8; 8],
    pub mon_anim: [u8; 8],
    pub _pad1: [u8; 8],
    pub seq_num: u8,
    pub durability: u8,
    pub shiver: u8,
    pub use_ar: i32,
    pub line_of_sight: u8,
    pub targetable_only: i32,
    pub search_enemy_xy: i32,
    pub search_monster_near: i32,
    pub select_corpse: i32,
    pub search_open_xy: i32,
    pub target_pet: u8,
    pub target_ally: u8,
    pub range: [u8; 8],
    pub _pad2: [u8; 6],
    pub attack_no_mana: i32,
    pub req_level: u8,
    pub req_str: u8,
    pub req_dex: u8,
    pub req_int: u8,
    pub req_vit: u8,
    pub req_skill1: [u8; 31],
    pub _pad3: [u8; 4],
    pub req_skill2: [u8; 31],
    pub _pad4: [u8; 5],
    pub req_skill3: [u8; 31],
    pub _pad5: [u8; 5],
    pub states: [i32; 3],
    pub skill_page: u8,
    pub skill_row: u8,
    pub skill_column: u8,
    pub icon_cel: u8,
    pub left_skl: u8,
    pub mana_shift: i32,
    pub mana: i32,
    pub lvl_mana: i32,
    pub interrupt: u8,
    pub in_town: u8,
    pub periodic: u8,
    pub passive: u8,
    pub params: [i32; 6],
    pub in_game: i32,
    pub open: i32,
    pub beta: i32,
    pub _pad6: [u8; 12],
    pub ar: i32,
    pub lvl_ar: i32,
    pub hit_shift: u8,
    pub use_src_dam: u8,
    pub dmg: Range<i32>,
    pub dmg_lvl: i32,
    pub el_ty: ElTy,
    pub el_dmg: Range<i32>,
    pub el_dmg_lvl: i32,
    pub el_len: i32,
    pub el_len_lvl: i32,
  }

  #[repr(packed)]
  pub struct SoundDef {
    pub filename: [u8; 60],
    pub volume: u8,
    pub group_size: u8,
    pub repeat: u8,
    pub fade_in: u8,
    pub fade_out: u8,
    pub defer_inst: u8,
    pub stop_inst: u8,
    pub duration: i16,
    pub compound: i16,
    pub falloff: i32,
    pub reverb: u8,
    pub cache: u8,
    pub async_only: u8,
    pub priority: u8,
    pub stream: u8,
    pub stereo: u8,
    pub tracking: u8,
    pub solo: u8,
    pub music_vol: u8,
    pub block: [i32; 3],
    pub _pad0: [u8; 46],
  }

  #[repr(C)]
  pub struct SubLvlDef {
    pub ty: i32,
    pub file: [u8; 60],
    pub check_all: i32,
    pub bord_ty: i32,
    pub dt1_mask: i32,
    pub grid_size: i32,
    pub _pad0: [u8; 204],
    pub weights: [i32; 5],
    pub trials: [i32; 5],
    pub max: [i32; 5],
  }

  #[repr(C)]
  pub struct UItemDef {
    pub code: ItemCode,
    pub name: [u8; 32],
    pub display_name: StrId,
    pub beta: u8,
    pub transform: u8,
    pub inv_transform: u8,
    pub transform_color: u8,
    pub mod_count: u8,
    pub lvl: i32,
    pub mods: [ItemMod; 10],
  }

  #[repr(C)]
  pub struct UMonDef {
    pub name: [u8; 60],
    pub wname: [u16; 60],
    pub class: i32,
    pub mods: [i32; 3],
    pub minion_count: Range<i32>,
  }

  #[repr(C)]
  pub struct UNameDef {
    pub name: [u8; 60],
    pub wname: [u16; 60],
    pub mon_tys: [Id8<NpcTy>; 36],
    pub _pad1: [u8; 374],
  }

  #[repr(C)]
  pub struct XpReqDef {
    pub by_pc: [i32; 5],
  }
}
