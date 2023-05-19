use crate::hooks::{
  draw_game, draw_game_paused, dypos_linear_whole_xpos, dypos_linear_whole_ypos, entity_iso_xpos,
  entity_iso_ypos, entity_linear_xpos, entity_linear_ypos, game_loop_sleep_hook,
  intercept_teleport, update_menu_char_frame, D2Module, ModulePatches, PatchSets,
};
use bin_patch::{patch_source, Patch};
use core::arch::global_asm;
use d2interface::v112::{DyPos, Entity};

#[rustfmt::skip]
pub(super) const PATCHES: PatchSets = PatchSets {
  menu_fps: &[ModulePatches::new(
    D2Module::Win,
    0x6f8e0000,
    &[
      // Draw menu framerate
      Patch::call_c(0xd92c, patch_source!("
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
      Patch::call_c(0x16386, patch_source!("
        8b 43 10
        8b 73 08
        8b 4b 0c
        03 f0
        8b c6
      "), update_menu_char_frame_112_asm_stub),
      // Menu sleep patch
      Patch::nop(0xd97e, patch_source!("
        8bc7
        7605
        b8 14000000
        8b0d $ecfa8f6f
        85c9
        7402
        33c0
        50
        ff15 $a8a28f6f
      ")),
    ],
  )],
  game_fps: &[ModulePatches::new(
    D2Module::Client,
    0x6fab0000,
    &[
      // Game loop sleep patch
      Patch::call_c(0x6cfbc, patch_source!("
        a1 $803ab96f
        85 c0
        75 17
        a1 $f8bfbc6f
        83 f8 06
        74 0d
        83 f8 08
        74 08
        6a 0a
        ff 15 $7cefb76f
      "), game_loop_sleep_hook),
      // Draw paused game framerate
      Patch::call_c(0x7cf55, patch_source!("ff 15 $9c32bb6f"), draw_game_paused),
      // Draw game framerate & entity sync
      Patch::call_c(0x7d1e1, patch_source!("
        39 1d $2834bd6f
        75 35
        a1 $d0c3bc6f
        3b c3
        74 38
        50
        e8 c2 ef f8 ff
        3b c3
        74 2e
        33 c9
        ff 15 $9c32bb6f
        8b 0d $ac32bb6f
        a1 $c432bb6f
        41
        40
        89 0d $ac32bb6f
        a3 $c432bb6f
        eb 0c
        39 5c 24 10
        74 06
        ff 05 $cc32bb6f
      "), draw_game::<Entity>),
    ],
  )],
  game_smoothing: &[
    ModulePatches::new(
      D2Module::Client,
      0x6fab0000,
      &[
        // Animated entity mouse detection refinement
        Patch::call_std1(0x1ff1e, patch_source!("e8 e5c2feff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x1ff43, patch_source!("e8 ccc2feff"), entity_iso_ypos::<Entity>),
        // Course entity mouse detection
        Patch::call_std1(0x20184, patch_source!("e8 7fc0feff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x2018c, patch_source!("e8 83c0feff"), entity_iso_ypos::<Entity>),
        // Entity minimap position
        Patch::call_std1(0x3f98b, patch_source!("e8 78c8fcff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x3f993, patch_source!("e8 7cc8fcff"), entity_iso_ypos::<Entity>),
        // Entity shift
        Patch::call_std1(0x5db52, patch_source!("e8 b1e6faff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x5db5a, patch_source!("e8 b5e6faff"), entity_iso_ypos::<Entity>),
        // Create entity light
        Patch::call_std1(0x5eb6a, patch_source!("e8 93d6faff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x5eb79, patch_source!("e8 b4d6faff"), entity_linear_ypos::<Entity>),
        // Apply entity light
        Patch::call_std1(0x5fa60, patch_source!("e8 9dc7faff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x5fa68, patch_source!("e8 c5c7faff"), entity_linear_ypos::<Entity>),
        // Lighting position
        Patch::call_std1(0x5fd21, patch_source!("e8 aec5faff"), dypos_linear_whole_xpos::<DyPos>),
        Patch::call_std1(0x5fd4d, patch_source!("e8 10c5faff"), dypos_linear_whole_ypos::<DyPos>),
        // Entity culling
        Patch::call_std1(0x6de91, patch_source!("e8 72e3f9ff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x6de9b, patch_source!("e8 74e3f9ff"), entity_iso_ypos::<Entity>),
        // Viewport position
        Patch::call_std1(0x7ca49, patch_source!("e8 baf7f8ff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x7ca60, patch_source!("e8 aff7f8ff"), entity_iso_ypos::<Entity>),
        // Charge effect pos
        Patch::call_std1(0x8747a, patch_source!("e8 894df8ff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x87484, patch_source!("e8 8b4df8ff"), entity_iso_ypos::<Entity>),
        // Perspective charge effect pos
        Patch::call_std1(0x875a1, patch_source!("e8 5c4cf8ff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x8759a, patch_source!("e8 934cf8ff"), entity_linear_ypos::<Entity>),
        // Perspective entity mouse-over text
        Patch::call_std1(0x8d4f0, patch_source!("e8 0dedf7ff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x8d4f8, patch_source!("e8 35edf7ff"), entity_linear_ypos::<Entity>),
        // Entity mouse-over text
        Patch::call_std1(0x8d573, patch_source!("e8 90ecf7ff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x8d589, patch_source!("e8 86ecf7ff"), entity_iso_ypos::<Entity>),
        // Entity spell overlay perspective
        Patch::call_std1(0x92d22, patch_source!("e8 db94f7ff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x92d1b, patch_source!("e8 1295f7ff"), entity_linear_ypos::<Entity>),
        // Entity spell overlay
        Patch::call_std1(0x92d7b, patch_source!("e8 8894f7ff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x92d90, patch_source!("e8 7f94f7ff"), entity_iso_ypos::<Entity>),
        // Perspective entity single color pos
        Patch::call_std1(0x93541, patch_source!("e8 bc8cf7ff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x9353a, patch_source!("e8 f38cf7ff"), entity_linear_ypos::<Entity>),
        // Entity single color pos
        Patch::call_std1(0x935d5, patch_source!("e8 2e8cf7ff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x93601, patch_source!("e8 0e8cf7ff"), entity_iso_ypos::<Entity>),
        // Perspective entity shadow pos
        Patch::call_std1(0x9392d, patch_source!("e8 d088f7ff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x93937, patch_source!("e8 f688f7ff"), entity_linear_ypos::<Entity>),
        // Entity shadow pos
        Patch::call_std1(0x939bf, patch_source!("e8 4488f7ff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x939cb, patch_source!("e8 4488f7ff"), entity_iso_ypos::<Entity>),
        // Npc mouse over perspective
        Patch::call_std1(0xb7d84, patch_source!("e8 7944f5ff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0xb7d7d, patch_source!("e8 b044f5ff"), entity_linear_ypos::<Entity>),
        // Npc mouse over
        Patch::call_std1(0xb7ddf, patch_source!("e8 2444f5ff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0xb7df4, patch_source!("e8 1b44f5ff"), entity_iso_ypos::<Entity>),
        // Summit background position
        Patch::call_std1(0xbab99, patch_source!("e8 6a16f5ff"), entity_iso_xpos::<Entity>),
        // Perspective viewport position
        Patch::call_std1(0xbb669, patch_source!("e8 940bf5ff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0xbb671, patch_source!("e8 bc0bf5ff"), entity_linear_ypos::<Entity>),
        // Perspective entity draw pos
        Patch::call_std1(0xbbabc, patch_source!("e8 4107f5ff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0xbbac4, patch_source!("e8 6907f5ff"), entity_linear_ypos::<Entity>),
        // Entity draw pos
        Patch::call_std1(0xbbb72, patch_source!("e8 9106f5ff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0xbbb7c, patch_source!("e8 9306f5ff"), entity_iso_ypos::<Entity>),
        // Perspective whirlwind effect pos
        Patch::call_std1(0xc491f, patch_source!("e8 de78f4ff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0xc4918, patch_source!("e8 1579f4ff"), entity_linear_ypos::<Entity>),
        // Whirlwind effect pos
        Patch::call_std1(0xc4953, patch_source!("e8 b078f4ff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0xc495d, patch_source!("e8 b278f4ff"), entity_iso_ypos::<Entity>),
      ],
    ),
    ModulePatches::new(
      D2Module::Common,
      0x6fd50000,
      &[Patch::call_c(0x4b467, patch_source!("e8 94faffff"), intercept_teleport_112_asm_stub)],
    ),
  ],
};

global_asm! {
  ".global _update_menu_char_frame_112_asm_stub",
  "_update_menu_char_frame_112_asm_stub:",
  "mov ecx, [ebx+0x10]",
  "lea edx, [ebx+0x08]",
  "call {}",
  "mov ecx, [ebx+0x0c]",
  "mov esi, eax",
  "ret",
  sym update_menu_char_frame,
}
extern "C" {
  pub fn update_menu_char_frame_112_asm_stub();
}

global_asm! {
  ".global _intercept_teleport_112_asm_stub",
  "_intercept_teleport_112_asm_stub:",
  "mov ecx, [esi+0x30]",
  "mov edx, [esp+0x8]",
  "push edx",
  "mov edx, [esp+0x8]",
  "call {}",
  "jmp eax",
  sym intercept_teleport::<Entity>,
}
extern "C" {
  pub fn intercept_teleport_112_asm_stub();
}
