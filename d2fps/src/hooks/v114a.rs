use crate::hooks::{
  draw_game, draw_game_paused, draw_menu, game_loop_sleep_hook, update_menu_char_frame, D2Module,
  ModulePatches, PatchSets,
};
use bin_patch::{patch_source, Patch};
use core::arch::global_asm;
use d2interface::v114a::Entity;

#[rustfmt::skip]
pub(super) static PATCHES: PatchSets = PatchSets {
  menu_fps: &[ModulePatches::new(
    D2Module::GameExe,
    0x00400000,
    &[
      // Draw menu framerate
      Patch::call_c(0x3da6b, patch_source!("
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
        7f28
        83c728
        81ff 18fcffff
        7d02
        33ff
        8b442434
        85c0
        740e
        8b742410
        56
        ffd0
        83c601
        89742410
        e8 20f3ffff
      "), draw_menu_114a_asm_stub),
      // Menu char frame rate
      Patch::call_c(0x3f92c, patch_source!("
        8b5710
        014708
        8b4708
      "), update_menu_char_frame_114a_asm_stub),
      // Menu sleep patch
      Patch::nop(0x3dabf, patch_source!("
        8bc7
        7605
        b8 14000000
        833d $e0927000 00
        7402
        33c0
        50
        ff15 $98d16c00
      ")),
    ],
  )],
  game_fps: &[ModulePatches::new(
    D2Module::GameExe,
    0x00400000,
    &[
      // Game loop sleep patch
      Patch::call_c(0x60b11, patch_source!("
        833d $30b17000 00
        7517
        a1 $b0e18200
        83f806
        740d
        83f808
        7408
        6a0a
        ff15 $98d16c00
      "), game_loop_sleep_hook),
      // Draw paused game framerate
      Patch::call_c(0x53958, patch_source!("ff15 $24e08200"), draw_game_paused),
      // Draw game framerate & entity sync
      Patch::call_c(0x53bc1, patch_source!("
        392d $a4e28200
        751f
        e8 229b0000
        85c0
        7422
        33c9
        ff15 $24e08200
        011d $34e08200
        011d $4ce08200
        eb0c
        396c2410
        7406
        011d $54e08200
      "), draw_game::<Entity>),
    ],
  )],
  game_smoothing: &[],
};

global_asm! {
  ".global _draw_menu_114a_asm_stub",
  "_draw_menu_114a_asm_stub:",
  "mov ecx, [ebp+0x8]",
  "lea edx, [ebp-0x4]",
  "call {}",
  "ret",
  sym draw_menu,
}
extern "C" {
  pub fn draw_menu_114a_asm_stub();
}

global_asm! {
  ".global _update_menu_char_frame_114a_asm_stub",
  "_update_menu_char_frame_114a_asm_stub:",
  "mov ecx, [edi+0x10]",
  "lea edx, [edi+0x08]",
  "call {}",
  "ret",
  sym update_menu_char_frame,
}
extern "C" {
  pub fn update_menu_char_frame_114a_asm_stub();
}
