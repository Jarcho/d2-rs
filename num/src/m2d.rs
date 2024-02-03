use crate::{
  CheckedAdd, Fixed, MulTrunc, WrappingAbs, WrappingAdd, WrappingDiv, WrappingFrom, WrappingInto,
  WrappingMul, WrappingSub,
};
use bytemuck::{Pod, Zeroable};
use core::{
  fmt,
  ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

#[repr(C)]
#[derive(Default, Clone, Copy, Eq)]
pub struct M2d<T> {
  pub x: T,
  pub y: T,
}
impl<T> M2d<T> {
  #[inline]
  pub const fn new(x: T, y: T) -> Self {
    Self { x, y }
  }

  #[inline]
  pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> M2d<U> {
    M2d::new(f(self.x), f(self.y))
  }
}

impl<T: PartialEq<U>, U> PartialEq<M2d<U>> for M2d<T> {
  fn eq(&self, other: &M2d<U>) -> bool {
    self.x == other.x && self.y == other.y
  }
}

impl<T: fmt::Display> fmt::Display for M2d<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}
impl<T: fmt::Debug> fmt::Debug for M2d<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("").field(&self.x).field(&self.y).finish()
  }
}

impl<T: WrappingFrom<U>, U> WrappingFrom<M2d<U>> for M2d<T> {
  #[inline]
  fn wfrom(x: M2d<U>) -> Self {
    M2d::new(x.x.winto(), x.y.winto())
  }
}

macro_rules! impl_uop {
  ($op:ident, $fn:ident) => {
    impl<T: $op> $op for M2d<T> {
      type Output = M2d<T::Output>;
      #[inline]
      fn $fn(self) -> Self::Output {
        M2d::new($op::$fn(self.x), $op::$fn(self.y))
      }
    }
  };
}
macro_rules! impl_op {
  ($op:ident, $fn:ident) => {
    impl<T: $op<U>, U> $op<M2d<U>> for M2d<T> {
      type Output = M2d<T::Output>;
      #[inline]
      fn $fn(self, rhs: M2d<U>) -> Self::Output {
        M2d::new($op::$fn(self.x, rhs.x), $op::$fn(self.y, rhs.y))
      }
    }
  };
}
macro_rules! impl_cop {
  ($op:ident, $fn:ident) => {
    impl<T: $op<U>, U> $op<M2d<U>> for M2d<T> {
      type Output = M2d<T::Output>;
      #[inline]
      fn $fn(self, rhs: M2d<U>) -> Option<Self::Output> {
        Some(M2d::new($op::$fn(self.x, rhs.x)?, $op::$fn(self.y, rhs.y)?))
      }
    }
  };
}
macro_rules! impl_op_assign {
  ($op:ident, $fn:ident) => {
    impl<T: $op<U>, U> $op<M2d<U>> for M2d<T> {
      #[inline]
      fn $fn(&mut self, rhs: M2d<U>) {
        $op::$fn(&mut self.x, rhs.x);
        $op::$fn(&mut self.y, rhs.y);
      }
    }
  };
}

macro_rules! impl_op_scalar {
  ($op:ident, $fn:ident, $($tys:ty),*) => {$(
    impl<T: $op<$tys>> $op<$tys> for M2d<T> {
      type Output = M2d<T::Output>;
      #[inline]
      fn $fn(self, rhs: $tys) -> Self::Output {
        M2d::new($op::$fn(self.x, rhs), $op::$fn(self.y, rhs))
      }
    }
  )*};
}
macro_rules! impl_cop_scalar {
  ($op:ident, $fn:ident, $($tys:ty),*) => {$(
    impl<T: $op<$tys>> $op<$tys> for M2d<T> {
      type Output = M2d<T::Output>;
      #[inline]
      fn $fn(self, rhs: $tys) -> Option<Self::Output> {
        Some(M2d::new($op::$fn(self.x, rhs)?, $op::$fn(self.y, rhs)?))
      }
    }
  )*};
}
macro_rules! impl_op_assign_scalar {
  ($op:ident, $fn:ident, $($tys:ty),*) => {$(
    impl<T: $op<$tys>> $op<$tys> for M2d<T> {
      #[inline]
      fn $fn(&mut self, rhs: $tys) {
        $op::$fn(&mut self.x, rhs);
        $op::$fn(&mut self.y, rhs);
      }
    }
  )*};
}

