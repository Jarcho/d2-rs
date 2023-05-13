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

pub struct Patch {
  address: usize,
  original: Cow<'static, [u8]>,
}
impl Patch {
  pub unsafe fn patch_call_location(
    address: usize,
    original: &'static u32,
    target: usize,
  ) -> Result<Self, ()> {
    let _mem = MemUnlock::new(address, 4)?;

    let p = address as *mut u32;
    if p.read_unaligned() != *original {
      return Err(());
    }
    p.write_unaligned((target - (address + 4)) as u32);
    Ok(Self {
      address,
      original: Borrowed(slice::from_raw_parts((original as *const u32).cast(), 4)),
    })
  }

  pub unsafe fn call_patch(
    address: usize,
    original: &'static [u8],
    relocs: &'static [bool],
    reloc_distance: usize,
    target: usize,
  ) -> Result<Self, ()> {
    assert_eq!(original.len(), relocs.len());

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

    assert!(original.len() > 5);
    let _mem = MemUnlock::new(address, original.len())?;

    let slice = slice::from_raw_parts_mut(address as *mut u8, original.len());
    if slice != original {
      return Err(());
    }
    slice[0] = 0xe8;
    slice
      .as_mut_ptr()
      .offset(1)
      .cast::<u32>()
      .write_unaligned((target - (address + 5)) as u32);
    let slice = &mut slice[5..];

    if slice.len() > 40 {
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
      let mut dst = slice.as_mut_ptr().cast::<u32>();
      for &x in &NOP_SEQUENCE[..slice.len() / 8 * 2] {
        dst.write_unaligned(x);
        dst = dst.offset(1);
      }
      let src = NOP_BY_SIZE[slice.len() % 8];
      dst.cast::<u8>().copy_from_nonoverlapping(src.as_ptr(), src.len());
    }

    if reloc_distance == 0 {
      Ok(Self { address, original: Borrowed(original) })
    } else {
      let mut relocated = original.to_owned();
      for i in relocs.iter().enumerate().filter_map(|(i, x)| x.then_some(i)) {
        let loc = (&mut relocated[i]) as *mut u8 as *mut usize;
        loc.write_unaligned(loc.read_unaligned().wrapping_add(reloc_distance));
      }
      Ok(Self { address, original: Owned(relocated) })
    }
  }
}
impl Drop for Patch {
  fn drop(&mut self) {
    unsafe {
      if let Ok(_mem) = MemUnlock::new(self.address, self.original.len()) {
        (self.address as *mut u8)
          .copy_from_nonoverlapping(self.original.as_ptr(), self.original.len());
      }
    }
  }
}
