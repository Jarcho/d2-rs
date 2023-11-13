use crate::{
  features::{FeaturePatches, ModulePatches},
  hooks::{
    draw_arcane_bg, draw_game, draw_game_paused, entity_iso_xpos, entity_iso_ypos,
    entity_linear_xpos, entity_linear_ypos, game_loop_sleep_hook, intercept_teleport, Hooks,
  },
};
use bin_patch::{patch_source, Patch};
use core::arch::global_asm;
use d2interface::{
  self as d2,
  v114c::{Entity, ADDRESSES, BASE_ADDRESSES},
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
        Patch::call_c(0xf7c8b, patch_source!("
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
          7f28
          xxxxxx
          81ff18fcffff
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
          e860f2ffff
        "), super::v110::draw_menu_110_asm_stub),
        // Menu char frame rate
        Patch::call_c(0x10172c, patch_source!("
          8b5710
          015708
          8b4708
        "), super::v114a::update_menu_char_frame_114a_asm_stub),
        // Menu sleep patch
        Patch::nop(0xf7cdf, patch_source!("
          8bc7
          76xx
          b814000000
          833d $58b27200 00
          xx02
          33c0
          50
          ff15 $2cd26c00
        ")),
      ],
    )],
    &[ModulePatches::new(
      d2::Module::GameExe,
      &[
        // Game loop sleep patch
        Patch::call_c(0x4d23f, patch_source!("
          833d $ece37000 00
          7517
          a1 $98767900
          83f806
          740d
          83f808
          7408
          xxxx
          xxxxxxxxxxxx
        "), game_loop_sleep_hook),
        // Draw paused game framerate
        Patch::call_c(0x4a746, patch_source!("ff15 $0c757900"), draw_game_paused),
        // Draw game framerate & entity sync
        Patch::call_c(0x4a9af, patch_source!("
          392d $8c777900
          751f
          e8e4540100
          85c0
          7422
          33c9
          ff15 $0c757900
          011d $1c757900
          011d $34757900
          eb0c
          396c2410
          7406
          011d $3c757900
        "), draw_game::<Entity>),
        // Cursor animation speed
        Patch::raw(0x63b7d, patch_source!("10"), &[0x28]),
        // Summit cloud move speed
        Patch::call_c(0x7260c, patch_source!("
          0197 $70cc7a00
        "), super::v114a::move_summit_cloud_114a_asm_stub),
      ],
    )],
    &[ModulePatches::new(
      d2::Module::GameExe,
      &[
        // Animated entity mouse detection refinement
        Patch::call_std1(0x5f956, patch_source!("e8550b1c00"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x5f977, patch_source!("e8840b1c00"), entity_iso_ypos::<Entity>),
        // Course entity mouse detection
        Patch::call_std1(0x621de, patch_source!("e8cde21b00"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x621e6, patch_source!("e815e31b00"), entity_iso_ypos::<Entity>),
        // Npc mouse over perspective
        Patch::call_std1(0xd83c7, patch_source!("e8847e1400"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0xd83c0, patch_source!("e8db7e1400"), entity_linear_ypos::<Entity>),
        // Npc mouse over
        Patch::call_std1(0xd840c, patch_source!("e89f801400"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0xd841f, patch_source!("e8dc801400"), entity_iso_ypos::<Entity>),
        // Intercept teleport call
        Patch::call_c(0x251ac2, patch_source!("
          8bc6
          e8f7f0ffff
        "), intercept_teleport_114c_asm_stub),
      ],
    )],
    &[ModulePatches::new(
      d2::Module::GameExe,
      &[
        Patch::call_c(0x729c5, patch_source!("e806f6ffff"), draw_arcane_bg),
      ],
    )],
  ),
};

global_asm! {
  ".global _intercept_teleport_114c_asm_stub",
  "_intercept_teleport_114c_asm_stub:",
  "mov ecx, [esi+0x30]",
  "mov edx, [ecx+0xc]",
  "mov ecx, [ecx]",
  "call {}",
  "mov ecx, eax",
  "mov eax, esi",
  "jmp ecx",
  sym intercept_teleport,
}
extern "C" {
  pub fn intercept_teleport_114c_asm_stub();
}
