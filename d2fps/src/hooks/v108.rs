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
  version: "1.08",
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
