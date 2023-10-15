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
  v108::{Entity, ADDRESSES, BASE_ADDRESSES},
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
        Patch::call_c(0xe9f4, patch_source!("
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
        Patch::call_c(0x257c, patch_source!("
          a1 $5047ba6f
          85c0
          7508
          6a00
          ff15 $a4bfb96f
        "), game_loop_sleep_hook),
        // Draw paused game framerate
        Patch::call_c(0x9398, patch_source!("ff15 $c4ebbd6f"), draw_game_paused),
        // Draw game framerate & entity sync
        Patch::call_c(0x9abf, patch_source!("
          392d $50eebd6f
          752b
          e884470800
          85c0
          742e
          33c9
          ff15 $c4ebbd6f
          8b0d $d4ebbd6f
          a1 $ecebbd6f
          41
          40
          890d $d4ebbd6f
          a3 $ecebbd6f
          eb0c
          396c2414
          7406
          ff05 $f4ebbd6f
        "), draw_game::<Entity>),
        // Draw cursor framerate
        Patch::call_c(0xb59f8, patch_source!("
          39a8 $d8dcbc6f
        "), super::v100::should_update_cursor_100_asm_stub),
        // Summit cloud move speed
        Patch::call_c(0x16836, patch_source!("
          03e9
          81c270010000
        "), super::v107::summit_cloud_move_amount_107_asm_stub),
      ],
    )],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          // Course entity mouse detection
          Patch::call_std1(0x8da3c, patch_source!("e8990e0300"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x8da44, patch_source!("e88b0e0300"), entity_iso_ypos::<Entity>),
          // Animated entity mouse detection refinement
          Patch::call_std1(0x8dee6, patch_source!("e8ef090300"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x8df0d, patch_source!("e8c2090300"), entity_iso_ypos::<Entity>),
          // Npc mouse over perspective
          Patch::call_std1(0xbde57, patch_source!("e80c0a0000"), entity_linear_xpos::<Entity>),
          Patch::call_std1(0xbde50, patch_source!("e80d0a0000"), entity_linear_ypos::<Entity>),
          // Npc mouse over
          Patch::call_std1(0xbdea4, patch_source!("e8310a0000"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0xbdeb7, patch_source!("e8180a0000"), entity_iso_ypos::<Entity>),
          // Arcane background
          Patch::call_c(0x160ca, patch_source!("
            a1 $407dbe6f
            33f6
            3bc6
            0f854e010000
            8b0d $8c6eba6f
            e870890a00
            8b0d $9877be6f
            e85f890a00
            8b0d $8c6eba6f
            8bd0
            e81a880a00
            89742414
            b893244992
            f7ee
            03d6
            c1fa02
            8bca
            c1e91f
            8dbc0a80000000
            8b0d $8c6eba6f
            e86f870a00
            83e03f
            8d5c38e0
            81fbff000000
            7e05
            bbff000000
            8b0d $8c6eba6f
            e850870a00
            83e03f
            8d6c38e0
            81fdff000000
            7e05
            bdff000000
            8b0d $8c6eba6f
            e831870a00
            83e03f
            8d4438e0
            3dff000000
            7e05
            b8ff000000
            50
            55
            53
            e8fe9d0a00
            81c680000000
            8b4c2414
            8881 $0c7dbe6f
            41
            81fe00040000
            894c2414
            0f8c63ffffff
            33f6
            8b15 $841bbd6f
            8b0d $8c6eba6f
            e86a860a00
            8b15 $801bbd6f
            8b0d $8c6eba6f
            8904b5 $5842be6f
            e852860a00
            8b0d $8c6eba6f
            ba05000000
            8904b5 $583ebe6f
            e83b860a00
            8b0d $8c6eba6f
            83caff
            2bd0
            8914b5 $e478be6f
            ba08000000
            e81f860a00
            25ff000000
            46
            81fe00010000
            8a80 $0c7dbe6f
            8886 $df77be6f
            7c8a
            8b6c2410
            8b5c2418
            c705 $407dbe6f 01000000
            33f6
            8a96 $e077be6f
            8b04b5 $583ebe6f
            8b0cb5 $5842be6f
            68ff000000
            52
            50
            51
            50
            51
            e8e2950a00
            46
            81fe00010000
            7cd4
            ff15 $9cbfb96f
            8b15 $d077be6f
            8bf8
            2bc2
            83f828
            7636
            33f6
            8b0cb5 $e478be6f
            8b04b5 $5842be6f
            03c1
            8904b5 $5842be6f
            790c
            ba01000000
            8bce
            e8cf000000
            46
            81fe00010000
            7cd2
            893d $d077be6f
          "), super::v100::draw_arcane_bg_100_asm_stub),
        ],
      ),
      ModulePatches::new(
        d2::Module::Common,
        &[
          Patch::call_c(0x5f3b7, patch_source!("
            893e
            8d442414
          "), super::v100::intercept_teleport_100_asm_stub),
        ],
      ),
    ],
  )
};
