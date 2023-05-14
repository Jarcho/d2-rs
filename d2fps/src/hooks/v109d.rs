use crate::{
  hooks::{
    draw_game, draw_game_paused, game_loop_sleep_hook, Module, D2CLIENT_DLL, D2GAME_DLL, D2GFX_DLL,
    D2WIN_DLL,
  },
  patch::{CallPatch, CallTargetPatch},
  tracker::UnitId,
};
use core::{arch::global_asm, ptr::NonNull};
use d2interface::{
  v109d::{D2ClientAccessor, D2GameAccessor, D2GfxAccessor, D2WinAccessor, DyPos, Entity},
  FixedU16, IsoPos, LinearPos,
};

#[rustfmt::skip]
static D2CLIENT_TARGET_PATCHES: [CallTargetPatch; 53] = [
  // Create entity light
  call_target_patch!(0x4306, 0x000ba1de, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0x4315, 0x000ba1c9, super::entity_linear_ypos::<Entity>),
  // Lighting position
  call_target_patch!(0x4ac0, 0x000b99ee, super::entity_linear_whole_xpos::<Entity>),
  call_target_patch!(0x4ac8, 0x000b99e0, super::entity_linear_whole_ypos::<Entity>),
  // Apply entity light
  call_target_patch!(0x4ba6, 0x000b993e, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0x4bae, 0x000b9930, super::entity_linear_ypos::<Entity>),
  // Viewport position
  call_target_patch!(0x8f50, 0x000b5606, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x8f6d, 0x000b55e3, super::entity_iso_ypos::<Entity>),
  // Entity shift
  call_target_patch!(0x14c67, 0x000a98ef, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x14c6f, 0x000a98e1, super::entity_iso_ypos::<Entity>),
  // Perspective viewport position
  call_target_patch!(0x1619d, 0x000a8347, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0x161a5, 0x000a8339, super::entity_linear_ypos::<Entity>),
  // Summit background position
  call_target_patch!(0x165c4, 0x000a7f92, super::entity_iso_xpos::<Entity>),
  // Perspective whirlwind overlay pos
  call_target_patch!(0x1e865, 0x0009fc7f, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0x1e85e, 0x0009fc80, super::entity_linear_ypos::<Entity>),
  // Whirlwind overlay pos
  call_target_patch!(0x1e885, 0x0009fcd1, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x1e88f, 0x0009fcc1, super::entity_iso_ypos::<Entity>),
  // Entity culling
  call_target_patch!(0x17642, 0x000a6f14, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x1764c, 0x000a6f04, super::entity_iso_ypos::<Entity>),
  // Charge overlay pos
  call_target_patch!(0x2379f, 0x0009adb7, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x237a9, 0x0009ada7, super::entity_iso_ypos::<Entity>),
  // Perspective charge overlay pos
  call_target_patch!(0x23913, 0x0009abd1, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0x2390c, 0x0009abd2, super::entity_linear_ypos::<Entity>),
  // Entity minimap position
  call_target_patch!(0x28301, 0x00096255, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x28309, 0x00096247, super::entity_iso_ypos::<Entity>),
  // Perspective entity mouse-over text
  call_target_patch!(0x8558a, 0x00038f5a, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0x85592, 0x00038f4c, super::entity_linear_ypos::<Entity>),
  // Entity mouse-over text
  call_target_patch!(0x85601, 0x00038f55, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x85616, 0x00038f3a, super::entity_iso_ypos::<Entity>),
  // Course entity mouse detection
  call_target_patch!(0x8d40d, 0x00031149, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x8d415, 0x0003113b, super::entity_iso_ypos::<Entity>),
  // Animated entity mouse detection refinement
  call_target_patch!(0x8d8b7, 0x00030c9f, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x8d8de, 0x00030c72, super::entity_iso_ypos::<Entity>),
  // Perspective entity draw pos
  call_target_patch!(0xaba34, 0x00012ab0, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0xaba40, 0x00012a9e, super::entity_linear_ypos::<Entity>),
  // Entity draw pos
  call_target_patch!(0xabb57, 0x000129ff, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0xabb61, 0x000129ef, super::entity_iso_ypos::<Entity>),
  // Perspective entity shadow pos
  call_target_patch!(0xb728a, 0x0000725a, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0xb7294, 0x0000724a, super::entity_linear_ypos::<Entity>),
  // Entity shadow pos
  call_target_patch!(0xb72ff, 0x00007257, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0xb730b, 0x00007245, super::entity_iso_ypos::<Entity>),
  // Perspective entity single color pos
  call_target_patch!(0xb7ad7, 0x00006a0d, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0xb7ad0, 0x00006a0e, super::entity_linear_ypos::<Entity>),
  // Entity single color pos
  call_target_patch!(0xb7b34, 0x00006a22, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0xb7b4f, 0x00006a01, super::entity_iso_ypos::<Entity>),
  // Entity spell overlay perspective
  call_target_patch!(0xb815b, 0x00006389, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0xb8154, 0x0000638a, super::entity_linear_ypos::<Entity>),
  // Entity spell overlay
  call_target_patch!(0xb81a7, 0x000063af, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0xb81ba, 0x00006396, super::entity_iso_ypos::<Entity>),
  // Npc mouse over perspective
  call_target_patch!(0xbdad8, 0x00000a0c, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0xbdad1, 0x00000a0d, super::entity_linear_ypos::<Entity>),
  // Npc mouse over
  call_target_patch!(0xbdb25, 0x00000a31, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0xbdb38, 0x00000a18, super::entity_iso_ypos::<Entity>),
];
#[rustfmt::skip]
static D2CLIENT_CALL_PATCHES: [CallPatch; 3] = [
  // Game loop sleep patch
  call_patch!(0x2637, [0xff, 0x15, reloc 0x9c, 0xbf, 0xb6, 0x6f], game_loop_sleep_hook as unsafe extern "stdcall" fn(_)),
  // Draw paused game framerate
  call_patch!(0x9438, [0xff, 0x15, reloc 0xb4, 0x09, 0xbb, 0x6f], draw_game_paused as unsafe extern "stdcall" fn()),
  // Draw game framerate & entity sync
  call_patch!(0x9b5f, [
    0x39, reloc 0x2d, 0x40, 0x0c, 0xbb, 0x6f,
    0x75, 0x2b,
    0xe8, 0xb4, 0x40, 0x08, 0x00,
    0x85, 0xc0,
    0x74, 0x2e,
    0x33, 0xc9,
    0xff, 0x15, reloc 0xb4, 0x09, 0xbb, 0x6f,
    0x8b, 0x0d, reloc 0xc4, 0x09, 0xbb, 0x6f,
    0xa1, reloc 0xdc, 0x09, 0xbb, 0x6f,
    0x41,
    0x40,
    0x89, 0x0d, reloc 0xc4, 0x09, 0xbb, 0x6f,
    0xa3, reloc 0xdc, 0x09, 0xbb, 0x6f,
    0xeb, 0x0c,
    0x39, 0x6c, 0x24, 0x14,
    0x74, 0x06,
    0xff, 0x05, reloc 0xe4, 0x09, 0xbb, 0x6f,
  ], draw_game::<Entity> as unsafe extern "stdcall" fn()),
];
#[rustfmt::skip]
static D2WIN_CALL_PATCHES: [CallPatch; 2] = [
  // Draw menu framerate
  call_patch!(0xebe4, [
    0x8d, 0x4c, 0x24, 0x14,
    0x51,
    0xff, 0x15, reloc 0xc8, 0xd1, 0x8b, 0x6f,
    0x8d, 0x54, 0x24, 0x1c,
    0x52,
    0xff, 0x15, reloc 0x80, 0xd1, 0x8b, 0x6f,
    0x8b, 0x74, 0x24, 0x14,
    0x8b, 0x7c, 0x24, 0x18,
    0x8b, 0xc6,
    0x8b, 0xcf,
    0x2b, 0xc3,
    0x6a, 0x00,
    0x1b, 0xcd,
    0x6a, 0x19,
    0x51,
    0x50,
    0xe8, 0x4b, 0x61, 0x00, 0x00,
    0x3b, 0x54, 0x24, 0x20,
    0x7c, 0x27,
    0x7f, 0x06,
    0x3b, 0x44, 0x24, 0x1c,
    0x76, 0x1f,
    0x8b, 0x44, 0x24, 0x44,
    0x8b, 0xde,
    0x85, 0xc0,
    0x8b, 0xef,
    0x74, 0x0e,
    0x8b, 0x54, 0x24, 0x10,
    0x8b, 0xca,
    0x42,
    0x51,
    0x89, 0x54, 0x24, 0x14,
    0xff, 0xd0,
    0xe8, 0x4e, 0x06, 0x00, 0x00,
  ], draw_menu_109d_asm_stub as unsafe extern "C" fn()),
  // Menu char frame rate
  call_patch!(0x1b6a, [
    0x8b, 0x4e, 0x10,
    0x8b, 0x46, 0x08,
    0x8b, 0x56, 0x0c,
    0x03, 0xc1,
  ], update_menu_char_frame_109d_asm_stub as unsafe extern "C" fn()),
];

