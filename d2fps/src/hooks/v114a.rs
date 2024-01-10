use crate::{
  features::{FeaturePatches, ModulePatches},
  hooks::{
    draw_game, draw_game_paused, game_loop_sleep_hook, update_menu_char_frame, HelperFns, Hooks,
  },
  INSTANCE,
};
use bin_patch::{patch_source, Patch};
use core::arch::global_asm;
use core::sync::atomic::Ordering::Relaxed;
use d2interface::{
  self as d2,
  v114a::{Entity, ADDRESSES, BASE_ADDRESSES},
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
        Patch::call_c(0x3da6b, patch_source!("
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
          e820f3ffff
        "), super::v110::draw_menu_110_asm_stub),
        // Menu char frame rate
        Patch::call_c(0x3f92c, patch_source!("
          8b5710
          015708
          8b4708
        "), update_menu_char_frame_114a_asm_stub),
        // Menu sleep patch
        Patch::nop(0x3dabf, patch_source!("
          8bc7
          76xx
          b814000000
          833d $e0927000 00
          xx02
          33c0
          50
          ff15 $98d16c00
        ")),
      ],
    )],
    &[ModulePatches::new(
      d2::Module::GameExe,
      &[
        // Game loop sleep patch
        Patch::call_c(0x60b11, patch_source!("
          833d $30b17000 00
          7517
          a1 $b0e18200
          83f806
          740d
          83f808
          7408
          xxxx
          xxxxxxxxxxxx
        "), game_loop_sleep_hook),
        // Draw paused game framerate
        Patch::call_c(0x53958, patch_source!("ff15 $24e08200"), draw_game_paused),
        // Draw game framerate & entity sync
        Patch::call_c(0x53bc1, patch_source!("
          392d $a4e28200
          751f
          e8 229b0000
          85c0
          7422
          33c9
          ff15 $24e08200
          011d $34e08200
          011d $4ce08200
          eb0c
          396c2410
          7406
          011d $54e08200
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
          Patch::nop(0x5a10d, patch_source!("
            56
            e83df2ffff
          ")),
        ]
      )
    ],
  ),
  helper_fns: HelperFns {
    gen_weather_particle: gen_weather_particle_114_trampoline,
  },
};

global_asm! {
  ".global _update_menu_char_frame_114a_asm_stub",
  "_update_menu_char_frame_114a_asm_stub:",
  "mov ecx, [edi+0x10]",
  "lea edx, [edi+0x08]",
  "call {}",
  "ret",
  sym update_menu_char_frame,
}
extern "C" {
  pub fn update_menu_char_frame_114a_asm_stub();
}

unsafe extern "fastcall" fn move_summit_cloud(i: usize, amount: d2::FU4) {
  (*INSTANCE.sync.lock().accessor.summit_cloud_x_pos)[i / 4] +=
    d2::FI4::from(f64::from(amount) * INSTANCE.update_time_fract.load(Relaxed));
}

global_asm! {
  ".global _move_summit_cloud_114a_asm_stub",
  "_move_summit_cloud_114a_asm_stub:",
  "push ecx",
  "mov ecx, edi",
  "call {}",
  "pop ecx",
  "ret",
  sym move_summit_cloud,
}
extern "C" {
  pub fn move_summit_cloud_114a_asm_stub();
}

global_asm! {
  ".global @gen_weather_particle_114_trampoline@8",
  "@gen_weather_particle_114_trampoline@8:",
  "mov eax, ecx",
  "jmp edx",
}
extern "fastcall" {
  pub fn gen_weather_particle_114_trampoline(
    _: *mut d2::Rng,
    _: usize, // (&mut d2::Rng @ eax)
  );
}
