use core::arch::global_asm;
use d2interface::v114d::{DyPos, Entity, GameAccessor};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;

impl super::HookManager {
  pub unsafe fn hook_v114d(&mut self) -> Result<(), ()> {
    let module = GetModuleHandleW(super::GAME_EXE);
    if module == 0 {
      return Err(());
    }
    let game = GameAccessor(module);
    self.accessor.active_entity_tables = game.active_entity_tables().cast();
    self.accessor.client_fps_frame_count = game.client_fps_frame_count();
    self.accessor.client_total_frame_count = game.client_frame_count();
    self.accessor.client_update_count = game.client_update_count();
    self.accessor.draw_game_fn = game.draw_game_fn();
    self.accessor.draw_menu = game.draw_menu();
    self.accessor.env_bubbles = game.env_bubbles();
    self.accessor.env_splashes = game.env_splashes();
    self.accessor.game_type = game.game_type();
    self.accessor.hwnd = game.hwnd();
    self.accessor.player = game.player().cast();
    self.accessor.render_in_perspective = game.render_in_perspective();
    self.accessor.server_update_time = game.server_update_time();

    unsafe {
      apply_patches!(self.patches,
        (module as usize, 0x00400000) => {
          // Viewport position
          (0x4c9cf, 0x001d3c7d, super::entity_iso_xpos::<Entity>),
          (0x4c9e7, 0x001d3cc5, super::entity_iso_ypos::<Entity>),
          // Perspective entity mouse-over text
          (0x54f7d, 0x001cb42f, super::entity_linear_xpos::<Entity>),
          (0x54f85, 0x001cb487, super::entity_linear_ypos::<Entity>),
          // Entity mouse-over text
          (0x54ff6, 0x001cb656, super::entity_iso_xpos::<Entity>),
          (0x5500b, 0x001cb6a1, super::entity_iso_ypos::<Entity>),
          // Entity minimap position
          (0x5a8b6, 0x001c5d96, super::entity_iso_xpos::<Entity>),
          (0x5a8be, 0x001c5dee, super::entity_iso_ypos::<Entity>),
          // Entity shift
          (0x5b48a, 0x001c51c2, super::entity_iso_xpos::<Entity>),
          (0x5b492, 0x001c521a, super::entity_iso_ypos::<Entity>),
          // Animated entity mouse detection refinement
          (0x6414b, 0x001bc501, super::entity_iso_xpos::<Entity>),
          (0x6416d, 0x001bc53f, super::entity_iso_ypos::<Entity>),
          // Course entity mouse detection
          (0x669d3, 0x001b9c79, super::entity_iso_xpos::<Entity>),
          (0x669db, 0x001b9cd1, super::entity_iso_ypos::<Entity>),
          // Entity spell overlay perspective
          (0x6e37a, 0x001b2032, super::entity_linear_xpos::<Entity>),
          (0x6e373, 0x001b2099, super::entity_linear_ypos::<Entity>),
          // Entity spell overlay
          (0x6e3d5, 0x001b2277, super::entity_iso_xpos::<Entity>),
          (0x6e3e7, 0x001b22c5, super::entity_iso_ypos::<Entity>),
          // Perspective entity shadow pos
          (0x716ce, 0x001aecde, super::entity_linear_xpos::<Entity>),
          (0x716d8, 0x001aed34, super::entity_linear_ypos::<Entity>),
          // Entity shadow pos
          (0x71738, 0x001aef14, super::entity_iso_xpos::<Entity>),
          (0x71743, 0x001aef69, super::entity_iso_ypos::<Entity>),
          // Perspective entity single color pos
          (0x71c19, 0x001ae793, super::entity_linear_xpos::<Entity>),
          (0x71c12, 0x001ae7fa, super::entity_linear_ypos::<Entity>),
          // Entity single color shadow pos
          (0x71c6b, 0x001ae9e1, super::entity_iso_xpos::<Entity>),
          (0x71c85, 0x001aea27, super::entity_iso_ypos::<Entity>),
          // Create entity light
          (0x741da, 0x001ac1d2, super::entity_linear_xpos::<Entity>),
          (0x741e9, 0x001ac223, super::entity_linear_ypos::<Entity>),
          // Apply entity light
          (0x755d2, 0x001aadda, super::entity_linear_xpos::<Entity>),
          (0x755da, 0x001aae32, super::entity_linear_ypos::<Entity>),
          // Lighting position
          (0x75823, 0x001d3099, super::dypos_linear_whole_xpos::<DyPos>),
          (0x7584f, 0x001d30ad, super::dypos_linear_whole_ypos::<DyPos>),
          // Summit background position
          (0x764b1, 0x001aa19b, super::entity_iso_xpos::<Entity>),
          // Perspective viewport position
          (0x76bfc, 0x001a97b0, super::entity_linear_xpos::<Entity>),
          (0x76c04, 0x001a9808, super::entity_linear_ypos::<Entity>),
          // Perspective whirlwind overlay pos
          (0xc90bf, 0x001572ed, super::entity_linear_xpos::<Entity>),
          (0xc90b8, 0x00157354, super::entity_linear_ypos::<Entity>),
          // Whirlwind overlay pos
          (0xc90dd, 0x0015756f, super::entity_iso_xpos::<Entity>),
          (0xc90e6, 0x001575c6, super::entity_iso_ypos::<Entity>),
          // Charge overlay pos
          (0xc99b3, 0x00156c99, super::entity_iso_xpos::<Entity>),
          (0xc99bc, 0x00156cf0, super::entity_iso_ypos::<Entity>),
          // Perspective charge overlay pos
          (0xc9ab9, 0x001568f3, super::entity_linear_xpos::<Entity>),
          (0xc9ab2, 0x0015695a, super::entity_linear_ypos::<Entity>),
          // Npc mouse over perspective
          (0xdb6a7, 0x00144d05, super::entity_linear_xpos::<Entity>),
          (0xdb6a0, 0x00144d6c, super::entity_linear_ypos::<Entity>),
          // Npc mouse over
          (0xdb6ea, 0x00144f62, super::entity_iso_xpos::<Entity>),
          (0xdb6fc, 0x00144fb0, super::entity_iso_ypos::<Entity>),
          // Perspective entity draw pos
          (0xdc83f, 0x00143b6d, super::entity_linear_xpos::<Entity>),
          (0xdc84a, 0x00143bc2, super::entity_linear_ypos::<Entity>),
          // Entity draw pos
          (0xdc8e0, 0x00143d6c, super::entity_iso_xpos::<Entity>),
          (0xdc8e9, 0x00143dc3, super::entity_iso_ypos::<Entity>),
          // Entity culling
          (0xdda43, 0x00142c09, super::entity_iso_xpos::<Entity>),
          (0xdda4c, 0x00142c60, super::entity_iso_ypos::<Entity>),

          // Game loop sleep patch
          (0x51c44, [0xff, 0x15, reloc 0x58, 0xc2, 0x6c, 0x00], super::game_loop_sleep_hook),

          // Draw paused game framerate
          (0x4f017, [0xff, 0x15, reloc 0x84, 0x04, 0x7a, 0x00], super::draw_game_paused),

          // Draw game framerate & entity sync
          (0x4f278, [
            0x39, 0x1d, reloc 0x04, 0x07, 0x7a, 0x00,
            0x75, 0x24,
            0xe8, 0x1b, 0x54, 0x01, 0x00,
            0x85, 0xc0,
            0x74, 0x27,
            0x33, 0xc9,
            0xff, 0x15, reloc 0x84, 0x04, 0x7a, 0x00,
            0xb8, 0x01, 0x00, 0x00, 0x00,
            0x01, 0x05, reloc 0x94, 0x04, 0x7a, 0x00,
            0x01, 0x05, reloc 0xac, 0x04, 0x7a, 0x00,
            0xeb, 0x0c,
            0x39, 0x5d, 0xfc,
            0x74, 0x07,
            0x83, 0x05, reloc 0xb4, 0x04, 0x7a, 0x00, 0x01,
          ], super::draw_game::<Entity>),

          // Draw menu framerate
          (0xfa606, [
            0xff, 0x15, reloc 0x60, 0xc2, 0x6c, 0x00,
            0x8b, 0xf0,
            0x2b, 0xf3,
            0xff, 0x15, reloc 0x60, 0xc2, 0x6c, 0x00,
            0x81, 0xfe, 0xe8, 0x03, 0x00, 0x00,
            0x8b, 0xd8,
            0x76, 0x05,
            0xbe, 0xe8, 0x03, 0x00, 0x00,
            0x2b, 0xfe,
            0x85, 0xff,
            0x7f, 0x25,
            0x83, 0xc7, 0x28,
            0x81, 0xff, 0x18, 0xfc, 0xff, 0xff,
            0x7d, 0x02,
            0x33, 0xff,
            0x8b, 0x45, 0x08,
            0x85, 0xc0,
            0x74, 0x0c,
            0x8b, 0x75, 0xfc,
            0x56,
            0xff, 0xd0,
            0x83, 0xc6, 0x01,
            0x89, 0x75, 0xfc,
            0xe8, 0x90, 0xf2, 0xff, 0xff,
          ], draw_menu_114d_asm_stub),

          // Menu char frame rate
          (0x103ddd, [
            0x8b, 0x47, 0x10,
            0x01, 0x47, 0x08,
            0x8b, 0x47, 0x08,
          ], update_menu_char_frame_114d_asm_stub),
        }
      );
    }
    Ok(())
  }
}

global_asm! {
  ".global _draw_menu_114d_asm_stub",
  "_draw_menu_114d_asm_stub:",
  "mov ecx, [ebp+0x8]",
  "lea edx, [ebp-0x4]",
  "call {}",
  "mov edi, eax",
  "ret",
  sym super::draw_menu,
}
extern "C" {
  pub fn draw_menu_114d_asm_stub();
}

global_asm! {
  ".global _update_menu_char_frame_114d_asm_stub",
  "_update_menu_char_frame_114d_asm_stub:",
  "mov ecx, [edi+0x10]",
  "lea edx, [edi+0x08]",
  "call {}",
  "ret",
  sym super::update_menu_char_frame,
}
extern "C" {
  pub fn update_menu_char_frame_114d_asm_stub();
}