impl super::HookManager {
  pub unsafe fn hook_v109d(&mut self) -> Result<(), ()> {
    self.modules.push(Module::new(D2CLIENT_DLL)?);
    self.modules.push(Module::new(D2GAME_DLL)?);
    self.modules.push(Module::new(D2GFX_DLL)?);
    self.modules.push(Module::new(D2WIN_DLL)?);

    let d2client = D2ClientAccessor(self.modules[0].0);
    let d2game = D2GameAccessor(self.modules[1].0);
    let d2gfx = D2GfxAccessor(self.modules[2].0);
    let d2win = D2WinAccessor(self.modules[3].0);

    self.accessor.active_entity_tables = d2client.active_entity_tables().cast();
    self.accessor.client_fps_frame_count = d2client.client_fps_frame_count();
    self.accessor.client_total_frame_count = d2client.client_frame_count();
    self.accessor.client_update_count = d2client.client_update_count();
    self.accessor.draw_game_fn = d2client.draw_game_fn();
    self.accessor.draw_menu = d2win.draw_menu();
    self.accessor.env_bubbles = d2client.env_bubbles();
    self.accessor.env_splashes = d2client.env_splashes();
    self.accessor.game_type = d2client.game_type();
    self.accessor.hwnd = d2gfx.hwnd();
    self.accessor.player = d2client.player().cast();
    self.accessor.render_in_perspective = d2gfx.render_in_perspective();
    self.accessor.server_update_time = d2game.server_update_time();

    apply_patches!(
      self,
      (
        "d2client.dll",
        d2client.0 as usize,
        0x6faa0000,
        &D2CLIENT_TARGET_PATCHES
      ),
      (
        "d2client.dll",
        d2client.0 as usize,
        0x6faa0000,
        &D2CLIENT_CALL_PATCHES
      ),
      (
        "d2win.dll",
        d2win.0 as usize,
        0x648a0000,
        &D2WIN_CALL_PATCHES
      ),
    )
  }
}

