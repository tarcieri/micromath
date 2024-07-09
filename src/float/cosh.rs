//! Hyperbolic cosine function

use super::F32;

impl F32 {
    /// Approximates `cosh(x)` with a maximum error of `0.003` in the range 0...π,
    /// and a maximum error of `0.1` in the range `π...τ`.
    ///
    /// ## Arguments
    /// * `self` - The angle in radians.
    pub fn cosh(self) -> Self {
        let exp_x = self.exp();
        let exp_neg_x = (-self).exp();
        (exp_x + exp_neg_x) / 2.0
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::F32;

    /// Maximum error in radians
    pub(crate) const MAX_ERROR_1: f32 = 0.001;
    pub(crate) const MAX_ERROR_2: f32 = 0.002;
    pub(crate) const MAX_ERROR_3: f32 = 0.003;
    pub(crate) const MAX_ERROR_4: f32 = 0.005;
    pub(crate) const MAX_ERROR_5: f32 = 0.02;
    pub(crate) const MAX_ERROR: f32 = 0.092;

    /// Cosine test vectors - `(input, output)`
    const TEST_VECTORS: &[(f32, f32, f32)] = &[
        (0.000, 1.000, MAX_ERROR_1),
        (0.140, 1.010, MAX_ERROR_1),
        (0.279, 1.039, MAX_ERROR_1),
        (0.419, 1.089, MAX_ERROR_1),
        (0.559, 1.160, MAX_ERROR_1),
        (0.698, 1.254, MAX_ERROR_1),
        (0.838, 1.372, MAX_ERROR_1),
        (0.977, 1.516, MAX_ERROR_1),
        (1.117, 1.691, MAX_ERROR_1),
        (1.257, 1.900, MAX_ERROR_1),
        (1.396, 2.143, MAX_ERROR_1),
        (1.536, 2.431, MAX_ERROR_1),
        (1.676, 2.766, MAX_ERROR_1),
        (1.815, 3.152, MAX_ERROR_1),
        (1.955, 3.603, MAX_ERROR_2),
        (2.094, 4.120, MAX_ERROR_2),
        (2.234, 4.722, MAX_ERROR_2),
        (2.374, 5.417, MAX_ERROR_2),
        (2.513, 6.211, MAX_ERROR_2),
        (2.653, 7.134, MAX_ERROR_3),
        (2.793, 8.196, MAX_ERROR_3),
        (2.932, 9.409, MAX_ERROR_3),
        (3.072, 10.816, MAX_ERROR_3),
        (3.211, 12.422, MAX_ERROR_3),
        (3.351, 14.283, MAX_ERROR_4),
        (3.491, 16.425, MAX_ERROR_4),
        (3.630, 18.870, MAX_ERROR_4),
        (3.770, 21.702, MAX_ERROR_4),
        (3.910, 24.959, MAX_ERROR_4),
        (4.049, 28.679, MAX_ERROR_5),
        (4.189, 32.986, MAX_ERROR_5),
        (4.328, 37.903, MAX_ERROR_5),
        (4.468, 43.597, MAX_ERROR_5),
        (4.608, 50.147, MAX_ERROR_5),
        (4.747, 57.623, MAX_ERROR),
        (4.887, 66.281, MAX_ERROR),
        (5.027, 76.241, MAX_ERROR),
        (5.166, 87.609, MAX_ERROR),
        (5.306, 100.774, MAX_ERROR),
        (5.445, 115.801, MAX_ERROR),
        (5.585, 133.202, MAX_ERROR),
        (5.725, 153.218, MAX_ERROR),
        (5.864, 176.066, MAX_ERROR),
        (6.004, 202.524, MAX_ERROR),
        (6.144, 232.958, MAX_ERROR),
        (6.283, 267.697, MAX_ERROR),
    ];

    #[test]
    fn sanity_check() {
        for &(x, expected, max_error) in TEST_VECTORS {
            let cosh_x = F32(x).cosh();
            let delta = (cosh_x - expected).abs();

            assert!(
                delta <= max_error,
                "delta {} too large: {} vs {}",
                delta,
                cosh_x,
                expected
            );
        }
    }
}
