use crate::{
  features::{FeaturePatches, ModulePatches},
  hooks::{draw_game, draw_game_paused, game_loop_sleep_hook, HelperFns, Hooks},
};
use bin_patch::{patch_source, Patch};
use d2interface::{
  self as d2,
  v114b::{Entity, ADDRESSES, BASE_ADDRESSES},
};

#[rustfmt::skip]
pub(super) const HOOKS: Hooks = Hooks {
  addresses: ADDRESSES,
  base_addresses: BASE_ADDRESSES,
  load_modules: d2::Modules::load_combined_module,
  patches: FeaturePatches::new(
    &[ModulePatches::new(
      d2::Module::GameExe,
      &[
        // Draw menu framerate
        Patch::call_c(0xf7c8b, patch_source!("
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
          7f28
          xxxxxx
          81ff18fcffff
          7d02
          33ff
          8b442434
          85c0
          740e
          8b742410
          56
          ffd0
          83c601
          89742410
          e860f2ffff
        "), super::v110::draw_menu_110_asm_stub),
        // Menu char frame rate
        Patch::call_c(0x10172c, patch_source!("
          8b5710
          015708
          8b4708
        "), super::v114a::update_menu_char_frame_114a_asm_stub),
        // Menu sleep patch
        Patch::nop(0xf7cdf, patch_source!("
          8bc7
          76xx
          b814000000
          833d $58c27200 00
          xx02
          33c0
          50
          ff15 $2cd26c00
        ")),
      ],
    )],
    &[ModulePatches::new(
      d2::Module::GameExe,
      &[
        // Game loop sleep patch
        Patch::call_c(0x4d23f, patch_source!("
          833d $ecf37000 00
          7517
          a1 $98867900
          83f806
          740d
          83f808
          7408
          xxxx
          xxxxxxxxxxxx
        "), game_loop_sleep_hook),
        // Draw paused game framerate
        Patch::call_c(0x4a746, patch_source!("ff15 $0c857900"), draw_game_paused),
        // Draw game framerate & entity sync
        Patch::call_c(0x4a9af, patch_source!("
          392d $8c877900
          751f
          e8e4540100
          85c0
          7422
          33c9
          ff15 $0c857900
          011d $1c857900
          011d $34857900
          eb0c
          396c2410
          7406
          011d $3c857900
        "), draw_game::<Entity>),
      ],
    )],
    &[],
    &[],
    &[],
    &[
      ModulePatches::new(
        d2::Module::GameExe,
        &[
          Patch::nop(0x6fc9d, patch_source!("
            56
            e83df2ffff
          ")),
        ]
      )
    ],
  ),
  helper_fns: HelperFns {
    gen_weather_particle: super::v114a::gen_weather_particle_114_trampoline,
  },
};
