use crate::hooks::{
  draw_game, draw_game_paused, draw_menu, entity_iso_xpos, entity_iso_ypos, entity_linear_xpos,
  entity_linear_ypos, game_loop_sleep_hook, intercept_teleport, ModulePatches, PatchSets,
};
use bin_patch::{patch_source, Patch};
use core::arch::global_asm;
use d2interface::{self as d2, v114d::Entity};

#[rustfmt::skip]
pub(super) const PATCHES: PatchSets = PatchSets {
  menu_fps: &[ModulePatches::new(
    d2::Module::GameExe,
    &[
      // Draw menu framerate
      Patch::call_c(0xfa606, patch_source!("
        ff 15 $60c26c00
        8b f0
        2b f3
        ff 15 $60c26c00
        81 fe e8 03 00 00
        8b d8
        76 05
        be e8 03 00 00
        2b fe
        85 ff
        7f 25
        83 c7 28
        81 ff 18 fc ff ff
        7d 02
        33 ff
        8b 45 08
        85 c0
        74 0c
        8b 75 fc
        56
        ff d0
        83 c6 01
        89 75 fc
        e8 90 f2 ff ff
      "), draw_menu_114d_asm_stub),
      // Menu char frame rate
      Patch::call_c(0x103ddd, patch_source!("
        8b 47 10
        01 47 08
        8b 47 08
      "), super::v114a::update_menu_char_frame_114a_asm_stub),
      // Menu sleep patch
      Patch::nop(0xfa65f, patch_source!("
        8bc7
        7605
        b8 14000000
        833d $44dc7200 00
        7402
        33c0
        50
        ff15 $58c26c00
      ")),
    ],
  )],
  game_fps: &[ModulePatches::new(
    d2::Module::GameExe,
    &[
      // Game loop sleep patch
      Patch::call_c(0x51c2a, patch_source!("
        83 3d $e0f77000 00
        75 17
        a1 $10067a00
        83 f8 06
        74 0d
        83 f8 08
        74 08
        6a 0a
        ff 15 $58c26c00
      "), game_loop_sleep_hook),
      // Draw paused game framerate
      Patch::call_c(0x4f017, patch_source!("ff 15 $84047a00"), draw_game_paused),
      // Draw game framerate & entity sync
      Patch::call_c(0x4f278, patch_source!("
        39 1d $04077a00
        75 24
        e8 1b 54 01 00
        85 c0
        74 27
        33 c9
        ff 15 $84047a00
        b8 01 00 00 00
        01 05 $94047a00
        01 05 $ac047a00
        eb 0c
        39 5d fc
        74 07
        83 05 $b4047a00 01
      "), draw_game::<Entity>),
    ],
  )],
  game_smoothing: &[ModulePatches::new(
    d2::Module::GameExe,
    &[
      // Animated entity mouse detection refinement
      Patch::call_std1(0x6414a, patch_source!("e8 01c51b00"), entity_iso_xpos::<Entity>),
      Patch::call_std1(0x6416c, patch_source!("e8 3fc51b00"), entity_iso_ypos::<Entity>),
      // Course entity mouse detection
      Patch::call_std1(0x669d2, patch_source!("e8 799c1b00"), entity_iso_xpos::<Entity>),
      Patch::call_std1(0x669da, patch_source!("e8 d19c1b00"), entity_iso_ypos::<Entity>),
      // Npc mouse over perspective
      Patch::call_std1(0xdb6a6, patch_source!("e8 054d1400"), entity_linear_xpos::<Entity>),
      Patch::call_std1(0xdb69f, patch_source!("e8 6c4d1400"), entity_linear_ypos::<Entity>),
      // Npc mouse over
      Patch::call_std1(0xdb6e9, patch_source!("e8 624f1400"), entity_iso_xpos::<Entity>),
      Patch::call_std1(0xdb6fb, patch_source!("e8 b04f1400"), entity_iso_ypos::<Entity>),
      // Intercept teleport call
      Patch::call_c(0x250a3e, patch_source!("e8 4df1ffff"), intercept_teleport_114d_asm_stub),
    ],
  )],
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

global_asm! {
  ".global _intercept_teleport_114d_asm_stub",
  "_intercept_teleport_114d_asm_stub:",
  "mov edx, [esp+0x4]",
  "push eax",
  "push ecx",
  "push edx",
  "mov ecx, [eax+0x30]",
  "mov edx, [ecx+0xc]",
  "mov ecx, [ecx]",
  "call {}",
  "mov ecx, eax",
  "pop eax",
  "jmp ecx",
  sym intercept_teleport,
}
extern "C" {
  pub fn intercept_teleport_114d_asm_stub();
}
