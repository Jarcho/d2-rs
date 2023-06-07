use core::{
  ffi::c_void,
  mem::{transmute, MaybeUninit},
  ptr::null_mut,
  slice,
};
use windows_sys::Win32::{
  Foundation::HMODULE,
  System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE},
};
use xxhash_rust::xxh3::xxh3_64;

pub use bin_patch_mac::patch_source;

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
  len: u16,
  hash: u64,
  relocs: &'static [u16],
  target: Option<unsafe extern "C" fn()>,
}
impl Patch {
  /// Create a patch which replaces the referenced code with a call to a `cdecl`
  /// function taking zero arguments.
  pub const fn call_c<R>(
    offset: usize,
    (len, hash, relocs): (u16, u64, &'static [u16]),
    target: unsafe extern "C" fn() -> R,
  ) -> Self {
    Self {
      offset,
      len,
      hash,
      relocs,
      target: Some(unsafe { transmute(target) }),
    }
  }

  /// Create a patch which replaces the referenced code with a call to a
  /// `stdcall` function taking one argument.
  pub const fn call_std1<T1, R>(
    offset: usize,
    (len, hash, relocs): (u16, u64, &'static [u16]),
    target: unsafe extern "stdcall" fn(T1) -> R,
  ) -> Self {
    Self {
      offset,
      len,
      hash,
      relocs,
      target: Some(unsafe { transmute(target) }),
    }
  }

  /// Create a patch which replaces the referenced code with code that does
  /// nothing.
  pub const fn nop(offset: usize, (len, hash, relocs): (u16, u64, &'static [u16])) -> Self {
    Self { offset, len, hash, relocs, target: None }
  }

  /// Checks if the memory at the patch location contains the expected bytes.
  ///
  /// # Safety
  /// This reads frin arbitrary memory and there is no way to guarantee memory
  /// safety.
  pub unsafe fn has_expected(&self, base: HMODULE, reloc_dist: isize) -> bool {
    let address = (base as usize + self.offset) as *mut c_void;
    let Ok(_mem) = MemUnlock::new(address, self.len.into()) else {
      return false;
    };

    let mut reloc_buf = [MaybeUninit::<u8>::uninit(); u16::MAX as usize];
    let slice = slice::from_raw_parts(address.cast::<u8>(), self.len.into());
    let slice = if reloc_dist == 0 || self.relocs.is_empty() {
      slice
    } else {
      reloc_buf
        .as_mut_ptr()
        .cast::<u8>()
        .copy_from_nonoverlapping(slice.as_ptr(), slice.len());
      for &reloc in self.relocs {
        let p = reloc_buf[reloc as usize..reloc as usize + 4]
          .as_mut_ptr()
          .cast::<isize>();
        p.write_unaligned(p.read_unaligned().wrapping_add(reloc_dist));
      }
      slice::from_raw_parts(reloc_buf.as_ptr().cast::<u8>(), slice.len())
    };

    xxh3_64(slice) == self.hash
  }

  /// Applies the patch to the given base with the given relocation applied.
  /// Fails if the memory cannot be read/written, or it does not contain the
  /// expected code.
  ///
  /// # Safety
  /// This writes to an arbitrary memory and there is no way to guarantee memory
  /// safety.
  pub unsafe fn apply(&self, base: HMODULE) {
    let address = (base as usize + self.offset) as *mut c_void;
    let _mem = MemUnlock::new(address, self.len.into()).unwrap();

    let mut slice = slice::from_raw_parts_mut(address as *mut u8, self.len.into());

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
    } else if slice.len() > 32 {
      slice[0] = 0xeb;
      slice[1] = (slice.len() - 2) as u8;
    } else {
      #[rustfmt::skip]
      const NOP_BY_SIZE: [u8; 28] = [
        0x90,
        0x66, 0x90,
        0x0f, 0x1f, 0x00,
        0x0f, 0x1f, 0x40, 0x00,
        0x0f, 0x1f, 0x44, 0x00, 0x00,
        0x66, 0x0f, 0x1f, 0x44, 0x00, 0x00,
        0x0f, 0x1f, 0x80, 0x00, 0x00, 0x00, 0x00,
      ];

      // Write eight byte NOP sequences
      let mut dst = slice.as_mut_ptr().cast::<u32>();
      for _ in 0..slice.len() / 8 {
        dst.write_unaligned(0x00841f0f);
        dst.offset(1).write_unaligned(0);
        dst = dst.offset(2);
      }

      // Write trailing 1-7 byte NOP
      let len = slice.len() % 8;
      let offset = len.wrapping_sub(1) * len / 2;
      let src = NOP_BY_SIZE.as_ptr().add(offset);
      if len != 0 && len != 4 && len != 3 && len != 1 {
        panic!("{}, {:?}", slice.len(), slice::from_raw_parts(src, len));
      }
      dst.cast::<u8>().copy_from_nonoverlapping(src, len);
    }
  }
}
