use core::{ffi::c_void, mem::transmute, ptr::null_mut, slice};
use windows_sys::Win32::{
  Foundation::HMODULE,
  System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE},
};

pub use bin_patch_mac::patch_source;

struct Uint2Iter<'a> {
  iter: slice::Iter<'a, u32>,
  cur: u32,
  remain: u8,
}
impl<'a> Uint2Iter<'a> {
  fn new(slice: &'a [u32]) -> Self {
    Self { iter: slice.iter(), cur: 0, remain: 0 }
  }
}
impl Iterator for Uint2Iter<'_> {
  type Item = u8;
  fn next(&mut self) -> Option<Self::Item> {
    self.remain = if self.remain == 0 {
      self.cur = *self.iter.next()?;
      15
    } else {
      self.remain - 1
    };
    let res = self.cur as u8 & 3;
    self.cur >>= 2;
    Some(res)
  }
}

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

enum PatchData {
  Target(unsafe extern "C" fn()),
  Raw(&'static [u8]),
}

/// A code patch which can be applied to a loaded module.
pub struct Patch {
  pub offset: usize,
  len: u16,
  hash: u32,
  control_stream: &'static [u32],
  data: PatchData,
}
impl Patch {
  /// Create a patch which replaces the referenced code with a call to a `cdecl`
  /// function taking zero arguments.
  pub const fn call_c<R>(
    offset: usize,
    (len, hash, control_stream): (u16, u32, &'static [u32]),
    target: unsafe extern "C" fn() -> R,
  ) -> Self {
    Self {
      offset,
      len,
      hash,
      control_stream,
      data: PatchData::Target(unsafe { transmute(target) }),
    }
  }

  /// Create a patch which replaces the referenced code with a call to a
  /// `stdcall` function taking one argument.
  pub const fn call_std1<T1, R>(
    offset: usize,
    (len, hash, control_stream): (u16, u32, &'static [u32]),
    target: unsafe extern "stdcall" fn(T1) -> R,
  ) -> Self {
    Self {
      offset,
      len,
      hash,
      control_stream,
      data: PatchData::Target(unsafe { transmute(target) }),
    }
  }

  /// Create a patch which replaces the referenced code with code that does
  /// nothing.
  pub const fn nop(offset: usize, (len, hash, control_stream): (u16, u32, &'static [u32])) -> Self {
    Self {
      offset,
      len,
      hash,
      control_stream,
      data: PatchData::Raw(&[]),
    }
  }

  /// Create a patch which replaces the referenced code with the given code.
  pub const fn raw(
    offset: usize,
    (len, hash, control_stream): (u16, u32, &'static [u32]),
    data: &'static [u8],
  ) -> Self {
    Self {
      offset,
      len,
      hash,
      control_stream,
      data: PatchData::Raw(data),
    }
  }

  /// Checks if the memory at the patch location contains the expected bytes.
  ///
  /// # Safety
  /// The memory in the slice `(base + patch.offset, patch.len)` must be
  /// initialized and readable.
  pub unsafe fn has_expected(&self, base: HMODULE, reloc_dist: isize) -> bool {
    let address = (base as usize + self.offset) as *mut c_void;
    let Ok(_mem) = MemUnlock::new(address, self.len.into()) else {
      return false;
    };
    let Ok(reloc_dist) = i32::try_from(reloc_dist) else {
      return false;
    };

    let mut bytes = slice::from_raw_parts(address.cast::<u8>(), self.len.into()).iter();
    let mut control_stream = Uint2Iter::new(self.control_stream);
    let mut hash = 0x01000193u32;

    loop {
      let control = control_stream.next().unwrap_or(0);
      assert!(control < 3);
      if control == 2 {
        let (head, tail) = bytes.as_slice().split_at(4);
        bytes = tail.iter();
        let buf = head
          .as_ptr()
          .cast::<i32>()
          .read_unaligned()
          .wrapping_sub(reloc_dist)
          .to_ne_bytes();
        for x in buf {
          hash = (hash ^ x as u32).wrapping_mul(0x01000193u32);
        }
      } else {
        let next = match bytes.next() {
          Some(&x) => x as u32,
          None => break,
        };
        let x = if control == 1 { 0 } else { next };
        hash = (hash ^ x).wrapping_mul(0x01000193u32);
      }
    }

    hash == self.hash
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

    // Write the patch data.
    match self.data {
      PatchData::Target(target) => {
        let (head, tail) = slice.split_at_mut(5);
        head[0] = 0xe8;
        head
          .as_mut_ptr()
          .offset(1)
          .cast::<i32>()
          .write_unaligned(((target as usize).wrapping_sub(address as usize + 5)) as i32);
        slice = tail;
      }
      PatchData::Raw(data) => {
        let (head, tail) = slice.split_at_mut(data.len());
        head.copy_from_slice(data);
        slice = tail;
      }
    }

    // Fill the remaining data with NOPs.
    if slice.len() > 129 {
      slice[0] = 0xe9;
      slice[1..]
        .as_mut_ptr()
        .cast::<u32>()
        .write_unaligned(slice.len() as u32 - 5);
      slice[5..].fill(0x90);
    } else if slice.len() > 32 {
      slice[0] = 0xeb;
      slice[1] = (slice.len() - 2) as u8;
      slice[2..].fill(0x90);
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
      dst.cast::<u8>().copy_from_nonoverlapping(src, len);
    }
  }
}
