//! arctangent approximation for a single-precision float using method
//! described at:
//!
//! <https://ieeexplore.ieee.org/document/6375931>

use super::abs::abs;
use core::f32;

/// Computes `atan(x)` approximation in radians.
pub(super) fn atan_approx(x: f32) -> f32 {
    f32::consts::FRAC_PI_2 * atan_norm_approx(x)
}

/// Approximates `atan(x)` normalized to the `[−1,1]` range with a maximum
/// error of `0.1620` degrees.
pub(super) fn atan_norm_approx(x: f32) -> f32 {
    const SIGN_MASK: u32 = 0x8000_0000;
    const B: f32 = 0.596_227;

    // Extract the sign bit
    let ux_s = SIGN_MASK & x.to_bits();

    // Calculate the arctangent in the first quadrant
    let bx_a = abs(B * x);
    let n = bx_a + x * x;
    let atan_1q = n / (1.0 + bx_a + n);

    // Restore the sign bit and convert to float
    f32::from_bits(ux_s | atan_1q.to_bits())
}

#[cfg(test)]
mod tests {
    use super::atan_approx;
    use core::f32;

    /// 0.1620 degrees in radians
    const MAX_ERROR: f32 = 0.003;

    #[test]
    fn sanity_check() {
        // Arctangent test vectors - `(input, output)`
        let test_vectors: &[(f32, f32)] = &[
            (3.0_f32.sqrt() / 3.0, f32::consts::FRAC_PI_6),
            (1.0, f32::consts::FRAC_PI_4),
            (3.0_f32.sqrt(), f32::consts::FRAC_PI_3),
            (-3.0_f32.sqrt() / 3.0, -f32::consts::FRAC_PI_6),
            (-1.0, -f32::consts::FRAC_PI_4),
            (-3.0_f32.sqrt(), -f32::consts::FRAC_PI_3),
        ];

        for (x, expected) in test_vectors {
            let actual = atan_approx(*x);
            let delta = actual - expected;

            assert!(
                delta <= MAX_ERROR,
                "delta {} too large: {} vs {}",
                delta,
                actual,
                expected
            );
        }
    }

    #[test]
    fn zero() {
        assert_eq!(atan_approx(0.0), 0.0);
    }

    #[test]
    fn nan() {
        assert!(atan_approx(f32::NAN).is_nan());
    }
}
