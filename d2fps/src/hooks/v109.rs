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
  v109::{Entity, ADDRESSES, BASE_ADDRESSES},
};

#[rustfmt::skip]
pub(super) const HOOKS: Hooks = Hooks {
  version: "1.09",
  addresses: ADDRESSES,
  base_addresses: BASE_ADDRESSES,
  load_modules: d2::Modules::load_split_modules,
  patches: FeaturePatches::new(
    &[ModulePatches::new(
      d2::Module::Win,
      &[
        // Draw menu framerate
        Patch::call_c(0xebe4, patch_source!("
          8d4c2414
          51
          ff15 $c8d18b6f
          8d54241c
          52
          ff15 $80d18b6f
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
          e84b610000
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
          e84e060000
        "), super::v100::draw_menu_100_asm_stub),
        // Menu char frame rate
        Patch::call_c(0x1b6a, patch_source!("
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
        Patch::call_c(0x262c, patch_source!("
          a1 $5057b76f
          85c0
          7508
          6a00
          ff15 $9ccfb66f
        "), game_loop_sleep_hook),
        // Draw paused game framerate
        Patch::call_c(0x9448, patch_source!("ff15 $541bbb6f"), draw_game_paused),
        // Draw game framerate & entity sync
        Patch::call_c(0x9b6f, patch_source!("
          392d $e01dbb6f
          752b
          e8244d0800
          85c0
          742e
          33c9
          ff15 $541bbb6f
          8b0d $641bbb6f
          a1 $7c1bbb6f
          41
          40
          890d $641bbb6f
          a3 $7c1bbb6f
          eb0c
          396c2414
          7406
          ff05 $841bbb6f
        "), draw_game::<Entity>),
      ],
    )],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          // Course entity mouse detection
          Patch::call_std1(0x8e08c, patch_source!("e849110300"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x8e094, patch_source!("e83b110300"), entity_iso_ypos::<Entity>),
          // Animated entity mouse detection refinement
          Patch::call_std1(0x8e536, patch_source!("e89f0c0300"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x8e55d, patch_source!("e8720c0300"), entity_iso_ypos::<Entity>),
          // Npc mouse over perspective
          Patch::call_std1(0xbe757, patch_source!("e80c0a0000"), entity_linear_xpos::<Entity>),
          Patch::call_std1(0xbe750, patch_source!("e80d0a0000"), entity_linear_ypos::<Entity>),
          // Npc mouse over
          Patch::call_std1(0xbe7a4, patch_source!("e8310a0000"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0xbe7b7, patch_source!("e8180a0000"), entity_iso_ypos::<Entity>),
        ],
      ),
      ModulePatches::new(
        d2::Module::Common,
        &[
          Patch::call_c(0x5f997, patch_source!("
            893e
            8d442414
          "), super::v100::intercept_teleport_100_asm_stub),
        ],
      ),
    ],
  )
};
