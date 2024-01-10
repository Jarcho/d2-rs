#![no_std]

mod fixed;

pub use fixed::Fixed;

pub trait WrappingAdd<T = Self> {
  type Output;
  fn wadd(self, rhs: T) -> Self::Output;
}

pub trait WrappingSub<T = Self> {
  type Output;
  fn wsub(self, rhs: T) -> Self::Output;
}

pub trait WrappingMul<T = Self> {
  type Output;
  fn wmul(self, rhs: T) -> Self::Output;
}

pub trait WrappingDiv<T = Self> {
  type Output;
  fn wdiv(self, rhs: T) -> Self::Output;
}

pub trait MulTrunc<T = Self> {
  type Output;
  fn mul_trunc(self, rhs: T) -> Self::Output;
}

pub trait WrappingFrom<T> {
  fn wfrom(_: T) -> Self;
}

pub trait WrappingInto<T> {
  fn winto(self) -> T;
}
impl<T: WrappingFrom<U>, U> WrappingInto<T> for U {
  fn winto(self) -> T {
    T::wfrom(self)
  }
}

pub trait ExPrecision {
  type ExTy;
}

macro_rules! impl_core_op {
  ($ty:ty, $t:ident, $trf:ident, $tyf:ident) => {
    impl $t for $ty {
      type Output = Self;
      #[inline]
      fn $trf(self, rhs: Self) -> Self {
        self.$tyf(rhs)
      }
    }
  };
}

macro_rules! impl_core_ops {
  ($ty:ty) => {
    impl_core_op!($ty, WrappingAdd, wadd, wrapping_add);
    impl_core_op!($ty, WrappingSub, wsub, wrapping_sub);
    impl_core_op!($ty, WrappingMul, wmul, wrapping_mul);
    impl_core_op!($ty, WrappingDiv, wdiv, wrapping_div);
  };
}

impl_core_ops!(i8);
impl_core_ops!(i16);
impl_core_ops!(i32);
impl_core_ops!(i64);
impl_core_ops!(i128);
impl_core_ops!(isize);
impl_core_ops!(u8);
impl_core_ops!(u16);
impl_core_ops!(u32);
impl_core_ops!(u64);
impl_core_ops!(u128);
impl_core_ops!(usize);

macro_rules! impl_wrapping_from {
  ($ty:ty, $($from_ty:ty),+) => {$(
    impl WrappingFrom<$from_ty> for $ty {
      fn wfrom(x: $from_ty) -> Self {
        x as $ty
      }
    }
  )+};
}

impl_wrapping_from!(i8, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_wrapping_from!(i16, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_wrapping_from!(i32, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_wrapping_from!(i64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_wrapping_from!(i128, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_wrapping_from!(isize, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_wrapping_from!(u8, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_wrapping_from!(u16, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_wrapping_from!(u32, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_wrapping_from!(u64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_wrapping_from!(u128, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_wrapping_from!(usize, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

macro_rules! impl_ex_prec {
  ($t:ty, $et:ty) => {
    impl ExPrecision for $t {
      type ExTy = $et;
    }
  };
}

impl_ex_prec!(i8, i16);
impl_ex_prec!(i16, i32);
impl_ex_prec!(i32, i64);
impl_ex_prec!(i64, i128);
#[cfg(target_pointer_width = "32")]
impl_ex_prec!(isize, i64);
#[cfg(target_pointer_width = "64")]
impl_ex_prec!(isize, i128);

impl_ex_prec!(u8, u16);
impl_ex_prec!(u16, u32);
impl_ex_prec!(u32, u64);
impl_ex_prec!(u64, u128);
#[cfg(target_pointer_width = "32")]
impl_ex_prec!(usize, u64);
#[cfg(target_pointer_width = "64")]
impl_ex_prec!(usize, u128);
