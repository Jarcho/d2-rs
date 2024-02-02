use crate::{module::Ordinal::Ordinal, Addresses};

pub use crate::v107::{
  DyPos, Entity, EntityPos, EntityTable, EntityTables, GameCursor, Room, StaticPos, BASE_ADDRESSES,
};

pub const ADDRESSES: Addresses = Addresses {
  player: 0x1245e0,
  env_effects: 0x10eb6c,
  game_type: 0x10edd0,
  entity_table: 0x122de0,
  entity_table2: 0x1239e0,
  client_loop_globals: 0x10ebc0,
  // Doesn't exist in this version
  apply_pos_change: 0,
  in_perspective: Ordinal(10010),
  hwnd: Ordinal(10027),
  server_update_time: 0xee68c,
  draw_menu: Ordinal(10019),
  cursor_table: 0xfdcd8,
  game_cursor: 0x129418,
  summit_cloud_x_pos: 0,
  draw_line: Ordinal(10057),
  find_closest_color: Ordinal(10034),
  viewport_width: 0x101b84,
  viewport_height: 0x101b80,
  viewport_shift: 0x113e24,
  max_weather_particles: 0x10eac0,
  weather_angle: 0x10eaac,
  rain_speed: 0x10eaa4,
  is_snowing: 0x10eb84,
  sine_table: 0x2022c,
  // Signature: fastcall(&mut Rng)
  gen_weather_particle: 0x7740,
  env_array_remove: Ordinal(10065),
};

pub mod dtbl {
  pub use crate::v107::dtbl::*;
  use crate::{
    dtbl::{Item, ItemTy},
    Id8,
  };
  use num::M2d;

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
    pub extra_unique_mon: i32,
    pub unique_dmg_bonus: i32,
    pub champion_dmg_bonus: i32,
    pub hireable_boss_dmg_pct: i32,
    pub static_field_min: i32,
  }

  #[repr(C)]
  pub struct LvlTyDef {
    pub files: [[u8; 60]; 32],
    pub act: u8,
    pub expansion: i32,
  }

  #[repr(C)]
  pub struct PresetLvlDef {
    pub def: i32,
    pub level_id: i32,
    pub populate: i32,
    pub logicals: i32,
    pub is_outdoors: i32,
    pub has_animated_tiles: i32,
    pub kill_edge: i32,
    pub fill_blanks: i32,
    pub expansion: i32,
    pub _pad0: [u8; 4],
    pub size: M2d<u32>,
    pub revealed_map: i32,
    pub scan: i32,
    pub pops: i32,
    pub pop_pad: i32,
    pub file_count: i32,
    pub files: [[u8; 60]; 6],
    pub dt1_mask: i32,
  }

  #[repr(C)]
  pub struct RuneWordDef {
    pub name: [u8; 64],
    pub rune_name: [u8; 64],
    pub complete: u8,
    pub server: u8,
    pub _pad0: [u8; 4],
    pub item_tys: [Id8<ItemTy>; 6],
    pub not_item_tys: [Id8<ItemTy>; 3],
    pub runes: [Item; 6],
    pub mods: [ItemMod; 7],
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
    pub expansion: i32,
  }
}
