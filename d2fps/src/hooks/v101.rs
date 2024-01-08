use crate::{
  features::{FeaturePatches, ModulePatches},
  hooks::{
    draw_game, draw_game_paused, entity_iso_xpos, entity_iso_ypos, entity_linear_xpos,
    entity_linear_ypos, game_loop_sleep_hook, HelperFns, Hooks,
  },
};
use bin_patch::{patch_source, Patch};
use d2interface::{
  self as d2,
  v101::{Entity, ADDRESSES, BASE_ADDRESSES},
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
        Patch::call_c(0xeb14, patch_source!("
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
          e8aa26ffff
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
        Patch::call_c(0x75ec, patch_source!("
          a1 $f0560f10
          85c0
          7508
          6a00
          ff15 $701d1810
        "), game_loop_sleep_hook),
        // Draw paused game framerate
        Patch::call_c(0xf957, patch_source!("ff15 $1cea1210"), draw_game_paused),
        // Draw game framerate & entity sync
        Patch::call_c(0xff51, patch_source!("
          391d $88ec1210
          752b
          e8 c42fffff
          85c0
          742e
          33c9
          ff15 $1cea1210
          8b0d $2cea1210
          a1 $44ea1210
          41
          40
          890d $2cea1210
          a3 $44ea1210
          eb0c
          395c2418
          7406
          ff05 $4cea1210
        "), draw_game::<Entity>),
      ],
    )],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          // Course entity mouse detection
          Patch::call_std1(0x9cfe9, patch_source!("e8a8920300"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x9cff1, patch_source!("e89a920300"), entity_iso_ypos::<Entity>),
          // Animated entity mouse detection refinement
          Patch::call_std1(0x9d5f6, patch_source!("e89b8c0300"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x9d61d, patch_source!("e86e8c0300"), entity_iso_ypos::<Entity>),
          // Npc mouse over perspective
          Patch::call_std1(0xd520e, patch_source!("e8f90f0000"), entity_linear_xpos::<Entity>),
          Patch::call_std1(0xd5207, patch_source!("e8fa0f0000"), entity_linear_ypos::<Entity>),
          // Npc mouse over
          Patch::call_std1(0xd523a, patch_source!("e857100000"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0xd524d, patch_source!("e83e100000"), entity_iso_ypos::<Entity>),
        ],
      ),
      ModulePatches::new(
        d2::Module::Common,
        &[
          Patch::call_c(0x5fcef, patch_source!("
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
          Patch::call_c(0x20bb7, patch_source!("
            a1 $70851310
            85c0
            0f8550010000
            8b0d $54820f10
            53
            e822580b00
            8b0d $2c801310
            e811580b00
            8b0d $54820f10
            8bd0
            e8e4560b00
            c744241400000000
            33f6
            b893244992
            f7ee
            03d6
            c1fa02
            8bca
            c1e91f
            8dbc0a80000000
            8b0d $54820f10
            e82d560b00
            83e03f
            8d5c38e0
            81fbff000000
            7e05
            bbff000000
            8b0d $54820f10
            e80e560b00
            83e03f
            8d6c38e0
            81fdff000000
            7e05
            bdff000000
            8b0d $54820f10
            e8ef550b00
            83e03f
            8d4438e0
            3dff000000
            7e05
            b8ff000000
            50
            55
            53
            e844670b00
            81c680000000
            8b4c2414
            8881 $44851310
            41
            81fe00040000
            894c2414
            0f8c63ffffff
            33f6
            5b
            8b0d $54820f10
            ba80020000
            e840550b00
            8b0d $54820f10
            bae0010000
            8904b5 $184b1310
            e829550b00
            8b0d $54820f10
            ba05000000
            8904b5 $18471310
            e812550b00
            8b0d $54820f10
            83caff
            2bd0
            8914b5 $44811310
            ba08000000
            e8f6540b00
            25ff000000
            46
            81fe00010000
            8a80 $44851310
            8886 $3f801310
            7c8c
            8b6c2414
            c705 $70851310 01000000
            33f6
            8a96 $40801310
            8b04b5 $18471310
            8b0cb5 $184b1310
            68ff000000
            52
            50
            51
            50
            51
            e865650b00
            46
            81fe00010000
            7cd4
            ff15 $681d1810
            8b15 $30801310
            8bf8
            2bc2
            83f828
            0f869c000000
            33f6
            8b0cb5 $44811310
            8b04b5 $184b1310
            03c1
            8904b5 $184b1310
            796e
            8b0d $54820f10
            e8c1540b00
            8b0d $54820f10
            83e007
            057f020000
            bae0010000
            8904b5 $184b1310
            e842540b00
            8b0d $54820f10
            ba05000000
            8904b5 $18471310
            e82b540b00
            8b0d $54820f10
            83caff
            2bd0
            8914b5 $44811310
            ba08000000
            e80f540b00
            25ff000000
            8a80 $44851310
            8886 $40801310
            46
            81fe00010000
            0f8c6cffffff
            893d $30801310
          "), super::v100::draw_arcane_bg_100_asm_stub),
        ],
      ),
    ],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          // Draw cursor framerate
          Patch::call_c(0xca148, patch_source!("
            39a8 $00ae1110
          "), super::v100::should_update_cursor_100_asm_stub),
        ],
      ),
    ],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          Patch::nop(0xd3fb, patch_source!("e860030000")),
        ]
      )
    ],
  ),
  helper_fns: HelperFns {
    gen_weather_particle: super::v100::gen_weather_particle_100_trampoline,
  },
};
