use crate::{
  hooks::{
    draw_game, draw_game_paused, draw_menu, dypos_linear_whole_xpos, dypos_linear_whole_ypos,
    entity_iso_xpos, entity_iso_ypos, entity_linear_xpos, entity_linear_ypos, game_loop_sleep_hook,
    intercept_teleport, update_menu_char_frame, D2Module, ModulePatches, PatchSets,
  },
  tracker::UnitId,
};
use bin_patch::{patch_source, Patch};
use core::{arch::global_asm, ptr::NonNull};
use d2interface::{
  v110::{DyPos, Entity},
  FixedU16, IsoPos, LinearPos,
};

#[rustfmt::skip]
pub(super) const PATCHES: PatchSets = PatchSets {
  menu_fps: &[ModulePatches::new(
    D2Module::Win,
    0x6f8a0000,
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
    D2Module::Client,
    0x6faa0000,
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
      D2Module::Client,
      0x6faa0000,
      &[
        // Create entity light
        Patch::call_std1(0x4300, patch_source!("e8 57660c00"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x430f, patch_source!("e8 42660c00"), entity_linear_ypos::<Entity>),
        // Lighting position
        Patch::call_std1(0x4b15, patch_source!("e8 0c5e0c00"), dypos_linear_whole_xpos::<DyPos>),
        Patch::call_std1(0x4b41, patch_source!("e8 da5d0c00"), dypos_linear_whole_ypos::<DyPos>),
        // Apply entity light
        Patch::call_std1(0x4d3c, patch_source!("e8 1b5c0c00"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x4d44, patch_source!("e8 0d5c0c00"), entity_linear_ypos::<Entity>),
        // Viewport position
        Patch::call_std1(0x967f, patch_source!("e8 32130c00"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x969c, patch_source!("e8 0f130c00"), entity_iso_ypos::<Entity>),
        // Entity shift
        Patch::call_std1(0x159a6, patch_source!("e8 0b500b00"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x159ae, patch_source!("e8 fd4f0b00"), entity_iso_ypos::<Entity>),
        // Perspective viewport position
        Patch::call_std1(0x1729f, patch_source!("e8 b8360b00"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x172a7, patch_source!("e8 aa360b00"), entity_linear_ypos::<Entity>),
        // Summit background position
        Patch::call_std1(0x17743, patch_source!("e8 6e320b00"), entity_iso_xpos::<Entity>),
        // Entity culling
        Patch::call_std1(0x1891f, patch_source!("e8 92200b00"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x18929, patch_source!("e8 82200b00"), entity_iso_ypos::<Entity>),
        // Perspective whirlwind effect pos
        Patch::call_std1(0x22453, patch_source!("e8 04850a00"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x2244c, patch_source!("e8 05850a00"), entity_linear_ypos::<Entity>),
        // Whirlwind effect pos
        Patch::call_std1(0x22473, patch_source!("e8 3e850a00"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x2247d, patch_source!("e8 2e850a00"), entity_iso_ypos::<Entity>),
        // Charge effect pos
        Patch::call_std1(0x29641, patch_source!("e8 70130a00"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x2964b, patch_source!("e8 60130a00"), entity_iso_ypos::<Entity>),
        // Perspective charge effect pos
        Patch::call_std1(0x29765, patch_source!("e8 f2110a00"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x2975e, patch_source!("e8 f3110a00"), entity_linear_ypos::<Entity>),
        // Entity minimap position
        Patch::call_std1(0x2e5b8, patch_source!("e8 f9c30900"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x2e5c0, patch_source!("e8 ebc30900"), entity_iso_ypos::<Entity>),
        // Perspective entity mouse-over text
        Patch::call_std1(0x81c62, patch_source!("e8 f58c0400"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0x81c6a, patch_source!("e8 e78c0400"), entity_linear_ypos::<Entity>),
        // Entity mouse-over text
        Patch::call_std1(0x81cd9, patch_source!("e8 d88c0400"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x81cee, patch_source!("e8 bd8c0400"), entity_iso_ypos::<Entity>),
        // Course entity mouse detection
        Patch::call_std1(0x888dc, patch_source!("e8 d5200400"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x888e4, patch_source!("e8 c7200400"), entity_iso_ypos::<Entity>),
        // Animated entity mouse detection refinement
        Patch::call_std1(0x88e36, patch_source!("e8 7b1b0400"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0x88e5d, patch_source!("e8 4e1b0400"), entity_iso_ypos::<Entity>),
        // Perspective entity draw pos
        Patch::call_std1(0xadbd5, patch_source!("e8 82cd0100"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0xadbe1, patch_source!("e8 70cd0100"), entity_linear_ypos::<Entity>),
        // Entity draw pos
        Patch::call_std1(0xadd2b, patch_source!("e8 86cc0100"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0xadd35, patch_source!("e8 76cc0100"), entity_iso_ypos::<Entity>),
        // Perspective entity shadow pos
        Patch::call_std1(0xb977c, patch_source!("e8 db110100"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0xb9786, patch_source!("e8 cb110100"), entity_linear_ypos::<Entity>),
        // Entity shadow pos
        Patch::call_std1(0xb97f9, patch_source!("e8 b8110100"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0xb9805, patch_source!("e8 a6110100"), entity_iso_ypos::<Entity>),
        // Perspective entity single color pos
        Patch::call_std1(0xb9f69, patch_source!("e8 ee090100"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0xb9f62, patch_source!("e8 ef090100"), entity_linear_ypos::<Entity>),
        // Entity single color pos
        Patch::call_std1(0xb9fc6, patch_source!("e8 eb090100"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0xb9fe1, patch_source!("e8 ca090100"), entity_iso_ypos::<Entity>),
        // Entity spell overlay perspective
        Patch::call_std1(0xba506, patch_source!("e8 51040100"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0xba4ff, patch_source!("e8 52040100"), entity_linear_ypos::<Entity>),
        // Entity spell overlay
        Patch::call_std1(0xba552, patch_source!("e8 5f040100"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0xba565, patch_source!("e8 46040100"), entity_iso_ypos::<Entity>),
        // Npc mouse over perspective
        Patch::call_std1(0xbf827, patch_source!("e8 30b10000"), entity_linear_xpos::<Entity>),
        Patch::call_std1(0xbf820, patch_source!("e8 31b10000"), entity_linear_ypos::<Entity>),
        // Npc mouse over
        Patch::call_std1(0xbf874, patch_source!("e8 3db10000"), entity_iso_xpos::<Entity>),
        Patch::call_std1(0xbf887, patch_source!("e8 24b10000"), entity_iso_ypos::<Entity>),
      ],
    ),
    ModulePatches::new(
      D2Module::Common,
      0x6fd40000,
      &[
        Patch::call_c(0x6d860, patch_source!("
          89 3e
          89 6e 04
        "), intercept_teleport_110_asm_stub),
      ],
    ),
  ],
};

impl super::DyPos for DyPos {
  type Entity = Entity;
  fn entity(&self) -> NonNull<Self::Entity> {
    self.entity
  }
  fn linear_pos(&self) -> LinearPos<FixedU16> {
    self.linear_pos
  }
}

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
