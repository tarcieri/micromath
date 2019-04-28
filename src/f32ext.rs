/// `f32` extension providing various arithmetic approximations and polyfills
/// for `std` functionality.
mod abs;
mod atan2;
mod radians;
mod sqrt;

/// `f32` extension providing various arithmetic approximations and polyfills
/// for `std` functionality.
pub trait F32Ext: Sized {
    /// Compute absolute value.
    ///
    /// Provides a constant-time, data-independent implementation.
    fn abs(self) -> f32;

    /// Compute four quadrant arctangent normalized between `[0, 4)`
    fn atan2_norm(self, other: f32) -> f32;

    /// Compute four quadrant arctangent
    fn atan2(self, other: f32) -> f32;

    /// Compute square root
    fn sqrt(self) -> f32;
}

impl F32Ext for f32 {
    fn abs(self) -> f32 {
        self::abs::abs(self)
    }

    fn atan2_norm(self, other: f32) -> f32 {
        self::atan2::atan2_norm_approx(self, other)
    }

    fn atan2(self, other: f32) -> f32 {
        self::atan2::atan2_approx(self, other)
    }

    fn sqrt(self) -> f32 {
        self::sqrt::sqrt_approx(self)
    }
}
