//! arcsin approximation for a single-precision float using method
//! described at:
//!
//! <https://dsp.stackexchange.com/questions/25770/looking-for-an-arcsin-algorithm>

use super::{atan::atan_approx, invsqrt::invsqrt_approx};

/// Computes `asin(x)` approximation in radians in the range `[-pi/2, pi/2]`.
pub(super) fn asin_approx(x: f32) -> f32 {
    atan_approx(x * invsqrt_approx(1.0 - x * x))
}

#[cfg(test)]
mod tests {
    use super::asin_approx;
    use crate::f32ext::{abs::abs, sin::sin_approx};
    use core::f32;

    #[test]
    fn sanity_check() {
        let f = f32::consts::FRAC_PI_2;
        let abs_difference = abs(asin_approx(sin_approx(f)) - f32::consts::FRAC_PI_2);
        assert!(abs_difference <= f32::EPSILON);
    }
}
