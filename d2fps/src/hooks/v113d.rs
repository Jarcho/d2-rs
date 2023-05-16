use crate::{
  hooks::{
    draw_game, draw_game_paused, game_loop_sleep_hook, D2CLIENT_IDX, D2COMMON_IDX, D2GAME_IDX,
    D2GFX_IDX, D2WIN_IDX,
  },
  patch::{CallPatch, CallTargetPatch},
};
use d2interface::v113d::{
  D2ClientAccessor, D2CommonAccessor, D2GameAccessor, D2GfxAccessor, D2WinAccessor, DyPos, Entity,
};

#[rustfmt::skip]
static D2CLIENT_TARGET_PATCHES: [CallTargetPatch; 53] = [
  // Perspective entity mouse-over text
  call_target_patch!(0x1a781, 0xffff1c19, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0x1a789, 0xffff1c3b, super::entity_linear_ypos::<Entity>),
  // Entity mouse-over text
  call_target_patch!(0x1a804, 0xffff1bae, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x1a81a, 0xffff1bb6, super::entity_iso_ypos::<Entity>),
  // Create entity light
  call_target_patch!(0x2260b, 0xfffe9d8f, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0x2261a, 0xfffe9daa, super::entity_linear_ypos::<Entity>),
  // Apply entity light
  call_target_patch!(0x23501, 0xfffe8e99, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0x23509, 0xfffe8ebb, super::entity_linear_ypos::<Entity>),
  // Lighting position
  call_target_patch!(0x237c2, 0xfffe8a7c, super::dypos_linear_whole_xpos::<DyPos>),
  call_target_patch!(0x237ee, 0xfffe8990, super::dypos_linear_whole_ypos::<DyPos>),
  // Viewport position
  call_target_patch!(0x452ba, 0xfffc70f8, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x452d1, 0xfffc70ff, super::entity_iso_ypos::<Entity>),
  // Entity shift
  call_target_patch!(0x5be03, 0xfffb05af, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x5be0b, 0xfffb05c5, super::entity_iso_ypos::<Entity>),
  // Entity spell overlay perspective
  call_target_patch!(0x5ee23, 0xfffad577, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0x5ee1c, 0xfffad5a8, super::entity_linear_ypos::<Entity>),
  // Entity spell overlay
  call_target_patch!(0x5ee7c, 0xfffad536, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x5ee91, 0xfffad53f, super::entity_iso_ypos::<Entity>),
  // Perspective entity single color pos
  call_target_patch!(0x5f642, 0xfffacd58, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0x5f63b, 0xfffacd89, super::entity_linear_ypos::<Entity>),
  // Entity single color pos
  call_target_patch!(0x5f6d6, 0xfffaccdc, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x5f702, 0xfffaccce, super::entity_iso_ypos::<Entity>),
  // Perspective entity shadow pos
  call_target_patch!(0x5fa3e, 0xfffac95c, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0x5fa48, 0xfffac97c, super::entity_linear_ypos::<Entity>),
  // Entity shadow pos
  call_target_patch!(0x5fad0, 0xfffac8e2, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x5fadc, 0xfffac8f4, super::entity_iso_ypos::<Entity>),
  // Animated entity mouse detection refinement
  call_target_patch!(0x62e2f, 0xfffa9583, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x62e54, 0xfffa957c, super::entity_iso_ypos::<Entity>),
  // Course entity mouse detection
  call_target_patch!(0x632f5, 0xfffa90bd, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x632fd, 0xfffa90d3, super::entity_iso_ypos::<Entity>),
  // Entity minimap position
  call_target_patch!(0x72eac, 0xfff99506, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x72eb4, 0xfff9951c, super::entity_iso_ypos::<Entity>),
  // Charge effect pos
  call_target_patch!(0x817ee, 0xfff8abc4, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x817f8, 0xfff8abd8, super::entity_iso_ypos::<Entity>),
  // Perspective charge effect pos
  call_target_patch!(0x81912, 0xfff8aa88, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0x8190b, 0xfff8aab9, super::entity_linear_ypos::<Entity>),
  // Npc mouse over perspective
  call_target_patch!(0xad335, 0xfff5f065, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0xad32e, 0xfff5f096, super::entity_linear_ypos::<Entity>),
  // Npc mouse over
  call_target_patch!(0xad390, 0xfff5f022, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0xad3a5, 0xfff5f02b, super::entity_iso_ypos::<Entity>),
  // Entity culling
  call_target_patch!(0xb3702, 0xfff58cb0, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0xb370c, 0xfff58cc4, super::entity_iso_ypos::<Entity>),
  // Perspective entity draw pos
  call_target_patch!(0xb4be3, 0xfff577b7, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0xb4beb, 0xfff577d9, super::entity_linear_ypos::<Entity>),
  // Entity draw pos
  call_target_patch!(0xb4c99, 0xfff57719, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0xb4ca3, 0xfff5772d, super::entity_iso_ypos::<Entity>),
  // Summit background position
  call_target_patch!(0xb564a, 0xfff56d68, super::entity_iso_xpos::<Entity>),
  // Perspective viewport position
  call_target_patch!(0xb605a, 0xfff56340, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0xb6062, 0xfff56362, super::entity_linear_ypos::<Entity>),
  // Perspective whirlwind effect pos
  call_target_patch!(0xc587d, 0xfff46b1d, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0xc5876, 0xfff46b4e, super::entity_linear_ypos::<Entity>),
  // Whirlwind effect pos
  call_target_patch!(0xc58b1, 0xfff46b01, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0xc58bb, 0xfff46b15, super::entity_iso_ypos::<Entity>),
];
#[rustfmt::skip]
static D2CLIENT_CALL_PATCHES: [CallPatch; 3] = [
  // Game loop sleep patch
  call_patch!(0x2770c, [
    0xa1, reloc 0x00, 0x7b, 0xba, 0x6f,
    0x85, 0xc0,
    0x75, 0x17,
    0xa1, reloc 0xdc, 0xd1, 0xbc, 0x6f,
    0x83, 0xf8, 0x06,
    0x74, 0x0d,
    0x83, 0xf8, 0x08,
    0x74, 0x08,
    0x6a, 0x0a,
    0xff, 0x15, reloc 0x6c, 0xff, 0xb7, 0x6f,
  ], game_loop_sleep_hook as unsafe extern "stdcall" fn()),
  // Draw paused game framerate
  call_patch!(0x45c15, [0xff, 0x15, reloc 0x44, 0x87, 0xbb, 0x6f], draw_game_paused as unsafe extern "stdcall" fn()),
  // Draw game framerate & entity sync
  call_patch!(0x45ea1, [
    0x39, 0x1d, reloc 0xf0, 0x46, 0xbd, 0x6f,
    0x75, 0x35,
    0xa1, reloc 0x50, 0xd0, 0xbc, 0x6f,
    0x3b, 0xc3,
    0x74, 0x38,
    0x50,
    0xe8, 0xf0, 0x63, 0xfc, 0xff,
    0x3b, 0xc3,
    0x74, 0x2e,
    0x33, 0xc9,
    0xff, 0x15, reloc 0x44, 0x87, 0xbb, 0x6f,
    0x8b, 0x0d, reloc 0x54, 0x87, 0xbb, 0x6f,
    0xa1, reloc 0x6c, 0x87, 0xbb, 0x6f,
    0x41,
    0x40,
    0x89, 0x0d, reloc 0x54, 0x87, 0xbb, 0x6f,
    0xa3, reloc 0x6c, 0x87, 0xbb, 0x6f,
    0xeb, 0x0c,
    0x39, 0x5c, 0x24, 0x10,
    0x74, 0x06,
    0xff, 0x05, reloc 0x74, 0x87, 0xbb, 0x6f,
  ], draw_game::<Entity> as unsafe extern "stdcall" fn()),
];
#[rustfmt::skip]
static D2WIN_CALL_PATCHES: [CallPatch; 2] = [
  // Draw menu framerate
  call_patch!(0xed4c, [
    0xff, 0xd5,
    0x8b, 0xf0,
    0x2b, 0xf3,
    0xff, 0xd5,
    0x81, 0xfe, 0xe8, 0x03, 0x00, 0x00,
    0x8b, 0xd8,
    0x76, 0x05,
    0xbe, 0xe8, 0x03, 0x00, 0x00,
    0x2b, 0xfe,
    0x85, 0xff,
    0x7f, 0x26,
    0x83, 0xc7, 0x28,
    0x81, 0xff, 0x18, 0xfc, 0xff, 0xff,
    0x7d, 0x02,
    0x33, 0xff,
    0x8b, 0x44, 0x24, 0x34,
    0x85, 0xc0,
    0x74, 0x0c,
    0x8b, 0x74, 0x24, 0x10,
    0x56,
    0xff, 0xd0,
    0x46,
    0x89, 0x74, 0x24, 0x10,
    0xe8, 0xa1, 0xfd, 0xff, 0xff,
  ], crate::hooks::v110::draw_menu_110_asm_stub as unsafe extern "C" fn()),
  // Menu char frame rate
  call_patch!(0xc226, [
    0x8b, 0x43, 0x10,
    0x8b, 0x73, 0x08,
    0x8b, 0x4b, 0x0c,
    0x03, 0xf0,
    0x8b, 0xc6,
  ], super::v112::update_menu_char_frame_112_asm_stub as unsafe extern "C" fn()),
];
#[rustfmt::skip]
static D2COMMON_TARGET_PATCHES: [CallTargetPatch; 1] = [
  call_target_patchc!(0x6f368, 0xfffffa94, crate::hooks::v112::intercept_teleport_112_asm_stub),
];

impl super::HookManager {
  pub unsafe fn hook_v113d(&mut self) -> Result<(), ()> {
    let modules = self.load_dlls()?;
    let d2client = D2ClientAccessor(modules[D2CLIENT_IDX].0);
    let d2common = D2CommonAccessor(modules[D2COMMON_IDX].0);
    let d2game = D2GameAccessor(modules[D2GAME_IDX].0);
    let d2gfx = D2GfxAccessor(modules[D2GFX_IDX].0);
    let d2win = D2WinAccessor(modules[D2WIN_IDX].0);

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
    self.accessor.apply_pos_change = d2common.apply_pos_change();

    apply_patches!(
      self,
      (
        "d2client.dll",
        d2client.0 as usize,
        0x6fab0000,
        &D2CLIENT_TARGET_PATCHES
      ),
      (
        "d2client.dll",
        d2client.0 as usize,
        0x6fab0000,
        &D2CLIENT_CALL_PATCHES
      ),
      (
        "d2win.dll",
        d2win.0 as usize,
        0x6f8e0000,
        &D2WIN_CALL_PATCHES
      ),
      (
        "d2common.dll",
        d2common.0 as usize,
        0x6fd50000,
        &D2COMMON_TARGET_PATCHES
      ),
    )
  }
}
