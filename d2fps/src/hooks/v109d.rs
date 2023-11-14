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
  v109d::{Entity, ADDRESSES, BASE_ADDRESSES},
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
        Patch::call_c(0xebe4, patch_source!("
          8d4c2414
          51
          ff15 $c8d18b6f
          8d54241c
          52
          ff15 $80d18b6f
          8b742414
          8b7c2418
          8bc6
          8bcf
          2bc3
          6a00
          1bcd
          6axx
          51
          50
          e84b610000
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
        "), super::v100::update_menu_char_frame_100_asm_stub),
      ],
    )],
    &[ModulePatches::new(
      d2::Module::Client,
      &[
        // Game loop sleep patch
        Patch::call_c(0x262c, patch_source!("
          a1 $3847b76f
          85c0
          7508
          6a00
          ff15 $9cbfb66f
        "), game_loop_sleep_hook),
        // Draw paused game framerate
        Patch::call_c(0x9438, patch_source!("ff15 $b409bb6f"), draw_game_paused),
        // Draw game framerate & entity sync
        Patch::call_c(0x9b5f, patch_source!("
          xxxxxxxxxxxx
          xxxx
          e8b4400800
          85c0
          742e
          33c9
          ff15 $b409bb6f
          8b0d $c409bb6f
          a1 $dc09bb6f
          41
          40
          890d $c409bb6f
          a3 $dc09bb6f
          eb0c
          396c2414
          7406
          ff05 $e409bb6f
        "), draw_game::<Entity>),
      ],
    )],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          // Course entity mouse detection
          Patch::call_std1(0x8d40c, patch_source!("e849110300"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x8d414, patch_source!("e83b110300"), entity_iso_ypos::<Entity>),
          // Animated entity mouse detection refinement
          Patch::call_std1(0x8d8b6, patch_source!("e89f0c0300"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x8d8dd, patch_source!("e8720c0300"), entity_iso_ypos::<Entity>),
          // Npc mouse over perspective
          Patch::call_std1(0xbdad7, patch_source!("e80c0a0000"), entity_linear_xpos::<Entity>),
          Patch::call_std1(0xbdad0, patch_source!("e80d0a0000"), entity_linear_ypos::<Entity>),
          // Npc mouse over
          Patch::call_std1(0xbdb24, patch_source!("e8310a0000"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0xbdb37, patch_source!("e8180a0000"), entity_iso_ypos::<Entity>),
        ],
      ),
      ModulePatches::new(
        d2::Module::Common,
        &[
          Patch::call_c(0x5f9f7, patch_source!("
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
          Patch::call_c(0x1623a, patch_source!("
            a1 $309bbb6f
            33f6
            3bc6
            0f854e010000
            8b0d $306eb76f
            e88c840a00
            8b0d $8c95bb6f
            e87b840a00
            8b0d $306eb76f
            8bd0
            e82a830a00
            89742414
            b893244992
            f7ee
            03d6
            c1fa02
            8bca
            c1e91f
            8dbc0a80000000
            8b0d $306eb76f
            e87f820a00
            83e03f
            8d5c38e0
            81fbff000000
            7e05
            bbff000000
            8b0d $306eb76f
            e860820a00
            83e03f
            8d6c38e0
            81fdff000000
            7e05
            bdff000000
            8b0d $306eb76f
            e841820a00
            83e03f
            8d4438e0
            3dff000000
            7e05
            b8ff000000
            50
            55
            53
            e802990a00
            81c680000000
            8b4c2414
            8881 $fc9abb6f
            41
            81fe00040000
            894c2414
            0f8c63ffffff
            33f6
            8b15 $7439ba6f
            8b0d $306eb76f
            e87a810a00
            8b15 $7039ba6f
            8b0d $306eb76f
            8904b5 $4860bb6f
            e862810a00
            8b0d $306eb76f
            ba05000000
            8904b5 $485cbb6f
            e84b810a00
            8b0d $306eb76f
            83caff
            2bd0
            8914b5 $d496bb6f
            ba08000000
            e82f810a00
            25ff000000
            46
            81fe00010000
            8a80 $fc9abb6f
            8886 $cf95bb6f
            7c8a
            8b6c2410
            8b5c2418
            c705 $309bbb6f 01000000
            33f6
            8a96 $d095bb6f
            8b04b5 $485cbb6f
            8b0cb5 $4860bb6f
            68ff000000
            52
            50
            51
            50
            51
            e8e6900a00
            46
            81fe00010000
            7cd4
            ff15 $94bfb66f
            8b15 $c095bb6f
            8bf8
            2bc2
            83f828
            7636
            33f6
            8b0cb5 $d496bb6f
            8b04b5 $4860bb6f
            03c1
            8904b5 $4860bb6f
            790c
            ba01000000
            8bce
            e8cf000000
            46
            81fe00010000
            7cd2
            893d $c095bb6f
          "), super::v100::draw_arcane_bg_100_asm_stub),
        ],
      ),
    ],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          // Draw cursor framerate
          Patch::call_c(0xb5678, patch_source!("
            39a8 $e8fab96f
          "), super::v100::should_update_cursor_100_asm_stub),
          // Summit cloud move speed
          Patch::call_c(0x169b4, patch_source!("
            03e9
            81c270010000
          "), super::v107::summit_cloud_move_amount_107_asm_stub),
        ],
      ),
    ],
  )
};
