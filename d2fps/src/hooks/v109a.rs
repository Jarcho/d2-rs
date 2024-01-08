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
  v109a::{Entity, ADDRESSES, BASE_ADDRESSES},
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
          6a19
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
          894608
        "), super::v100::update_menu_char_frame_100_asm_stub),
      ],
    )],
    &[ModulePatches::new(
      d2::Module::Client,
      &[
        // Game loop sleep patch
        Patch::call_c(0x262c, patch_source!("
          a1 $5057b76f
          85c0
          7508
          6a00
          ff15 $9ccfb66f
        "), game_loop_sleep_hook),
        // Draw paused game framerate
        Patch::call_c(0x9448, patch_source!("ff15 $541bbb6f"), draw_game_paused),
        // Draw game framerate & entity sync
        Patch::call_c(0x9b6f, patch_source!("
          xxxxxxxxxxxx
          xxxx
          e8244d0800
          85c0
          742e
          33c9
          ff15 $541bbb6f
          8b0d $641bbb6f
          a1 $7c1bbb6f
          41
          40
          890d $641bbb6f
          a3 $7c1bbb6f
          eb0c
          396c2414
          7406
          ff05 $841bbb6f
        "), draw_game::<Entity>),
      ],
    )],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          // Course entity mouse detection
          Patch::call_std1(0x8e08c, patch_source!("e849110300"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x8e094, patch_source!("e83b110300"), entity_iso_ypos::<Entity>),
          // Animated entity mouse detection refinement
          Patch::call_std1(0x8e536, patch_source!("e89f0c0300"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x8e55d, patch_source!("e8720c0300"), entity_iso_ypos::<Entity>),
          // Npc mouse over perspective
          Patch::call_std1(0xbe757, patch_source!("e80c0a0000"), entity_linear_xpos::<Entity>),
          Patch::call_std1(0xbe750, patch_source!("e80d0a0000"), entity_linear_ypos::<Entity>),
          // Npc mouse over
          Patch::call_std1(0xbe7a4, patch_source!("e8310a0000"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0xbe7b7, patch_source!("e8180a0000"), entity_iso_ypos::<Entity>),
        ],
      ),
      ModulePatches::new(
        d2::Module::Common,
        &[
          Patch::call_c(0x5f997, patch_source!("
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
          Patch::call_c(0x1624a, patch_source!("
            a1 $d0acbb6f
            33f6
            3bc6
            0f854e010000
            8b0d $8c7eb76f
            e8fc900a00
            8b0d $2ca7bb6f
            e8eb900a00
            8b0d $8c7eb76f
            8bd0
            e89a8f0a00
            89742414
            b893244992
            f7ee
            03d6
            c1fa02
            8bca
            c1e91f
            8dbc0a80000000
            8b0d $8c7eb76f
            e8ef8e0a00
            83e03f
            8d5c38e0
            81fbff000000
            7e05
            bbff000000
            8b0d $8c7eb76f
            e8d08e0a00
            83e03f
            8d6c38e0
            81fdff000000
            7e05
            bdff000000
            8b0d $8c7eb76f
            e8b18e0a00
            83e03f
            8d4438e0
            3dff000000
            7e05
            b8ff000000
            50
            55
            53
            e872a50a00
            81c680000000
            8b4c2414
            8881 $9cacbb6f
            41
            81fe00040000
            894c2414
            0f8c63ffffff
            33f6
            8b15 $144bba6f
            8b0d $8c7eb76f
            e8ea8d0a00
            8b15 $104bba6f
            8b0d $8c7eb76f
            8904b5 $e871bb6f
            e8d28d0a00
            8b0d $8c7eb76f
            ba05000000
            8904b5 $e86dbb6f
            e8bb8d0a00
            8b0d $8c7eb76f
            83caff
            2bd0
            8914b5 $74a8bb6f
            ba08000000
            e89f8d0a00
            25ff000000
            46
            81fe00010000
            8a80 $9cacbb6f
            8886 $6fa7bb6f
            7c8a
            8b6c2410
            8b5c2418
            c705 $d0acbb6f 01000000
            33f6
            8a96 $70a7bb6f
            8b04b5 $e86dbb6f
            8b0cb5 $e871bb6f
            68ff000000
            52
            50
            51
            50
            51
            e8569d0a00
            46
            81fe00010000
            7cd4
            ff15 $94cfb66f
            8b15 $60a7bb6f
            8bf8
            2bc2
            83f828
            7636
            33f6
            8b0cb5 $74a8bb6f
            8b04b5 $e871bb6f
            03c1
            8904b5 $e871bb6f
            790c
            ba01000000
            8bce
            e8cf000000
            46
            81fe00010000
            7cd2
            893d $60a7bb6f
          "), super::v100::draw_arcane_bg_100_asm_stub),
        ],
      ),
    ],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          // Draw cursor framerate
          Patch::call_c(0xb62f8, patch_source!("
            39a8 $780cba6f
          "), super::v100::should_update_cursor_100_asm_stub),
          // Summit cloud move speed
          Patch::call_c(0x169c4, patch_source!("
            03e9
            81c270010000
          "), super::v107::summit_cloud_move_amount_107_asm_stub),
        ],
      ),
    ],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          Patch::nop(0x74b6, patch_source!("
            391d $141bbb6f
            7429
            8b0d $3c1abb6f
            51
            e8b2940b00
            d9e1
            d9542410
            d81d $acd2b66f
            dfe0
            f6c401
            7420
            c74424100000803e
            eb16
            d905 $341abb6f
            d80d $a8d2b66f
            d805 $a4d2b66f
            d95c2410
            a1 $041bbb6f
            8b8810010000
            8bb01c010000
            85c9
            0f8c42010000
            66833e00
            0f8423010000
            db4614
            d84c2410
            e8e49d0b00
            8b15 $3c1abb6f
            8944241c
            db44241c
            52
            d95c2420
            e838940b00
            d84c241c
            e8c39d0b00
            8bf8
            a1 $3c1abb6f
            50
            e81c940b00
            d84c241c
            e8ad9d0b00
            8b0d $dc1dbb6f
            8b5608
            2bc1
            8b4e0c
            03d0
            8bc2
            895608
            3bc1
            7e07
            c7461c01000000
            8b461c
            85c0
            7445
            8b4620
            85c0
            752e
            8b0d $041bbb6f
            8bd3
            c1e210
            e8c9930b00
            8b0d $041bbb6f
            a1 $501abb6f
            8b9114010000
            3bd0
            7d19
            8bcd
            e833020000
            eb10
            8b4620
            33ff
            48
            c7461400000000
            894620
            66833e00
            746e
            a1 $141bbb6f
            85c0
            7434
            8b461c
            85c0
            752d
            8b0d $2c1bbb6f
            b81f85eb51
            c1e109
            f7e1
            8b4618
            c1ea03
            03d0
            81e2ff010000
            52
            e86d930b00
            dcc0
            e8009d0b00
            03f8
            8b5604
            a1 $d81dbb6f
            2bd0
            8d043a
            894604
            8b0d $e450b76f
            3bc1
            7c0b
            2bc1
            894604
            8b0d $e450b76f
            8b4604
            85c0
            7d05
            03c1
            894604
            a1 $041bbb6f
            43
            83c628
            3b9810010000
            0f8ebefeffff
            ff05 $2c1bbb6f
          ")),
        ]
      )
    ],
  ),
  helper_fns: HelperFns {
    gen_weather_particle: super::v100::gen_weather_particle_100_trampoline,
  },
};
