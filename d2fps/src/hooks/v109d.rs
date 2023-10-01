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
          6axx
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
        "), super::v100::update_menu_char_frame_100_asm_stub),
      ],
    )],
    &[ModulePatches::new(
      d2::Module::Client,
      &[
        // Game loop sleep patch
        Patch::call_c(0x262c, patch_source!("
          a1 $3847b76f
          85c0
          7508
          6a00
          ff15 $9cbfb66f
        "), game_loop_sleep_hook),
        // Draw paused game framerate
        Patch::call_c(0x9438, patch_source!("ff15 $b409bb6f"), draw_game_paused),
        // Draw game framerate & entity sync
        Patch::call_c(0x9b5f, patch_source!("
          xxxxxxxxxxxx
          xxxx
          e8b4400800
          85c0
          742e
          33c9
          ff15 $b409bb6f
          8b0d $c409bb6f
          a1 $dc09bb6f
          41
          40
          890d $c409bb6f
          a3 $dc09bb6f
          eb0c
          396c2414
          7406
          ff05 $e409bb6f
        "), draw_game::<Entity>),
      ],
    )],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          // Course entity mouse detection
          Patch::call_std1(0x8d40c, patch_source!("e849110300"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x8d414, patch_source!("e83b110300"), entity_iso_ypos::<Entity>),
          // Animated entity mouse detection refinement
          Patch::call_std1(0x8d8b6, patch_source!("e89f0c0300"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x8d8dd, patch_source!("e8720c0300"), entity_iso_ypos::<Entity>),
          // Npc mouse over perspective
          Patch::call_std1(0xbdad7, patch_source!("e80c0a0000"), entity_linear_xpos::<Entity>),
          Patch::call_std1(0xbdad0, patch_source!("e80d0a0000"), entity_linear_ypos::<Entity>),
          // Npc mouse over
          Patch::call_std1(0xbdb24, patch_source!("e8310a0000"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0xbdb37, patch_source!("e8180a0000"), entity_iso_ypos::<Entity>),
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
