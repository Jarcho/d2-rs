use crate::Range;
use core::{
  fmt,
  ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};
use num::{
  Fixed, Measure, MulTrunc, WrappingAdd, WrappingDiv, WrappingFrom, WrappingInto, WrappingMul,
  WrappingSub,
};

macro_rules! impl_op {
  ($op:ident $(<$trait:ident>)?, $ty:ident $(<$ty_arg:ident>)?, $other_ty:ty, $fn:ident, $(($field_lhs:tt, $($field_rhs:tt)?)),+) => {
    impl<T: $op<U>, U $(: $trait)?, $($ty_arg)?> $op<$other_ty> for $ty<T, $($ty_arg)?> {
      type Output = $ty<T::Output, $($ty_arg)?>;
      fn $fn(self, rhs: $other_ty) -> Self::Output {
        $ty::new($(
          $op::$fn(self.$field_lhs, rhs$(.$field_rhs)?)
        ),+)
      }
    }
  };
}

macro_rules! impl_op_assign {
  ($op:ident $(<$trait:ident>)?, $ty:ident $(<$ty_arg:ident>)?, $other_ty:ty, $fn:ident, $(($field_lhs:tt, $($field_rhs:tt)?)),+) => {
    impl<T: $op<U>, U $(: $trait)?, $($ty_arg)?> $op<$other_ty> for $ty<T, $($ty_arg)?> {
      fn $fn(&mut self, rhs: $other_ty) {$(
        $op::$fn(&mut self.$field_lhs, rhs$(.$field_rhs)?);
      )+}
    }
  };
}

/// The main coordinate system used to position entities.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LinearSys;

/// The isometric coordinate system used to position the camera.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IsoSys;

/// The coordinate system is unknown at compile time.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnknownSys;

/// The coordinate system used to position things on the screen. Origin is the upper-left.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScreenSys;

/// The coordinate system used to position entities on tiles.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TileSys;

/// A two dimensional position in a specific coordinate system.
#[derive(Default, Clone, Copy, Eq)]
#[repr(C)]
pub struct Pos<T> {
  pub x: T,
  pub y: T,
}
impl<T> Pos<T> {
  #[inline]
  pub const fn new(x: T, y: T) -> Self {
    Self { x, y }
  }

  pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> Pos<U> {
    Pos::new(f(self.x), f(self.y))
  }
}

impl<T: fmt::Display> fmt::Display for Pos<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}
impl<T: fmt::Debug> fmt::Debug for Pos<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("").field(&self.x).field(&self.y).finish()
  }
}

impl<T: PartialEq<U>, U> PartialEq<Pos<U>> for Pos<T> {
  fn eq(&self, other: &Pos<U>) -> bool {
    self.x == other.x && self.y == other.y
  }
}

impl<T: WrappingFrom<U>, U> WrappingFrom<Pos<U>> for Pos<T> {
  fn wfrom(x: Pos<U>) -> Self {
    Pos::new(x.x.winto(), x.y.winto())
  }
}

impl<T: Neg> Neg for Pos<T> {
  type Output = Pos<T::Output>;
  fn neg(self) -> Self::Output {
    Pos::new(-self.x, -self.y)
  }
}

impl_op!(Add, Pos, Pos<U>, add, (x, x), (y, y));
impl_op!(Sub, Pos, Pos<U>, sub, (x, x), (y, y));
impl_op!(Mul<Copy>, Pos, U, mul, (x,), (y,));
impl_op!(MulTrunc<Copy>, Pos, U, mul_trunc, (x,), (y,));
impl_op!(Div<Copy>, Pos, U, div, (x,), (y,));
impl_op!(WrappingAdd, Pos, Pos<U>, wadd, (x, x), (y, y));
impl_op!(WrappingSub, Pos, Pos<U>, wsub, (x, x), (y, y));
impl_op!(WrappingMul<Copy>, Pos, U, wmul, (x,), (y,));
impl_op!(WrappingDiv<Copy>, Pos, U, wdiv, (x,), (y,));

impl_op_assign!(AddAssign, Pos, Pos<U>, add_assign, (x, x), (y, y));
impl_op_assign!(SubAssign, Pos, Pos<U>, sub_assign, (x, x), (y, y));
impl_op_assign!(MulAssign<Copy>, Pos, U, mul_assign, (x,), (y,));
impl_op_assign!(DivAssign<Copy>, Pos, U, div_assign, (x,), (y,));

pub type LinearPos<T> = Pos<Measure<T, LinearSys>>;
pub type IsoPos<T> = Pos<Measure<T, IsoSys>>;
pub type TilePos<T> = Pos<Measure<T, TileSys>>;
pub type ScreenPos<T> = Pos<Measure<T, ScreenSys>>;

impl<const N: u8> From<LinearPos<Fixed<u32, N>>> for IsoPos<i32> {
  fn from(p: LinearPos<Fixed<u32, N>>) -> Self {
    let x = p.x.0.with_prec::<5>().repr() as i32;
    let y = p.y.0.with_prec::<5>().repr() as i32;
    IsoPos::new(
      Measure::new((x.wrapping_sub(y)) >> 1),
      Measure::new((x.wrapping_add(y)) >> 2),
    )
  }
}

/// A two dimensional size.
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Size<T> {
  pub width: T,
  pub height: T,
}
impl<T> Size<T> {
  pub const fn new(width: T, height: T) -> Self {
    Self { width, height }
  }
}

/// A rectangle defined by two points.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Rect<T> {
  pub upper_left: Pos<T>,
  pub lower_right: Pos<T>,
}

/// A rectangle defined by the x-bounds and y-bounds.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RectLr<T> {
  pub x: Range<T>,
  pub y: Range<T>,
}

pub type ScreenRectLr<T> = RectLr<Measure<T, ScreenSys>>;

/// A rectangle defined by a position and size.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RectS<T, U> {
  pub pos: Pos<T>,
  pub size: Size<U>,
}

pub type ScreenRectS<T, U> = RectS<Measure<T, ScreenSys>, Measure<U, ScreenSys>>;
