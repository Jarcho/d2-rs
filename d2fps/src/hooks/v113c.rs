use crate::{
  hooks::{
    draw_game, draw_game_paused, game_loop_sleep_hook, D2CLIENT_IDX, D2COMMON_IDX, D2GAME_IDX,
    D2GFX_IDX, D2WIN_IDX,
  },
  patch::{CallPatch, CallTargetPatch},
};
use d2interface::v113c::{
  D2ClientAccessor, D2CommonAccessor, D2GameAccessor, D2GfxAccessor, D2WinAccessor, DyPos, Entity,
};

#[rustfmt::skip]
static D2CLIENT_TARGET_PATCHES: [CallTargetPatch; 53] = [
  // Charge effect pos
  call_target_patch!(0x2b982, 0xfffe084c, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x2b98c, 0xfffe0866, super::entity_iso_ypos::<Entity>),
  // Perspective charge effect pos
  call_target_patch!(0x2baa2, 0xfffe0708, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0x2ba9b, 0xfffe0793, super::entity_linear_ypos::<Entity>),
  // Entity shift
  call_target_patch!(0x3f8a3, 0xfffcc92b, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x3f8ab, 0xfffcc947, super::entity_iso_ypos::<Entity>),
  // Viewport position
  call_target_patch!(0x4426a, 0xfffc7f64, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x44281, 0xfffc7f71, super::entity_iso_ypos::<Entity>),
  // Entity minimap position
  call_target_patch!(0x61a4c, 0xfffaa782, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x61a54, 0xfffaa79e, super::entity_iso_ypos::<Entity>),
  // Perspective entity draw pos
  call_target_patch!(0x6675d, 0xfffa5a4d, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0x66765, 0xfffa5ac9, super::entity_linear_ypos::<Entity>),
  // Entity draw pos
  call_target_patch!(0x66813, 0xfffa59bb, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x6681d, 0xfffa59d5, super::entity_iso_ypos::<Entity>),
  // Entity spell overlay perspective
  call_target_patch!(0x6af43, 0xfffa1267, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0x6af3c, 0xfffa12f2, super::entity_linear_ypos::<Entity>),
  // Entity spell overlay
  call_target_patch!(0x6af9c, 0xfffa1232, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x6afb1, 0xfffa1241, super::entity_iso_ypos::<Entity>),
  // Perspective entity single color pos
  call_target_patch!(0x6b762, 0xfffa0a48, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0x6b75b, 0xfffa0ad3, super::entity_linear_ypos::<Entity>),
  // Entity single color pos
  call_target_patch!(0x6b7f6, 0xfffa09d8, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x6b822, 0xfffa09d0, super::entity_iso_ypos::<Entity>),
  // Perspective entity shadow pos
  call_target_patch!(0x6bb5e, 0xfffa064c, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0x6bb68, 0xfffa06c6, super::entity_linear_ypos::<Entity>),
  // Entity shadow pos
  call_target_patch!(0x6bbf0, 0xfffa05de, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x6bbfc, 0xfffa05f6, super::entity_iso_ypos::<Entity>),
  // Npc mouse over perspective
  call_target_patch!(0x6e6a5, 0xfff9db05, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0x6e69e, 0xfff9db90, super::entity_linear_ypos::<Entity>),
  // Npc mouse over
  call_target_patch!(0x6e700, 0xfff9dace, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x6e715, 0xfff9dadd, super::entity_iso_ypos::<Entity>),
  // Entity culling
  call_target_patch!(0x7a892, 0xfff9193c, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x7a89c, 0xfff91956, super::entity_iso_ypos::<Entity>),
  // Summit background position
  call_target_patch!(0x8a73a, 0xfff81a94, super::entity_iso_xpos::<Entity>),
  // Perspective viewport position
  call_target_patch!(0x8b14a, 0xfff81060, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0x8b152, 0xfff810dc, super::entity_linear_ypos::<Entity>),
  // Animated entity mouse detection refinement
  call_target_patch!(0xa67ef, 0xfff659df, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0xa6814, 0xfff659de, super::entity_iso_ypos::<Entity>),
  // Course entity mouse detection
  call_target_patch!(0xa6a55, 0xfff65779, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0xa6a5d, 0xfff65795, super::entity_iso_ypos::<Entity>),
  // Create entity light
  call_target_patch!(0xa8c9b, 0xfff6350f, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0xa8caa, 0xfff63584, super::entity_linear_ypos::<Entity>),
  // Apply entity light
  call_target_patch!(0xa9b91, 0xfff62619, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0xa9b99, 0xfff62695, super::entity_linear_ypos::<Entity>),
  // Lighting position
  call_target_patch!(0xa9e52, 0xfff62568, super::dypos_linear_whole_xpos::<DyPos>),
  call_target_patch!(0xa9e7e, 0xfff6246a, super::dypos_linear_whole_ypos::<DyPos>),
  // Perspective entity mouse-over text
  call_target_patch!(0xc0d81, 0xfff4b429, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0xc0d89, 0xfff4b4a5, super::entity_linear_ypos::<Entity>),
  // Entity mouse-over text
  call_target_patch!(0xc0e04, 0xfff4b3ca, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0xc0e1a, 0xfff4b3d8, super::entity_iso_ypos::<Entity>),
  // Perspective whirlwind effect pos
  call_target_patch!(0xc5eaa, 0xfff46300, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0xc5ea3, 0xfff4638b, super::entity_linear_ypos::<Entity>),
  // Whirlwind effect pos
  call_target_patch!(0xc5ede, 0xfff462f0, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0xc5ee8, 0xfff4630a, super::entity_iso_ypos::<Entity>),
];
#[rustfmt::skip]
static D2CLIENT_CALL_PATCHES: [CallPatch; 3] = [
  // Game loop sleep patch
  call_patch!(0x3cb96, [0xff, 0x15, reloc 0xa0, 0xef, 0xb7, 0x6f], game_loop_sleep_hook as unsafe extern "stdcall" fn(_)),
  // Draw paused game framerate
  call_patch!(0x44bc5, [0xff, 0x15, reloc 0xe4, 0x97, 0xbc, 0x6f], draw_game_paused as unsafe extern "stdcall" fn()),
  // Draw game framerate & entity sync
  call_patch!(0x44e51, [
    0x39, 0x1d, reloc 0x90, 0x34, 0xbd, 0x6f,
    0x75, 0x35,
    0xa1, reloc 0xfc, 0xbb, 0xbc, 0x6f,
    0x3b, 0xc3,
    0x74, 0x38,
    0x50,
    0xe8, 0x88, 0x73, 0xfc, 0xff,
    0x3b, 0xc3,
    0x74, 0x2e,
    0x33, 0xc9,
    0xff, 0x15, reloc 0xe4, 0x97, 0xbc, 0x6f,
    0x8b, 0x0d, reloc 0xf4, 0x97, 0xbc, 0x6f,
    0xa1, reloc 0x0c, 0x98, 0xbc, 0x6f,
    0x41,
    0x40,
    0x89, 0x0d, reloc 0xf4, 0x97, 0xbc, 0x6f,
    0xa3, reloc 0x0c, 0x98, 0xbc, 0x6f,
    0xeb, 0x0c,
    0x39, 0x5c, 0x24, 0x10,
    0x74, 0x06,
    0xff, 0x05, reloc 0x14, 0x98, 0xbc, 0x6f,
  ], draw_game::<Entity> as unsafe extern "stdcall" fn()),
];
#[rustfmt::skip]
static D2WIN_CALL_PATCHES: [CallPatch; 2] = [
  // Draw menu framerate
  call_patch!(0x189fc, [
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
  call_patch!(0xe836, [
    0x8b, 0x43, 0x10,
    0x8b, 0x73, 0x08,
    0x8b, 0x4b, 0x0c,
    0x03, 0xf0,
    0x8b, 0xc6,
  ], crate::hooks::v112::update_menu_char_frame_112_asm_stub as unsafe extern "C" fn()),
];
#[rustfmt::skip]
static D2COMMON_TARGET_PATCHES: [CallTargetPatch; 1] = [
  call_target_patchc!(0xe0b8, 0xfffff984, crate::hooks::v112::intercept_teleport_112_asm_stub),
];

impl super::HookManager {
  pub unsafe fn hook_v113c(&mut self) -> Result<(), ()> {
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
