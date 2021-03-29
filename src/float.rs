//! Floating point operations

pub(crate) mod abs;
pub(crate) mod acos;
pub(crate) mod asin;
pub(crate) mod atan;
pub(crate) mod atan2;
pub(crate) mod ceil;
pub(crate) mod copysign;
pub(crate) mod cos;
pub(crate) mod div_euclid;
pub(crate) mod exp;
pub(crate) mod floor;
pub(crate) mod fract;
pub(crate) mod hypot;
pub(crate) mod inv;
pub(crate) mod invsqrt;
pub(crate) mod ln;
pub(crate) mod log;
pub(crate) mod log10;
pub(crate) mod log2;
pub(crate) mod powf;
pub(crate) mod powi;
pub(crate) mod rem_euclid;
pub(crate) mod round;
pub(crate) mod sin;
pub(crate) mod sqrt;
pub(crate) mod tan;
pub(crate) mod trunc;
pub(crate) mod utils;

use core::{
    fmt::{self, Display, LowerExp, UpperExp},
    iter::{Product, Sum},
    num::ParseFloatError,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
    str::FromStr,
};

/// 32-bit floating point wrapper which implements fast approximation-based
/// operations.
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct F32(pub f32);

impl Add for F32 {
    type Output = F32;

    #[inline]
    fn add(self, rhs: F32) -> F32 {
        F32(self.0 + rhs.0)
    }
}

impl Add<f32> for F32 {
    type Output = F32;

    #[inline]
    fn add(self, rhs: f32) -> F32 {
        F32(self.0 + rhs)
    }
}

impl Add<F32> for f32 {
    type Output = F32;

    #[inline]
    fn add(self, rhs: F32) -> F32 {
        F32(self + rhs.0)
    }
}

impl AddAssign for F32 {
    #[inline]
    fn add_assign(&mut self, rhs: F32) {
        self.0 += rhs.0;
    }
}

impl AddAssign<f32> for F32 {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        self.0 += rhs;
    }
}

impl AddAssign<F32> for f32 {
    #[inline]
    fn add_assign(&mut self, rhs: F32) {
        *self /= rhs.0;
    }
}

impl Display for F32 {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

impl Div for F32 {
    type Output = F32;

    #[inline]
    fn div(self, rhs: F32) -> F32 {
        F32(self.0 / rhs.0)
    }
}

impl Div<f32> for F32 {
    type Output = F32;

    #[inline]
    fn div(self, rhs: f32) -> F32 {
        F32(self.0 / rhs)
    }
}

impl Div<F32> for f32 {
    type Output = F32;

    #[inline]
    fn div(self, rhs: F32) -> F32 {
        F32(self / rhs.0)
    }
}

impl DivAssign for F32 {
    #[inline]
    fn div_assign(&mut self, rhs: F32) {
        self.0 /= rhs.0;
    }
}

impl DivAssign<f32> for F32 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs;
    }
}

impl DivAssign<F32> for f32 {
    #[inline]
    fn div_assign(&mut self, rhs: F32) {
        *self /= rhs.0;
    }
}

impl From<f32> for F32 {
    #[inline]
    fn from(n: f32) -> F32 {
        F32(n)
    }
}

impl From<F32> for f32 {
    #[inline]
    fn from(n: F32) -> f32 {
        n.0
    }
}

impl From<i8> for F32 {
    #[inline]
    fn from(n: i8) -> F32 {
        F32(n.into())
    }
}

impl From<i16> for F32 {
    #[inline]
    fn from(n: i16) -> F32 {
        F32(n.into())
    }
}

impl From<u8> for F32 {
    #[inline]
    fn from(n: u8) -> F32 {
        F32(n.into())
    }
}

impl From<u16> for F32 {
    #[inline]
    fn from(n: u16) -> F32 {
        F32(n.into())
    }
}

impl FromStr for F32 {
    type Err = ParseFloatError;

    #[inline]
    fn from_str(src: &str) -> Result<F32, ParseFloatError> {
        f32::from_str(src).map(F32)
    }
}

impl LowerExp for F32 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:e}", self.0)
    }
}

impl Mul for F32 {
    type Output = F32;

    #[inline]
    fn mul(self, rhs: F32) -> F32 {
        F32(self.0 * rhs.0)
    }
}

impl Mul<f32> for F32 {
    type Output = F32;

    #[inline]
    fn mul(self, rhs: f32) -> F32 {
        F32(self.0 * rhs)
    }
}

impl Mul<F32> for f32 {
    type Output = F32;

    #[inline]
    fn mul(self, rhs: F32) -> F32 {
        F32(self * rhs.0)
    }
}

impl MulAssign for F32 {
    #[inline]
    fn mul_assign(&mut self, rhs: F32) {
        self.0 *= rhs.0;
    }
}

impl MulAssign<f32> for F32 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
    }
}

impl MulAssign<F32> for f32 {
    #[inline]
    fn mul_assign(&mut self, rhs: F32) {
        *self *= rhs.0;
    }
}

impl Neg for F32 {
    type Output = F32;

    #[inline]
    fn neg(self) -> F32 {
        F32(-self.0)
    }
}

impl Product for F32 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = F32>,
    {
        F32(f32::product(iter.map(f32::from)))
    }
}

impl Rem for F32 {
    type Output = F32;

    #[inline]
    fn rem(self, rhs: F32) -> F32 {
        F32(self.0 % rhs.0)
    }
}

impl Rem<f32> for F32 {
    type Output = F32;

    #[inline]
    fn rem(self, rhs: f32) -> F32 {
        F32(self.0 % rhs)
    }
}

impl Rem<F32> for f32 {
    type Output = F32;

    #[inline]
    fn rem(self, rhs: F32) -> F32 {
        F32(self % rhs.0)
    }
}

impl RemAssign for F32 {
    #[inline]
    fn rem_assign(&mut self, rhs: F32) {
        self.0 %= rhs.0;
    }
}

impl RemAssign<f32> for F32 {
    #[inline]
    fn rem_assign(&mut self, rhs: f32) {
        self.0 %= rhs;
    }
}

impl Sub for F32 {
    type Output = F32;

    #[inline]
    fn sub(self, rhs: F32) -> F32 {
        F32(self.0 - rhs.0)
    }
}

impl Sub<f32> for F32 {
    type Output = F32;

    #[inline]
    fn sub(self, rhs: f32) -> F32 {
        F32(self.0 - rhs)
    }
}

impl Sub<F32> for f32 {
    type Output = F32;

    #[inline]
    fn sub(self, rhs: F32) -> F32 {
        F32(self - rhs.0)
    }
}

impl SubAssign for F32 {
    #[inline]
    fn sub_assign(&mut self, rhs: F32) {
        self.0 -= rhs.0;
    }
}

impl SubAssign<f32> for F32 {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        self.0 -= rhs;
    }
}

impl SubAssign<F32> for f32 {
    #[inline]
    fn sub_assign(&mut self, rhs: F32) {
        *self -= rhs.0;
    }
}

impl Sum for F32 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = F32>,
    {
        F32(f32::sum(iter.map(f32::from)))
    }
}

impl UpperExp for F32 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:E}", self.0)
    }
}
