use core::{fmt, marker::PhantomData, ops};

/// A fixed-point number with `N` bits of precision.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FixedPoint<T, const N: u8>(T);
impl<T, const N: u8> FixedPoint<T, N> {
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
impl<T, const N: u8> FixedPoint<T, N>
where
  T: ops::Shl<u8, Output = T> + ops::Shr<u8, Output = T>,
{
  /// Creates a fixed-point number from an integer. Wraps on overflow
  #[inline]
  pub fn from_wrapping(value: T) -> Self {
    Self(value << N)
  }

  /// Truncates a fixed-point number to an integer.
  #[inline]
  pub fn truncate(self) -> T {
    self.0 >> N
  }

  /// Changes the number bits used for the fractional part. Wraps on overflow.
  #[inline]
  pub fn change_precision<const M: u8>(self) -> FixedPoint<T, M> {
    if N < M {
      FixedPoint(self.0 << (M - N))
    } else {
      FixedPoint(self.0 >> (N - M))
    }
  }
}
impl<T: ops::Add<Output = T>, const N: u8> ops::Add for FixedPoint<T, N> {
  type Output = Self;
  #[inline]
  fn add(self, rhs: Self) -> Self::Output {
    Self(self.0 + rhs.0)
  }
}
impl<T: ops::Sub<Output = T>, const N: u8> ops::Sub for FixedPoint<T, N> {
  type Output = Self;
  #[inline]
  fn sub(self, rhs: Self) -> Self::Output {
    Self(self.0 - rhs.0)
  }
}
impl<T: ops::Mul<Output = T>, const N: u8> ops::Mul<T> for FixedPoint<T, N> {
  type Output = Self;
  #[inline]
  fn mul(self, rhs: T) -> Self::Output {
    Self(self.0 * rhs)
  }
}
impl<T: ops::Div<Output = T>, const N: u8> ops::Div<T> for FixedPoint<T, N> {
  type Output = Self;
  #[inline]
  fn div(self, rhs: T) -> Self::Output {
    Self(self.0 / rhs)
  }
}
impl<T: Copy, const N: u8> fmt::Display for FixedPoint<T, N>
where
  f64: From<T>,
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    (f64::from(self.repr()) / <f64 as From<u32>>::from(1u32 << N)).fmt(f)
  }
}
impl<T, const N: u8> fmt::Debug for FixedPoint<T, N>
where
  FixedPoint<T, N>: fmt::Display,
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    <Self as fmt::Display>::fmt(self, f)
  }
}

pub type FixedU16 = FixedPoint<u32, 16>;
pub type FixedU8 = FixedPoint<u32, 8>;
pub type FixedU3 = FixedPoint<u32, 3>;
pub type FixedI16 = FixedPoint<i32, 16>;

/// The linear coordinate system.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinearSystem;

/// The isometric coordinate systems used by the camera.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IsometricSystem;

/// The coordinate system is unknown at compile time.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnknownSystem;

/// A two dimensional position in a specific coordinate system.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Pos<T, System> {
  pub x: T,
  pub y: T,
  system: PhantomData<System>,
}
impl<T, System> Pos<T, System> {
  #[inline]
  pub const fn new(x: T, y: T) -> Self {
    Self { x, y, system: PhantomData }
  }

  #[inline]
  pub fn cast<U>(self, mut f: impl FnMut(T) -> U) -> Pos<U, System> {
    Pos::new(f(self.x), f(self.y))
  }
}
impl<T: fmt::Display, System> fmt::Display for Pos<T, System> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}
impl<T: fmt::Debug, System> fmt::Debug for Pos<T, System> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("").field(&self.x).field(&self.y).finish()
  }
}

pub type LinearPos<T> = Pos<T, LinearSystem>;
pub type IsoPos<T> = Pos<T, IsometricSystem>;
pub type UnknownPos<T> = Pos<T, UnknownSystem>;

impl From<Pos<FixedU16, LinearSystem>> for Pos<i32, IsometricSystem> {
  fn from(value: Pos<FixedU16, LinearSystem>) -> Self {
    let x = value.x.change_precision::<5>().repr() as i32;
    let y = value.y.change_precision::<5>().repr() as i32;
    Pos::new((x.wrapping_sub(y)) >> 1, (x.wrapping_add(y)) >> 2)
  }
}

impl From<Pos<FixedI16, LinearSystem>> for Pos<i32, IsometricSystem> {
  fn from(value: Pos<FixedI16, LinearSystem>) -> Self {
    let x = value.x.change_precision::<5>().repr();
    let y = value.y.change_precision::<5>().repr();
    Pos::new((x.wrapping_sub(y)) / 2, (x.wrapping_add(y)) / 4)
  }
}

/// A two dimensional size.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Size<T> {
  pub width: T,
  pub height: T,
}
impl<T> Size<T> {
  #[inline]
  pub fn new(width: T, height: T) -> Self {
    Self { width, height }
  }
}

/// A rectangle defined by two points.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Rect<T, System> {
  pub upper_left: Pos<T, System>,
  pub lower_right: Pos<T, System>,
}
