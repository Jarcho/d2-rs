use crate::{
  features::{FeaturePatches, ModulePatches},
  hooks::{
    draw_game, draw_game_paused, draw_menu, entity_iso_xpos, entity_iso_ypos, entity_linear_xpos,
    entity_linear_ypos, game_loop_sleep_hook, intercept_teleport, update_menu_char_frame, Hooks,
    UnitId,
  },
};
use bin_patch::{patch_source, Patch};
use core::arch::global_asm;
use d2interface::{
  self as d2,
  v100::{Entity, ADDRESSES, BASE_ADDRESSES},
};

#[rustfmt::skip]
pub(super) const HOOKS: Hooks = Hooks {
  version: "1.00",
  addresses: ADDRESSES,
  base_addresses: BASE_ADDRESSES,
  load_modules: d2::Modules::load_split_modules,
  patches: FeaturePatches::new(
    &[ModulePatches::new(
      d2::Module::Win,
      &[
        // Draw menu framerate
        Patch::call_c(0xea24, patch_source!("
          8d4c2414
          51
          ff15 $78780f10
          8d54241c
          52
          ff15 $c0780f10
          8b742414
          8b7c2418
          8bc6
          8bcf
          2bc3
          6a00
          1bcd
          6a19
          51
          50
          e8 0b660000
          3b542420
          7c27
          7f06
          3b44241c
          761f
          8b442444
          8bde
          85c0
          8bef
          740e
          8b542410
          8bca
          42
          51
          89542414
          ffd0
          e8 9a27ffff
        "), draw_menu_100_asm_stub),
        // Menu char frame rate
        Patch::call_c(0x26ea, patch_source!("
          8b4e10
          8b4608
          8b560c
          03c1
          894608
        "), update_menu_char_frame_100_asm_stub),
      ],
    )],
    &[ModulePatches::new(
      d2::Module::Client,
      &[
        // Game loop sleep patch
        Patch::call_c(0x75dc, patch_source!("
          a1 $f0560f10
          85c0
          7508
          6a00
          ff15 $781d1810
        "), game_loop_sleep_hook),
        // Draw paused game framerate
        Patch::call_c(0xfcf7, patch_source!("ff15 $f4eb1210"), draw_game_paused),
        // Draw game framerate & entity sync
        Patch::call_c(0x102f1, patch_source!("
          391d $60ee1210
          752b
          e8 152cffff
          85c0
          742e
          33c9
          ff15 $f4eb1210
          8b0d $04ec1210
          a1 $1cec1210
          41
          40
          890d $04ec1210
          a3 $1cec1210
          eb0c
          395c2418
          7406
          ff05 $24ec1210
        "), draw_game::<Entity>),
      ],
    )],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          // Course entity mouse detection
          Patch::call_std1(0x9cde9, patch_source!("e8 e4900300"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x9cdf1, patch_source!("e8 d6900300"), entity_iso_ypos::<Entity>),
          // Animated entity mouse detection refinement
          Patch::call_std1(0x9d3f6, patch_source!("e8 d78a0300"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x9d41d, patch_source!("e8 aa8a0300"), entity_iso_ypos::<Entity>),
          // Npc mouse over perspective
          Patch::call_std1(0xd4e3e, patch_source!("e8 f90f0000"), entity_linear_xpos::<Entity>),
          Patch::call_std1(0xd4e37, patch_source!("e8 fa0f0000"), entity_linear_ypos::<Entity>),
          // Npc mouse over
          Patch::call_std1(0xd4e6a, patch_source!("e8 63100000"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0xd4e7d, patch_source!("e8 4a100000"), entity_iso_ypos::<Entity>),
        ],
      ),
      ModulePatches::new(
        d2::Module::Common,
        &[
          Patch::call_c(0x5fc9f, patch_source!("
            893e
            8d442414
          "), intercept_teleport_100_asm_stub),
        ],
      ),
    ],
  ),

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
}

global_asm! {
  ".global _draw_menu_100_asm_stub",
  "_draw_menu_100_asm_stub:",
  "mov ecx, [esp+0x48]",
  "lea edx, [esp+0x14]",
  "call {}",
  "ret",
  sym draw_menu,
}
extern "C" {
  pub fn draw_menu_100_asm_stub();
}

global_asm! {
  ".global _update_menu_char_frame_100_asm_stub",
  "_update_menu_char_frame_100_asm_stub:",
  "mov ecx, [esi+0x10]",
  "lea edx, [esi+0x08]",
  "call {}",
  "mov edx, [esi+0x0c]",
  "ret",
  sym update_menu_char_frame,
}
extern "C" {
  pub fn update_menu_char_frame_100_asm_stub();
}

global_asm! {
  ".global _intercept_teleport_100_asm_stub",
  "_intercept_teleport_100_asm_stub:",
  "mov [esi], edi",
  "mov ecx, [esi+0x30]",
  "mov edx, [ecx+0x8]",
  "mov ecx, [ecx]",
  "call {}",
  "lea eax, [esp+0x18]",
  "ret",
  sym intercept_teleport,
}
extern "C" {
  pub fn intercept_teleport_100_asm_stub();
}
