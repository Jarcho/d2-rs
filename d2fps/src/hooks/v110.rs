use crate::{
  features::{FeaturePatches, ModulePatches},
  hooks::{
    draw_game, draw_game_paused, draw_menu, entity_iso_xpos, entity_iso_ypos, entity_linear_xpos,
    entity_linear_ypos, game_loop_sleep_hook, intercept_teleport, summit_cloud_move_amount,
    update_menu_char_frame, Hooks, UnitId,
  },
};
use bin_patch::{patch_source, Patch};
use core::arch::global_asm;
use d2interface::{
  self as d2,
  v110::{Entity, ADDRESSES, BASE_ADDRESSES},
};

#[rustfmt::skip]
pub(super) const HOOKS: Hooks = Hooks {
  version: "1.10",
  addresses: ADDRESSES,
  base_addresses: BASE_ADDRESSES,
  load_modules: d2::Modules::load_split_modules,
  patches: FeaturePatches::new(
    &[ModulePatches::new(
      d2::Module::Win,
      &[
        // Draw menu framerate
        Patch::call_c(0xd00c, patch_source!("
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
          8b542434
          85d2
          740e
          8b4c2410
          8bc1
          41
          50
          894c2414
          ffd2
          e89f060000
        "), draw_menu_110_asm_stub),
        // Menu char frame rate
        Patch::call_c(0x1abf, patch_source!("
          8b4610
          8b4e08
          03c8
          894e08
          8bc1
        "), update_menu_char_frame_110_asm_stub),
        // Menu sleep patch
        Patch::nop(0xd060, patch_source!("
          8bc7
          7605
          b814000000
          8b0d $20de8b6f
          85c9
          7402
          33c0
          50
          ff15 $c0a18b6f
        ")),
      ],
    )],
    &[ModulePatches::new(
      d2::Module::Client,
      &[
        // Game loop sleep patch
        Patch::call_c(0x266c, patch_source!("
          a1 $c047b76f
          85c0
          7517
          a1 $6079ba6f
          83f806
          740d
          83f808
          7408
          xxxx
          xxxxxxxxxxxx
        "), game_loop_sleep_hook),
        // Draw paused game framerate
        Patch::call_c(0x9b78, patch_source!("ff15 $5477ba6f"), draw_game_paused),
        // Draw game framerate & entity sync
        Patch::call_c(0xa2c4, patch_source!("
          a1 $e079ba6f
          xxxx
          xxxx
          e89ef00700
          85c0
          7430
          33c9
          ff15 $5477ba6f
          8b0d $6477ba6f
          a1 $7c77ba6f
          41
          40
          890d $6477ba6f
          a3 $7c77ba6f
          eb0e
          8b442414
          85c0
          7406
          ff05 $8477ba6f
        "), draw_game::<Entity>),
        // Draw cursor framerate
        Patch::call_c(0xb77e8, patch_source!("
          39a8 $586bb96f
        "), super::v100::should_update_cursor_100_asm_stub),
        // Summit cloud move speed
        Patch::call_c(0x17b75, patch_source!("
          03d8
          81c170010000
        "), summit_cloud_move_amount_110_asm_stub),
      ],
    )],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          // Course entity mouse detection
          Patch::call_std1(0x888dc, patch_source!("e8d5200400"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x888e4, patch_source!("e8c7200400"), entity_iso_ypos::<Entity>),
          // Animated entity mouse detection refinement
          Patch::call_std1(0x88e36, patch_source!("e87b1b0400"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x88e5d, patch_source!("e84e1b0400"), entity_iso_ypos::<Entity>),
          // Npc mouse over perspective
          Patch::call_std1(0xbf827, patch_source!("e830b10000"), entity_linear_xpos::<Entity>),
          Patch::call_std1(0xbf820, patch_source!("e831b10000"), entity_linear_ypos::<Entity>),
          // Npc mouse over
          Patch::call_std1(0xbf874, patch_source!("e83db10000"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0xbf887, patch_source!("e824b10000"), entity_iso_ypos::<Entity>),
        ],
      ),
      ModulePatches::new(
        d2::Module::Common,
        &[
          Patch::call_c(0x6d85c, patch_source!("
            8d442428
            893e
          "), intercept_teleport_110_asm_stub),
        ],
      ),
    ],
  )
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
  "mov ecx, [esi+0x30]",
  "mov edx, [ecx+0xc]",
  "mov ecx, [ecx]",
  "call {}",
  "lea eax, [esp+0x2c]",
  "ret",
  sym intercept_teleport,
}
extern "C" {
  pub fn intercept_teleport_110_asm_stub();
}

global_asm! {
  ".global _summit_cloud_move_amount_110_asm_stub",
  "_summit_cloud_move_amount_110_asm_stub:",
  "add ecx, 0x170",
  "push ecx",
  "mov ecx, eax",
  "call {}",
  "add ebx, eax",
  "pop ecx",
  "ret",
  sym summit_cloud_move_amount,
}
extern "C" {
  pub fn summit_cloud_move_amount_110_asm_stub();
}
