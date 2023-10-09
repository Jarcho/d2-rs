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
  v104b::{Entity, ADDRESSES, BASE_ADDRESSES},
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
        Patch::call_c(0xb6b4, patch_source!("
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
          e8eb570000
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
          ff15 $7cafc06f
        "), game_loop_sleep_hook),
        // Draw paused game framerate
        Patch::call_c(0x7b08, patch_source!("ff15 $3c2ec46f"), draw_game_paused),
        // Draw game framerate & entity sync
        Patch::call_c(0x8248, patch_source!("
          391d $c030c46f
          752b
          e81bb20600
          85c0
          742e
          33c9
          ff15 $3c2ec46f
          8b0d $4c2ec46f
          a1 $642ec46f
          41
          40
          890d $4c2ec46f
          a3 $642ec46f
          eb0c
          395c2414
          7406
          ff05 $6c2ec46f
        "), draw_game::<Entity>),
        // Draw cursor framerate
        Patch::call_c(0x944a8, patch_source!("
          39a8 $f819c36f
        "), super::v100::should_update_cursor_100_asm_stub),
      ],
    )],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          // Course entity mouse detection
          Patch::call_std1(0x72c99, patch_source!("e89aa40200"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x72ca1, patch_source!("e88ca40200"), entity_iso_ypos::<Entity>),
          // Animated entity mouse detection refinement
          Patch::call_std1(0x73136, patch_source!("e8fd9f0200"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x7315d, patch_source!("e8d09f0200"), entity_iso_ypos::<Entity>),
          // Npc mouse over perspective
          Patch::call_std1(0x9c75e, patch_source!("e84b090000"), entity_linear_xpos::<Entity>),
          Patch::call_std1(0x9c757, patch_source!("e84c090000"), entity_linear_ypos::<Entity>),
          // Npc mouse over
          Patch::call_std1(0x9c78a, patch_source!("e8a9090000"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x9c79d, patch_source!("e890090000"), entity_iso_ypos::<Entity>),
        ],
      ),
      ModulePatches::new(
        d2::Module::Common,
        &[
          Patch::call_c(0x4a317, patch_source!("
            893e
            8d442414
          "), super::v100::intercept_teleport_100_asm_stub),
        ],
      ),
    ],
  )
};
