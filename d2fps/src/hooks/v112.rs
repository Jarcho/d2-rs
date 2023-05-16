use crate::{
  hooks::{
    draw_game, draw_game_paused, game_loop_sleep_hook, D2CLIENT_IDX, D2COMMON_IDX, D2GAME_IDX,
    D2GFX_IDX, D2WIN_IDX,
  },
  patch::{CallPatch, CallTargetPatch},
};
use core::arch::global_asm;
use d2interface::v112::{
  D2ClientAccessor, D2CommonAccessor, D2GameAccessor, D2GfxAccessor, D2WinAccessor, DyPos, Entity,
};

#[rustfmt::skip]
static D2CLIENT_TARGET_PATCHES: [CallTargetPatch; 53] = [
  // Animated entity mouse detection refinement
  call_target_patch!(0x1ff1f, 0xfffec2e5, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x1ff44, 0xfffec2cc, super::entity_iso_ypos::<Entity>),
  // Course entity mouse detection
  call_target_patch!(0x20185, 0xfffec07f, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x2018d, 0xfffec083, super::entity_iso_ypos::<Entity>),
  // Entity minimap position
  call_target_patch!(0x3f98c, 0xfffcc878, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x3f994, 0xfffcc87c, super::entity_iso_ypos::<Entity>),
  // Entity shift
  call_target_patch!(0x5db53, 0xfffae6b1, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x5db5b, 0xfffae6b5, super::entity_iso_ypos::<Entity>),
  // Create entity light
  call_target_patch!(0x5eb6b, 0xfffad693, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0x5eb7a, 0xfffad6b4, super::entity_linear_ypos::<Entity>),
  // Apply entity light
  call_target_patch!(0x5fa61, 0xfffac79d, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0x5fa69, 0xfffac7c5, super::entity_linear_ypos::<Entity>),
  // Lighting position
  call_target_patch!(0x5fd22, 0xfffac5ae, super::dypos_linear_whole_xpos::<DyPos>),
  call_target_patch!(0x5fd4e, 0xfffac510, super::dypos_linear_whole_ypos::<DyPos>),
  // Entity culling
  call_target_patch!(0x6de92, 0xfff9e372, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x6de9c, 0xfff9e374, super::entity_iso_ypos::<Entity>),
  // Viewport position
  call_target_patch!(0x7ca4a, 0xfff8f7ba, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x7ca61, 0xfff8f7af, super::entity_iso_ypos::<Entity>),
  // Charge effect pos
  call_target_patch!(0x8747b, 0xfff84d89, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x87485, 0xfff84d8b, super::entity_iso_ypos::<Entity>),
  // Perspective charge effect pos
  call_target_patch!(0x875a2, 0xfff84c5c, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0x8759b, 0xfff84c93, super::entity_linear_ypos::<Entity>),
  // Perspective entity mouse-over text
  call_target_patch!(0x8d4f1, 0xfff7ed0d, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0x8d4f9, 0xfff7ed35, super::entity_linear_ypos::<Entity>),
  // Entity mouse-over text
  call_target_patch!(0x8d574, 0xfff7ec90, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x8d58a, 0xfff7ec86, super::entity_iso_ypos::<Entity>),
  // Entity spell overlay perspective
  call_target_patch!(0x92d23, 0xfff794db, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0x92d1c, 0xfff79512, super::entity_linear_ypos::<Entity>),
  // Entity spell overlay
  call_target_patch!(0x92d7c, 0xfff79488, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x92d91, 0xfff7947f, super::entity_iso_ypos::<Entity>),
  // Perspective entity single color pos
  call_target_patch!(0x93542, 0xfff78cbc, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0x9353b, 0xfff78cf3, super::entity_linear_ypos::<Entity>),
  // Entity single color pos
  call_target_patch!(0x935d6, 0xfff78c2e, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x93602, 0xfff78c0e, super::entity_iso_ypos::<Entity>),
  // Perspective entity shadow pos
  call_target_patch!(0x9392e, 0xfff788d0, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0x93938, 0xfff788f6, super::entity_linear_ypos::<Entity>),
  // Entity shadow pos
  call_target_patch!(0x939c0, 0xfff78844, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0x939cc, 0xfff78844, super::entity_iso_ypos::<Entity>),
  // Npc mouse over perspective
  call_target_patch!(0xb7d85, 0xfff54479, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0xb7d7e, 0xfff544b0, super::entity_linear_ypos::<Entity>),
  // Npc mouse over
  call_target_patch!(0xb7de0, 0xfff54424, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0xb7df5, 0xfff5441b, super::entity_iso_ypos::<Entity>),
  // Summit background position
  call_target_patch!(0xbab9a, 0xfff5166a, super::entity_iso_xpos::<Entity>),
  // Perspective viewport position
  call_target_patch!(0xbb66a, 0xfff50b94, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0xbb672, 0xfff50bbc, super::entity_linear_ypos::<Entity>),
  // Perspective entity draw pos
  call_target_patch!(0xbbabd, 0xfff50741, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0xbbac5, 0xfff50769, super::entity_linear_ypos::<Entity>),
  // Entity draw pos
  call_target_patch!(0xbbb73, 0xfff50691, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0xbbb7d, 0xfff50693, super::entity_iso_ypos::<Entity>),
  // Perspective whirlwind effect pos
  call_target_patch!(0xc4920, 0xfff478de, super::entity_linear_xpos::<Entity>),
  call_target_patch!(0xc4919, 0xfff47915, super::entity_linear_ypos::<Entity>),
  // Whirlwind effect pos
  call_target_patch!(0xc4954, 0xfff478b0, super::entity_iso_xpos::<Entity>),
  call_target_patch!(0xc495e, 0xfff478b2, super::entity_iso_ypos::<Entity>),
];
#[rustfmt::skip]
static D2CLIENT_CALL_PATCHES: [CallPatch; 3] = [
  // Game loop sleep patch
  call_patch!(0x6cfbc, [
    0xa1, reloc 0x80, 0x3a, 0xb9, 0x6f,
    0x85, 0xc0,
    0x75, 0x17,
    0xa1, reloc 0xf8, 0xbf, 0xbc, 0x6f,
    0x83, 0xf8, 0x06,
    0x74, 0x0d,
    0x83, 0xf8, 0x08,
    0x74, 0x08,
    0x6a, 0x0a,
    0xff, 0x15, reloc 0x7c, 0xef, 0xb7, 0x6f,
  ], game_loop_sleep_hook as unsafe extern "stdcall" fn()),
  // Draw paused game framerate
  call_patch!(0x7cf55, [0xff, 0x15, reloc 0x9c, 0x32, 0xbb, 0x6f], draw_game_paused as unsafe extern "stdcall" fn()),
  // Draw game framerate & entity sync
  call_patch!(0x7d1e1, [
    0x39, 0x1d, reloc 0x28, 0x34, 0xbd, 0x6f,
    0x75, 0x35,
    0xa1, reloc 0xd0, 0xc3, 0xbc, 0x6f,
    0x3b, 0xc3,
    0x74, 0x38,
    0x50,
    0xe8, 0xc2, 0xef, 0xf8, 0xff,
    0x3b, 0xc3,
    0x74, 0x2e,
    0x33, 0xc9,
    0xff, 0x15, reloc 0x9c, 0x32, 0xbb, 0x6f,
    0x8b, 0x0d, reloc 0xac, 0x32, 0xbb, 0x6f,
    0xa1, reloc 0xc4, 0x32, 0xbb, 0x6f,
    0x41,
    0x40,
    0x89, 0x0d, reloc 0xac, 0x32, 0xbb, 0x6f,
    0xa3, reloc 0xc4, 0x32, 0xbb, 0x6f,
    0xeb, 0x0c,
    0x39, 0x5c, 0x24, 0x10,
    0x74, 0x06,
    0xff, 0x05, reloc 0xcc, 0x32, 0xbb, 0x6f,
  ], draw_game::<Entity> as unsafe extern "stdcall" fn()),
];
#[rustfmt::skip]
static D2WIN_CALL_PATCHES: [CallPatch; 2] = [
  // Draw menu framerate
  call_patch!(0xd92c, [
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
  call_patch!(0x16386, [
    0x8b, 0x43, 0x10,
    0x8b, 0x73, 0x08,
    0x8b, 0x4b, 0x0c,
    0x03, 0xf0,
    0x8b, 0xc6,
  ], update_menu_char_frame_112_asm_stub as unsafe extern "C" fn()),
];
#[rustfmt::skip]
static D2COMMON_TARGET_PATCHES: [CallTargetPatch; 1] = [
  call_target_patchc!(0x4b468, 0xfffffa94, intercept_teleport_112_asm_stub),
];

impl super::HookManager {
  pub unsafe fn hook_v112(&mut self) -> Result<(), ()> {
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

global_asm! {
  ".global _update_menu_char_frame_112_asm_stub",
  "_update_menu_char_frame_112_asm_stub:",
  "mov ecx, [ebx+0x10]",
  "lea edx, [ebx+0x08]",
  "call {}",
  "mov ecx, [ebx+0x0c]",
  "mov esi, eax",
  "ret",
  sym super::update_menu_char_frame,
}
extern "C" {
  pub fn update_menu_char_frame_112_asm_stub();
}

global_asm! {
  ".global _intercept_teleport_112_asm_stub",
  "_intercept_teleport_112_asm_stub:",
  "mov ecx, [esi+0x30]",
  "mov edx, [esp+0x8]",
  "push edx",
  "mov edx, [esp+0x8]",
  "call {}",
  "jmp eax",
  sym super::intercept_teleport::<Entity>,
}
extern "C" {
  pub fn intercept_teleport_112_asm_stub();
}
