//! `f32` extension providing various arithmetic approximations and polyfills
//! for `std` functionality.

mod abs;
mod acos;
mod asin;
mod atan;
mod atan2;
mod ceil;
mod copysign;
mod cos;
mod div_euclid;
mod exp;
mod floor;
mod fract;
mod hypot;
mod inv;
mod invsqrt;
mod ln;
mod log;
mod log10;
mod log2;
mod powf;
mod powi;
mod rem_euclid;
mod round;
mod sin;
mod sqrt;
mod tan;
mod trunc;
mod utils;

/// `f32` extension providing various arithmetic approximations and polyfills
/// for `std` functionality.
pub trait F32Ext: Sized {
    /// Compute absolute value with a constant-time, data-independent
    /// implementation.
    fn abs(self) -> f32;

    /// Approximate `asin(x)` in radians in the range `[-pi/2, pi/2]`.
    fn asin(self) -> f32;

    /// Approximate `acos(x)` in radians in the range `[0, pi]`
    fn acos(self) -> f32;

    /// Approximate `atan(x)` in radians with a maximum error of `0.002`.
    fn atan(self) -> f32;

    /// Approximate `atan(x)` normalized to the `[−1,1]` range with a maximum
    /// error of `0.1620` degrees.
    fn atan_norm(self) -> f32;

    /// Approximate the four quadrant arctangent `atan2(x)` in radians, with
    /// a maximum error of `0.002`.
    fn atan2(self, other: f32) -> f32;

    /// Approximate the four quadrant arctangent.
    /// Normalized to the `[0,4)` range with a maximum error of `0.1620` degrees.
    fn atan2_norm(self, other: f32) -> f32;

    /// Approximate floating point ceiling.
    fn ceil(self) -> f32;

    /// Approximate cosine in radians with a maximum error of `0.002`.
    fn cos(self) -> f32;

    /// Calculates Euclidean division, the matching method for `rem_euclid`.
    fn div_euclid(self, other: f32) -> f32;

    /// Approximate floating point floor.
    fn floor(self) -> f32;

    /// Approximate the length of the hypotenuse of a right-angle triangle given
    /// legs of length `x` and `y`.
    fn hypot(self, other: f32) -> f32;

    /// Approximate `1/x` with an average deviation of ~8%.
    fn inv(self) -> f32;

    /// Approximate inverse square root with an average deviation of ~5%.
    fn invsqrt(self) -> f32;

    /// Calculates the least nonnegative remainder of `self (mod other)`.
    fn rem_euclid(self, other: f32) -> f32;

    /// Approximate sine in radians with a maximum error of `0.002`.
    fn sin(self) -> f32;

    /// Approximate square root with an average deviation of ~5%.
    fn sqrt(self) -> f32;

    /// Approximate `tan(x)` in radians with a maximum error of `0.6`.
    fn tan(self) -> f32;

    /// Retrieve whole number part of floating point with sign.
    fn trunc(self) -> f32;

    /// Round the number part of floating point with sign.
    fn round(self) -> f32;

    /// Retrieve the fractional part of floating point with sign.
    fn fract(self) -> f32;

    /// Copies the sign from one number to another and returns it.
    fn copysign(self, sign: f32) -> f32;

    /// Approximate `ln(x)`.
    fn ln(self) -> f32;

    /// Approximate `e^x`.
    fn exp(self) -> f32;

    /// Approximate `log` with an arbitrary base.
    fn log(self, base: f32) -> f32;

    /// Approximate `log2`.
    fn log2(self) -> f32;

    /// Approximate `log10`.
    fn log10(self) -> f32;

    /// Approximate `self^n`.
    fn powf(self, n: f32) -> f32;

    /// Approximate `self^n` where n is an `i32`
    fn powi(self, n: i32) -> f32;
}

impl F32Ext for f32 {
    fn abs(self) -> f32 {
        self::abs::abs(self)
    }

    fn asin(self) -> f32 {
        self::asin::asin_approx(self)
    }

    fn acos(self) -> f32 {
        self::acos::acos_approx(self)
    }

    fn atan(self) -> f32 {
        self::atan::atan_approx(self)
    }

    fn atan_norm(self) -> f32 {
        self::atan::atan_norm_approx(self)
    }

    fn atan2(self, other: f32) -> f32 {
        self::atan2::atan2_approx(self, other)
    }

    fn atan2_norm(self, other: f32) -> f32 {
        self::atan2::atan2_norm_approx(self, other)
    }

    fn ceil(self) -> f32 {
        self::ceil::ceil(self)
    }

    fn cos(self) -> f32 {
        self::cos::cos_approx(self)
    }

    fn div_euclid(self, other: f32) -> f32 {
        self::div_euclid::div_euclid(self, other)
    }

    fn floor(self) -> f32 {
        self::floor::floor(self)
    }

    fn hypot(self, other: f32) -> f32 {
        self::hypot::hypot_approx(self, other)
    }

    fn inv(self) -> f32 {
        self::inv::inv_approx(self)
    }

    fn invsqrt(self) -> f32 {
        self::invsqrt::invsqrt_approx(self)
    }

    fn rem_euclid(self, other: f32) -> f32 {
        self::rem_euclid::rem_euclid(self, other)
    }

    fn sin(self) -> f32 {
        self::sin::sin_approx(self)
    }

    fn sqrt(self) -> f32 {
        self::sqrt::sqrt_approx(self)
    }

    fn tan(self) -> f32 {
        self::tan::tan_approx(self)
    }

    fn trunc(self) -> f32 {
        self::trunc::trunc_sign(self)
    }

    fn round(self) -> f32 {
        self::round::round(self)
    }

    fn fract(self) -> f32 {
        self::fract::fract_sign(self)
    }

    fn copysign(self, sign: f32) -> f32 {
        self::copysign::copysign(self, sign)
    }

    fn ln(self) -> f32 {
        self::ln::ln_1to2_series_approximation(self)
    }

    fn exp(self) -> f32 {
        self::exp::exp_ln2_approximation(self, 4)
    }

    fn log(self, base: f32) -> f32 {
        self::log::log_ln_approx(self, base)
    }

    fn log2(self) -> f32 {
        self::log2::log2_ln_approx(self)
    }

    fn log10(self) -> f32 {
        self::log10::log10_ln_approx(self)
    }

    fn powf(self, n: f32) -> f32 {
        self::powf::powf_exp_ln_approx(self, n)
    }

    fn powi(self, n: i32) -> f32 {
        self::powi::powi_exp_by_squaring(self, n)
    }
}
