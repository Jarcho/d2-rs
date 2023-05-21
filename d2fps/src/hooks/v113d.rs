use crate::hooks::{
  draw_game, draw_game_paused, entity_iso_xpos, entity_iso_ypos, entity_linear_xpos,
  entity_linear_ypos, game_loop_sleep_hook, ModulePatches, PatchSets,
};
use bin_patch::{patch_source, Patch};
use d2interface::{self as d2, v113d::Entity};

#[rustfmt::skip]
pub(super) const PATCHES: PatchSets = PatchSets {
  menu_fps: &[ModulePatches::new(
    d2::Module::Win,
    &[
      // Draw menu framerate
      Patch::call_c(0xed4c, patch_source!("
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
      "), super::v110::draw_menu_110_asm_stub),
      // Menu char frame rate
      Patch::call_c(0xc226, patch_source!("
        8b 43 10
        8b 73 08
        8b 4b 0c
        03 f0
        8b c6
      "), super::v111::update_menu_char_frame_111_asm_stub),
      // Menu sleep patch
      Patch::nop(0xed9e, patch_source!("
        8bc7
        7605
        b8 14000000
        8b0d $c0fb8f6f
        85c9
        7402
        33c0
        50
        ff15 $b8b28f6f
      ")),
    ],
  )],
  game_fps: &[ModulePatches::new(
    d2::Module::Client,
    &[
      // Game loop sleep patch
      Patch::call_c(0x2770c, patch_source!("
        a1 $007bba6f
        85 c0
        75 17
        a1 $dcd1bc6f
        83 f8 06
        74 0d
        83 f8 08
        74 08
        6a 0a
        ff 15 $6cffb76f
      "), game_loop_sleep_hook),
      // Draw paused game framerate
      Patch::call_c(0x45c15, patch_source!("ff 15 $4487bb6f"), draw_game_paused),
      // Draw game framerate & entity sync
      Patch::call_c(0x45ea1, patch_source!("
        39 1d $f046bd6f
        75 35
        a1 $50d0bc6f
        3b c3
        74 38
        50
        e8 f0 63 fc ff
        3b c3
        74 2e
        33 c9
        ff 15 $4487bb6f
        8b 0d $5487bb6f
        a1 $6c87bb6f
        41
        40
        89 0d $5487bb6f
        a3 $6c87bb6f
        eb 0c
        39 5c 24 10
        74 06
        ff 05 $7487bb6f
      "), draw_game::<Entity>),
    ],
  )],
  game_smoothing: &[
    ModulePatches::new(
      d2::Module::Client,
      &[
        // Animated entity mouse detection refinement
        Patch::call_std1(0x62e2e, patch_source!("e8 8395faff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x62e53, patch_source!("e8 7c95faff"), entity_iso_ypos::<Entity>),
        // Course entity mouse detection
        Patch::call_std1(0x632f4, patch_source!("e8 bd90faff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x632fc, patch_source!("e8 d390faff"), entity_iso_ypos::<Entity>),
        // Npc mouse over perspective
        Patch::call_std1(0xad334, patch_source!("e8 65f0f5ff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0xad32d, patch_source!("e8 96f0f5ff"), entity_linear_ypos::<Entity>),
        // Npc mouse over
        Patch::call_std1(0xad38f, patch_source!("e8 22f0f5ff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0xad3a4, patch_source!("e8 2bf0f5ff"), entity_iso_ypos::<Entity>),
      ],
    ),
    ModulePatches::new(
      d2::Module::Common,
      &[Patch::call_c(0x6f367, patch_source!("e8 94faffff"), super::v112::intercept_teleport_112_asm_stub)],
    ),
  ],
};
