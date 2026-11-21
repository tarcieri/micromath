//! Exponential approximation for a single-precision float.

use super::{exp2::scale, F32};
use core::f32::consts::LOG2_E;

impl F32 {
    /// Approximates e raised to this power.
    #[inline]
    pub fn exp(self) -> Self {
        let x = self.0;
        if x >= 89.0 {
            return Self::INFINITY;
        }
        if x <= -104.0 {
            return Self::ZERO;
        }
        if x.is_nan() {
            return self;
        }
        // Reduce to |r| <= ln(2)/2. The high part has enough trailing
        // zero bits for the product with any exponent here to be exact.
        let n = x * LOG2_E;
        let k = (n + if n < 0.0 { -0.5 } else { 0.5 }) as i32;
        let r = (x - k as f32 * 0.693_145_75) - k as f32 * 1.428_606_8e-6;
        let tail = 1.0 / 120.0 + r * (1.0 / 720.0 + r / 5040.0);
        let p = 1.0 + r * (1.0 + r * (0.5 + r * (1.0 / 6.0 + r * (1.0 / 24.0 + r * tail))));
        Self(scale(p, k))
    }
}

#[cfg(test)]
mod tests {
    use super::F32;

    pub(crate) const MAX_ERROR: f32 = 0.001;

    /// exp test vectors - `(input, output)`
    pub(crate) const TEST_VECTORS: &[(f32, f32)] = &[
        (1e-07, 1.0000001),
        (1e-06, 1.000001),
        (1e-05, 1.00001),
        (1e-04, 1.0001),
        (0.001, 1.0010005),
        (0.01, 1.0100502),
        (0.1, 1.105171),
        (1.0, 2.7182817),
        (10.0, 22026.465),
        (-1e-08, 1.0),
        (-1e-07, 0.9999999),
        (-1e-06, 0.999999),
        (-1e-05, 0.99999),
        (-1e-04, 0.9999),
        (-0.001, 0.9990005),
        (-0.01, 0.99004984),
        (-0.1, 0.9048374),
        (-1.0, 0.36787945),
        (-10.0, 4.539_993e-5),
    ];

    #[test]
    fn sanity_check() {
        assert_eq!(F32(-1000000.0).exp(), F32::ZERO);

        for &(x, expected) in TEST_VECTORS {
            let exp_x = F32(x).exp();
            let relative_error = (exp_x - expected).abs() / expected;

            assert!(
                relative_error <= MAX_ERROR,
                "relative_error {} too large for input {} : {} vs {}",
                relative_error,
                x,
                exp_x,
                expected
            );
        }
    }

    #[test]
    fn reduction_and_scaling() {
        assert_eq!(F32::NEG_INFINITY.exp(), F32::ZERO);
        assert_eq!(F32::INFINITY.exp(), F32::INFINITY);
        assert!(F32::NAN.exp().is_nan());
        let check = |x: f32| {
            let expected = (f64::from(x).exp() as f32).to_bits();
            let actual = F32(x).exp().to_bits();
            assert!(
                expected.abs_diff(actual) <= 2,
                "exp({}) expected bits {:x}, got {:x}",
                x,
                expected,
                actual
            );
        };
        for i in -26_880..=23_040 {
            check(i as f32 / 256.0);
        }
        for center in [-103.972_08_f32, -87.336_55, 88.722_84] {
            for bits in center.to_bits() - 3..=center.to_bits() + 3 {
                check(f32::from_bits(bits));
            }
        }
    }
}
