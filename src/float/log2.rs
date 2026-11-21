//! log base 2 approximation for a single-precision float.

use super::{F32, MANTISSA_MASK};
use core::f32::consts::{LOG2_E, SQRT_2};

impl F32 {
    /// Approximates the base 2 logarithm of the number.
    #[inline]
    pub fn log2(self) -> Self {
        let x = self.0;
        if x <= 0.0 {
            return if x == 0.0 {
                Self::NEG_INFINITY
            } else {
                Self::NAN
            };
        }
        if !x.is_finite() {
            return self;
        }
        let bits = self.to_bits();
        let (normal, adjustment) = if bits < f32::MIN_POSITIVE.to_bits() {
            // A positive subnormal is its integer fraction times 2^-149.
            (Self(bits as f32), -149)
        } else {
            (self, 0)
        };
        let mut e = normal.extract_exponent_value() + adjustment;
        let mut m = f32::from_bits((normal.to_bits() & MANTISSA_MASK) | 0x3f80_0000);
        if m > SQRT_2 {
            m *= 0.5;
            e += 1;
        }
        // ln(m) = 2 atanh((m - 1)/(m + 1)); |s| <= 0.172. Adding the
        // binary exponent last also gives exact results for powers of two.
        let s = (m - 1.0) / (m + 1.0);
        let z = s * s;
        let p = 1.0 + z * (1.0 / 3.0 + z * (1.0 / 5.0 + z * (1.0 / 7.0 + z / 9.0)));
        Self(e as f32 + (2.0 * LOG2_E) * s * p)
    }
}

#[cfg(test)]
mod tests {
    use super::F32;

    pub(crate) const MAX_ERROR: f32 = 0.001;

    /// log2(x) test vectors - `(input, output)`
    pub(crate) const TEST_VECTORS: &[(f32, f32)] = &[
        (1e-20, -66.43856),
        (1e-19, -63.116634),
        (1e-18, -59.794704),
        (1e-17, -56.47278),
        (1e-16, -53.15085),
        (1e-15, -49.828922),
        (1e-14, -46.506992),
        (1e-13, -43.185066),
        (1e-12, -39.863136),
        (1e-11, -36.54121),
        (1e-10, -33.21928),
        (1e-09, -29.897352),
        (1e-08, -26.575424),
        (1e-07, -23.253496),
        (1e-06, -19.931568),
        (1e-05, -16.60964),
        (1e-04, -13.287712),
        (0.001, -9.965784),
        (0.01, -6.643856),
        (0.1, -3.321928),
        (10.0, 3.321928),
        (100.0, 6.643856),
        (1000.0, 9.965784),
        (10000.0, 13.287712),
        (100000.0, 16.60964),
        (1000000.0, 19.931568),
        (10000000.0, 23.253496),
        (100000000.0, 26.575424),
        (1000000000.0, 29.897352),
        (10000000000.0, 33.21928),
        (100000000000.0, 36.54121),
        (1000000000000.0, 39.863136),
        (10000000000000.0, 43.185066),
        (100000000000000.0, 46.506992),
        (1000000000000000.0, 49.828922),
        (1e+16, 53.15085),
        (1e+17, 56.47278),
        (1e+18, 59.794704),
        (1e+19, 63.116634),
    ];

    #[test]
    fn sanity_check() {
        assert_eq!(F32::ONE.log2(), F32::ZERO);

        for &(x, expected) in TEST_VECTORS {
            let ln_x = F32(x).log2().0;
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

    #[test]
    fn binary_exponents_and_centered_mantissas() {
        assert_eq!(F32(-0.0).log2(), F32::NEG_INFINITY);
        assert_eq!(F32::INFINITY.log2(), F32::INFINITY);
        assert!(F32(-1.0).log2().is_nan() && F32::NAN.log2().is_nan());
        for k in -149..=127 {
            let power = if k < -126 {
                1 << (k + 149)
            } else {
                ((k + 127) as u32) << 23
            };
            assert_eq!(F32::from_bits(power).log2(), F32(k as f32));
        }
        for k in 0..255_u32 {
            for frac in [1, 0x233333, 0x3504f3, 0x400000, 0x7fffff] {
                let x = f32::from_bits((k << 23) | frac);
                let reference = f64::from(x).log2() as f32;
                let actual = F32(x).log2().0;
                assert!((actual - reference).abs() <= 4.0 * f32::EPSILON * reference.abs());
            }
        }
    }
}
