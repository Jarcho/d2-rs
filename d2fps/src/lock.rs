use core::{
  cell::UnsafeCell,
  ops::{Deref, DerefMut},
  sync::atomic::{
    AtomicBool,
    Ordering::{Acquire, Relaxed, Release},
  },
};

pub struct Mutex<T> {
  item: UnsafeCell<T>,
  locked: AtomicBool,
}
unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}
impl<T> Mutex<T> {
  pub const fn new(item: T) -> Self {
    Self {
      item: UnsafeCell::new(item),
      locked: AtomicBool::new(false),
    }
  }

  pub fn lock(&self) -> LockGuard<T> {
    while self
      .locked
      .compare_exchange_weak(false, true, Acquire, Relaxed)
      .is_err()
    {
      // Wait
    }
    LockGuard { lock: self }
  }

  pub fn try_lock(&self) -> Option<LockGuard<T>> {
    if self.locked.compare_exchange(false, true, Acquire, Relaxed).is_ok() {
      Some(LockGuard { lock: self })
    } else {
      None
    }
  }
}

pub struct LockGuard<'a, T> {
  lock: &'a Mutex<T>,
}
impl<T> Drop for LockGuard<'_, T> {
  fn drop(&mut self) {
    self.lock.locked.store(false, Release);
  }
}
impl<T> Deref for LockGuard<'_, T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    unsafe { &*self.lock.item.get() }
  }
}
impl<T> DerefMut for LockGuard<'_, T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe { &mut *self.lock.item.get() }
  }
}
