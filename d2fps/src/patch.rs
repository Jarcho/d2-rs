use core::{ffi::c_void, ptr, slice};
use std::borrow::Cow::{self, Borrowed, Owned};
use windows_sys::Win32::System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE};
struct MemUnlock(u32, usize, usize);
impl MemUnlock {
  unsafe fn new(address: usize, len: usize) -> Result<Self, ()> {
    let mut prev = 0u32;
    if VirtualProtect(
      address as *const c_void,
      4,
      PAGE_EXECUTE_READWRITE,
      &mut prev,
    ) != 0
    {
      Ok(Self(prev, address, len))
    } else {
      Err(())
    }
  }
}
impl Drop for MemUnlock {
  fn drop(&mut self) {
    unsafe {
      VirtualProtect(self.1 as *const c_void, self.2, self.0, ptr::null_mut());
    }
  }
}

pub trait Patch {
  fn offset(&self) -> usize;
  unsafe fn apply(&self, base: usize, reloc_dist: usize) -> Result<AppliedPatch, ()>;
}

pub struct CallTargetPatch {
  pub offset: usize,
  pub original: &'static u32,
  pub target: unsafe fn(),
}
impl CallTargetPatch {
  pub const fn new(offset: usize, original: &'static u32, target: unsafe fn()) -> Self {
    Self { offset, original, target }
  }
}
impl Patch for CallTargetPatch {
  fn offset(&self) -> usize {
    self.offset
  }

  unsafe fn apply(&self, base: usize, _: usize) -> Result<AppliedPatch, ()> {
    let address = base + self.offset;
    let _mem = MemUnlock::new(address, 4)?;

    let p = address as *mut u32;
    if p.read_unaligned() != *self.original {
      return Err(());
    }
    p.write_unaligned(((self.target as usize).wrapping_sub(address + 4)) as u32);
    Ok(AppliedPatch {
      address,
      original: Borrowed(slice::from_raw_parts(
        (self.original as *const u32).cast(),
        4,
      )),
    })
  }
}

pub struct CallPatch {
  pub offset: usize,
  pub original: &'static [u8],
  pub relocs: &'static [bool],
  pub target: unsafe fn(),
}
impl CallPatch {
  pub const fn new(
    offset: usize,
    original: &'static [u8],
    relocs: &'static [bool],
    target: unsafe fn(),
  ) -> Self {
    Self { offset, original, relocs, target }
  }
}
impl Patch for CallPatch {
  fn offset(&self) -> usize {
    self.offset
  }

  unsafe fn apply(&self, base: usize, reloc_dist: usize) -> Result<AppliedPatch, ()> {
    static NOP_SEQUENCE: [u32; 10] = [
      0x00841f0f, 0x00000000, 0x00841f0f, 0x00000000, 0x00841f0f, 0x00000000, 0x00841f0f,
      0x00000000, 0x00841f0f, 0x00000000,
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

    assert_eq!(self.original.len(), self.relocs.len());
    assert!(self.original.len() >= 5);

    let address = base.wrapping_add(self.offset);
    let _mem = MemUnlock::new(address, self.original.len())?;

    // Apply relocation if needed
    let original = if reloc_dist == 0 {
      Borrowed(self.original)
    } else {
      let mut relocated = self.original.to_owned();
      for i in self.relocs.iter().enumerate().filter_map(|(i, x)| x.then_some(i)) {
        let loc = (&mut relocated[i]) as *mut u8 as *mut usize;
        loc.write_unaligned(loc.read_unaligned().wrapping_add(reloc_dist));
      }
      Owned(relocated)
    };

    let slice = slice::from_raw_parts_mut(address as *mut u8, original.len());
    if slice != &*original {
      return Err(());
    }
    slice[0] = 0xe8;
    slice
      .as_mut_ptr()
      .offset(1)
      .cast::<u32>()
      .write_unaligned(((self.target as usize).wrapping_sub(address + 5)) as u32);
    let slice = &mut slice[5..];

    if slice.len() > 40 {
      // Write either a short or a long jump depending on the length.
      if slice.len() > 129 {
        slice[0] = 0xe9;
        slice[1..]
          .as_mut_ptr()
          .cast::<u32>()
          .write_unaligned(slice.len() as u32 - 5);
        slice[5..].fill(0x90);
      } else {
        slice[0] = 0xeb;
        slice[1] = (slice.len() - 2) as u8;
        slice[2..].fill(0x90);
      }
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

pub struct AppliedPatch {
  address: usize,
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
