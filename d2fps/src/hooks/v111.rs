use crate::hooks::{
  draw_game, draw_game_paused, game_loop_sleep_hook, update_menu_char_frame, D2Module,
  ModulePatches, PatchSets,
};
use bin_patch::{patch_source, Patch};
use core::arch::global_asm;
use d2interface::v111::Entity;

#[rustfmt::skip]
pub(super) const PATCHES: PatchSets = PatchSets {
  menu_fps: &[ModulePatches::new(
    D2Module::Win,
    0x6f8e0000,
    &[
      // Draw menu framerate
      Patch::call_c(0xc61c, patch_source!("
        ffd5
        8bf0
        2bf3
        ffd5
        81fe e8030000
        8bd8
        7605
        be e8030000
        2bfe
        85ff
        7f26
        83c728
        81ff 18fcffff
        7d02
        33ff
        8b442434
        85c0
        740c
        8b742410
        56
        ffd0
        46
        89742410
        e8 a1fdffff
      "), crate::hooks::v110::draw_menu_110_asm_stub),
      // Menu char frame rate
      Patch::call_c(0x17b66, patch_source!("
        8b4310
        8b7308
        8b4b0c
        03f0
        8bc6
      "), update_menu_char_frame_111_asm_stub),
      // Menu sleep patch
      Patch::nop(0xc66e, patch_source!("
        8bc7
        7605
        b8 14000000
        8b0d $f0fa8f6f
        85c9
        7402
        33c0
        50
        ff15 $a0a28f6f
      ")),
    ],
  )],
  game_fps: &[ModulePatches::new(
    D2Module::Client,
    0x6fab0000,
    &[
      // Game loop sleep patch
      Patch::call_c(0x8bcfc, patch_source!("
        a1 $5010b96f
        85c0
        7517
        a1 $bcbfbc6f
        83f8 06
        740d
        83f8 08
        7408
        6a0a
        ff15 $64efb76f
      "), game_loop_sleep_hook),
      // Draw paused game framerate
      Patch::call_c(0x897c5, patch_source!("ff15 $84b3ba6f"), draw_game_paused),
      // Draw game framerate & entity sync
      Patch::call_c(0x89a51, patch_source!("
        391d $2034bd6f
        7535
        a1 $f0c4bc6f
        3bc3
        7438
        50
        e8 4628f8ff
        3bc3
        742e
        33c9
        ff15 $84b3ba6f
        8b0d $94b3ba6f
        a1 $acb3ba6f
        41
        40
        890d $94b3ba6f
        a3 $acb3ba6f
        eb0c
        395c2410
        7406
        ff05 $b4b3ba6f
      "), draw_game::<Entity>),
    ],
  )],
  game_smoothing: &[],
};

global_asm! {
  ".global _update_menu_char_frame_111_asm_stub",
  "_update_menu_char_frame_111_asm_stub:",
  "mov ecx, [ebx+0x10]",
  "lea edx, [ebx+0x08]",
  "call {}",
  "mov ecx, [ebx+0x0c]",
  "mov esi, eax",
  "ret",
  sym update_menu_char_frame,
}
extern "C" {
  pub fn update_menu_char_frame_111_asm_stub();
}
