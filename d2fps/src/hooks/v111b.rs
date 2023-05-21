use crate::hooks::{draw_game, draw_game_paused, game_loop_sleep_hook, ModulePatches, PatchSets};
use bin_patch::{patch_source, Patch};
use d2interface::{self as d2, v111b::Entity};

#[rustfmt::skip]
pub(super) const PATCHES: PatchSets = PatchSets {
  menu_fps: &[ModulePatches::new(
    d2::Module::Win,
    &[
      // Draw menu framerate
      Patch::call_c(0x13f3c, patch_source!("
        ffd5
        8bf0
        2bf3
        ffd5
        81fee8030000
        8bd8
        7605
        bee8030000
        2bfe
        85ff
        7f26
        83c728
        81ff18fcffff
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
      Patch::call_c(0xbd36, patch_source!("
        8b4310
        8b7308
        8b4b0c
        03f0
        8bc6
      "), super::v111::update_menu_char_frame_111_asm_stub),
      // Menu sleep patch
      Patch::nop(0x13f8e, patch_source!("
        8bc7
        7605
        b8 14000000
        8b0d $bceb8f6f
        85c9
        7402
        33c0
        50
        ff15 $a8a28f6f
      ")),
    ],
  )],
  game_fps: &[ModulePatches::new(
    d2::Module::Client,
    &[
      // Game loop sleep patch
      Patch::call_c(0x5d48c, patch_source!("
        a1 $5cf2b96f
        85c0
        7517
        a1 $acc2bc6f
        83f8 06
        740d
        83f8 08
        7408
        6a0a
        ff15 $8cefb76f
      "), game_loop_sleep_hook),
      // Draw paused game framerate
      Patch::call_c(0x33305, patch_source!("ff15 $84a2bc6f"), draw_game_paused),
      // Draw game framerate & entity sync
      Patch::call_c(0x33591, patch_source!("
        391d $8036bd6f
        7535
        a1 $e0c1bc6f
        3bc3
        7438
        50
        e8 148bfdff
        3bc3
        742e
        33c9
        ff15 $84a2bc6f
        8b0d $94a2bc6f
        a1 $aca2bc6f
        41
        40
        890d $94a2bc6f
        a3 $aca2bc6f
        eb0c
        395c2410
        7406
        ff05 $b4a2bc6f
      "), draw_game::<Entity>),
    ],
  )],
  game_smoothing: &[],
};
