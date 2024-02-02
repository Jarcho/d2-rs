use crate::{
  features::{FeaturePatches, ModulePatches},
  hooks::{
    draw_game, draw_game_paused, draw_menu, entity_iso_xpos, entity_iso_ypos, entity_linear_xpos,
    entity_linear_ypos, game_loop_sleep_hook, intercept_teleport, summit_cloud_move_amount,
    update_menu_char_frame, HelperFns, Hooks, UnitId,
  },
};
use bin_patch::{patch_source, Patch};
use core::arch::global_asm;
use d2interface::{
  self as d2,
  v110::{Entity, ADDRESSES, BASE_ADDRESSES},
  IntoSys,
};
use num::WrappingInto;

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
        Patch::call_c(0xd00c, patch_source!("
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
          8b542434
          85d2
          740e
          8b4c2410
          8bc1
          41
          50
          894c2414
          ffd2
          e89f060000
        "), draw_menu_110_asm_stub),
        // Menu char frame rate
        Patch::call_c(0x1abf, patch_source!("
          8b4610
          8b4e08
          03c8
          894e08
          8bc1
        "), update_menu_char_frame_110_asm_stub),
        // Menu sleep patch
        Patch::nop(0xd060, patch_source!("
          8bc7
          7605
          b814000000
          8b0d $20de8b6f
          85c9
          7402
          33c0
          50
          ff15 $c0a18b6f
        ")),
      ],
    )],
    &[ModulePatches::new(
      d2::Module::Client,
      &[
        // Game loop sleep patch
        Patch::call_c(0x266c, patch_source!("
          a1 $c047b76f
          85c0
          7517
          a1 $6079ba6f
          83f806
          740d
          83f808
          7408
          xxxx
          xxxxxxxxxxxx
        "), game_loop_sleep_hook),
        // Draw paused game framerate
        Patch::call_c(0x9b78, patch_source!("ff15 $5477ba6f"), draw_game_paused),
        // Draw game framerate & entity sync
        Patch::call_c(0xa2c4, patch_source!("
          a1 $e079ba6f
          xxxx
          xxxx
          e89ef00700
          85c0
          7430
          33c9
          ff15 $5477ba6f
          8b0d $6477ba6f
          a1 $7c77ba6f
          41
          40
          890d $6477ba6f
          a3 $7c77ba6f
          eb0e
          8b442414
          85c0
          7406
          ff05 $8477ba6f
        "), draw_game::<Entity>),
      ],
    )],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          // Course entity mouse detection
          Patch::call_std1(0x888dc, patch_source!("e8d5200400"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x888e4, patch_source!("e8c7200400"), entity_iso_ypos::<Entity>),
          // Animated entity mouse detection refinement
          Patch::call_std1(0x88e36, patch_source!("e87b1b0400"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0x88e5d, patch_source!("e84e1b0400"), entity_iso_ypos::<Entity>),
          // Npc mouse over perspective
          Patch::call_std1(0xbf827, patch_source!("e830b10000"), entity_linear_xpos::<Entity>),
          Patch::call_std1(0xbf820, patch_source!("e831b10000"), entity_linear_ypos::<Entity>),
          // Npc mouse over
          Patch::call_std1(0xbf874, patch_source!("e83db10000"), entity_iso_xpos::<Entity>),
          Patch::call_std1(0xbf887, patch_source!("e824b10000"), entity_iso_ypos::<Entity>),
        ],
      ),
      ModulePatches::new(
        d2::Module::Common,
        &[
          Patch::call_c(0x6d85c, patch_source!("
            8d442428
            893e
          "), intercept_teleport_110_asm_stub),
        ],
      ),
    ],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          Patch::call_c(0x1733a, patch_source!("
            a1 $ecf8ba6f
            bdff000000
            85c0
            0f8507010000
            8b0d $2c71b76f
            e815380b00
            8b0d $44f3ba6f
            e804380b00
            8b0d $2c71b76f
            8bd0
            e87d360b00
            c744241800000000
            33ff
            b893244992
            f7ef
            03d7
            c1fa02
            8bca
            c1e91f
            8db40a80000000
            8b0d $2c71b76f
            e83415ffff
            83e03f
            8d4430e0
            3bc5
            89442410
            7e04
            896c2410
            8b0d $2c71b76f
            e81615ffff
            83e03f
            8d4430e0
            3bc5
            89442414
            7e04
            896c2414
            8b0d $2c71b76f
            bac590c66a
            33ed
            894c2420
            8b01
            8b5904
            f7e2
            03c3
            13d5
            8901
            83e03f
            bdff000000
            895104
            8d4430e0
            3bc5
            7e02
            8bc5
            8b542414
            50
            8b442414
            52
            50
            e87e4b0b00
            81c780000000
            8b4c2418
            8881 $b8f8ba6f
            41
            81ff00040000
            894c2418
            0f8c48ffffff
            33f6
            33d2
            8bce
            e854010000
            46
            81fe00010000
            7cee
            8b7c2424
            c705 $ecf8ba6f 01000000
            33f6
            8a96 $88f3ba6f
            8b04b5 $00baba6f
            8b0cb5 $00beba6f
            55
            52
            50
            51
            50
            51
            e8d6430b00
            46
            81fe00010000
            7cd8
            ff15 $14dfb66f
            8b15 $78f3ba6f
            8bd8
            2bc2
            83f828
            7636
            33f6
            8b0cb5 $90f4ba6f
            8b04b5 $00beba6f
            03c1
            8904b5 $00beba6f
            790c
            ba01000000
            8bce
            e8d7000000
            46
            81fe00010000
            7cd2
            891d $78f3ba6f
            8b6c241c
          "), super::v100::draw_arcane_bg_100_asm_stub),
        ],
      ),
    ],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          // Draw cursor framerate
          Patch::call_c(0xb77e8, patch_source!("
            39a8 $586bb96f
          "), super::v100::should_update_cursor_100_asm_stub),
          // Summit cloud move speed
          Patch::call_c(0x17b75, patch_source!("
            03d8
            81c170010000
          "), summit_cloud_move_amount_110_asm_stub),
        ],
      ),
    ],
    &[
      ModulePatches::new(
        d2::Module::Client,
        &[
          Patch::nop(0x7a46, patch_source!("
            391d $1477ba6f
            7429
            8b0d $4476ba6f
            51
            e8c22c0c00
            d9e1
            d9542410
            d81d $2ce2b66f
            dfe0
            f6c405
            7a20
            c74424100000803e
            eb16
            d905 $3c76ba6f
            d80d $28e2b66f
            d805 $24e2b66f
            d95c2410
            a1 $0477ba6f
            8b8810010000
            8bb01c010000
            85c9
            0f8c3f010000
            66833e00
            0f8420010000
            db4614
            d84c2410
            e8648f0b00
            8b15 $4476ba6f
            8944241c
            db44241c
            52
            d95c2420
            e8482c0c00
            d84c241c
            e8438f0b00
            8bf8
            a1 $4476ba6f
            50
            e82c2c0c00
            d84c241c
            e82d8f0b00
            8b0d $dc79ba6f
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
            7442
            8b4620
            85c0
            752e
            8b0d $0477ba6f
            8bd3
            c1e210
            e8d92b0c00
            8b0d $0477ba6f
            a1 $5876ba6f
            8b9114010000
            3bd0
            7d16
            8bcd
            e823020000
            eb0d
            33ff
            48
            c7461400000000
            894620
            66833e00
            746e
            a1 $1477ba6f
            85c0
            7434
            8b461c
            85c0
            752d
            8b0d $2c77ba6f
            b81f85eb51
            c1e109
            f7e1
            8b4618
            c1ea03
            03d0
            81e2ff010000
            52
            e8802b0c00
            dcc0
            e8838e0b00
            03f8
            8b5604
            a1 $d879ba6f
            2bd0
            8d043a
            894604
            8b0d $ec40b76f
            3bc1
            7c0b
            2bc1
            894604
            8b0d $ec40b76f
            8b4604
            85c0
            7d05
            03c1
            894604
            a1 $0477ba6f
            43
            83c628
            3b9810010000
            0f8ec1feffff
            ff05 $2c77ba6f
          ")),
        ]
      )
    ],
  ),
  helper_fns: HelperFns {
    gen_weather_particle: super::v100::gen_weather_particle_100_trampoline,
  },
};

impl super::Entity for Entity {
  fn unit_id(&self) -> UnitId {
    UnitId::new(self.kind, self.id)
  }

  fn has_room(&self) -> bool {
    self.has_room()
  }

  fn linear_pos(&self) -> d2::LinearM2d<d2::FU16> {
    self.pos(|pos| pos.linear_pos.winto(), |pos| pos.linear_pos).unwrap()
  }

  fn iso_pos(&self) -> d2::IsoP2d<i32> {
    self.pos(|pos| pos.iso_pos, |pos| pos.iso_pos).unwrap()
  }

  fn set_pos(&mut self, pos: d2::LinearM2d<d2::FU16>) {
    unsafe {
      if let Some(mut epos) = self.pos.d {
        epos.as_mut().linear_pos = pos;
        epos.as_mut().iso_pos = pos.into_sys();
      }
    }
  }

  fn rng(&mut self) -> &mut d2::Rng {
    &mut self.rng
  }
}

global_asm! {
  ".global _draw_menu_110_asm_stub",
  "_draw_menu_110_asm_stub:",
  "mov ecx, [esp+0x38]",
  "lea edx, [esp+0x14]",
  "call {}",
  "ret",
  sym draw_menu,
}
extern "C" {
  pub fn draw_menu_110_asm_stub();
}

global_asm! {
  ".global _update_menu_char_frame_110_asm_stub",
  "_update_menu_char_frame_110_asm_stub:",
  "mov ecx, [esi+0x10]",
  "lea edx, [esi+0x08]",
  "call {}",
  "ret",
  sym update_menu_char_frame,
}
extern "C" {
  pub fn update_menu_char_frame_110_asm_stub();
}

global_asm! {
  ".global _intercept_teleport_110_asm_stub",
  "_intercept_teleport_110_asm_stub:",
  "mov [esi], edi",
  "mov ecx, [esi+0x30]",
  "mov edx, [ecx+0xc]",
  "mov ecx, [ecx]",
  "call {}",
  "lea eax, [esp+0x2c]",
  "ret",
  sym intercept_teleport,
}
extern "C" {
  pub fn intercept_teleport_110_asm_stub();
}

global_asm! {
  ".global _summit_cloud_move_amount_110_asm_stub",
  "_summit_cloud_move_amount_110_asm_stub:",
  "add ecx, 0x170",
  "push ecx",
  "mov ecx, eax",
  "call {}",
  "add ebx, eax",
  "pop ecx",
  "ret",
  sym summit_cloud_move_amount,
}
extern "C" {
  pub fn summit_cloud_move_amount_110_asm_stub();
}
