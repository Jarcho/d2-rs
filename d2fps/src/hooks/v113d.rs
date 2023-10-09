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
  v113d::{Entity, ADDRESSES, BASE_ADDRESSES},
};

#[rustfmt::skip]
pub(super) const HOOKS: Hooks = Hooks {
  version: "1.13d",
  addresses: ADDRESSES,
  base_addresses: BASE_ADDRESSES,
  load_modules: d2::Modules::load_split_modules,
  patches: FeaturePatches::new(
    &[ModulePatches::new(
      d2::Module::Win,
      &[
        // Draw menu framerate
        Patch::call_c(0xed4c, patch_source!("
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
        "), super::v110::draw_menu_110_asm_stub),
        // Menu char frame rate
        Patch::call_c(0xc226, patch_source!("
          8b4310
          8b7308
          8b4b0c
          03f0
          8bc6
        "), super::v111::update_menu_char_frame_111_asm_stub),
        // Menu sleep patch
        Patch::nop(0xed9e, patch_source!("
          8bc7
          76xx
          b814000000
          8b0d $c0fb8f6f
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
        Patch::call_c(0x2770c, patch_source!("
          a1 $007bba6f
          85c0
          7517
          a1 $dcd1bc6f
          83f806
          740d
          83f808
          7408
          xxxx
          xxxxxxxxxxxx
        "), game_loop_sleep_hook),
        // Draw paused game framerate
        Patch::call_c(0x45c15, patch_source!("ff15 $4487bb6f"), draw_game_paused),
        // Draw game framerate & entity sync
        Patch::call_c(0x45ea1, patch_source!("
          xxxxxxxxxxxx
          xxxx
          a1 $50d0bc6f
          3bc3
          7438
          50
          e8f063fcff
          3bc3
          742e
          33c9
          ff15 $4487bb6f
          8b0d $5487bb6f
          a1 $6c87bb6f
          41
          40
          890d $5487bb6f
          a3 $6c87bb6f
          eb0c
          395c2410
          7406
          ff05 $7487bb6f
        "), draw_game::<Entity>),
        // Draw cursor framerate
        Patch::call_c(0x14c89, patch_source!("
          6bc01c
          8b88 $309aba6f
          85c9
        "), super::v111::should_update_cursor_111_asm_stub),
        // Summit cloud move speed
        Patch::call_c(0xb5a6c, patch_source!("
          03da
          8bc3
          3bc7
        "), super::v111::summit_cloud_move_amount_111_asm_stub),
      ],
    )],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          // Animated entity mouse detection refinement
          Patch::call_std1(0x62e2e, patch_source!("e88395faff"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x62e53, patch_source!("e87c95faff"), entity_iso_ypos::<Entity>),
          // Course entity mouse detection
          Patch::call_std1(0x632f4, patch_source!("e8bd90faff"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x632fc, patch_source!("e8d390faff"), entity_iso_ypos::<Entity>),
          // Npc mouse over perspective
          Patch::call_std1(0xad334, patch_source!("e865f0f5ff"), entity_linear_xpos::<Entity>),
          Patch::call_std1(0xad32d, patch_source!("e896f0f5ff"), entity_linear_ypos::<Entity>),
          // Npc mouse over
          Patch::call_std1(0xad38f, patch_source!("e822f0f5ff"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0xad3a4, patch_source!("e82bf0f5ff"), entity_iso_ypos::<Entity>),
        ],
      ),
      ModulePatches::new(
        d2::Module::Common,
        &[Patch::call_c(0x6f367, patch_source!("e894faffff"), super::v111::intercept_teleport_111_asm_stub)],
      ),
    ],
  )
};
