use crate::{
  features::{FeaturePatches, ModulePatches},
  hooks::{
    draw_game, draw_game_paused, entity_iso_xpos, entity_iso_ypos, entity_linear_xpos,
    entity_linear_ypos, game_loop_sleep_hook, Hooks,
  },
};
use bin_patch::{patch_source, Patch};
use d2interface::{
  self as d2,
  v102::{Entity, ADDRESSES, BASE_ADDRESSES},
};

#[rustfmt::skip]
pub(super) const HOOKS: Hooks = Hooks {
  version: "1.02",
  addresses: ADDRESSES,
  base_addresses: BASE_ADDRESSES,
  load_modules: d2::Modules::load_split_modules,
  patches: FeaturePatches::new(
    &[ModulePatches::new(
      d2::Module::Win,
      &[
        // Draw menu framerate
        Patch::call_c(0xeb44, patch_source!("
          8d4c2414
          51
          ff15 $78780f10
          8d54241c
          52
          ff15 $c0780f10
          8b742414
          8b7c2418
          8bc6
          8bcf
          2bc3
          6a00
          1bcd
          6a19
          51
          50
          e80b660000
          3b542420
          7c27
          7f06
          3b44241c
          761f
          8b442444
          8bde
          85c0
          8bef
          740e
          8b542410
          8bca
          42
          51
          89542414
          ffd0
          e87a26ffff
        "), super::v100::draw_menu_100_asm_stub),
        // Menu char frame rate
        Patch::call_c(0x26ea, patch_source!("
          8b4e10
          8b4608
          8b560c
          03c1
          894608
        "), super::v100::update_menu_char_frame_100_asm_stub),
      ],
    )],
    &[ModulePatches::new(
      d2::Module::Client,
      &[
        // Game loop sleep patch
        Patch::call_c(0x762c, patch_source!("
          a1 $f0560f10
          85c0
          7508
          6a00
          ff15 $941d1810
        "), game_loop_sleep_hook),
        // Draw paused game framerate
        Patch::call_c(0xf4e5, patch_source!("ff15 $5ce91210"), draw_game_paused),
        // Draw game framerate & entity sync
        Patch::call_c(0xfc5d, patch_source!("
          391d $e8eb1210
          752b
          e8d132ffff
          85c0
          742e
          33c9
          ff15 $5ce91210
          8b0d $6ce91210
          a1 $84e91210
          41
          40
          890d $6ce91210
          a3 $84e91210
          eb0c
          395c2418
          7406
          ff05 $8ce91210
        "), draw_game::<Entity>),
      ],
    )],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          // Course entity mouse detection
          Patch::call_std1(0x9d479, patch_source!("e878920300"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x9d481, patch_source!("e86a920300"), entity_iso_ypos::<Entity>),
          // Animated entity mouse detection refinement
          Patch::call_std1(0x9da86, patch_source!("e86b8c0300"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x9daad, patch_source!("e83e8c0300"), entity_iso_ypos::<Entity>),
          // Npc mouse over perspective
          Patch::call_std1(0xd566e, patch_source!("e8f90f0000"), entity_linear_xpos::<Entity>),
          Patch::call_std1(0xd5667, patch_source!("e8fa0f0000"), entity_linear_ypos::<Entity>),
          // Npc mouse over
          Patch::call_std1(0xd569a, patch_source!("e857100000"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0xd56ad, patch_source!("e83e100000"), entity_iso_ypos::<Entity>),
        ],
      ),
      ModulePatches::new(
        d2::Module::Common,
        &[
          Patch::call_c(0x6024f, patch_source!("
            893e
            8d442414
          "), super::v100::intercept_teleport_100_asm_stub),
        ],
      ),
    ],
  )
};
