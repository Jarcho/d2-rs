use crate::hooks::{
  draw_game, draw_game_paused, dypos_linear_whole_xpos, dypos_linear_whole_ypos, entity_iso_xpos,
  entity_iso_ypos, entity_linear_xpos, entity_linear_ypos, game_loop_sleep_hook, D2Module,
  ModulePatches, PatchSets,
};
use bin_patch::{patch_source, Patch};
use d2interface::v113c::{DyPos, Entity};

#[rustfmt::skip]
pub(super) static PATCHES: PatchSets = PatchSets {
  menu_fps: &[ModulePatches::new(
    D2Module::Win,
    0x6f8e0000,
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
      "), crate::hooks::v112::update_menu_char_frame_112_asm_stub),
    ],
  )],
  game_fps: &[ModulePatches::new(
    D2Module::Client,
    0x6fab0000,
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
      D2Module::Client,
      0x6fab0000,
      &[
        // Charge effect pos
        Patch::call_std1(0x2b981, patch_source!("e8 4c08feff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x2b98b, patch_source!("e8 6608feff"), entity_iso_ypos::<Entity>),
        // Perspective charge effect pos
        Patch::call_std1(0x2baa1, patch_source!("e8 0807feff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x2ba9a, patch_source!("e8 9307feff"), entity_linear_ypos::<Entity>),
        // Entity shift
        Patch::call_std1(0x3f8a2, patch_source!("e8 2bc9fcff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x3f8aa, patch_source!("e8 47c9fcff"), entity_iso_ypos::<Entity>),
        // Viewport position
        Patch::call_std1(0x44269, patch_source!("e8 647ffcff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x44280, patch_source!("e8 717ffcff"), entity_iso_ypos::<Entity>),
        // Entity minimap position
        Patch::call_std1(0x61a4b, patch_source!("e8 82a7faff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x61a53, patch_source!("e8 9ea7faff"), entity_iso_ypos::<Entity>),
        // Perspective entity draw pos
        Patch::call_std1(0x6675c, patch_source!("e8 4d5afaff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x66764, patch_source!("e8 c95afaff"), entity_linear_ypos::<Entity>),
        // Entity draw pos
        Patch::call_std1(0x66812, patch_source!("e8 bb59faff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x6681c, patch_source!("e8 d559faff"), entity_iso_ypos::<Entity>),
        // Entity spell overlay perspective
        Patch::call_std1(0x6af42, patch_source!("e8 6712faff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x6af3b, patch_source!("e8 f212faff"), entity_linear_ypos::<Entity>),
        // Entity spell overlay
        Patch::call_std1(0x6af9b, patch_source!("e8 3212faff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x6afb0, patch_source!("e8 4112faff"), entity_iso_ypos::<Entity>),
        // Perspective entity single color pos
        Patch::call_std1(0x6b761, patch_source!("e8 480afaff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x6b75a, patch_source!("e8 d30afaff"), entity_linear_ypos::<Entity>),
        // Entity single color pos
        Patch::call_std1(0x6b7f5, patch_source!("e8 d809faff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x6b821, patch_source!("e8 d009faff"), entity_iso_ypos::<Entity>),
        // Perspective entity shadow pos
        Patch::call_std1(0x6bb5d, patch_source!("e8 4c06faff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x6bb67, patch_source!("e8 c606faff"), entity_linear_ypos::<Entity>),
        // Entity shadow pos
        Patch::call_std1(0x6bbef, patch_source!("e8 de05faff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x6bbfb, patch_source!("e8 f605faff"), entity_iso_ypos::<Entity>),
        // Npc mouse over perspective
        Patch::call_std1(0x6e6a4, patch_source!("e8 05dbf9ff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x6e69d, patch_source!("e8 90dbf9ff"), entity_linear_ypos::<Entity>),
        // Npc mouse over
        Patch::call_std1(0x6e6ff, patch_source!("e8 cedaf9ff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x6e714, patch_source!("e8 dddaf9ff"), entity_iso_ypos::<Entity>),
        // Entity culling
        Patch::call_std1(0x7a891, patch_source!("e8 3c19f9ff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x7a89b, patch_source!("e8 5619f9ff"), entity_iso_ypos::<Entity>),
        // Summit background position
        Patch::call_std1(0x8a739, patch_source!("e8 941af8ff"), entity_iso_xpos::<Entity>),
        // Perspective viewport position
        Patch::call_std1(0x8b149, patch_source!("e8 6010f8ff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x8b151, patch_source!("e8 dc10f8ff"), entity_linear_ypos::<Entity>),
        // Animated entity mouse detection refinement
        Patch::call_std1(0xa67ee, patch_source!("e8 df59f6ff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0xa6813, patch_source!("e8 de59f6ff"), entity_iso_ypos::<Entity>),
        // Course entity mouse detection
        Patch::call_std1(0xa6a54, patch_source!("e8 7957f6ff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0xa6a5c, patch_source!("e8 9557f6ff"), entity_iso_ypos::<Entity>),
        // Create entity light
        Patch::call_std1(0xa8c9a, patch_source!("e8 0f35f6ff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0xa8ca9, patch_source!("e8 8435f6ff"), entity_linear_ypos::<Entity>),
        // Apply entity light
        Patch::call_std1(0xa9b90, patch_source!("e8 1926f6ff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0xa9b98, patch_source!("e8 9526f6ff"), entity_linear_ypos::<Entity>),
        // Lighting position
        Patch::call_std1(0xa9e51, patch_source!("e8 6825f6ff"), dypos_linear_whole_xpos::<DyPos>),
        Patch::call_std1(0xa9e7d, patch_source!("e8 6a24f6ff"), dypos_linear_whole_ypos::<DyPos>),
        // Perspective entity mouse-over text
        Patch::call_std1(0xc0d80, patch_source!("e8 29b4f4ff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0xc0d88, patch_source!("e8 a5b4f4ff"), entity_linear_ypos::<Entity>),
        // Entity mouse-over text
        Patch::call_std1(0xc0e03, patch_source!("e8 cab3f4ff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0xc0e19, patch_source!("e8 d8b3f4ff"), entity_iso_ypos::<Entity>),
        // Perspective whirlwind effect pos
        Patch::call_std1(0xc5ea9, patch_source!("e8 0063f4ff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0xc5ea2, patch_source!("e8 8b63f4ff"), entity_linear_ypos::<Entity>),
        // Whirlwind effect pos
        Patch::call_std1(0xc5edd, patch_source!("e8 f062f4ff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0xc5ee7, patch_source!("e8 0a63f4ff"), entity_iso_ypos::<Entity>),
      ],
    ),
    ModulePatches::new(
      D2Module::Common,
      0x6fd50000,
      &[Patch::call_c(0xe0b7, patch_source!("e8 84f9ffff"), super::v112::intercept_teleport_112_asm_stub)],
    ),
  ],
};
