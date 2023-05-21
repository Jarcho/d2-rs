use crate::UnknownPos;
use core::{iter, ops, ptr::NonNull, slice};

decl_enum! { EntityKind(u32) {
  Pc = 0,
  Npc = 1,
  Object = 2,
  Missile = 3,
  Item = 4,
  Tile = 5,
}}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ActId(pub u32);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ActIdS(pub u8);

impl From<ActId> for ActIdS {
  #[inline]
  fn from(value: ActId) -> Self {
    Self(value.0 as u8)
  }
}
impl From<ActIdS> for ActId {
  #[inline]
  fn from(value: ActIdS) -> Self {
    Self(value.0.into())
  }
}

decl_enum! { GameType(u32) {
  Sp = 0,
  Sp2 = 1,
  Bnet = 3,
  OpenBnetHost = 6,
  OpenBnet = 7,
  TcpHost = 8,
  Tcp = 9,
}}
impl GameType {
  #[inline]
  pub fn is_sp(self) -> bool {
    matches!(self, Self::Sp | Self::Sp2)
  }

  #[inline]
  pub fn is_host(self) -> bool {
    matches!(self, Self::OpenBnetHost | Self::TcpHost)
  }
}

pub struct InRoom;

pub trait LinkedList<T = Self>: Sized {
  fn next(&self) -> Option<NonNull<Self>>;
}

#[repr(transparent)]
pub struct EntityTables<T>([EntityTable<T>; 6]);
impl<T> ops::Index<EntityKind> for EntityTables<T> {
  type Output = EntityTable<T>;
  #[inline]
  fn index(&self, index: EntityKind) -> &Self::Output {
    &self.0[index.0 as usize]
  }
}
impl<T> ops::IndexMut<EntityKind> for EntityTables<T> {
  #[inline]
  fn index_mut(&mut self, index: EntityKind) -> &mut Self::Output {
    &mut self.0[index.0 as usize]
  }
}
impl<T: LinkedList> EntityTables<T> {
  pub fn for_each_dy(&self, mut f: impl FnMut(&T)) {
    unsafe { iter_lists(slice::from_raw_parts(self.0[0].0.as_ptr(), 256)) }.for_each(&mut f);
    self.0[3].iter().for_each(&mut f);
  }

  pub fn for_each_dy_mut(&mut self, mut f: impl FnMut(&mut T)) {
    unsafe { iter_mut_lists(slice::from_raw_parts_mut(self.0[0].0.as_mut_ptr(), 256)) }
      .for_each(&mut f);
    self.0[3].iter_mut().for_each(&mut f);
  }
}

#[repr(transparent)]
pub struct EntityTable<T>([Option<NonNull<T>>; 128]);
impl<T: LinkedList> EntityTable<T> {
  #[inline]
  pub fn iter(&self) -> impl Iterator<Item = &T> {
    unsafe { iter_lists(&self.0) }
  }

  #[inline]
  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
    unsafe { iter_mut_lists(&mut self.0) }
  }
}

/// Gets an iterator over all elements in the slice of linked lists.
///
/// # Safety
/// Creating a reference to each element in the lists must be valid.
pub unsafe fn iter_lists<T: LinkedList<U>, U>(
  lists: &[Option<NonNull<T>>],
) -> impl Iterator<Item = &T> {
  lists.iter().flat_map(|&(mut p)| {
    iter::from_fn(move || {
      p.map(|i| unsafe {
        p = i.as_ref().next();
        &*i.as_ptr()
      })
    })
  })
}

/// Gets an iterator over all elements in the slice of linked lists.
///
/// # Safety
/// Creating a mutable reference to each element in the lists must be valid, and
/// each item may appear only once amongst all the lists.
pub unsafe fn iter_mut_lists<T: LinkedList<U>, U>(
  lists: &mut [Option<NonNull<T>>],
) -> impl Iterator<Item = &mut T> {
  lists.iter_mut().flat_map(|&mut mut p| {
    iter::from_fn(move || {
      p.map(|i| unsafe {
        p = i.as_ref().next();
        &mut *i.as_ptr()
      })
    })
  })
}

#[repr(C)]
pub struct EnvArray<T> {
  pub name: [u8; 0x100],
  pub len: u32,
  pub element_size: u32,
  pub initialized: u32,
  pub next_free_idx: u32,
  pub last_active_idx: i32,
  pub active_count: u32,
  pub _padding: u16,
  pub data: NonNull<T>,
}
impl<T> EnvArray<T> {
  pub fn as_slice(&self) -> &[T] {
    unsafe {
      slice::from_raw_parts(
        self.data.as_ptr(),
        if self.initialized != 0 {
          (self.last_active_idx + 1) as usize
        } else {
          0
        },
      )
    }
  }

  pub fn as_mut_slice(&mut self) -> &mut [T] {
    unsafe {
      slice::from_raw_parts_mut(
        self.data.as_ptr(),
        if self.initialized != 0 {
          (self.last_active_idx + 1) as usize
        } else {
          0
        },
      )
    }
  }
}

#[repr(C)]
pub struct EnvImage {
  pub active: u16,
  /// Linear space when rendering in perspective. Camera space when not.
  pub pos: UnknownPos<u32>,
  pub file_idx: u32,
  pub frame: u32,
  pub till_next_frame: u32,
}
pub type EnvImages = EnvArray<EnvImage>;
