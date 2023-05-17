use crate::hooks::{
  draw_game, draw_game_paused, dypos_linear_whole_xpos, dypos_linear_whole_ypos, entity_iso_xpos,
  entity_iso_ypos, entity_linear_xpos, entity_linear_ypos, game_loop_sleep_hook, D2Module,
  ModulePatches, PatchSets,
};
use bin_patch::{patch_source, Patch};
use d2interface::v113d::{DyPos, Entity};

#[rustfmt::skip]
pub(super) static PATCHES: PatchSets = PatchSets {
  menu_fps: &[ModulePatches::new(
    D2Module::Win,
    0x6f8e0000,
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
      "), super::v112::update_menu_char_frame_112_asm_stub),
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
    D2Module::Client,
    0x6fab0000,
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
      D2Module::Client,
      0x6fab0000,
      &[
        // Perspective entity mouse-over text
        Patch::call_std1(0x1a780, patch_source!("e8 191cffff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x1a788, patch_source!("e8 3b1cffff"), entity_linear_ypos::<Entity>),
        // Entity mouse-over text
        Patch::call_std1(0x1a803, patch_source!("e8 ae1bffff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x1a819, patch_source!("e8 b61bffff"), entity_iso_ypos::<Entity>),
        // Create entity light
        Patch::call_std1(0x2260a, patch_source!("e8 8f9dfeff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x22619, patch_source!("e8 aa9dfeff"), entity_linear_ypos::<Entity>),
        // Apply entity light
        Patch::call_std1(0x23500, patch_source!("e8 998efeff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x23508, patch_source!("e8 bb8efeff"), entity_linear_ypos::<Entity>),
        // Lighting position
        Patch::call_std1(0x237c1, patch_source!("e8 7c8afeff"), dypos_linear_whole_xpos::<DyPos>),
        Patch::call_std1(0x237ed, patch_source!("e8 9089feff"), dypos_linear_whole_ypos::<DyPos>),
        // Viewport position
        Patch::call_std1(0x452b9, patch_source!("e8 f870fcff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x452d0, patch_source!("e8 ff70fcff"), entity_iso_ypos::<Entity>),
        // Entity shift
        Patch::call_std1(0x5be02, patch_source!("e8 af05fbff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x5be0a, patch_source!("e8 c505fbff"), entity_iso_ypos::<Entity>),
        // Entity spell overlay perspective
        Patch::call_std1(0x5ee22, patch_source!("e8 77d5faff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x5ee1b, patch_source!("e8 a8d5faff"), entity_linear_ypos::<Entity>),
        // Entity spell overlay
        Patch::call_std1(0x5ee7b, patch_source!("e8 36d5faff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x5ee90, patch_source!("e8 3fd5faff"), entity_iso_ypos::<Entity>),
        // Perspective entity single color pos
        Patch::call_std1(0x5f641, patch_source!("e8 58cdfaff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x5f63a, patch_source!("e8 89cdfaff"), entity_linear_ypos::<Entity>),
        // Entity single color pos
        Patch::call_std1(0x5f6d5, patch_source!("e8 dcccfaff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x5f701, patch_source!("e8 ceccfaff"), entity_iso_ypos::<Entity>),
        // Perspective entity shadow pos
        Patch::call_std1(0x5fa3d, patch_source!("e8 5cc9faff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x5fa47, patch_source!("e8 7cc9faff"), entity_linear_ypos::<Entity>),
        // Entity shadow pos
        Patch::call_std1(0x5facf, patch_source!("e8 e2c8faff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x5fadb, patch_source!("e8 f4c8faff"), entity_iso_ypos::<Entity>),
        // Animated entity mouse detection refinement
        Patch::call_std1(0x62e2e, patch_source!("e8 8395faff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x62e53, patch_source!("e8 7c95faff"), entity_iso_ypos::<Entity>),
        // Course entity mouse detection
        Patch::call_std1(0x632f4, patch_source!("e8 bd90faff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x632fc, patch_source!("e8 d390faff"), entity_iso_ypos::<Entity>),
        // Entity minimap position
        Patch::call_std1(0x72eab, patch_source!("e8 0695f9ff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x72eb3, patch_source!("e8 1c95f9ff"), entity_iso_ypos::<Entity>),
        // Charge effect pos
        Patch::call_std1(0x817ed, patch_source!("e8 c4abf8ff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x817f7, patch_source!("e8 d8abf8ff"), entity_iso_ypos::<Entity>),
        // Perspective charge effect pos
        Patch::call_std1(0x81911, patch_source!("e8 88aaf8ff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x8190a, patch_source!("e8 b9aaf8ff"), entity_linear_ypos::<Entity>),
        // Npc mouse over perspective
        Patch::call_std1(0xad334, patch_source!("e8 65f0f5ff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0xad32d, patch_source!("e8 96f0f5ff"), entity_linear_ypos::<Entity>),
        // Npc mouse over
        Patch::call_std1(0xad38f, patch_source!("e8 22f0f5ff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0xad3a4, patch_source!("e8 2bf0f5ff"), entity_iso_ypos::<Entity>),
        // Entity culling
        Patch::call_std1(0xb3701, patch_source!("e8 b08cf5ff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0xb370b, patch_source!("e8 c48cf5ff"), entity_iso_ypos::<Entity>),
        // Perspective entity draw pos
        Patch::call_std1(0xb4be2, patch_source!("e8 b777f5ff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0xb4bea, patch_source!("e8 d977f5ff"), entity_linear_ypos::<Entity>),
        // Entity draw pos
        Patch::call_std1(0xb4c98, patch_source!("e8 1977f5ff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0xb4ca2, patch_source!("e8 2d77f5ff"), entity_iso_ypos::<Entity>),
        // Summit background position
        Patch::call_std1(0xb5649, patch_source!("e8 686df5ff"), entity_iso_xpos::<Entity>),
        // Perspective viewport position
        Patch::call_std1(0xb6059, patch_source!("e8 4063f5ff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0xb6061, patch_source!("e8 6263f5ff"), entity_linear_ypos::<Entity>),
        // Perspective whirlwind effect pos
        Patch::call_std1(0xc587c, patch_source!("e8 1d6bf4ff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0xc5875, patch_source!("e8 4e6bf4ff"), entity_linear_ypos::<Entity>),
        // Whirlwind effect pos
        Patch::call_std1(0xc58b0, patch_source!("e8 016bf4ff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0xc58ba, patch_source!("e8 156bf4ff"), entity_iso_ypos::<Entity>),
      ],
    ),
    ModulePatches::new(
      D2Module::Common,
      0x6fd50000,
      &[Patch::call_c(0x6f367, patch_source!("e8 94faffff"), super::v112::intercept_teleport_112_asm_stub)],
    ),
  ],
};
