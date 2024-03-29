use crate::{module::Ordinal::Ordinal, Addresses, BaseAddresses};

pub use crate::v103::{DyPos, Entity, EntityTable, EntityTables, GameCursor, Room, StaticPos};

pub const ADDRESSES: Addresses = Addresses {
  player: 0xf6788,
  env_effects: 0xe2dec,
  game_type: 0xe3048,
  entity_table: 0xf4f88,
  entity_table2: 0xf5b88,
  client_loop_globals: 0xe2e38,
  // Doesn't exist in this version
  apply_pos_change: 0x0,
  in_perspective: Ordinal(10012),
  hwnd: Ordinal(10029),
  server_update_time: 0xc4774,
  draw_menu: Ordinal(10015),
  cursor_table: 0xd19f8,
  game_cursor: 0xfb4e8,
  summit_cloud_x_pos: 0,
  draw_line: Ordinal(10061),
  find_closest_color: Ordinal(10030),
  viewport_width: 0,
  viewport_height: 0,
  viewport_shift: 0xe7b6c,
  max_weather_particles: 0xe2d68,
  weather_angle: 0xe2d54,
  rain_speed: 0xe2d40,
  is_snowing: 0,
  sine_table: 0x1d1e4,
  // Signature: fastcall(&mut Rng)
  gen_weather_particle: 0x62a0,
  env_array_remove: Ordinal(10044),
};
pub const BASE_ADDRESSES: BaseAddresses = BaseAddresses {
  client: 0x6fb60000,
  common: 0x6fd80000,
  fog: 0x6ff60000,
  game: 0x6fcb0000,
  gfx: 0x6fb30000,
  win: 0x6f9a0000,
};

pub mod dtbl {
  pub use crate::v103::dtbl::*;
  use crate::{
    dtbl::{ByEqComponent, I32Code, ItemCode, Missile},
    ArmorTy, BodyLoc, Component, Id16, ItemHitClass, Range, StrId,
  };
  use num::M2d;

  #[repr(C)]
  pub struct GambleItemDef {
    pub id: ItemCode,
    pub lvl: u32,
    pub item: *const ItemDef,
  }

  #[repr(C)]
  pub struct ItemMod {
    pub prop: [u8; 5],
    pub offset: i32,
    pub param: i32,
    pub value: Range<i32>,
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
  pub struct ItemDef {
    pub completed: u8,
    pub body_locs: [BodyLoc; 2],
    pub throwable: u8,
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
    pub inv_size: M2d<u8>,
    pub block: u8,
    pub durability: u8,
    pub indestructible: u8,
    pub missile: u8,
    pub component: Component,
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
    pub _pad0: [u8; 1],
    pub light_size: u8,
    pub belt: u8,
    pub auto_belt: u8,
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
    pub hit_class: ItemHitClass,
    pub multi_handed: u8,
    pub version: i16,
    pub transform: u8,
    pub inv_trans: u8,
    pub compact_save: u8,
    pub speed: i32,
    pub bitfield1: i32,
    pub cost: i32,
    pub stack_size: Range<i32>,
    pub _pad1: [u8; 4],
    pub gem_offset: i32,
    pub code: ItemCode,
    pub alt_gfx: I32Code,
    pub uber_code: ItemCode,
    pub wclass: I32Code,
    pub wclass_2h: I32Code,
    pub _pad2: [u8; 16],
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
    pub nm_upg: ItemCode,
    pub hell_upg: ItemCode,
    pub can_sell_out: u8,
  }

  #[repr(C)]
  pub struct MAffixDef {
    pub name: [u8; 32],
    pub display_name: StrId,
    pub version: u16,
    pub mod_count: u8,
    pub mods: [ItemMod; 3],
    pub spawnable: u8,
    pub transform: u8,
    pub transform_color: u8,
    pub lvl: i32,
    pub group: i32,
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
    pub div: i32,
    pub mul: i32,
    pub add: i32,
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
  pub struct SetDef {
    pub name: [u8; 96],
    pub version: i16,
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
  pub struct UItemDef {
    pub code: ItemCode,
    pub version: i16,
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
}
