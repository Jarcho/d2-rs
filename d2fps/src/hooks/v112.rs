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
  v112::{Entity, ADDRESSES, BASE_ADDRESSES},
};

#[rustfmt::skip]
pub(super) const HOOKS: Hooks = Hooks {
  version: "1.12",
  addresses: ADDRESSES,
  base_addresses: BASE_ADDRESSES,
  load_modules: d2::Modules::load_split_modules,
  patches: FeaturePatches::new(
    &[ModulePatches::new(
      d2::Module::Win,
      &[
        // Draw menu framerate
        Patch::call_c(0xd92c, patch_source!("
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
        Patch::call_c(0x16386, patch_source!("
          8b4310
          8b7308
          8b4b0c
          03f0
          8bc6
        "), super::v111::update_menu_char_frame_111_asm_stub),
        // Menu sleep patch
        Patch::nop(0xd97e, patch_source!("
          8bc7
          7605
          b814000000
          8b0d $ecfa8f6f
          85c9
          7402
          33c0
          50
          ff15 $a8a28f6f
        ")),
      ],
    )],
    &[ModulePatches::new(
      d2::Module::Client,
      &[
        // Game loop sleep patch
        Patch::call_c(0x6cfbc, patch_source!("
          a1 $803ab96f
          85c0
          7517
          a1 $f8bfbc6f
          83f806
          740d
          83f808
          7408
          xxxx
          xxxxxxxxxxxx
        "), game_loop_sleep_hook),
        // Draw paused game framerate
        Patch::call_c(0x7cf55, patch_source!("ff15 $9c32bb6f"), draw_game_paused),
        // Draw game framerate & entity sync
        Patch::call_c(0x7d1e1, patch_source!("
          xxxxxxxxxxxx
          xxxx
          a1 $d0c3bc6f
          3bc3
          7438
          50
          e8c2eff8ff
          3bc3
          742e
          33c9
          ff15 $9c32bb6f
          8b0d $ac32bb6f
          a1 $c432bb6f
          41
          40
          890d $ac32bb6f
          a3 $c432bb6f
          eb0c
          395c2410
          7406
          ff05 $cc32bb6f
        "), draw_game::<Entity>),
        // Draw cursor framerate
        Patch::call_c(0x9f329, patch_source!("
          6bc01c
          8b88 $b8d7b86f
          85c9
        "), super::v111::should_update_cursor_111_asm_stub),
      ],
    )],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          // Animated entity mouse detection refinement
          Patch::call_std1(0x1ff1e, patch_source!("e8e5c2feff"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x1ff43, patch_source!("e8ccc2feff"), entity_iso_ypos::<Entity>),
          // Course entity mouse detection
          Patch::call_std1(0x20184, patch_source!("e87fc0feff"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x2018c, patch_source!("e883c0feff"), entity_iso_ypos::<Entity>),
          // Npc mouse over perspective
          Patch::call_std1(0xb7d84, patch_source!("e87944f5ff"), entity_linear_xpos::<Entity>),
          Patch::call_std1(0xb7d7d, patch_source!("e8b044f5ff"), entity_linear_ypos::<Entity>),
          // Npc mouse over
          Patch::call_std1(0xb7ddf, patch_source!("e82444f5ff"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0xb7df4, patch_source!("e81b44f5ff"), entity_iso_ypos::<Entity>),
        ],
      ),
      ModulePatches::new(
        d2::Module::Common,
        &[Patch::call_c(0x4b467, patch_source!("e894faffff"), super::v111::intercept_teleport_111_asm_stub)],
      ),
    ],
  )
};
