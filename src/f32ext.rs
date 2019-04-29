/// `f32` extension providing various arithmetic approximations and polyfills
/// for `std` functionality.
mod abs;
mod atan;
mod atan2;
mod sqrt;

/// `f32` extension providing various arithmetic approximations and polyfills
/// for `std` functionality.
pub trait F32Ext: Sized {
    /// Compute absolute value.
    ///
    /// Provides a constant-time, data-independent implementation.
    fn abs(self) -> f32;

    /// Computes an `atan` approximation in radians.
    fn atan(self) -> f32;

    /// Approximates `atan(x)` normalized to the `[−1,1]` range with a maximum
    /// error of `0.1620` degrees.
    fn atan_norm(self) -> f32;

    /// Compute four quadrant arctangent
    fn atan2(self, other: f32) -> f32;

    /// Approximates the four quadrant arctangent.
    /// Normalized to the `[0,4)` range with a maximum error of `0.1620` degrees.
    fn atan2_norm(self, other: f32) -> f32;

    /// Compute square root
    fn sqrt(self) -> f32;
}

impl F32Ext for f32 {
    fn abs(self) -> f32 {
        self::abs::abs(self)
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

    fn sqrt(self) -> f32 {
        self::sqrt::sqrt_approx(self)
    }
}
