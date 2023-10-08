use crate::{
  features::{FeaturePatches, ModulePatches},
  hooks::{
    draw_game, draw_game_paused, entity_iso_xpos, entity_iso_ypos, entity_linear_xpos,
    entity_linear_ypos, game_loop_sleep_hook, Hooks,
  },
};
use bin_patch::{patch_source, Patch};
use d2interface::{
  self as d2,
  v113c::{Entity, ADDRESSES, BASE_ADDRESSES},
};

#[rustfmt::skip]
pub(super) const HOOKS: Hooks = Hooks {
  version: "1.13c",
  addresses: ADDRESSES,
  base_addresses: BASE_ADDRESSES,
  load_modules: d2::Modules::load_split_modules,
  patches: FeaturePatches::new(
    &[ModulePatches::new(
      d2::Module::Win,
      &[
        // Draw menu framerate
        Patch::call_c(0x189fc, patch_source!("
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
          7f26
          xxxxxx
          81ff18fcffff
          7d02
          33ff
          8b442434
          85c0
          740c
          8b742410
          56
          ffd0
          46
          89742410
          e8a1fdffff
        "), crate::hooks::v110::draw_menu_110_asm_stub),
        // Menu char frame rate
        Patch::call_c(0xe836, patch_source!("
          8b4310
          8b7308
          8b4b0c
          03f0
          8bc6
        "), crate::hooks::v111::update_menu_char_frame_111_asm_stub),
        // Menu sleep patch
        Patch::nop(0x18a4e, patch_source!("
          8bc7
          76xx
          b814000000
          8b0d $a8fb8f6f
          85c9
          xx02
          33c0
          xx
          xxxxxxxxxxxx
        ")),
      ],
    )],
    &[ModulePatches::new(
      d2::Module::Client,
      &[
        // Game loop sleep patch
        Patch::call_c(0x3cb7c, patch_source!("
          a1 $604aba6f
          85c0
          7517
          a1 $94c3bc6f
          83f806
          740d
          83f808
          xx08
          6a0a
          ff15 $a0efb76f
        "), game_loop_sleep_hook),
        // Draw paused game framerate
        Patch::call_c(0x44bc5, patch_source!("ff15 $e497bc6f"), draw_game_paused),
        // Draw game framerate & entity sync
        Patch::call_c(0x44e51, patch_source!("
          xxxxxxxxxxxx
          xxxx
          a1 $fcbbbc6f
          3bc3
          7438
          50
          e88873fcff
          3bc3
          742e
          33c9
          ff15 $e497bc6f
          8b0d $f497bc6f
          a1 $0c98bc6f
          41
          40
          890d $f497bc6f
          a3 $0c98bc6f
          eb0c
          395c2410
          7406
          ff05 $1498bc6f
        "), draw_game::<Entity>),
        // Draw cursor framerate
        Patch::call_c(0x16809, patch_source!("
          6bc01c
          8b88 $5885ba6f
          85c9
        "), super::v111::should_update_cursor_111_asm_stub),
      ],
    )],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          // Npc mouse over perspective
          Patch::call_std1(0x6e6a4, patch_source!("e805dbf9ff"), entity_linear_xpos::<Entity>),
          Patch::call_std1(0x6e69d, patch_source!("e890dbf9ff"), entity_linear_ypos::<Entity>),
          // Npc mouse over
          Patch::call_std1(0x6e6ff, patch_source!("e8cedaf9ff"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x6e714, patch_source!("e8dddaf9ff"), entity_iso_ypos::<Entity>),
          // Animated entity mouse detection refinement
          Patch::call_std1(0xa67ee, patch_source!("e8df59f6ff"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0xa6813, patch_source!("e8de59f6ff"), entity_iso_ypos::<Entity>),
          // Course entity mouse detection
          Patch::call_std1(0xa6a54, patch_source!("e87957f6ff"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0xa6a5c, patch_source!("e89557f6ff"), entity_iso_ypos::<Entity>),
        ],
      ),
      ModulePatches::new(
        d2::Module::Common,
        &[Patch::call_c(0xe0b7, patch_source!("e884f9ffff"), super::v111::intercept_teleport_111_asm_stub)],
      ),
    ],
  )
};
