use core::{
  ffi::c_void,
  mem::{size_of, transmute},
  ptr::null_mut,
  slice,
};
use std::borrow::Cow::{self, Borrowed, Owned};
use windows_sys::Win32::{
  Foundation::HMODULE,
  System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE},
};

pub use bin_patch_mac::patch_source;

static NOP_SEQUENCE: [u32; 10] = [
  0x00841f0f, 0x00000000, 0x00841f0f, 0x00000000, 0x00841f0f, 0x00000000, 0x00841f0f, 0x00000000,
  0x00841f0f, 0x00000000,
];
static NOP_BY_SIZE: [&[u8]; 8] = [
  &[],
  &[0x90],
  &[0x66, 0x90],
  &[0x0f, 0x1f, 0x00],
  &[0x0f, 0x1f, 0x40, 0x00],
  &[0x0f, 0x1f, 0x44, 0x00, 0x00],
  &[0x66, 0x0f, 0x1f, 0x44, 0x00, 0x00],
  &[0x0f, 0x1f, 0x80, 0x00, 0x00, 0x00, 0x00],
];

struct MemUnlock {
  prev: u32,
  address: *const c_void,
  len: usize,
}
impl MemUnlock {
  unsafe fn new(address: *const c_void, len: usize) -> Result<Self, ()> {
    let mut prev = 0u32;
    if VirtualProtect(address, 4, PAGE_EXECUTE_READWRITE, &mut prev) != 0 {
      Ok(Self { prev, address, len })
    } else {
      Err(())
    }
  }
}
impl Drop for MemUnlock {
  fn drop(&mut self) {
    unsafe {
      VirtualProtect(self.address, self.len, self.prev, null_mut());
    }
  }
}

/// A code patch which can be applied to a loaded module.
pub struct Patch {
  pub offset: usize,
  original: &'static [u8],
  relocs: &'static [u16],
  target: Option<unsafe extern "C" fn()>,
}
impl Patch {
  /// Create a patch which replaces the referenced code with a call to a `cdecl`
  /// function taking zero arguments.
  pub const fn call_c<R>(
    offset: usize,
    (original, relocs): (&'static [u8], &'static [u16]),
    target: unsafe extern "C" fn() -> R,
  ) -> Self {
    Self {
      offset,
      original,
      relocs,
      target: Some(unsafe { transmute(target) }),
    }
  }

  /// Create a patch which replaces the referenced code with a call to a
  /// `stdcall` function taking one argument.
  pub const fn call_std1<T1, R>(
    offset: usize,
    (original, relocs): (&'static [u8], &'static [u16]),
    target: unsafe extern "stdcall" fn(T1) -> R,
  ) -> Self {
    Self {
      offset,
      original,
      relocs,
      target: Some(unsafe { transmute(target) }),
    }
  }

  /// Create a patch which replaces the referenced code with code that does
  /// nothing.
  pub const fn nop(offset: usize, (original, relocs): (&'static [u8], &'static [u16])) -> Self {
    Self { offset, original, relocs, target: None }
  }

  /// Applies the patch to the given base with the given relocation applied.
  /// Fails if the memory cannot be read/written, or it does not contain the
  /// expected code.
  ///
  /// # Safety
  /// This writes to an arbitrary memory and there is no way to guarantee memory
  /// safety.
  pub unsafe fn apply(&self, base: HMODULE, reloc_dist: isize) -> Result<AppliedPatch, ()> {
    let address = (base as usize + self.offset) as *mut c_void;
    let _mem = MemUnlock::new(address, self.original.len())?;

    // Apply relocation if needed
    let original = if reloc_dist == 0 || self.relocs.is_empty() {
      Borrowed(self.original)
    } else {
      let mut relocated = self.original.to_owned();
      for &reloc in self.relocs {
        let p = relocated[reloc as usize..reloc as usize + size_of::<usize>()]
          .as_mut_ptr()
          .cast::<isize>();
        p.write_unaligned(p.read_unaligned().wrapping_add(reloc_dist));
      }
      Owned(relocated)
    };

    let mut slice = slice::from_raw_parts_mut(address as *mut u8, original.len());
    if slice != &*original {
      return Err(());
    }

    // Write the call instruction.
    if let Some(target) = self.target {
      let (head, tail) = slice.split_at_mut(5);
      head[0] = 0xe8;
      head
        .as_mut_ptr()
        .offset(1)
        .cast::<i32>()
        .write_unaligned(((target as usize).wrapping_sub(address as usize + 5)) as i32);
      slice = tail;
    }

    if slice.len() > 129 {
      slice[0] = 0xe9;
      slice[1..]
        .as_mut_ptr()
        .cast::<u32>()
        .write_unaligned(slice.len() as u32 - 5);
      slice[5..].fill(0x90);
    } else if slice.len() > NOP_SEQUENCE.len() * size_of::<u32>() {
      slice[0] = 0xeb;
      slice[1] = (slice.len() - 2) as u8;
      slice[2..].fill(0x90);
    } else {
      // Write eight byte NOP sequences
      let mut dst = slice.as_mut_ptr().cast::<u32>();
      for &x in &NOP_SEQUENCE[..slice.len() / 8 * 2] {
        dst.write_unaligned(x);
        dst = dst.offset(1);
      }
      // Write final 1-7 byte NOP
      let src = NOP_BY_SIZE[slice.len() % 8];
      dst.cast::<u8>().copy_from_nonoverlapping(src.as_ptr(), src.len());
    }

    Ok(AppliedPatch { address, original })
  }
}

/// An applied patch which will be reverted on drop.
pub struct AppliedPatch {
  address: *mut c_void,
  original: Cow<'static, [u8]>,
}
impl Drop for AppliedPatch {
  fn drop(&mut self) {
    unsafe {
      if let Ok(_mem) = MemUnlock::new(self.address, self.original.len()) {
        (self.address as *mut u8)
          .copy_from_nonoverlapping(self.original.as_ptr(), self.original.len());
      }
    }
  }
}
unsafe impl Send for AppliedPatch {}
unsafe impl Sync for AppliedPatch {}
