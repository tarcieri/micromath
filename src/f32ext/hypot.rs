//! Calculate length of the hypotenuse of a right triangle

use super::sqrt::sqrt_approx;

/// Calculate the length of the hypotenuse of a right-angle triangle given
/// legs of length `x` and `y`.
pub(super) fn hypot_approx(x: f32, y: f32) -> f32 {
    sqrt_approx(x * x + y * y)
}

#[cfg(test)]
mod tests {
    use super::{hypot_approx, sqrt_approx};
    use crate::f32ext::abs::abs;
    use core::f32;

    #[test]
    fn sanity_check() {
        let x = 3.0f32;
        let y = 4.0f32;
        let abs_difference = abs(hypot_approx(x, y) - sqrt_approx(25.0));
        assert!(abs_difference <= f32::EPSILON);
    }
}
