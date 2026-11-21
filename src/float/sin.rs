//! Sine approximation, implemented in terms of `cos(x)`.

use super::F32;
use core::f32::consts::PI;

impl F32 {
    /// Approximates `sin(x)` in radians with a maximum error of `0.002`.
    pub fn sin(self) -> Self {
        (self - PI / 2.0).cos()
    }

    /// Approximates sin(2π × turns), with maximum absolute error below 0.002.
    /// One full rotation is one turn. Quarter-turns have exact values.
    #[inline]
    pub fn sin_turns(self) -> Self {
        let a = self.abs();
        let phase = a.0 - a.floor().0;
        let t = if phase < 0.5 {
            2.0 * phase
        } else {
            2.0 * phase - 2.0
        };
        let y = 4.0 * t * (1.0 - Self(t).abs().0);
        // Parabolic sine with the same correction used by the cosine.
        let value = y + 0.225 * (y * Self(y).abs().0 - y);
        Self(if self.is_sign_negative() {
            -value
        } else {
            value
        })
    }
}

#[cfg(test)]
mod tests {
    use super::F32;
    use crate::float::cos::tests::MAX_ERROR;

    /// Sine test vectors - `(input, output)`
    const TEST_VECTORS: &[(f32, f32)] = &[
        (0.000, 0.000),
        (0.140, 0.139),
        (0.279, 0.276),
        (0.419, 0.407),
        (0.559, 0.530),
        (0.698, 0.643),
        (0.838, 0.743),
        (0.977, 0.829),
        (1.117, 0.899),
        (1.257, 0.951),
        (1.396, 0.985),
        (1.536, 0.999),
        (1.676, 0.995),
        (1.815, 0.970),
        (1.955, 0.927),
        (2.094, 0.866),
        (2.234, 0.788),
        (2.374, 0.695),
        (2.513, 0.588),
        (2.653, 0.469),
        (2.793, 0.342),
        (2.932, 0.208),
        (3.072, 0.070),
        (3.211, -0.070),
        (3.351, -0.208),
        (3.491, -0.342),
        (3.630, -0.469),
        (3.770, -0.588),
        (3.910, -0.695),
        (4.049, -0.788),
        (4.189, -0.866),
        (4.328, -0.927),
        (4.468, -0.970),
        (4.608, -0.995),
        (4.747, -0.999),
        (4.887, -0.985),
        (5.027, -0.951),
        (5.166, -0.899),
        (5.306, -0.829),
        (5.445, -0.743),
        (5.585, -0.643),
        (5.725, -0.530),
        (5.864, -0.407),
        (6.004, -0.276),
        (6.144, -0.139),
        (6.283, 0.000),
    ];

    #[test]
    fn sanity_check() {
        for &(x, expected) in TEST_VECTORS {
            let sin_x = F32(x).sin();
            let delta = (sin_x - expected).abs();

            assert!(
                delta <= MAX_ERROR,
                "delta {} too large: {} vs {}",
                delta,
                sin_x,
                expected
            );
        }
    }

    #[test]
    fn turns_and_quadrants() {
        use crate::F32Ext;
        for (x, y) in [
            (0.0_f32, 0.0_f32),
            (0.25, 1.0),
            (0.5, 0.0),
            (0.75, -1.0),
            (1.0, 0.0),
        ] {
            assert_eq!(F32Ext::sin_turns(x), y);
        }
        assert_eq!(F32(-0.0).sin_turns().to_bits(), (-0.0_f32).to_bits());
        assert!(F32::INFINITY.sin_turns().is_nan() && F32::NAN.sin_turns().is_nan());
        for i in -2048..=2048 {
            let turns = i as f32 / 1024.0;
            let expected = (f64::from(turns) * core::f64::consts::TAU).sin();
            assert!((f64::from(F32(turns).sin_turns().0) - expected).abs() < 0.002);
        }
    }
}
