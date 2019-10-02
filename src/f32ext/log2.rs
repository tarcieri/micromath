/// log base 2 approximation for f32
use super::ln;
use core::f32;
pub(super) fn log2_ln_approx(x: f32) -> f32 {
    //using change of base log2(x) = ln(x)/ln(2)
    let ln2_recip: f32 = f32::consts::LOG2_E;
    let fract_base_ln = ln2_recip;
    let value_ln = ln::ln_1to2_series_approximation(x);
    value_ln * fract_base_ln
}

#[cfg(test)]
mod tests {
    use super::super::abs;
    use super::log2_ln_approx;
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
        assert_eq!(log2_ln_approx(1_f32), 0_f32);
        for (x, expected) in TEST_VECTORS {
            let ln_x = log2_ln_approx(*x);
            let relative_error = abs::abs(ln_x - *expected) / *expected;

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
