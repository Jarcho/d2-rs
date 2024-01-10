use crate::{
  ExPrecision, MulTrunc, WrappingAdd, WrappingDiv, WrappingFrom, WrappingInto, WrappingMul,
  WrappingSub,
};
use core::{
  fmt,
  ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Shl, Shr, Sub, SubAssign},
};

/// A fixed-point number with `N` bits of precision.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Fixed<T, const N: u8>(T);
impl<T, const N: u8> Fixed<T, N> {
  /// Creates a fixed-point number from it's underlying representation.
  #[inline]
  pub const fn from_repr(value: T) -> Self {
    Self(value)
  }

  /// Gets the fixed-point number's representation.
  #[inline]
  pub fn repr(self) -> T {
    self.0
  }
}
impl<T, const N: u8> Fixed<T, N>
where
  T: Shl<u8, Output = T> + Shr<u8, Output = T>,
{
  /// Truncates a fixed-point number to an integer.
  #[inline]
  pub fn trunc(self) -> T {
    self.0 >> N
  }

  /// Changes the number bits used for the fractional part. Wraps on overflow.
  #[inline]
  pub fn with_prec<const M: u8>(self) -> Fixed<T, M> {
    if N < M {
      Fixed(self.0 << (M - N))
    } else {
      Fixed(self.0 >> (N - M))
    }
  }
}

impl<T: Into<f64>, const N: u8> From<Fixed<T, N>> for f64 {
  fn from(x: Fixed<T, N>) -> Self {
    x.repr().into() / <f64 as From<u32>>::from(1u32 << N)
  }
}
impl<const N: u8> From<f64> for Fixed<u32, N> {
  fn from(x: f64) -> Self {
    Self((x * 2u32.pow(N as u32) as f64) as u32)
  }
}
impl<const N: u8> From<f64> for Fixed<i32, N> {
  fn from(x: f64) -> Self {
    Self((x * 2u32.pow(N as u32) as f64) as i32)
  }
}

impl<T: WrappingFrom<U>, U: Shl<u8, Output = U> + Shr<u8, Output = U>, const N: u8, const M: u8>
  WrappingFrom<Fixed<U, M>> for Fixed<T, N>
{
  fn wfrom(x: Fixed<U, M>) -> Self {
    Self(x.with_prec::<N>().0.winto())
  }
}

impl<T: Shl<u8, Output = T>, const N: u8> WrappingFrom<T> for Fixed<T, N> {
  fn wfrom(x: T) -> Self {
    Self(x << N)
  }
}

impl<T, U, const N: u8> MulAssign<U> for Fixed<T, N>
where
  Self: Copy + Mul<U, Output = Self>,
{
  fn mul_assign(&mut self, rhs: U) {
    *self = *self * rhs
  }
}

impl<T, U, const N: u8> DivAssign<U> for Fixed<T, N>
where
  Self: Copy + Div<U, Output = Self>,
{
  fn div_assign(&mut self, rhs: U) {
    *self = *self / rhs
  }
}

macro_rules! impl_op {
  ($op:ident, $f:ident, $other:ty $(, $field:tt)?) => {
    impl<T: $op<Output = T>, const N: u8> $op<$other> for Fixed<T, N> {
      type Output = Self;
      fn $f(self, rhs: $other) -> Self::Output {
        Self($op::$f(self.0, rhs $(.$field)?))
      }
    }
  };
}

macro_rules! impl_op_rev {
  ($op:ident, $f:ident, $($ty:ty),*) => {$(
    impl<const N: u8> $op<Fixed<$ty, N>> for $ty {
      type Output = <Fixed<$ty, N> as $op<Self>>::Output;
      fn $f(self, rhs: Fixed<$ty, N>) -> Self::Output {
        $op::$f(rhs, self)
      }
    }
  )*};
}

macro_rules! impl_op_assign {
  ($($op:ident)::+, $f:ident, $other:ty $(, $field:tt)?) => {
    impl<T: $($op)::*, const N: u8> $($op)::*<$other> for Fixed<T, N> {
      fn $f(&mut self, other: $other) {
        $($op)::*::$f(&mut self.0, other $(.$field)?)
      }
    }
  };
}

impl_op!(Add, add, Self, 0);
impl_op!(Sub, sub, Self, 0);
impl_op!(Mul, mul, T);
impl_op!(Div, div, T);
impl_op!(WrappingAdd, wadd, Self, 0);
impl_op!(WrappingSub, wsub, Self, 0);
impl_op!(WrappingMul, wmul, T);
impl_op!(WrappingDiv, wdiv, T);
impl_op_rev!(Mul, mul, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
#[rustfmt::skip]
impl_op_rev!(
  WrappingMul, wmul,
  i8, i16, i32, i64, i128, isize,
  u8, u16, u32, u64, u128, usize
);
impl_op_rev!(MulTrunc, mul_trunc, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
impl_op_assign!(AddAssign, add_assign, Self, 0);
impl_op_assign!(SubAssign, sub_assign, Self, 0);

impl<T: Neg<Output = T>, const N: u8> Neg for Fixed<T, N> {
  type Output = Self;
  #[inline]
  fn neg(self) -> Self::Output {
    Self(-self.0)
  }
}

impl<T: ExPrecision + WrappingFrom<T::ExTy>, const N: u8> Mul for Fixed<T, N>
where
  T::ExTy: Mul<Output = T::ExTy> + Shr<u8, Output = T::ExTy> + WrappingFrom<T>,
{
  type Output = Self;
  fn mul(self, rhs: Self) -> Self::Output {
    Self(T::wfrom(
      (T::ExTy::wfrom(self.0) * T::ExTy::wfrom(rhs.0)) >> N,
    ))
  }
}

impl<T: ExPrecision + WrappingFrom<T::ExTy>, const N: u8> Div for Fixed<T, N>
where
  T::ExTy:
    Div<Output = T::ExTy> + Shl<u8, Output = T::ExTy> + Shr<u8, Output = T::ExTy> + WrappingFrom<T>,
{
  type Output = Self;
  fn div(self, rhs: Self) -> Self::Output {
    Self(T::wfrom(
      ((T::ExTy::wfrom(self.0) << (2 * N)) / T::ExTy::wfrom(rhs.0)) >> N,
    ))
  }
}

impl<T: ExPrecision + WrappingFrom<T::ExTy>, const N: u8, const M: u8> MulTrunc<Fixed<T, M>>
  for Fixed<T, N>
where
  T::ExTy: Mul<Output = T::ExTy> + Shr<u8, Output = T::ExTy> + WrappingFrom<T>,
{
  type Output = T;
  fn mul_trunc(self, rhs: Fixed<T, M>) -> Self::Output {
    T::wfrom((T::ExTy::wfrom(self.0) * T::ExTy::wfrom(rhs.0)) >> (N + M))
  }
}

impl<T: ExPrecision + WrappingFrom<T::ExTy>, const N: u8> MulTrunc<T> for Fixed<T, N>
where
  T::ExTy: Mul<Output = T::ExTy> + Shr<u8, Output = T::ExTy> + WrappingFrom<T>,
{
  type Output = T;
  fn mul_trunc(self, rhs: T) -> Self::Output {
    T::wfrom((T::ExTy::wfrom(self.0) * T::ExTy::wfrom(rhs)) >> N)
  }
}

impl<T: Copy + Into<f64>, const N: u8> fmt::Display for Fixed<T, N> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f64::from(*self).fmt(f)
  }
}
impl<T, const N: u8> fmt::Debug for Fixed<T, N>
where
  Fixed<T, N>: fmt::Display,
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    <Self as fmt::Display>::fmt(self, f)
  }
}
