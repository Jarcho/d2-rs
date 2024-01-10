use crate::{module::Ordinal::Ordinal, Addresses, BaseAddresses};

pub use crate::v108::{
  DyPos, Entity, EntityPos, EntityTable, EntityTables, GameCursor, Room, StaticPos,
};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x127578,
  env_effects: 0x111afc,
  game_type: 0x111d60,
  entity_table: 0x125d78,
  entity_table2: 0x126978,
  client_loop_globals: 0x111b50,
  // Doesn't exist in this version
  apply_pos_change: 0,
  in_perspective: Ordinal(10010),
  hwnd: Ordinal(10027),
  server_update_time: 0xf4300,
  draw_menu: Ordinal(10019),
  cursor_table: 0x100c78,
  game_cursor: 0x12c2a8,
  summit_cloud_x_pos: 0,
  draw_line: Ordinal(10057),
  find_closest_color: Ordinal(10034),
  viewport_width: 0x104b14,
  viewport_height: 0x104b10,
  viewport_shift: 0x116db4,
  max_weather_particles: 0x111a50,
  weather_angle: 0x111a3c,
  rain_speed: 0x111a34,
  is_snowing: 0x111b14,
  sine_table: 0x2122c,
  // Signature: fastcall(&mut Rng)
  gen_weather_particle: 0x77f0,
  env_array_remove: Ordinal(10065),
};
pub const BASE_ADDRESSES: BaseAddresses = BaseAddresses {
  client: 0x6faa0000,
  common: 0x6fd40000,
  fog: 0x6ff50000,
  game: 0x6fc30000,
  gfx: 0x6fa70000,
  win: 0x6f8a0000,
};

pub mod dtbl {
  pub use crate::v108::dtbl::*;
  use crate::{
    dtbl::{
      AccByLvl3, ByEqComponent, ByLvl, ByNgLvl, DropSet, I32Code, ItemCode, ItemTy, ItemTyCode,
      Missile, Prop, Skill,
    },
    ArmorTy, BodyLoc, Component, ElTy, Id16, Id8, ItemHitClass, Pc, Range, RgbColor, Size,
    StorePage, FI7,
  };

  #[repr(C)]
  pub struct GambleItemDef {
    pub id: ItemCode,
    pub lvl: u32,
    pub item: *const ItemDef,
  }

  #[repr(C)]
  pub struct ItemRatioDef {
    pub unique: i32,
    pub unique_div: i32,
    pub unique_min: i32,
    pub rare: i32,
    pub rare_divi: i32,
    pub rare_min: i32,
    pub set: i32,
    pub set_div: i32,
    pub set_min: i32,
    pub magic: i32,
    pub magic_div: i32,
    pub magic_min: i32,
    pub hq: i32,
    pub hq_div: i32,
    pub normal: i32,
    pub normal_div: i32,
    pub version: i16,
    pub uber: u8,
    pub class_specific: u8,
  }

