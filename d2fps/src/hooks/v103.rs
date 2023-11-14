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
  v103::{Entity, ADDRESSES, BASE_ADDRESSES},
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
        Patch::call_c(0xeb94, patch_source!("
          8d4c2414
          51
          ff15 $78780f10
          8d54241c
          52
          ff15 $c0780f10
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
          e80b660000
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
          e82a26ffff
        "), super::v100::draw_menu_100_asm_stub),
        // Menu char frame rate
        Patch::call_c(0x26ea, patch_source!("
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
        Patch::call_c(0x767c, patch_source!("
          a1 $f0560f10
          85c0
          7508
          6a00
          ff15 $241e1810
        "), game_loop_sleep_hook),
        // Draw paused game framerate
        Patch::call_c(0xf535, patch_source!("ff15 $bce81210"), draw_game_paused),
        // Draw game framerate & entity sync
        Patch::call_c(0xfcad, patch_source!("
          391d $48eb1210
          752b
          e89032ffff
          85c0
          742e
          33c9
          ff15 $bce81210
          8b0d $cce81210
          a1 $e4e81210
          41
          40
          890d $cce81210
          a3 $e4e81210
          eb0c
          395c2418
          7406
          ff05 $ece81210
        "), draw_game::<Entity>),
      ],
    )],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          // Course entity mouse detection
          Patch::call_std1(0x9daa9, patch_source!("e8c8920300"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x9dab1, patch_source!("e8ba920300"), entity_iso_ypos::<Entity>),
          // Animated entity mouse detection refinement
          Patch::call_std1(0x9e0b6, patch_source!("e8bb8c0300"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x9e0dd, patch_source!("e88e8c0300"), entity_iso_ypos::<Entity>),
          // Npc mouse over perspective
          Patch::call_std1(0xd5cee, patch_source!("e8f90f0000"), entity_linear_xpos::<Entity>),
          Patch::call_std1(0xd5ce7, patch_source!("e8fa0f0000"), entity_linear_ypos::<Entity>),
          // Npc mouse over
          Patch::call_std1(0xd5d1a, patch_source!("e857100000"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0xd5d2d, patch_source!("e83e100000"), entity_iso_ypos::<Entity>),
        ],
      ),
      ModulePatches::new(
        d2::Module::Common,
        &[
          Patch::call_c(0x60cef, patch_source!("
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
          Patch::call_c(0x20f77, patch_source!("
            a1 $30841310
            85c0
            0f8550010000
            8b0d $24820f10
            53
            e8485f0b00
            8b0d $ec7e1310
            e8375f0b00
            8b0d $24820f10
            8bd0
            e8045e0b00
            c744241400000000
            33f6
            b893244992
            f7ee
            03d6
            c1fa02
            8bca
            c1e91f
            8dbc0a80000000
            8b0d $24820f10
            e84d5d0b00
            83e03f
            8d5c38e0
            81fbff000000
            7e05
            bbff000000
            8b0d $24820f10
            e82e5d0b00
            83e03f
            8d6c38e0
            81fdff000000
            7e05
            bdff000000
            8b0d $24820f10
            e80f5d0b00
            83e03f
            8d4438e0
            3dff000000
            7e05
            b8ff000000
            50
            55
            53
            e87c6e0b00
            81c680000000
            8b4c2414
            8881 $04841310
            41
            81fe00040000
            894c2414
            0f8c63ffffff
            33f6
            5b
            8b0d $24820f10
            ba80020000
            e8605c0b00
            8b0d $24820f10
            bae0010000
            8904b5 $d8491310
            e8495c0b00
            8b0d $24820f10
            ba05000000
            8904b5 $d8451310
            e8325c0b00
            8b0d $24820f10
            83caff
            2bd0
            8914b5 $04801310
            ba08000000
            e8165c0b00
            25ff000000
            46
            81fe00010000
            8a80 $04841310
            8886 $ff7e1310
            7c8c
            8b6c2414
            c705 $30841310 01000000
            33f6
            8a96 $007f1310
            8b04b5 $d8451310
            8b0cb5 $d8491310
            68ff000000
            52
            50
            51
            50
            51
            e89d6c0b00
            46
            81fe00010000
            7cd4
            ff15 $1c1e1810
            8b15 $f07e1310
            8bf8
            2bc2
            83f828
            0f869c000000
            33f6
            8b0cb5 $04801310
            8b04b5 $d8491310
            03c1
            8904b5 $d8491310
            796e
            8b0d $24820f10
            e8e15b0b00
            8b0d $24820f10
            83e007
            057f020000
            bae0010000
            8904b5 $d8491310
            e8625b0b00
            8b0d $24820f10
            ba05000000
            8904b5 $d8451310
            e84b5b0b00
            8b0d $24820f10
            83caff
            2bd0
            8914b5 $04801310
            ba08000000
            e82f5b0b00
            25ff000000
            8a80 $04841310
            8886 $007f1310
            46
            81fe00010000
            0f8c6cffffff
            893d $f07e1310
          "), super::v100::draw_arcane_bg_100_asm_stub),
        ],
      ),
    ],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          // Draw cursor framerate
          Patch::call_c(0xcac28, patch_source!("
            39a8 $d0ac1110
          "), super::v100::should_update_cursor_100_asm_stub),
        ],
      ),
    ],
  ),
};
