use crate::hooks::{
  draw_game, draw_game_paused, dypos_linear_whole_xpos, dypos_linear_whole_ypos, entity_iso_xpos,
  entity_iso_ypos, entity_linear_xpos, entity_linear_ypos, game_loop_sleep_hook,
  intercept_teleport, D2Module, ModulePatches, PatchSets,
};
use bin_patch::{patch_source, Patch};
use core::arch::global_asm;
use d2interface::v114d::{DyPos, Entity};

#[rustfmt::skip]
pub(super) static PATCHES: PatchSets = PatchSets {
  menu_fps: &[ModulePatches::new(
    D2Module::GameExe,
    0x00400000,
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
      "), super::v114a::draw_menu_114a_asm_stub),
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
    D2Module::GameExe,
    0x00400000,
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
    D2Module::GameExe,
    0x00400000,
    &[
      // Viewport position
      Patch::call_std1(0x4c9ce, patch_source!("e8 7d3c1d00"), entity_iso_xpos::<Entity>),
      Patch::call_std1(0x4c9e6, patch_source!("e8 c53c1d00"), entity_iso_ypos::<Entity>),
      // Perspective entity mouse-over text
      Patch::call_std1(0x54f7c, patch_source!("e8 2fb41c00"), entity_linear_xpos::<Entity>),
      Patch::call_std1(0x54f84, patch_source!("e8 87b41c00"), entity_linear_ypos::<Entity>),
      // Entity mouse-over text
      Patch::call_std1(0x54ff5, patch_source!("e8 56b61c00"), entity_iso_xpos::<Entity>),
      Patch::call_std1(0x5500a, patch_source!("e8 a1b61c00"), entity_iso_ypos::<Entity>),
      // Entity minimap position
      Patch::call_std1(0x5a8b5, patch_source!("e8 965d1c00"), entity_iso_xpos::<Entity>),
      Patch::call_std1(0x5a8bd, patch_source!("e8 ee5d1c00"), entity_iso_ypos::<Entity>),
      // Entity shift
      Patch::call_std1(0x5b489, patch_source!("e8 c2511c00"), entity_iso_xpos::<Entity>),
      Patch::call_std1(0x5b491, patch_source!("e8 1a521c00"), entity_iso_ypos::<Entity>),
      // Animated entity mouse detection refinement
      Patch::call_std1(0x6414a, patch_source!("e8 01c51b00"), entity_iso_xpos::<Entity>),
      Patch::call_std1(0x6416c, patch_source!("e8 3fc51b00"), entity_iso_ypos::<Entity>),
      // Course entity mouse detection
      Patch::call_std1(0x669d2, patch_source!("e8 799c1b00"), entity_iso_xpos::<Entity>),
      Patch::call_std1(0x669da, patch_source!("e8 d19c1b00"), entity_iso_ypos::<Entity>),
      // Entity spell overlay perspective
      Patch::call_std1(0x6e379, patch_source!("e8 32201b00"), entity_linear_xpos::<Entity>),
      Patch::call_std1(0x6e372, patch_source!("e8 99201b00"), entity_linear_ypos::<Entity>),
      // Entity spell overlay
      Patch::call_std1(0x6e3d4, patch_source!("e8 77221b00"), entity_iso_xpos::<Entity>),
      Patch::call_std1(0x6e3e6, patch_source!("e8 c5221b00"), entity_iso_ypos::<Entity>),
      // Perspective entity shadow pos
      Patch::call_std1(0x716cd, patch_source!("e8 deec1a00"), entity_linear_xpos::<Entity>),
      Patch::call_std1(0x716d7, patch_source!("e8 34ed1a00"), entity_linear_ypos::<Entity>),
      // Entity shadow pos
      Patch::call_std1(0x71737, patch_source!("e8 14ef1a00"), entity_iso_xpos::<Entity>),
      Patch::call_std1(0x71742, patch_source!("e8 69ef1a00"), entity_iso_ypos::<Entity>),
      // Perspective entity single color pos
      Patch::call_std1(0x71c18, patch_source!("e8 93e71a00"), entity_linear_xpos::<Entity>),
      Patch::call_std1(0x71c11, patch_source!("e8 fae71a00"), entity_linear_ypos::<Entity>),
      // Entity single color shadow pos
      Patch::call_std1(0x71c6a, patch_source!("e8 e1e91a00"), entity_iso_xpos::<Entity>),
      Patch::call_std1(0x71c84, patch_source!("e8 27ea1a00"), entity_iso_ypos::<Entity>),
      // Create entity light
      Patch::call_std1(0x741d9, patch_source!("e8 d2c11a00"), entity_linear_xpos::<Entity>),
      Patch::call_std1(0x741e8, patch_source!("e8 23c21a00"), entity_linear_ypos::<Entity>),
      // Apply entity light
      Patch::call_std1(0x755d1, patch_source!("e8 daad1a00"), entity_linear_xpos::<Entity>),
      Patch::call_std1(0x755d9, patch_source!("e8 32ae1a00"), entity_linear_ypos::<Entity>),
      // Lighting position
      Patch::call_std1(0x75822, patch_source!("e8 99301d00"), dypos_linear_whole_xpos::<DyPos>),
      Patch::call_std1(0x7584e, patch_source!("e8 ad301d00"), dypos_linear_whole_ypos::<DyPos>),
      // Summit background position
      Patch::call_std1(0x764b0, patch_source!("e8 9ba11a00"), entity_iso_xpos::<Entity>),
      // Perspective viewport position
      Patch::call_std1(0x76bfb, patch_source!("e8 b0971a00"), entity_linear_xpos::<Entity>),
      Patch::call_std1(0x76c03, patch_source!("e8 08981a00"), entity_linear_ypos::<Entity>),
      // Perspective whirlwind overlay pos
      Patch::call_std1(0xc90be, patch_source!("e8 ed721500"), entity_linear_xpos::<Entity>),
      Patch::call_std1(0xc90b7, patch_source!("e8 54731500"), entity_linear_ypos::<Entity>),
      // Whirlwind overlay pos
      Patch::call_std1(0xc90dc, patch_source!("e8 6f751500"), entity_iso_xpos::<Entity>),
      Patch::call_std1(0xc90e5, patch_source!("e8 c6751500"), entity_iso_ypos::<Entity>),
      // Charge overlay pos
      Patch::call_std1(0xc99b2, patch_source!("e8 996c1500"), entity_iso_xpos::<Entity>),
      Patch::call_std1(0xc99bb, patch_source!("e8 f06c1500"), entity_iso_ypos::<Entity>),
      // Perspective charge overlay pos
      Patch::call_std1(0xc9ab8, patch_source!("e8 f3681500"), entity_linear_xpos::<Entity>),
      Patch::call_std1(0xc9ab1, patch_source!("e8 5a691500"), entity_linear_ypos::<Entity>),
      // Npc mouse over perspective
      Patch::call_std1(0xdb6a6, patch_source!("e8 054d1400"), entity_linear_xpos::<Entity>),
      Patch::call_std1(0xdb69f, patch_source!("e8 6c4d1400"), entity_linear_ypos::<Entity>),
      // Npc mouse over
      Patch::call_std1(0xdb6e9, patch_source!("e8 624f1400"), entity_iso_xpos::<Entity>),
      Patch::call_std1(0xdb6fb, patch_source!("e8 b04f1400"), entity_iso_ypos::<Entity>),
      // Perspective entity draw pos
      Patch::call_std1(0xdc83e, patch_source!("e8 6d3b1400"), entity_linear_xpos::<Entity>),
      Patch::call_std1(0xdc849, patch_source!("e8 c23b1400"), entity_linear_ypos::<Entity>),
      // Entity draw pos
      Patch::call_std1(0xdc8df, patch_source!("e8 6c3d1400"), entity_iso_xpos::<Entity>),
      Patch::call_std1(0xdc8e8, patch_source!("e8 c33d1400"), entity_iso_ypos::<Entity>),
      // Entity culling
      Patch::call_std1(0xdda42, patch_source!("e8 092c1400"), entity_iso_xpos::<Entity>),
      Patch::call_std1(0xdda4b, patch_source!("e8 602c1400"), entity_iso_ypos::<Entity>),
      // Intercept teleport call
      Patch::call_c(0x250a3e, patch_source!("e8 4df1ffff"), intercept_teleport_114d_asm_stub),
    ],
  )],
};

global_asm! {
  ".global _intercept_teleport_114d_asm_stub",
  "_intercept_teleport_114d_asm_stub:",
  "push eax",
  "mov ecx, [eax+0x30]",
  "mov edx, [esp+0xc]",
  "push edx",
  "mov edx, [esp+0xc]",
  "call {}",
  "mov ecx, eax",
  "pop eax",
  "jmp ecx",
  sym intercept_teleport::<Entity>,
}
extern "C" {
  pub fn intercept_teleport_114d_asm_stub();
}
