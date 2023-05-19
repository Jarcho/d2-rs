use crate::hooks::{
  draw_game, draw_game_paused, entity_iso_xpos, entity_iso_ypos, entity_linear_xpos,
  entity_linear_ypos, game_loop_sleep_hook, intercept_teleport, D2Module, ModulePatches, PatchSets,
};
use bin_patch::{patch_source, Patch};
use core::arch::global_asm;
use d2interface::v112::Entity;

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
      "), super::v111::update_menu_char_frame_111_asm_stub),
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
        // Npc mouse over perspective
        Patch::call_std1(0xb7d84, patch_source!("e8 7944f5ff"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0xb7d7d, patch_source!("e8 b044f5ff"), entity_linear_ypos::<Entity>),
        // Npc mouse over
        Patch::call_std1(0xb7ddf, patch_source!("e8 2444f5ff"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0xb7df4, patch_source!("e8 1b44f5ff"), entity_iso_ypos::<Entity>),
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
