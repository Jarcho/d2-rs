use crate::{
  hooks::{
    draw_game, draw_game_paused, entity_iso_xpos, entity_iso_ypos, entity_linear_xpos,
    entity_linear_ypos, game_loop_sleep_hook, ModulePatches, PatchSets,
  },
  tracker::UnitId,
};
use bin_patch::{patch_source, Patch};
use d2interface::{self as d2, v107::Entity};

#[rustfmt::skip]
pub(super) const PATCHES: PatchSets = PatchSets {
  menu_fps: &[ModulePatches::new(
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
  game_fps: &[ModulePatches::new(
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
  game_smoothing: &[
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
