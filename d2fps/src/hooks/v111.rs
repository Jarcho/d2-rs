use crate::{
  features::{FeaturePatches, ModulePatches},
  hooks::{
    draw_arcane_bg, draw_game, draw_game_paused, entity_iso_xpos, entity_iso_ypos,
    entity_linear_xpos, entity_linear_ypos, game_loop_sleep_hook, intercept_teleport,
    should_update_cursor, summit_cloud_move_amount, update_menu_char_frame, HelperFns, Hooks,
  },
};
use bin_patch::{patch_source, Patch};
use core::arch::global_asm;
use d2interface::{
  self as d2,
  v111::{Entity, ADDRESSES, BASE_ADDRESSES},
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
        Patch::call_c(0xc61c, patch_source!("
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
        Patch::call_c(0x17b66, patch_source!("
          8b4310
          8b7308
          8b4b0c
          03f0
          8bc6
        "), update_menu_char_frame_111_asm_stub),
        // Menu sleep patch
        Patch::nop(0xc66e, patch_source!("
          8bc7
          7605
          b814000000
          8b0d $f0fa8f6f
          85c9
          7402
          33c0
          50
          ff15 $a0a28f6f
        ")),
      ],
    )],
    &[ModulePatches::new(
      d2::Module::Client,
      &[
        // Game loop sleep patch
        Patch::call_c(0x8bcfc, patch_source!("
          a1 $5010b96f
          85c0
          7517
          a1 $bcbfbc6f
          83f806
          740d
          83f808
          7408
          xxxx
          xxxxxxxxxxxx
        "), game_loop_sleep_hook),
        // Draw paused game framerate
        Patch::call_c(0x897c5, patch_source!("ff15 $84b3ba6f"), draw_game_paused),
        // Draw game framerate & entity sync
        Patch::call_c(0x89a51, patch_source!("
          391d $2034bd6f
          7535
          a1 $f0c4bc6f
          3bc3
          7438
          50
          e84628f8ff
          3bc3
          742e
          33c9
          ff15 $84b3ba6f
          8b0d $94b3ba6f
          a1 $acb3ba6f
          41
          40
          890d $94b3ba6f
          a3 $acb3ba6f
          eb0c
          395c2410
          7406
          ff05 $b4b3ba6f
        "), draw_game::<Entity>),
      ],
    )],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          // Course entity mouse detection
          Patch::call_std1(0x27274, patch_source!("e89d4efeff"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x2727c, patch_source!("e8bf4efeff"), entity_iso_ypos::<Entity>),
          // Animated entity mouse detection refinement
          Patch::call_std1(0x2700e, patch_source!("e80351feff"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x27033, patch_source!("e80851feff"), entity_iso_ypos::<Entity>),
          // Npc mouse over perspective
          Patch::call_std1(0xc3934, patch_source!("e89b87f4ff"), entity_linear_xpos::<Entity>),
          Patch::call_std1(0xc392d, patch_source!("e86288f4ff"), entity_linear_ypos::<Entity>),
          // Npc mouse over
          Patch::call_std1(0xc398f, patch_source!("e88287f4ff"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0xc39a4, patch_source!("e89787f4ff"), entity_iso_ypos::<Entity>),
        ],
      ),
      ModulePatches::new(
        d2::Module::Common,
        &[
          Patch::call_c(0x11797, patch_source!("e874f8ffff"), intercept_teleport_111_asm_stub),
        ],
      ),
    ],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          Patch::call_c(0x5c3d1, patch_source!("e81afcffff"), draw_arcane_bg),
        ],
      ),
    ],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          // Draw cursor framerate
          Patch::call_c(0x38c69, patch_source!("
            6bc01c
            8b88 $f036ba6f
            85c9
          "), should_update_cursor_111_asm_stub),
          // Summit cloud move speed
          Patch::call_c(0x5bc8c, patch_source!("
            03da
            8bc3
            3bc7
          "), summit_cloud_move_amount_111_asm_stub),
        ],
      ),
    ],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          Patch::nop(0x4dc02, patch_source!("
            53
            e828fbffff
          ")),
        ]
      )
    ],
  ),
  helper_fns: HelperFns {
    gen_weather_particle: gen_weather_particle_111_trampoline,
  },
};

global_asm! {
  ".global _update_menu_char_frame_111_asm_stub",
  "_update_menu_char_frame_111_asm_stub:",
  "mov ecx, [ebx+0x10]",
  "lea edx, [ebx+0x08]",
  "call {}",
  "mov ecx, [ebx+0x0c]",
  "mov esi, eax",
  "ret",
  sym update_menu_char_frame,
}
extern "C" {
  pub fn update_menu_char_frame_111_asm_stub();
}

global_asm! {
  ".global _intercept_teleport_111_asm_stub",
  "_intercept_teleport_111_asm_stub:",
  "mov ecx, [esi+0x30]",
  "mov edx, [ecx+0xc]",
  "mov ecx, [ecx]",
  "call {}",
  "jmp eax",
  sym intercept_teleport,
}
extern "C" {
  pub fn intercept_teleport_111_asm_stub();
}

global_asm! {
  ".global _should_update_cursor_111_asm_stub",
  "_should_update_cursor_111_asm_stub:",
  "mov ecx, eax",
  "call {}",
  "test eax, eax",
  "ret",
  sym should_update_cursor,
}
extern "C" {
  pub fn should_update_cursor_111_asm_stub();
}

global_asm! {
  ".global _summit_cloud_move_amount_111_asm_stub",
  "_summit_cloud_move_amount_111_asm_stub:",
  "push ecx",
  "mov ecx, edx",
  "call {}",
  "add eax, ebx",
  "mov ebx, eax",
  "cmp eax, edi",
  "pop ecx",
  "ret",
  sym summit_cloud_move_amount,
}
extern "C" {
  pub fn summit_cloud_move_amount_111_asm_stub();
}

global_asm! {
  ".global @gen_weather_particle_111_trampoline@8",
  "@gen_weather_particle_111_trampoline@8:",
  "pop eax",
  "push ecx",
  "push eax",
  "jmp edx",
}
extern "fastcall" {
  pub fn gen_weather_particle_111_trampoline(
    _: *mut d2::Rng,
    _: usize, // stdcall(&mut d2::Rng),
  );
}