  #[repr(C)]
  pub struct ItemTyDef {
    pub code: ItemTyCode,
    pub equiv: [Id8<ItemTy>; 2],
    pub can_repair: u8,
    pub body: u8,
    pub body_loc: [BodyLoc; 2],
    pub shoots: ItemTy,
    pub quiver: ItemTy,
    pub is_throwable: u8,
    pub is_reloadable: u8,
    pub is_reequipable: u8,
    pub can_auto_stack: u8,
    pub magic: u8,
    pub rare: u8,
    pub normal: u8,
    pub is_charm: u8,
    pub is_gem: u8,
    pub is_beltable: u8,
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
    pub name: [u8; 64],
    pub wname: [u16; 64],
    pub flippy_file: [u8; 32],
    pub inv_file: [u8; 32],
    pub uinv_file: [u8; 32],
    pub sinv_file: [u8; 32],
    pub better_gem: ItemCode,
    pub code: ItemCode,
    pub norm_code: ItemCode,
    pub uber_code: ItemCode,
    pub ultra_code: ItemCode,
    pub alt_gfx: I32Code,
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
    pub _pad1: [u8; 4],
    pub gem_offset: i32,
    pub version: i16,
    pub auto_prefix: i16,
    pub missile_ty: Id16<Missile>,
    pub rarity: u8,
    pub lvl: u8,
    pub dmg: Range<u8>,
    pub dmg_throw: Range<u8>,
    pub dmg_2h: Range<u8>,
    pub melee_range: u8,
    pub str_bonus: u8,
    pub dex_bonus: u8,
    pub req_str: u8,
    pub req_dex: u8,
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
    pub ty: [Id8<ItemTy>; 2],
    pub sub_ty: u8,
    pub sound: u8,
    pub unique: u8,
    pub quest: u8,
    pub transparent: u8,
    pub trans_tbl: u8,
    pub _pad2: [u8; 1],
    pub light_size: u8,
    pub belt: u8,
    pub auto_belt: u8,
    pub stackable: u8,
    pub spawnable: u8,
    pub spell_icon: u8,
    pub dur_warning: u8,
    pub qnt_warning: u8,
    pub has_sockets: u8,
    pub socket_count: u8,
    pub transmogrify: u8,
    pub tmog_qnt: Range<u8>,
    pub hit_class: ItemHitClass,
    pub multi_handed: u8,
    pub gem_apply_ty: u8,
    pub lvl_req: u8,
    pub mlvl: u8,
    pub transform: u8,
    pub inv_trans: u8,
    pub compact_save: u8,
    pub skip_name: u8,
    pub nameable: u8,
    pub vend_qnt: Range<PerVendor>,
    pub vend_mqnt: Range<PerVendor>,
    pub vend_mlvl: PerVendor,
    pub nm_upg: I32Code,
    pub hell_upg: I32Code,
    pub can_sell_out: u8,
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
    pub draw_edges: u8,
    pub warp_dist: i32,
    pub mlvl: ByNgLvl<u8>,
    pub mlvl_ex: ByNgLvl<u8>,
    pub mon_density: i32,
    pub umon_spawn_count: Range<u8>,
    pub mon_wndr: u8,
    pub mon_spc_walk: u8,
    pub quest: u8,
    pub mon_count: u8,
    pub mons: [i32; 25],
    pub smons: [i32; 25],
    pub umon_count: u8,
    pub umons: [i32; 25],
    pub critters: [i32; 5],
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
    pub floor_filter: i32,
    pub blank_screen: i32,
    pub sound_env: u8,
  }

  #[repr(C)]
  pub struct MissileDef {
    pub vel: u8,
    pub max_vel: u8,
    pub accel: i32,
    pub range: i32,
    pub lvl_range: i32,
    pub light_size: u8,
    pub flicker_size: u8,
    pub color: RgbColor,
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
    pub last_collide: u8,
    pub can_destroy: u8,
    pub client_send: u8,
    pub collision_rate_limit: u8,
    pub collision_rate_frames: u8,
    pub size: u8,
    pub use_ar: u8,
    pub always_explode: u8,
    pub is_explosion: u8,
    pub can_slow: u8,
    pub trigger_target_effects: u8,
    pub trigger_recovery: u8,
    pub knock_back: u8,
    pub trans: u8,
    pub qty: u8,
    pub inherit_pierce_chance: u8,
    pub params: [i32; 2],
    pub open: u8,
    pub beta: u8,
    pub special_setup: i32,
    pub skill: Skill,
    pub hit_shift: u8,
    pub use_src_dmg: FI7,
    pub dmg: Range<i32>,
    pub dmg_lvl: AccByLvl3<i32>,
    pub el_ty: ElTy,
    pub el_dmg: Range<i32>,
    pub el_dmg_lvl: AccByLvl3<i32>,
    pub el_len: i32,
    pub el_len_lvl: AccByLvl3<i32>,
    pub _pad0: [u8; 4],
    pub hit_class: i32,
    pub dmg_rate: i32,
    pub direction_count: u8,
    pub anim_speed: u8,
    pub local_blood: u8,
  }

  #[repr(C)]
  pub struct RecipeMod {
    pub prop: Prop,
    pub param: i16,
    pub value: Range<u16>,
    pub weight: u8,
  }

  #[repr(C)]
  pub struct RecipeDef {
    pub enabled: u8,
    pub min_ng_lvl: u8,
    pub start_day: u8,
    pub stop_day: u8,
    pub weekday: u8,
    pub class: Pc,
    pub input_count: u8,
    pub version: i16,
    pub inputs: [([u8; 24], [u16; 4]); 7],
    pub _pad6: [u8; 2],
    pub output: [u8; 24],
    pub _pad7: [u8; 9],
    pub lvl: u8,
    pub plvl: u8,
    pub ilvl: u8,
    pub _pad8: [u8; 12],
    pub mods: [RecipeMod; 5],
  }

  #[repr(C)]
  pub struct UMonDef {
    pub name: [u8; 60],
    pub wname: [u16; 60],
    pub class: i32,
    pub mods: [i32; 3],
    pub group_size: Range<i32>,
    pub auto_pos: i32,
    pub eclass: i32,
    pub stacks: i32,
    pub drop_set: ByNgLvl<DropSet>,
  }
}