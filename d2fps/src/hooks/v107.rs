use crate::{
  features::{FeaturePatches, ModulePatches},
  hooks::{
    draw_game, draw_game_paused, entity_iso_xpos, entity_iso_ypos, entity_linear_xpos,
    entity_linear_ypos, game_loop_sleep_hook, summit_cloud_move_amount, Hooks, UnitId,
  },
};
use bin_patch::{patch_source, Patch};
use core::arch::global_asm;
use d2interface::{
  self as d2,
  v107::{Entity, ADDRESSES, BASE_ADDRESSES},
};

#[rustfmt::skip]
pub(super) const HOOKS: Hooks = Hooks {
  addresses: ADDRESSES,
  base_addresses: BASE_ADDRESSES,
  load_modules: d2::Modules::load_split_modules,
  patches: FeaturePatches::new(
    &[ModulePatches::new(
      d2::Module::Win,
      &[
        // Draw menu framerate
        Patch::call_c(0xe944, patch_source!("
          8d4c2414
          51
          ff15 $c8d1916f
          8d54241c
          52
          ff15 $80d1916f
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
          e83b610000
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
          e84e060000
        "), super::v100::draw_menu_100_asm_stub),
        // Menu char frame rate
        Patch::call_c(0x1b6a, patch_source!("
          8b4e10
          8b4608
          8b560c
          03c1
          894608
        "), super::v100::update_menu_char_frame_100_asm_stub),
      ],
    )],
    &[ModulePatches::new(
      d2::Module::Client,
      &[
        // Game loop sleep patch
        Patch::call_c(0x256c, patch_source!("
          a1 $50e7ba6f
          85c0
          7508
          6a00
          ff15 $a060ba6f
        "), game_loop_sleep_hook),
        // Draw paused game framerate
        Patch::call_c(0x9388, patch_source!("ff15 $5490be6f"), draw_game_paused),
        // Draw game framerate & entity sync
        Patch::call_c(0x9aaf, patch_source!("
          392d $e092be6f
          752b
          e8249a0800
          85c0
          742e
          33c9
          ff15 $5490be6f
          8b0d $6490be6f
          a1 $7c90be6f
          41
          40
          890d $6490be6f
          a3 $7c90be6f
          eb0c
          396c2414
          7406
          ff05 $8490be6f
        "), draw_game::<Entity>),
      ],
    )],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          // Course entity mouse detection
          Patch::call_std1(0x92ccc, patch_source!("e8ff540300"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x92cd4, patch_source!("e8f1540300"), entity_iso_ypos::<Entity>),
          // Animated entity mouse detection refinement
          Patch::call_std1(0x93176, patch_source!("e855500300"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x9319d, patch_source!("e828500300"), entity_iso_ypos::<Entity>),
          // Npc mouse over perspective
          Patch::call_std1(0xc7747, patch_source!("e80c0a0000"), entity_linear_xpos::<Entity>),
          Patch::call_std1(0xc7740, patch_source!("e80d0a0000"), entity_linear_ypos::<Entity>),
          // Npc mouse over
          Patch::call_std1(0xc7794, patch_source!("e8370a0000"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0xc77a7, patch_source!("e81e0a0000"), entity_iso_ypos::<Entity>),
        ],
      ),
      ModulePatches::new(
        d2::Module::Common,
        &[
          Patch::call_c(0x5ff67, patch_source!("
            893e
            8d442414
          "), super::v100::intercept_teleport_100_asm_stub),
        ],
      ),
    ],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          Patch::call_c(0x1681a, patch_source!("
            a1 $d021bf6f
            33f6
            3bc6
            0f854e010000
            8b0d $8c0ebb6f
            e8161b0b00
            8b0d $281cbf6f
            e8051b0b00
            8b0d $8c0ebb6f
            8bd0
            e8c0190b00
            89742414
            b893244992
            f7ee
            03d6
            c1fa02
            8bca
            c1e91f
            8dbc0a80000000
            8b0d $8c0ebb6f
            e80f190b00
            83e03f
            8d5c38e0
            81fbff000000
            7e05
            bbff000000
            8b0d $8c0ebb6f
            e8f0180b00
            83e03f
            8d6c38e0
            81fdff000000
            7e05
            bdff000000
            8b0d $8c0ebb6f
            e8d1180b00
            83e03f
            8d4438e0
            3dff000000
            7e05
            b8ff000000
            50
            55
            53
            e800310b00
            81c680000000
            8b4c2414
            8881 $9c21bf6f
            41
            81fe00040000
            894c2414
            0f8c63ffffff
            33f6
            8b15 $14c0bd6f
            8b0d $8c0ebb6f
            e80a180b00
            8b15 $10c0bd6f
            8b0d $8c0ebb6f
            8904b5 $e8e6be6f
            e8f2170b00
            8b0d $8c0ebb6f
            ba05000000
            8904b5 $e8e2be6f
            e8db170b00
            8b0d $8c0ebb6f
            83caff
            2bd0
            8914b5 $741dbf6f
            ba08000000
            e8bf170b00
            25ff000000
            46
            81fe00010000
            8a80 $9c21bf6f
            8886 $6f1cbf6f
            7c8a
            8b6c2410
            8b5c2418
            c705 $d021bf6f 01000000
            33f6
            8a96 $701cbf6f
            8b04b5 $e8e2be6f
            8b0cb5 $e8e6be6f
            68ff000000
            52
            50
            51
            50
            51
            e8e4280b00
            46
            81fe00010000
            7cd4
            ff15 $9860ba6f
            8b15 $601cbf6f
            8bf8
            2bc2
            83f828
            7636
            33f6
            8b0cb5 $741dbf6f
            8b04b5 $e8e6be6f
            03c1
            8904b5 $e8e6be6f
            790c
            ba01000000
            8bce
            e8cf000000
            46
            81fe00010000
            7cd2
            893d $601cbf6f
          "), super::v100::draw_arcane_bg_100_asm_stub),
        ],
      ),
    ],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          // Draw cursor framerate
          Patch::call_c(0xbf308, patch_source!("
            39a8 $6081bd6f
          "), super::v100::should_update_cursor_100_asm_stub),
          // Summit cloud move speed
          Patch::call_c(0x16f86, patch_source!("
            03e9
            81c270010000
          "), summit_cloud_move_amount_107_asm_stub),
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
  ".global _summit_cloud_move_amount_107_asm_stub",
  "_summit_cloud_move_amount_107_asm_stub:",
  "add edx, 0x170",
  "push edx",
  "call {}",
  "add ebp, eax",
  "pop edx",
  "ret",
  sym summit_cloud_move_amount,
}
extern "C" {
  pub fn summit_cloud_move_amount_107_asm_stub();
}
