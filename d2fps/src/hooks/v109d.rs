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
  v109d::{Entity, ADDRESSES, BASE_ADDRESSES},
};

#[rustfmt::skip]
pub(super) const HOOKS: Hooks = Hooks {
  version: "1.09d",
  addresses: ADDRESSES,
  base_addresses: BASE_ADDRESSES,
  load_modules: d2::Modules::load_split_modules,
  patches: FeaturePatches::new(
    &[ModulePatches::new(
      d2::Module::Win,
      &[
        // Draw menu framerate
        Patch::call_c(0xebe4, patch_source!("
          8d 4c 24 14
          51
          ff 15 $c8d18b6f
          8d 54 24 1c
          52
          ff 15 $80d18b6f
          8b 74 24 14
          8b 7c 24 18
          8b c6
          8b cf
          2b c3
          6a 00
          1b cd
          6a 19
          51
          50
          e8 4b 61 00 00
          3b 54 24 20
          7c 27
          7f 06
          3b 44 24 1c
          76 1f
          8b 44 24 44
          8b de
          85 c0
          8b ef
          74 0e
          8b 54 24 10
          8b ca
          42
          51
          89 54 24 14
          ff d0
          e8 4e 06 00 00
        "), super::v100::draw_menu_100_asm_stub),
        // Menu char frame rate
        Patch::call_c(0x1b6a, patch_source!("
          8b 4e 10
          8b 46 08
          8b 56 0c
          03 c1
        "), super::v100::update_menu_char_frame_100_asm_stub),
      ],
    )],
    &[ModulePatches::new(
      d2::Module::Client,
      &[
        // Game loop sleep patch
        Patch::call_c(0x262c, patch_source!("
          a1 $3847b76f
          85 c0
          75 08
          6a 00
          ff 15 $9cbfb66f
        "), game_loop_sleep_hook),
        // Draw paused game framerate
        Patch::call_c(0x9438, patch_source!("ff 15 $b409bb6f"), draw_game_paused),
        // Draw game framerate & entity sync
        Patch::call_c(0x9b5f, patch_source!("
          39 2d $400cbb6f
          75 2b
          e8 b4 40 08 00
          85 c0
          74 2e
          33 c9
          ff 15 $b409bb6f
          8b 0d $c409bb6f
          a1 $dc09bb6f
          41
          40
          89 0d $c409bb6f
          a3 $dc09bb6f
          eb 0c
          39 6c 24 14
          74 06
          ff 05 $e409bb6f
        "), draw_game::<Entity>),
      ],
    )],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          // Course entity mouse detection
          Patch::call_std1(0x8d40c, patch_source!("e8 49110300"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x8d414, patch_source!("e8 3b110300"), entity_iso_ypos::<Entity>),
          // Animated entity mouse detection refinement
          Patch::call_std1(0x8d8b6, patch_source!("e8 9f0c0300"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x8d8dd, patch_source!("e8 720c0300"), entity_iso_ypos::<Entity>),
          // Npc mouse over perspective
          Patch::call_std1(0xbdad7, patch_source!("e8 0c0a0000"), entity_linear_xpos::<Entity>),
          Patch::call_std1(0xbdad0, patch_source!("e8 0d0a0000"), entity_linear_ypos::<Entity>),
          // Npc mouse over
          Patch::call_std1(0xbdb24, patch_source!("e8 310a0000"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0xbdb37, patch_source!("e8 180a0000"), entity_iso_ypos::<Entity>),
        ],
      ),
      ModulePatches::new(
        d2::Module::Common,
        &[
          Patch::call_c(0x5f9f7, patch_source!("
            893e
            8d442414
          "), super::v100::intercept_teleport_100_asm_stub),
        ],
      ),
    ],
  )
};
