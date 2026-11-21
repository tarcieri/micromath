//! Natural log (ln) approximation for a single-precision float.
//!
//! Method described at: <https://stackoverflow.com/a/44232045/>
//!
//! Modified to not be restricted to int range and only values of x above 1.0.
//! Also got rid of most of the slow conversions. Handles all inputs: NaN for negative numbers,
//! -inf for zero.

use super::{EXPONENT_MASK, F32};
use core::f32::consts::LN_2;

/// 2^24, enough to lift any subnormal into the normal range.
const SUBNORMAL_SCALE: f32 = 16_777_216.0;

/// Base 2 exponent of [`SUBNORMAL_SCALE`].
const SUBNORMAL_SCALE_EXPONENT: i32 = 24;

impl F32 {
    /// Approximates the natural logarithm of the number.
    // Note: excessive precision ignored because it hides the origin of the numbers used for the
    // ln(1.0->2.0) polynomial
    #[allow(clippy::excessive_precision)]
    pub fn ln(self) -> Self {
        // Negative numbers and NaN
        if self.is_nan() || self < Self::ZERO {
            return Self::NAN;
        }

        // Matches both +0.0 and -0.0
        if self == Self::ZERO {
            return Self::NEG_INFINITY;
        }

        if self == Self::INFINITY {
            return Self::INFINITY;
        }

        // x may essentially be 1.0 but, as clippy notes, these kinds of
        // floating point comparisons can fail when the bit pattern is not the sames
        if (self - Self::ONE).abs() < f32::EPSILON {
            return Self::ZERO;
        }

        let x_less_than_1 = self < 1.0;

        // For x < 1 we compute ln(1/x) and negate it. This is a true division: the fast `inv`
        // approximation is too imprecise here.
        //
        // `1/x` overflows below 2^-128, so lift subnormals by 2^24 first (exact for subnormals)
        // and fold that back into the exponent: ln(1/x) = ln(1/(x * 2^24)) + 24 * ln(2).
        let (x_working, scale_exponent) = if !x_less_than_1 {
            (self, 0)
        } else if self < Self::MIN_POSITIVE {
            (
                Self::ONE / (self * SUBNORMAL_SCALE),
                SUBNORMAL_SCALE_EXPONENT,
            )
        } else {
            (Self::ONE / self, 0)
        };

        // according to the SO post ln(x) = ln((2^n)*y)= ln(2^n) + ln(y) = ln(2) * n + ln(y)
        // get exponent value
        let base2_exponent = (x_working.extract_exponent_value() + scale_exponent) as u32;
        let divisor = f32::from_bits(x_working.to_bits() & EXPONENT_MASK);

        // supposedly normalizing between 1.0 and 2.0
        let x_working = x_working / divisor;

        // approximate polynomial generated from maple in the post using Remez Algorithm:
        // https://en.wikipedia.org/wiki/Remez_algorithm
        let ln_1to2_polynomial = -1.741_793_9
            + (2.821_202_6
                + (-1.469_956_8 + (0.447_179_55 - 0.056_570_851 * x_working) * x_working)
                    * x_working)
                * x_working;

        // ln(2) * n + ln(y)
        let result = (base2_exponent as f32) * LN_2 + ln_1to2_polynomial;

        if x_less_than_1 {
            -result
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::F32;

    pub(crate) const MAX_ERROR: f32 = 0.001;

    /// ln(x) test vectors - `(input, output)`
    pub(crate) const TEST_VECTORS: &[(f32, f32)] = &[
        (1e-20, -46.0517),
        (1e-19, -43.749115),
        (1e-18, -41.446533),
        (1e-17, -39.143948),
        (1e-16, -36.841362),
        (1e-15, -34.538776),
        (1e-14, -32.23619),
        (1e-13, -29.933607),
        (1e-12, -27.631021),
        (1e-11, -25.328436),
        (1e-10, -23.02585),
        (1e-09, -20.723267),
        (1e-08, -18.420681),
        (1e-07, -16.118095),
        (1e-06, -13.815511),
        (1e-05, -11.512925),
        (1e-04, -9.2103405),
        (0.001, -6.9077554),
        (0.01, -4.6051702),
        (0.1, -2.3025851),
        (0.5, -0.6931472),
        (0.75, -0.2876821),
        (0.9, -0.1053605),
        (10.0, 2.3025851),
        (100.0, 4.6051702),
        (1000.0, 6.9077554),
        (10000.0, 9.2103405),
        (100000.0, 11.512925),
        (1000000.0, 13.815511),
        (10000000.0, 16.118095),
        (100000000.0, 18.420681),
        (1000000000.0, 20.723267),
        (10000000000.0, 23.02585),
        (100000000000.0, 25.328436),
        (1000000000000.0, 27.631021),
        (10000000000000.0, 29.933607),
        (100000000000000.0, 32.23619),
        (1000000000000000.0, 34.538776),
        (1e+16, 36.841362),
        (1e+17, 39.143948),
        (1e+18, 41.446533),
        (1e+19, 43.749115),
    ];

    #[test]
    fn special_values() {
        assert!(F32(-1.0).ln().is_nan());
        assert!(F32(-1e-40).ln().is_nan());
        assert!(F32(f32::NEG_INFINITY).ln().is_nan());
        assert!(F32::NAN.ln().is_nan());
        assert_eq!(F32(0.0).ln(), F32::NEG_INFINITY);
        assert_eq!(F32(-0.0).ln(), F32::NEG_INFINITY);
        assert_eq!(F32::INFINITY.ln(), F32::INFINITY);
    }

    #[test]
    fn subnormals() {
        // 2^-128, where `1/x` first overflows, its neighbours, and the smallest and largest
        // subnormals
        for &bits in &[
            0x0020_0000_u32,
            0x001f_ffff,
            0x0020_0001,
            0x0000_0001,
            0x007f_ffff,
        ] {
            let x = f32::from_bits(bits);
            let expected = (bits as f32).ln() - 149.0 * core::f32::consts::LN_2;
            let ln_x = F32(x).ln().0;
            let relative_error = (ln_x - expected).abs() / expected.abs();

            assert!(
                relative_error <= MAX_ERROR,
                "relative_error {} too large: ln({:e}) reads {} vs {}",
                relative_error,
                x,
                ln_x,
                expected
            );
        }
    }

    #[test]
    fn sanity_check() {
        assert_eq!(F32::ONE.ln(), F32::ZERO);
        for &(x, expected) in TEST_VECTORS {
            let ln_x = F32(x).ln();
            let relative_error = (ln_x - expected).abs() / expected.abs();

            assert!(
                relative_error <= MAX_ERROR,
                "relative_error {} too large: {} vs {}",
                relative_error,
                ln_x,
                expected
            );
        }
    }
}
