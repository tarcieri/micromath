//! arccos approximation for a single-precision float using method
//! described at:
//!
//! <https://math.stackexchange.com/questions/2908908/express-arccos-in-terms-of-arctan>

use super::{atan::atan_approx, sqrt::sqrt_approx};

/// Computes `acos(x)` approximation in radians in the range `[0, pi]`
pub(super) fn acos_approx(x: f32) -> f32 {
    atan_approx(sqrt_approx(1.0 - x * x) / x)
}

#[cfg(test)]
mod tests {
    use super::acos_approx;
    use crate::f32ext::{abs::abs, cos::cos_approx};
    use core::f32;

    const MAX_ERROR: f32 = 0.03;

    #[test]
    fn sanity_check() {
        let f = f32::consts::FRAC_PI_4;
        let abs_difference = abs(acos_approx(cos_approx(f)) - f32::consts::FRAC_PI_4);
        assert!(abs_difference <= MAX_ERROR);
    }
}
