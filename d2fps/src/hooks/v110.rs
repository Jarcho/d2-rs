use crate::{
  hooks::{
    draw_game, draw_game_paused, draw_menu, entity_iso_xpos, entity_iso_ypos, entity_linear_xpos,
    entity_linear_ypos, game_loop_sleep_hook, intercept_teleport, update_menu_char_frame,
    ModulePatches, PatchSets,
  },
  tracker::UnitId,
};
use bin_patch::{patch_source, Patch};
use core::arch::global_asm;
use d2interface::{self as d2, v110::Entity};

#[rustfmt::skip]
pub(super) const PATCHES: PatchSets = PatchSets {
  menu_fps: &[ModulePatches::new(
    d2::Module::Win,
    &[
      // Draw menu framerate
      Patch::call_c(0xd00c, patch_source!("
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
        7f 28
        83 c7 28
        81 ff 18 fc ff ff
        7d 02
        33 ff
        8b 54 24 34
        85 d2
        74 0e
        8b 4c 24 10
        8b c1
        41
        50
        89 4c 24 14
        ff d2
        e8 9f 06 00 00
      "), draw_menu_110_asm_stub),
      // Menu char frame rate
      Patch::call_c(0x1abf, patch_source!("
        8b 46 10
        8b 4e 08
        03 c8
        89 4e 08
        8b c1
      "), update_menu_char_frame_110_asm_stub),
      // Menu sleep patch
      Patch::nop(0xd060, patch_source!("
        8bc7
        7605
        b8 14000000
        8b0d $20de8b6f
        85c9
        7402
        33c0
        50
        ff15 $c0a18b6f
      ")),
    ],
  )],
  game_fps: &[ModulePatches::new(
    d2::Module::Client,
    &[
      // Game loop sleep patch
      Patch::call_c(0x266c, patch_source!("
        a1 $c047b76f
        85 c0
        75 17
        a1 $6079ba6f
        83 f8 06
        74 0d
        83 f8 08
        74 08
        6a 0a
        ff 15 $1cdfb66f
      "), game_loop_sleep_hook),
      // Draw paused game framerate
      Patch::call_c(0x9b78, patch_source!("ff 15 $5477ba6f"), draw_game_paused),
      // Draw game framerate & entity sync
      Patch::call_c(0xa2c4, patch_source!("
        a1 $e079ba6f
        85 c0
        75 2b
        e8 9e f0 07 00
        85 c0
        74 30
        33 c9
        ff 15 $5477ba6f
        8b 0d $6477ba6f
        a1 $7c77ba6f
        41
        40
        89 0d $6477ba6f
        a3 $7c77ba6f
        eb 0e
        8b 44 24 14
        85 c0
        74 06
        ff 05 $8477ba6f
      "), draw_game::<Entity>),
    ],
  )],
  game_smoothing: &[
    ModulePatches::new(
      d2::Module::Client,
      &[
        // Course entity mouse detection
        Patch::call_std1(0x888dc, patch_source!("e8 d5200400"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x888e4, patch_source!("e8 c7200400"), entity_iso_ypos::<Entity>),
        // Animated entity mouse detection refinement
        Patch::call_std1(0x88e36, patch_source!("e8 7b1b0400"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x88e5d, patch_source!("e8 4e1b0400"), entity_iso_ypos::<Entity>),
        // Npc mouse over perspective
        Patch::call_std1(0xbf827, patch_source!("e8 30b10000"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0xbf820, patch_source!("e8 31b10000"), entity_linear_ypos::<Entity>),
        // Npc mouse over
        Patch::call_std1(0xbf874, patch_source!("e8 3db10000"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0xbf887, patch_source!("e8 24b10000"), entity_iso_ypos::<Entity>),
      ],
    ),
    ModulePatches::new(
      d2::Module::Common,
      &[
        Patch::call_c(0x6d860, patch_source!("
          89 3e
          89 6e 04
        "), intercept_teleport_110_asm_stub),
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

  fn linear_pos(&self) -> d2::LinearPos<d2::FixedU16> {
    self
      .pos(
        |pos| {
          d2::LinearPos::new(
            d2::FixedU16::from_wrapping(pos.linear_pos.x),
            d2::FixedU16::from_wrapping(pos.linear_pos.y),
          )
        },
        |pos| pos.linear_pos,
      )
      .unwrap()
  }

  fn iso_pos(&self) -> d2::IsoPos<i32> {
    self.pos(|pos| pos.iso_pos, |pos| pos.iso_pos).unwrap()
  }

  fn set_pos(&mut self, pos: d2::LinearPos<d2::FixedU16>) {
    unsafe {
      if let Some(mut epos) = self.pos.d {
        epos.as_mut().linear_pos = pos;
        epos.as_mut().iso_pos = pos.into();
      }
    }
  }

  unsafe fn tracker_pos(&self) -> (d2::LinearPos<d2::FixedU16>, d2::LinearPos<u16>) {
    self.pos.d.map_or_else(Default::default, |pos| {
      (pos.as_ref().linear_pos, pos.as_ref().target_pos[0])
    })
  }
}

global_asm! {
  ".global _draw_menu_110_asm_stub",
  "_draw_menu_110_asm_stub:",
  "mov ecx, [esp+0x38]",
  "lea edx, [esp+0x14]",
  "call {}",
  "ret",
  sym draw_menu,
}
extern "C" {
  pub fn draw_menu_110_asm_stub();
}

global_asm! {
  ".global _update_menu_char_frame_110_asm_stub",
  "_update_menu_char_frame_110_asm_stub:",
  "mov ecx, [esi+0x10]",
  "lea edx, [esi+0x08]",
  "call {}",
  "ret",
  sym update_menu_char_frame,
}
extern "C" {
  pub fn update_menu_char_frame_110_asm_stub();
}

global_asm! {
  ".global _intercept_teleport_110_asm_stub",
  "_intercept_teleport_110_asm_stub:",
  "mov [esi], edi",
  "mov [esi+0x4], ebp",
  "push eax",
  "mov ecx, [esi+0x30]",
  "mov edx, edi",
  "push ebp",
  "call {}",
  "pop eax",
  "ret",
  sym intercept_teleport::<Entity>,
}
extern "C" {
  pub fn intercept_teleport_110_asm_stub();
}
