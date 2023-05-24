use crate::hooks::{
  draw_game, draw_game_paused, entity_iso_xpos, entity_iso_ypos, entity_linear_xpos,
  entity_linear_ypos, game_loop_sleep_hook, ModulePatches, PatchSets,
};
use bin_patch::{patch_source, Patch};
use d2interface::{self as d2, v113c::Entity};

#[rustfmt::skip]
pub(super) const PATCHES: PatchSets = PatchSets {
  menu_fps: &[ModulePatches::new(
    d2::Module::Win,
    &[
      // Draw menu framerate
      Patch::call_c(0x189fc, patch_source!("
        ff d5
        8b f0
        2b f3
        ff d5
        81 fe e8 03 00 00
        8b d8
        76 05
        be e8 03 00 00
        2b fe
        85 ff
        7f 26
        83 c7 28
        81 ff 18 fc ff ff
        7d 02
        33 ff
        8b 44 24 34
        85 c0
        74 0c
        8b 74 24 10
        56
        ff d0
        46
        89 74 24 10
        e8 a1 fd ff ff
      "), crate::hooks::v110::draw_menu_110_asm_stub),
      // Menu char frame rate
      Patch::call_c(0xe836, patch_source!("
        8b 43 10
        8b 73 08
        8b 4b 0c
        03 f0
        8b c6
      "), crate::hooks::v111::update_menu_char_frame_111_asm_stub),
      // Menu sleep patch
      Patch::nop(0x18a4e, patch_source!("
        8bc7
        7605
        b8 14000000
        8b0d $a8fb8f6f
        85c9
        7402
        33c0
        50
        ff15 $c8b28f6f
      ")),
    ],
  )],
  game_fps: &[ModulePatches::new(
    d2::Module::Client,
    &[
      // Game loop sleep patch
      Patch::call_c(0x3cb7c, patch_source!("
        a1 $604aba6f
        85 c0
        75 17
        a1 $94c3bc6f
        83 f8 06
        74 0d
        83 f8 08
        74 08
        6a 0a
        ff 15 $a0efb76f
      "), game_loop_sleep_hook),
      // Draw paused game framerate
      Patch::call_c(0x44bc5, patch_source!("ff 15 $e497bc6f"), draw_game_paused),
      // Draw game framerate & entity sync
      Patch::call_c(0x44e51, patch_source!("
        39 1d $9034bd6f
        75 35
        a1 $fcbbbc6f
        3b c3
        74 38
        50
        e8 88 73 fc ff
        3b c3
        74 2e
        33 c9
        ff 15 $e497bc6f
        8b 0d $f497bc6f
        a1 $0c98bc6f
        41
        40
        89 0d $f497bc6f
        a3 $0c98bc6f
        eb 0c
        39 5c 24 10
        74 06
        ff 05 $1498bc6f
      "), draw_game::<Entity>),
    ],
  )],
  game_smoothing: &[
    ModulePatches::new(
      d2::Module::Client,
      &[
        // Npc mouse over perspective
        Patch::call_std1(0x6e6a4, patch_source!("e8 05dbf9ff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x6e69d, patch_source!("e8 90dbf9ff"), entity_linear_ypos::<Entity>),
        // Npc mouse over
        Patch::call_std1(0x6e6ff, patch_source!("e8 cedaf9ff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x6e714, patch_source!("e8 dddaf9ff"), entity_iso_ypos::<Entity>),
        // Animated entity mouse detection refinement
        Patch::call_std1(0xa67ee, patch_source!("e8 df59f6ff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0xa6813, patch_source!("e8 de59f6ff"), entity_iso_ypos::<Entity>),
        // Course entity mouse detection
        Patch::call_std1(0xa6a54, patch_source!("e8 7957f6ff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0xa6a5c, patch_source!("e8 9557f6ff"), entity_iso_ypos::<Entity>),
      ],
    ),
    ModulePatches::new(
      d2::Module::Common,
      &[Patch::call_c(0xe0b7, patch_source!("e8 84f9ffff"), super::v111::intercept_teleport_111_asm_stub)],
    ),
  ],
};