impl super::Entity for Entity {
  fn unit_id(&self) -> UnitId {
    UnitId::new(self.kind, self.id)
  }

  fn has_room(&self) -> bool {
    self.has_room()
  }

  fn linear_pos(&self) -> LinearPos<FixedU16> {
    self
      .pos(
        |pos| {
          LinearPos::new(
            FixedU16::from_wrapping(pos.linear_pos.x),
            FixedU16::from_wrapping(pos.linear_pos.y),
          )
        },
        |pos| pos.linear_pos,
      )
      .unwrap()
  }

  fn iso_pos(&self) -> IsoPos<i32> {
    self.pos(|pos| pos.iso_pos, |pos| pos.iso_pos).unwrap()
  }

  unsafe fn tracker_pos(&self) -> (LinearPos<FixedU16>, LinearPos<u16>) {
    self.pos.d.map_or_else(Default::default, |pos| {
      (pos.as_ref().linear_pos, pos.as_ref().target_pos[0])
    })
  }
}

impl super::DyPos for DyPos {
  type Entity = Entity;
  fn entity(&self) -> NonNull<Self::Entity> {
    self.entity
  }
  fn linear_pos(&self) -> LinearPos<FixedU16> {
    self.linear_pos
  }
}

global_asm! {
  ".global _draw_menu_109d_asm_stub",
  "_draw_menu_109d_asm_stub:",
  "mov ecx, [esp+0x48]",
  "lea edx, [esp+0x18]",
  "call {}",
  "ret",
  sym super::draw_menu_with_sleep,
}
extern "C" {
  pub fn draw_menu_109d_asm_stub();
}

global_asm! {
  ".global _update_menu_char_frame_109d_asm_stub",
  "_update_menu_char_frame_109d_asm_stub:",
  "mov ecx, [esi+0x10]",
  "lea edx, [esi+0x08]",
  "call {}",
  "mov edx, [esi+0x0c]",
  "ret",
  sym super::update_menu_char_frame,
}
extern "C" {
  pub fn update_menu_char_frame_109d_asm_stub();
}
