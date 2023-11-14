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
  v105::{Entity, ADDRESSES, BASE_ADDRESSES},
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
        Patch::call_c(0xb6d4, patch_source!("
          8d4c2414
          51
          ff15 $c0919b6f
          8d54241c
          52
          ff15 $bc919b6f
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
          e8fb570000
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
          e85e060000
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
        Patch::call_c(0x219c, patch_source!("
          a1 $9026c16f
          85c0
          7508
          6a00
          ff15 $84afc06f
        "), game_loop_sleep_hook),
        // Draw paused game framerate
        Patch::call_c(0x7ae8, patch_source!("ff15 $1c2ec46f"), draw_game_paused),
        // Draw game framerate & entity sync
        Patch::call_c(0x8228, patch_source!("
          391d $a030c46f
          752b
          e8dbb40600
          85c0
          742e
          33c9
          ff15 $1c2ec46f
          8b0d $2c2ec46f
          a1 $442ec46f
          41
          40
          890d $2c2ec46f
          a3 $442ec46f
          eb0c
          395c2414
          7406
          ff05 $4c2ec46f
        "), draw_game::<Entity>),
      ],
    )],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          // Course entity mouse detection
          Patch::call_std1(0x72f2d, patch_source!("e8a6a50200"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x72f35, patch_source!("e898a50200"), entity_iso_ypos::<Entity>),
          // Animated entity mouse detection refinement
          Patch::call_std1(0x733d6, patch_source!("e8fda00200"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x733fd, patch_source!("e8d0a00200"), entity_iso_ypos::<Entity>),
          // Npc mouse over perspective
          Patch::call_std1(0x9cafe, patch_source!("e84b090000"), entity_linear_xpos::<Entity>),
          Patch::call_std1(0x9caf7, patch_source!("e84c090000"), entity_linear_ypos::<Entity>),
          // Npc mouse over
          Patch::call_std1(0x9cb2a, patch_source!("e8a9090000"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x9cb3d, patch_source!("e890090000"), entity_iso_ypos::<Entity>),
        ],
      ),
      ModulePatches::new(
        d2::Module::Common,
        &[
          Patch::call_c(0x4a5a7, patch_source!("
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
          Patch::call_c(0x13cfe, patch_source!("
            391d $d8b9c46f
            0f854c010000
            8b0d $a84bc16f
            e819990800
            8b0d $94b4c46f
            e808990800
            8b0d $a84bc16f
            8bd0
            e8e1970800
            895c2414
            33f6
            b893244992
            f7ee
            03d6
            c1fa02
            8bca
            c1e91f
            8dbc0a80000000
            8b0d $a84bc16f
            e82e970800
            83e03f
            8d5c38e0
            81fbff000000
            7e05
            bbff000000
            8b0d $a84bc16f
            e80f970800
            83e03f
            8d6c38e0
            81fdff000000
            7e05
            bdff000000
            8b0d $a84bc16f
            e8f0960800
            83e03f
            8d4438e0
            3dff000000
            7e05
            b8ff000000
            50
            55
            53
            e8b7ad0800
            81c680000000
            8b4c2414
            8881 $acb9c46f
            41
            81fe00040000
            894c2414
            0f8c63ffffff
            33f6
            8b0d $a84bc16f
            ba80020000
            e842960800
            8b0d $a84bc16f
            bae0010000
            8904b5 $807fc46f
            e82b960800
            8b0d $a84bc16f
            ba05000000
            8904b5 $807bc46f
            e814960800
            8b0d $a84bc16f
            83caff
            2bd0
            8914b5 $acb5c46f
            ba08000000
            e8f8950800
            25ff000000
            46
            81fe00010000
            8a80 $acb9c46f
            8886 $a7b4c46f
            7c8c
            8b6c2418
            c705 $d8b9c46f 01000000
            33db
            33f6
            8a96 $a8b4c46f
            8b04b5 $807bc46f
            8b0cb5 $807fc46f
            68ff000000
            52
            50
            51
            50
            51
            e8aba50800
            46
            81fe00010000
            7cd4
            ff15 $7cafc06f
            8b15 $98b4c46f
            8bf8
            2bc2
            83f828
            0f869c000000
            33f6
            8b0cb5 $acb5c46f
            8b04b5 $807fc46f
            03c1
            8904b5 $807fc46f
            796e
            8b0d $a84bc16f
            e8c1950800
            8b0d $a84bc16f
            83e007
            057f020000
            bae0010000
            8904b5 $807fc46f
            e842950800
            8b0d $a84bc16f
            ba05000000
            8904b5 $807bc46f
            e82b950800
            8b0d $a84bc16f
            83caff
            2bd0
            8914b5 $acb5c46f
            ba08000000
            e80f950800
            25ff000000
            8a80 $acb9c46f
            8886 $a8b4c46f
            46
            81fe00010000
            0f8c6cffffff
            893d $98b4c46f
          "), super::v100::draw_arcane_bg_100_asm_stub),
        ],
      ),
    ],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          // Draw cursor framerate
          Patch::call_c(0x94848, patch_source!("
            39a8 $d819c36f
          "), super::v100::should_update_cursor_100_asm_stub),
        ],
      ),
    ],
  ),
};
