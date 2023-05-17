use crate::{
  hooks::{
    draw_game, draw_game_paused, draw_menu_with_sleep, entity_iso_xpos, entity_iso_ypos,
    entity_linear_xpos, entity_linear_ypos, game_loop_sleep_hook, intercept_teleport,
    update_menu_char_frame, D2Module, ModulePatches, PatchSets,
  },
  tracker::UnitId,
};
use bin_patch::{patch_source, Patch};
use core::{arch::global_asm, ptr::NonNull};
use d2interface::{
  v109d::{DyPos, Entity},
  FixedU16, IsoPos, LinearPos,
};

use super::{entity_linear_whole_xpos, entity_linear_whole_ypos};

#[rustfmt::skip]
pub(super) static PATCHES: PatchSets = PatchSets {
  menu_fps: &[ModulePatches::new(
    D2Module::Win,
    0x6f8a0000,
    &[
      // Draw menu framerate
      Patch::call_c(0xebe4, patch_source!("
        8d 4c 24 14
        51
        ff 15 $c8d18b6f
        8d 54 24 1c
        52
        ff 15 $80d18b6f
        8b 74 24 14
        8b 7c 24 18
        8b c6
        8b cf
        2b c3
        6a 00
        1b cd
        6a 19
        51
        50
        e8 4b 61 00 00
        3b 54 24 20
        7c 27
        7f 06
        3b 44 24 1c
        76 1f
        8b 44 24 44
        8b de
        85 c0
        8b ef
        74 0e
        8b 54 24 10
        8b ca
        42
        51
        89 54 24 14
        ff d0
        e8 4e 06 00 00
      "), draw_menu_109d_asm_stub),
      // Menu char frame rate
      Patch::call_c(0x1b6a, patch_source!("
        8b 4e 10
        8b 46 08
        8b 56 0c
        03 c1
      "), update_menu_char_frame_109d_asm_stub),
    ],
  )],
  game_fps: &[ModulePatches::new(
    D2Module::Client,
    0x6faa0000,
    &[
      // Game loop sleep patch
      Patch::call_c(0x262c, patch_source!("
        a1 $3847b76f
        85 c0
        75 08
        6a 00
        ff 15 $9cbfb66f
      "), game_loop_sleep_hook),
      // Draw paused game framerate
      Patch::call_c(0x9438, patch_source!("ff 15 $b409bb6f"), draw_game_paused),
      // Draw game framerate & entity sync
      Patch::call_c(0x9b5f, patch_source!("
        39 2d $400cbb6f
        75 2b
        e8 b4 40 08 00
        85 c0
        74 2e
        33 c9
        ff 15 $b409bb6f
        8b 0d $c409bb6f
        a1 $dc09bb6f
        41
        40
        89 0d $c409bb6f
        a3 $dc09bb6f
        eb 0c
        39 6c 24 14
        74 06
        ff 05 $e409bb6f
      "), draw_game::<Entity>),
    ],
  )],
  game_smoothing: &[
    ModulePatches::new(
      D2Module::Client,
      0x6faa0000,
      &[
        // Create entity light
        Patch::call_std1(0x4305, patch_source!("e8 dea10b00"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x4314, patch_source!("e8 c9a10b00"), entity_linear_ypos::<Entity>),
        // Lighting position
        Patch::call_std1(0x4abf, patch_source!("e8 ee990b00"), entity_linear_whole_xpos::<Entity>),
        Patch::call_std1(0x4ac7, patch_source!("e8 e0990b00"), entity_linear_whole_ypos::<Entity>),
        // Apply entity light
        Patch::call_std1(0x4ba5, patch_source!("e8 3e990b00"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x4bad, patch_source!("e8 30990b00"), entity_linear_ypos::<Entity>),
        // Viewport position
        Patch::call_std1(0x8f4f, patch_source!("e8 06560b00"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x8f6c, patch_source!("e8 e3550b00"), entity_iso_ypos::<Entity>),
        // Entity shift
        Patch::call_std1(0x14c66, patch_source!("e8 ef980a00"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x14c6e, patch_source!("e8 e1980a00"), entity_iso_ypos::<Entity>),
        // Perspective viewport position
        Patch::call_std1(0x1619c, patch_source!("e8 47830a00"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x161a4, patch_source!("e8 39830a00"), entity_linear_ypos::<Entity>),
        // Summit background position
        Patch::call_std1(0x165c3, patch_source!("e8 927f0a00"), entity_iso_xpos::<Entity>),
        // Perspective whirlwind overlay pos
        Patch::call_std1(0x1e864, patch_source!("e8 7ffc0900"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x1e85d, patch_source!("e8 80fc0900"), entity_linear_ypos::<Entity>),
        // Whirlwind overlay pos
        Patch::call_std1(0x1e884, patch_source!("e8 d1fc0900"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x1e88e, patch_source!("e8 c1fc0900"), entity_iso_ypos::<Entity>),
        // Entity culling
        Patch::call_std1(0x17641, patch_source!("e8 146f0a00"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x1764b, patch_source!("e8 046f0a00"), entity_iso_ypos::<Entity>),
        // Charge overlay pos
        Patch::call_std1(0x2379e, patch_source!("e8 b7ad0900"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x237a8, patch_source!("e8 a7ad0900"), entity_iso_ypos::<Entity>),
        // Perspective charge overlay pos
        Patch::call_std1(0x23912, patch_source!("e8 d1ab0900"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x2390b, patch_source!("e8 d2ab0900"), entity_linear_ypos::<Entity>),
        // Entity minimap position
        Patch::call_std1(0x28300, patch_source!("e8 55620900"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x28308, patch_source!("e8 47620900"), entity_iso_ypos::<Entity>),
        // Perspective entity mouse-over text
        Patch::call_std1(0x85589, patch_source!("e8 5a8f0300"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x85591, patch_source!("e8 4c8f0300"), entity_linear_ypos::<Entity>),
        // Entity mouse-over text
        Patch::call_std1(0x85600, patch_source!("e8 558f0300"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x85615, patch_source!("e8 3a8f0300"), entity_iso_ypos::<Entity>),
        // Course entity mouse detection
        Patch::call_std1(0x8d40c, patch_source!("e8 49110300"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x8d414, patch_source!("e8 3b110300"), entity_iso_ypos::<Entity>),
        // Animated entity mouse detection refinement
        Patch::call_std1(0x8d8b6, patch_source!("e8 9f0c0300"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x8d8dd, patch_source!("e8 720c0300"), entity_iso_ypos::<Entity>),
        // Perspective entity draw pos
        Patch::call_std1(0xaba33, patch_source!("e8 b02a0100"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0xaba3f, patch_source!("e8 9e2a0100"), entity_linear_ypos::<Entity>),
        // Entity draw pos
        Patch::call_std1(0xabb56, patch_source!("e8 ff290100"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0xabb60, patch_source!("e8 ef290100"), entity_iso_ypos::<Entity>),
        // Perspective entity shadow pos
        Patch::call_std1(0xb7289, patch_source!("e8 5a720000"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0xb7293, patch_source!("e8 4a720000"), entity_linear_ypos::<Entity>),
        // Entity shadow pos
        Patch::call_std1(0xb72fe, patch_source!("e8 57720000"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0xb730a, patch_source!("e8 45720000"), entity_iso_ypos::<Entity>),
        // Perspective entity single color pos
        Patch::call_std1(0xb7ad6, patch_source!("e8 0d6a0000"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0xb7acf, patch_source!("e8 0e6a0000"), entity_linear_ypos::<Entity>),
        // Entity single color pos
        Patch::call_std1(0xb7b33, patch_source!("e8 226a0000"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0xb7b4e, patch_source!("e8 016a0000"), entity_iso_ypos::<Entity>),
        // Entity spell overlay perspective
        Patch::call_std1(0xb815a, patch_source!("e8 89630000"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0xb8153, patch_source!("e8 8a630000"), entity_linear_ypos::<Entity>),
        // Entity spell overlay
        Patch::call_std1(0xb81a6, patch_source!("e8 af630000"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0xb81b9, patch_source!("e8 96630000"), entity_iso_ypos::<Entity>),
        // Npc mouse over perspective
        Patch::call_std1(0xbdad7, patch_source!("e8 0c0a0000"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0xbdad0, patch_source!("e8 0d0a0000"), entity_linear_ypos::<Entity>),
        // Npc mouse over
        Patch::call_std1(0xbdb24, patch_source!("e8 310a0000"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0xbdb37, patch_source!("e8 180a0000"), entity_iso_ypos::<Entity>),
      ],
    ),
    ModulePatches::new(
      D2Module::Common,
      0x6fd40000,
      &[
        Patch::call_c(0x5f9f7, patch_source!("
          89 3e
          8d 44 24 14
          89 6e 04
        "), intercept_teleport_109d_asm_stub),
      ],
    ),
  ],
};

impl super::Entity for Entity {
  fn unit_id(&self) -> UnitId {
    UnitId::new(self.kind, self.id)
  }

  fn has_room(&self) -> bool {
    self.has_room()
  }

  fn linear_pos(&self) -> LinearPos<FixedU16> {
    self
      .pos(
        |pos| {
          LinearPos::new(
            FixedU16::from_wrapping(pos.linear_pos.x),
            FixedU16::from_wrapping(pos.linear_pos.y),
          )
        },
        |pos| pos.linear_pos,
      )
      .unwrap()
  }

  fn iso_pos(&self) -> IsoPos<i32> {
    self.pos(|pos| pos.iso_pos, |pos| pos.iso_pos).unwrap()
  }

  unsafe fn tracker_pos(&self) -> (LinearPos<FixedU16>, LinearPos<u16>) {
    self.pos.d.map_or_else(Default::default, |pos| {
      (pos.as_ref().linear_pos, pos.as_ref().target_pos[0])
    })
  }
}

impl super::DyPos for DyPos {
  type Entity = Entity;
  fn entity(&self) -> NonNull<Self::Entity> {
    self.entity
  }
  fn linear_pos(&self) -> LinearPos<FixedU16> {
    self.linear_pos
  }
}

global_asm! {
  ".global _draw_menu_109d_asm_stub",
  "_draw_menu_109d_asm_stub:",
  "mov ecx, [esp+0x48]",
  "lea edx, [esp+0x18]",
  "call {}",
  "ret",
  sym draw_menu_with_sleep,
}
extern "C" {
  pub fn draw_menu_109d_asm_stub();
}

global_asm! {
  ".global _update_menu_char_frame_109d_asm_stub",
  "_update_menu_char_frame_109d_asm_stub:",
  "mov ecx, [esi+0x10]",
  "lea edx, [esi+0x08]",
  "call {}",
  "mov edx, [esi+0x0c]",
  "ret",
  sym update_menu_char_frame,
}
extern "C" {
  pub fn update_menu_char_frame_109d_asm_stub();
}

global_asm! {
  ".global _intercept_teleport_109d_asm_stub",
  "_intercept_teleport_109d_asm_stub:",
  "mov [esi], edi",
  "mov [esi+0x4], ebp",
  "mov ecx, [esi+0x30]",
  "mov edx, edi",
  "push ebp",
  "call {}",
  "lea eax, [esp+0x18]",
  "ret",
  sym intercept_teleport::<Entity>,
}
extern "C" {
  pub fn intercept_teleport_109d_asm_stub();
}
