use crate::{
  hooks::{
    draw_game_paused, game_loop_sleep_hook, Module, D2CLIENT_DLL, D2GAME_DLL, D2GFX_DLL, D2WIN_DLL,
  },
  tracker::UnitId,
};
use core::arch::global_asm;
use d2interface::{
  v110::{D2ClientAccessor, D2GameAccessor, D2GfxAccessor, D2WinAccessor, DyPos, Entity},
  FixedU16, IsoPos, LinearPos,
};

impl super::HookManager {
  pub unsafe fn hook_v110(&mut self) -> Result<(), ()> {
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

    unsafe {
      apply_patches!(self.patches,
        (d2client.0 as usize, 0x6faa0000) => {
          // Create entity light
          (0x4301, 0x000c6657, super::entity_linear_xpos::<Entity>),
          (0x4310, 0x000c6642, super::entity_linear_ypos::<Entity>),
          // Lighting position
          (0x4b16, 0x000c5e0c, super::dypos_linear_whole_xpos::<DyPos>),
          (0x4b42, 0x000c5dda, super::dypos_linear_whole_ypos::<DyPos>),
          // Apply entity light
          (0x4d3d, 0x000c5c1b, super::entity_linear_xpos::<Entity>),
          (0x4d45, 0x000c5c0d, super::entity_linear_ypos::<Entity>),
          // Viewport position
          (0x9680, 0x000c1332, super::entity_iso_xpos::<Entity>),
          (0x969d, 0x000c130f, super::entity_iso_ypos::<Entity>),
          // Entity shift
          (0x159a7, 0x000b500b, super::entity_iso_xpos::<Entity>),
          (0x159af, 0x000b4ffd, super::entity_iso_ypos::<Entity>),
          // Perspective viewport position
          (0x172a0, 0x000b36b8, super::entity_linear_xpos::<Entity>),
          (0x172a8, 0x000b36aa, super::entity_linear_ypos::<Entity>),
          // Summit background position
          (0x17744, 0x000b326e, super::entity_iso_xpos::<Entity>),
          // Entity culling
          (0x18920, 0x000b2092, super::entity_iso_xpos::<Entity>),
          (0x1892a, 0x000b2082, super::entity_iso_ypos::<Entity>),
          // Perspective whirlwind effect pos
          (0x22454, 0x000a8504, super::entity_linear_xpos::<Entity>),
          (0x2244d, 0x000a8505, super::entity_linear_ypos::<Entity>),
          // Whirlwind effect pos
          (0x22474, 0x000a853e, super::entity_iso_xpos::<Entity>),
          (0x2247e, 0x000a852e, super::entity_iso_ypos::<Entity>),
          // Charge effect pos
          (0x29642, 0x000a1370, super::entity_iso_xpos::<Entity>),
          (0x2964c, 0x000a1360, super::entity_iso_ypos::<Entity>),
          // Perspective charge effect pos
          (0x29766, 0x000a11f2, super::entity_linear_xpos::<Entity>),
          (0x2975f, 0x000a11f3, super::entity_linear_ypos::<Entity>),
          // Entity minimap position
          (0x2e5b9, 0x0009c3f9, super::entity_iso_xpos::<Entity>),
          (0x2e5c1, 0x0009c3eb, super::entity_iso_ypos::<Entity>),
          // Perspective entity mouse-over text
          (0x81c63, 0x00048cf5, super::entity_linear_xpos::<Entity>),
          (0x81c6b, 0x00048ce7, super::entity_linear_ypos::<Entity>),
          // Entity mouse-over text
          (0x81cda, 0x00048cd8, super::entity_iso_xpos::<Entity>),
          (0x81cef, 0x00048cbd, super::entity_iso_ypos::<Entity>),
          // Course entity mouse detection
          (0x888dd, 0x000420d5, super::entity_iso_xpos::<Entity>),
          (0x888e5, 0x000420c7, super::entity_iso_ypos::<Entity>),
          // Animated entity mouse detection refinement
          (0x88e37, 0x00041b7b, super::entity_iso_xpos::<Entity>),
          (0x88e5e, 0x00041b4e, super::entity_iso_ypos::<Entity>),
          // Perspective entity draw pos
          (0xadbd6, 0x0001cd82, super::entity_linear_xpos::<Entity>),
          (0xadbe2, 0x0001cd70, super::entity_linear_ypos::<Entity>),
          // Entity draw pos
          (0xadd2c, 0x0001cc86, super::entity_iso_xpos::<Entity>),
          (0xadd36, 0x0001cc76, super::entity_iso_ypos::<Entity>),
          // Perspective entity shadow pos
          (0xb977d, 0x000111db, super::entity_linear_xpos::<Entity>),
          (0xb9787, 0x000111cb, super::entity_linear_ypos::<Entity>),
          // Entity shadow pos
          (0xb97fa, 0x000111b8, super::entity_iso_xpos::<Entity>),
          (0xb9806, 0x000111a6, super::entity_iso_ypos::<Entity>),
          // Perspective entity single color pos
          (0xb9f6a, 0x000109ee, super::entity_linear_xpos::<Entity>),
          (0xb9f63, 0x000109ef, super::entity_linear_ypos::<Entity>),
          // Entity single color pos
          (0xb9fc7, 0x000109eb, super::entity_iso_xpos::<Entity>),
          (0xb9fe2, 0x000109ca, super::entity_iso_ypos::<Entity>),
          // Entity spell overlay perspective
          (0xba507, 0x00010451, super::entity_linear_xpos::<Entity>),
          (0xba500, 0x00010452, super::entity_linear_ypos::<Entity>),
          // Entity spell overlay
          (0xba553, 0x0001045f, super::entity_iso_xpos::<Entity>),
          (0xba566, 0x00010446, super::entity_iso_ypos::<Entity>),
          // Npc mouse over perspective
          (0xbf828, 0x0000b130, super::entity_linear_xpos::<Entity>),
          (0xbf821, 0x0000b131, super::entity_linear_ypos::<Entity>),
          // Npc mouse over
          (0xbf875, 0x0000b13d, super::entity_iso_xpos::<Entity>),
          (0xbf888, 0x0000b124, super::entity_iso_ypos::<Entity>),

          // Game loop sleep patch
          (0x2686, [0xff, 0x15, reloc 0x1c, 0xdf, 0xb6, 0x6f], game_loop_sleep_hook),

          // Draw paused game framerate
          (0x9b78, [0xff, 0x15, reloc 0x54, 0x77, 0xba, 0x6f], draw_game_paused),

          // Draw game framerate & entity sync
          (0xa2c4, [
            0xa1, reloc 0xe0, 0x79, 0xba, 0x6f,
            0x85, 0xc0,
            0x75, 0x2b,
            0xe8, 0x9e, 0xf0, 0x07, 0x00,
            0x85, 0xc0,
            0x74, 0x30,
            0x33, 0xc9,
            0xff, 0x15, reloc 0x54, 0x77, 0xba, 0x6f,
            0x8b, 0x0d, reloc 0x64, 0x77, 0xba, 0x6f,
            0xa1, reloc 0x7c, 0x77, 0xba, 0x6f,
            0x41,
            0x40,
            0x89, 0x0d, reloc 0x64, 0x77, 0xba, 0x6f,
            0xa3, reloc 0x7c, 0x77, 0xba, 0x6f,
            0xeb, 0x0e,
            0x8b, 0x44, 0x24, 0x14,
            0x85, 0xc0,
            0x74, 0x06,
            0xff, 0x05, reloc 0x84, 0x77, 0xba, 0x6f,
          ], super::draw_game::<Entity>),
        }
        (d2win.0 as usize, 0x6f8a0000) => {
          // Draw menu framerate
          (0xd00c, [
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
            0x7f, 0x28,
            0x83, 0xc7, 0x28,
            0x81, 0xff, 0x18, 0xfc, 0xff, 0xff,
            0x7d, 0x02,
            0x33, 0xff,
            0x8b, 0x54, 0x24, 0x34,
            0x85, 0xd2,
            0x74, 0x0e,
            0x8b, 0x4c, 0x24, 0x10,
            0x8b, 0xc1,
            0x41,
            0x50,
            0x89, 0x4c, 0x24, 0x14,
            0xff, 0xd2,
            0xe8, 0x9f, 0x06, 0x00, 0x00,
          ], draw_menu_110_asm_stub),

          // Menu char frame rate
          (0x1abf, [
            0x8b, 0x46, 0x10,
            0x8b, 0x4e, 0x08,
            0x03, 0xc8,
            0x89, 0x4e, 0x08,
            0x8b, 0xc1,
          ], update_menu_char_frame_110_asm_stub),
        }
      );
    }
    Ok(())
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

global_asm! {
  ".global _draw_menu_110_asm_stub",
  "_draw_menu_110_asm_stub:",
  "mov ecx, [esp+0x38]",
  "lea edx, [esp+0x14]",
  "call {}",
  "mov edi, eax",
  "ret",
  sym super::draw_menu,
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
  sym super::update_menu_char_frame,
}
extern "C" {
  pub fn update_menu_char_frame_110_asm_stub();
}
