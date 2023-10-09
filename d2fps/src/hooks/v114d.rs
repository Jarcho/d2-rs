use crate::{
  features::{FeaturePatches, ModulePatches},
  hooks::{
    draw_game, draw_game_paused, draw_menu, entity_iso_xpos, entity_iso_ypos, entity_linear_xpos,
    entity_linear_ypos, game_loop_sleep_hook, Hooks,
  },
};
use bin_patch::{patch_source, Patch};
use core::arch::global_asm;
use d2interface::{
  self as d2,
  v114d::{Entity, ADDRESSES, BASE_ADDRESSES},
};

#[rustfmt::skip]
pub(super) const HOOKS: Hooks = Hooks {
  addresses: ADDRESSES,
  base_addresses: BASE_ADDRESSES,
  load_modules: d2::Modules::load_combined_module,
  patches: FeaturePatches::new(
    &[ModulePatches::new(
      d2::Module::GameExe,
      &[
        // Draw menu framerate
        Patch::call_c(0xfa606, patch_source!("
          ff15 $60c26c00
          8bf0
          2bf3
          ff15 $60c26c00
          81fee8030000
          8bd8
          7605
          bee8030000
          2bfe
          85ff
          7f25
          xxxxxx
          81ff18fcffff
          7d02
          33ff
          8b4508
          85c0
          740c
          8b75fc
          56
          ffd0
          83c601
          8975fc
          e890f2ffff
        "), draw_menu_114d_asm_stub),
        // Menu char frame rate
        Patch::call_c(0x103ddd, patch_source!("
          8b4710
          014708
          8b4708
        "), super::v114a::update_menu_char_frame_114a_asm_stub),
        // Menu sleep patch
        Patch::nop(0xfa65f, patch_source!("
          8bc7
          76xx
          b814000000
          833d $44dc7200 00
          xx02
          33c0
          50
          ff15 $58c26c00
        ")),
      ],
    )],
    &[ModulePatches::new(
      d2::Module::GameExe,
      &[
        // Game loop sleep patch
        Patch::call_c(0x51c2a, patch_source!("
          833d $e0f77000 00
          7517
          a1 $10067a00
          83f806
          740d
          83f808
          7408
          xxxx
          xxxxxxxxxxxx
        "), game_loop_sleep_hook),
        // Draw paused game framerate
        Patch::call_c(0x4f017, patch_source!("ff15 $84047a00"), draw_game_paused),
        // Draw game framerate & entity sync
        Patch::call_c(0x4f278, patch_source!("
          xxxxxxxxxxxx
          xxxx
          e81b540100
          85c0
          7427
          33c9
          ff15 $84047a00
          b801000000
          0105 $94047a00
          0105 $ac047a00
          eb0c
          395dfc
          7407
          8305 $b4047a00 01
        "), draw_game::<Entity>),
        // Cursor animation speed
        Patch::raw(0x6836d, patch_source!("10"), &[0x28]),
        // Summit cloud move speed
        Patch::call_c(0x768cd, patch_source!("
          0197 $e85b7b00
        "), super::v114a::move_summit_cloud_114a_asm_stub),
      ],
    )],
    &[ModulePatches::new(
      d2::Module::GameExe,
      &[
        // Animated entity mouse detection refinement
        Patch::call_std1(0x6414a, patch_source!("e801c51b00"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x6416c, patch_source!("e83fc51b00"), entity_iso_ypos::<Entity>),
        // Course entity mouse detection
        Patch::call_std1(0x669d2, patch_source!("e8799c1b00"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x669da, patch_source!("e8d19c1b00"), entity_iso_ypos::<Entity>),
        // Npc mouse over perspective
        Patch::call_std1(0xdb6a6, patch_source!("e8054d1400"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0xdb69f, patch_source!("e86c4d1400"), entity_linear_ypos::<Entity>),
        // Npc mouse over
        Patch::call_std1(0xdb6e9, patch_source!("e8624f1400"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0xdb6fb, patch_source!("e8b04f1400"), entity_iso_ypos::<Entity>),
        // Intercept teleport call
        Patch::call_c(0x250a3c, patch_source!("
          8bc6
          e84df1ffff
        "), super::v114c::intercept_teleport_114c_asm_stub),
      ],
    )],
  )
};

global_asm! {
  ".global _draw_menu_114d_asm_stub",
  "_draw_menu_114d_asm_stub:",
  "mov ecx, [ebp+0x8]",
  "lea edx, [ebp-0x4]",
  "call {}",
  "ret",
  sym draw_menu,
}
extern "C" {
  pub fn draw_menu_114d_asm_stub();
}
