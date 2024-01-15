use crate::{
  features::{FeaturePatches, ModulePatches},
  hooks::{
    draw_arcane_bg, draw_game, draw_game_paused, entity_iso_xpos, entity_iso_ypos,
    entity_linear_xpos, entity_linear_ypos, game_loop_sleep_hook, HelperFns, Hooks,
  },
};
use bin_patch::{patch_source, Patch};
use d2interface::{
  self as d2,
  v111b::{Entity, ADDRESSES, BASE_ADDRESSES},
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
        Patch::call_c(0x13f3c, patch_source!("
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
          83c728
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
        Patch::call_c(0xbd36, patch_source!("
          8b4310
          8b7308
          8b4b0c
          03f0
          8bc6
        "), super::v111a::update_menu_char_frame_111_asm_stub),
        // Menu sleep patch
        Patch::nop(0x13f8e, patch_source!("
          8bc7
          7605
          b814000000
          8b0d $bceb8f6f
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
        Patch::call_c(0x5d48c, patch_source!("
          a1 $5cf2b96f
          85c0
          7517
          a1 $acc2bc6f
          83f806
          740d
          83f808
          7408
          xxxx
          xxxxxxxxxxxx
        "), game_loop_sleep_hook),
        // Draw paused game framerate
        Patch::call_c(0x33305, patch_source!("ff15 $84a2bc6f"), draw_game_paused),
        // Draw game framerate & entity sync
        Patch::call_c_at(0x33591, patch_source!("
          xxxxxxxxxxxx
          xxxx
          a1 $e0c1bc6f
          3bc3
          7438
          50
          e8148bfdff
          3bc3
          742e
          33c9
          ff15 $84a2bc6f
          8b0d $94a2bc6f
          a1 $aca2bc6f
          41
          40
          890d $94a2bc6f
          a3 $aca2bc6f
          eb0c
          395c2410
          7406
          ff05 $b4a2bc6f
        "), draw_game::<Entity>, 8),
      ],
    )],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          // Course entity mouse detection
          Patch::call_std1(0x4d0e4, patch_source!("e81df1fbff"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x4d0ec, patch_source!("e839f1fbff"), entity_iso_ypos::<Entity>),
          // Animated entity mouse detection refinement
          Patch::call_std1(0x4ce1e, patch_source!("e8e3f3fbff"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x4ce43, patch_source!("e8e2f3fbff"), entity_iso_ypos::<Entity>),
          // Npc mouse over perspective
          Patch::call_std1(0xb9f04, patch_source!("e8c122f5ff"), entity_linear_xpos::<Entity>),
          Patch::call_std1(0xb9efd, patch_source!("e87c23f5ff"), entity_linear_ypos::<Entity>),
          // Npc mouse over
          Patch::call_std1(0xb9f5f, patch_source!("e8a222f5ff"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0xb9f74, patch_source!("e8b122f5ff"), entity_iso_ypos::<Entity>),
        ],
      ),
      ModulePatches::new(
        d2::Module::Common,
        &[
          Patch::call_c(0x7c777, patch_source!("e884f9ffff"), super::v111a::intercept_teleport_111_asm_stub),
        ],
      ),
    ],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          Patch::call_c(0x93511, patch_source!("e81afcffff"), draw_arcane_bg),
        ],
      ),
    ],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          // Draw cursor framerate
          Patch::call_c(0x28719, patch_source!("
            6bc01c
            8b88 $304aba6f
            85c9
          "), super::v111a::should_update_cursor_111_asm_stub),
          // Summit cloud move speed
          Patch::call_c(0x92dcc, patch_source!("
            03da
            8bc3
            3bc7
          "), super::v111a::summit_cloud_move_amount_111_asm_stub),
        ],
      ),
    ],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          Patch::nop(0x13ff2, patch_source!("
            53
            e8d8f5ffff
          ")),
        ]
      )
    ],
  ),
  helper_fns: HelperFns {
    gen_weather_particle: super::v111a::gen_weather_particle_111_trampoline,
  },
};