macro_rules! impl_op_fixed {
  ($op:ident, $fn:ident) => {
    impl<T: $op<Fixed<U, N>>, U: Copy, const N: u8> $op<Fixed<U, N>> for M2d<T> {
      type Output = M2d<T::Output>;
      #[inline]
      fn $fn(self, rhs: Fixed<U, N>) -> Self::Output {
        M2d::new($op::$fn(self.x, rhs), $op::$fn(self.y, rhs))
      }
    }
  };
}
macro_rules! impl_op_assign_fixed {
  ($op:ident, $fn:ident) => {
    impl<T: $op<Fixed<U, N>>, U: Copy, const N: u8> $op<Fixed<U, N>> for M2d<T> {
      #[inline]
      fn $fn(&mut self, rhs: Fixed<U, N>) {
        $op::$fn(&mut self.x, rhs);
        $op::$fn(&mut self.y, rhs);
      }
    }
  };
}

impl_uop!(Neg, neg);
impl_uop!(WrappingAbs, wabs);

impl_op!(Add, add);
impl_op!(Sub, sub);
impl_op!(Mul, mul);
impl_op!(Div, div);
impl_op!(Rem, rem);
impl_op!(WrappingAdd, wadd);
impl_op!(WrappingSub, wsub);
impl_op!(WrappingMul, wmul);
impl_op!(WrappingDiv, wdiv);
impl_op!(MulTrunc, mul_trunc);

impl_cop!(CheckedAdd, cadd);

impl_op_assign!(AddAssign, add_assign);
impl_op_assign!(SubAssign, sub_assign);
impl_op_assign!(MulAssign, mul_assign);
impl_op_assign!(DivAssign, div_assign);
impl_op_assign!(RemAssign, rem_assign);

impl_op_scalar!(Add, add, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_op_scalar!(Sub, sub, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_op_scalar!(Mul, mul, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_op_scalar!(Div, div, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_op_scalar!(Rem, rem, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_op_scalar!(
  MulTrunc, mul_trunc, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);
#[rustfmt::skip]
impl_op_scalar!(
  WrappingAdd, wadd,
  i8, i16, i32, i64, i128, isize,
  u8, u16, u32, u64, u128, usize
);
#[rustfmt::skip]
impl_op_scalar!(
  WrappingSub, wsub,
  i8, i16, i32, i64, i128, isize,
  u8, u16, u32, u64, u128, usize
);
#[rustfmt::skip]
impl_op_scalar!(
  WrappingMul, wmul,
  i8, i16, i32, i64, i128, isize,
  u8, u16, u32, u64, u128, usize
);
#[rustfmt::skip]
impl_op_scalar!(
  WrappingDiv, wdiv,
  i8, i16, i32, i64, i128, isize,
  u8, u16, u32, u64, u128, usize
);

#[rustfmt::skip]
impl_cop_scalar!(
  CheckedAdd, cadd,
  i8, i16, i32, i64, i128, isize,
  u8, u16, u32, u64, u128, usize
);

impl_op_assign_scalar!(
  AddAssign, add_assign, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);
impl_op_assign_scalar!(
  SubAssign, sub_assign, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);
impl_op_assign_scalar!(
  MulAssign, mul_assign, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);
impl_op_assign_scalar!(
  DivAssign, div_assign, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);
impl_op_assign_scalar!(
  RemAssign, rem_assign, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

impl_op_fixed!(Add, add);
impl_op_fixed!(Sub, sub);
impl_op_fixed!(Mul, mul);
impl_op_fixed!(Div, div);
impl_op_fixed!(WrappingAdd, wadd);
impl_op_fixed!(WrappingSub, wsub);
impl_op_fixed!(WrappingMul, wmul);
impl_op_fixed!(WrappingDiv, wdiv);
impl_op_fixed!(MulTrunc, mul_trunc);

impl_op_assign_fixed!(AddAssign, add_assign);
impl_op_assign_fixed!(SubAssign, sub_assign);
impl_op_assign_fixed!(MulAssign, mul_assign);
impl_op_assign_fixed!(DivAssign, div_assign);

unsafe impl<T: Pod> Pod for M2d<T> {}
unsafe impl<T: Zeroable> Zeroable for M2d<T> {}
