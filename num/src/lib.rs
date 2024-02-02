#![no_std]

mod fixed;
mod m2d;
mod measure;

pub use fixed::Fixed;
pub use m2d::M2d;
pub use measure::Measure;

/// The addition operator, but returning `None` on overflow.
pub trait CheckedAdd<T = Self> {
  type Output;
  fn cadd(self, rhs: T) -> Option<Self::Output>;
}

/// The addition operator, but wrapping on overflow.
pub trait WrappingAdd<T = Self> {
  type Output;
  fn wadd(self, rhs: T) -> Self::Output;
}

/// The subtraction operator, but wrapping on overflow.
pub trait WrappingSub<T = Self> {
  type Output;
  fn wsub(self, rhs: T) -> Self::Output;
}

/// The multiplication operator, but wrapping on overflow.
pub trait WrappingMul<T = Self> {
  type Output;
  fn wmul(self, rhs: T) -> Self::Output;
}

/// The division operator, but wrapping on overflow.
pub trait WrappingDiv<T = Self> {
  type Output;
  fn wdiv(self, rhs: T) -> Self::Output;
}

/// The multiplication operator, but wrapping on overflow and truncating the
/// result to the nearest whole number.
pub trait MulTrunc<T = Self> {
  type Output;
  fn mul_trunc(self, rhs: T) -> Self::Output;
}

/// Infallible numeric value conversion. Oversized values are wrapped into the
/// target domain.
pub trait WrappingFrom<T> {
  fn wfrom(_: T) -> Self;
}

/// The inverse of `WrappingFrom`.
pub trait WrappingInto<T> {
  fn winto(self) -> T;
}
impl<T: WrappingFrom<U>, U> WrappingInto<T> for U {
  #[inline]
  fn winto(self) -> T {
    T::wfrom(self)
  }
}

/// The integer type one step larger than the current type.
pub trait ExInt {
  type ExInt;
}

/// The integer type with the largest size of two integer types, but the
/// signedness of `Self`
pub trait WithLargestBitSize<T> {
  type Sized;
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
macro_rules! impl_core_cop {
  ($ty:ty, $t:ident, $trf:ident, $tyf:ident) => {
    impl $t for $ty {
      type Output = Self;
      #[inline]
      fn $trf(self, rhs: Self) -> Option<Self> {
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
    impl_core_cop!($ty, CheckedAdd, cadd, checked_add);
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
      #[inline]
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

macro_rules! impl_ex_int {
  ($t:ty, $et:ty) => {
    impl ExInt for $t {
      type ExInt = $et;
    }
  };
}

impl_ex_int!(i8, i16);
impl_ex_int!(i16, i32);
impl_ex_int!(i32, i64);
impl_ex_int!(i64, i128);
#[cfg(target_pointer_width = "32")]
impl_ex_int!(isize, i64);
#[cfg(target_pointer_width = "64")]
impl_ex_int!(isize, i128);

impl_ex_int!(u8, u16);
impl_ex_int!(u16, u32);
impl_ex_int!(u32, u64);
impl_ex_int!(u64, u128);
#[cfg(target_pointer_width = "32")]
impl_ex_int!(usize, u64);
#[cfg(target_pointer_width = "64")]
impl_ex_int!(usize, u128);

macro_rules! impl_with_largest_bit_size {
  ($ty:ty) => {
    impl WithLargestBitSize<$ty> for $ty {
      type Sized = $ty;
    }
  };
}
macro_rules! impl_with_largest_bit_size2 {
  ($ty:ty, $($ty2:ty),*) => {$(
    impl WithLargestBitSize<$ty> for $ty2 {
      type Sized = $ty2;
    }
    impl WithLargestBitSize<$ty2> for $ty {
      type Sized = $ty2;
    }
  )*};
}
macro_rules! impl_with_largest_bit_size3 {
  ($ty:ty, $(($ty2:ty, $res:ty)),*) => {$(
    impl WithLargestBitSize<$ty> for $ty2 {
      type Sized = $res;
    }
  )*};
}

impl_with_largest_bit_size!(i8);
impl_with_largest_bit_size!(i16);
impl_with_largest_bit_size!(i32);
impl_with_largest_bit_size!(i64);
impl_with_largest_bit_size!(i128);
impl_with_largest_bit_size!(isize);
impl_with_largest_bit_size!(u8);
impl_with_largest_bit_size!(u16);
impl_with_largest_bit_size!(u32);
impl_with_largest_bit_size!(u64);
impl_with_largest_bit_size!(u128);
impl_with_largest_bit_size!(usize);

impl_with_largest_bit_size2!(i8, i16, i32, isize, i64, i128);
impl_with_largest_bit_size2!(i16, i32, isize, i64, i128);
impl_with_largest_bit_size2!(i32, isize, i64, i128);
impl_with_largest_bit_size2!(isize, i64, i128);
impl_with_largest_bit_size2!(i64, i128);

impl_with_largest_bit_size2!(u8, u16, u32, usize, u64, u128);
impl_with_largest_bit_size2!(u16, u32, usize, u64, u128);
impl_with_largest_bit_size2!(u32, usize, u64, u128);
impl_with_largest_bit_size2!(usize, u64, u128);
impl_with_largest_bit_size2!(u64, u128);

impl_with_largest_bit_size3!(
  i8,
  (u8, i8),
  (u16, i16),
  (u32, i32),
  (usize, isize),
  (u64, i64),
  (u128, i128)
);
impl_with_largest_bit_size3!(
  i16,
  (u16, i16),
  (u32, i32),
  (usize, isize),
  (u64, i64),
  (u128, i128)
);
impl_with_largest_bit_size3!(i32, (u32, i32), (usize, isize), (u64, i64), (u128, i128));
impl_with_largest_bit_size3!(isize, (usize, isize), (u64, i64), (u128, i128));
impl_with_largest_bit_size3!(i64, (u64, i64), (u128, i128));
impl_with_largest_bit_size3!(i128, (u128, i128));

impl_with_largest_bit_size3!(
  u8,
  (i8, u8),
  (i16, u16),
  (i32, u32),
  (isize, usize),
  (i64, u64),
  (i128, u128)
);
impl_with_largest_bit_size3!(
  u16,
  (i16, u16),
  (i32, u32),
  (isize, usize),
  (i64, u64),
  (i128, u128)
);
impl_with_largest_bit_size3!(u32, (i32, u32), (isize, usize), (i64, u64), (i128, u128));
impl_with_largest_bit_size3!(usize, (isize, usize), (i64, u64), (i128, u128));
impl_with_largest_bit_size3!(u64, (i64, u64), (i128, u128));
impl_with_largest_bit_size3!(u128, (i128, u128));
