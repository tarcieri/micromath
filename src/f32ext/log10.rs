/// log base 2 approximation for f32
use super::ln;
use core::f32;
pub(super) fn log10_ln_approx(x: f32) -> f32 {
    //using change of base log10(x) = ln(x)/ln(10)
    let ln10_recip = f32::consts::LOG10_E;
    let fract_base_ln = ln10_recip;
    let value_ln = ln::ln_1to2_series_approximation(x);
    value_ln * fract_base_ln
}

#[cfg(test)]
mod tests {
    use super::super::abs;
    use super::log10_ln_approx;
    pub(crate) const MAX_ERROR: f32 = 0.001;
    /// log10(x) test vectors - `(input, output)`
    pub(crate) const TEST_VECTORS: &[(f32, f32)] = &[
        (1e-20, -20.0),
        (1e-19, -19.0),
        (1e-18, -18.0),
        (1e-17, -17.0),
        (1e-16, -16.0),
        (1e-15, -15.0),
        (1e-14, -14.0),
        (1e-13, -13.0),
        (1e-12, -12.0),
        (1e-11, -11.0),
        (1e-10, -10.0),
        (1e-09, -9.0),
        (1e-08, -8.0),
        (1e-07, -7.0),
        (1e-06, -6.0),
        (1e-05, -5.0),
        (1e-04, -4.0),
        (0.001, -3.0),
        (0.01, -2.0),
        (0.1, -1.0),
        (10.0, 1.0),
        (100.0, 2.0),
        (1000.0, 3.0),
        (10000.0, 4.0),
        (100000.0, 5.0),
        (1000000.0, 6.0),
        (10000000.0, 7.0),
        (100000000.0, 8.0),
        (1000000000.0, 9.0),
        (10000000000.0, 10.0),
        (100000000000.0, 11.0),
        (1000000000000.0, 12.0),
        (10000000000000.0, 13.0),
        (100000000000000.0, 14.0),
        (1000000000000000.0, 15.0),
        (1e+16, 16.0),
        (1e+17, 17.0),
        (1e+18, 18.0),
        (1e+19, 19.0),
    ];

    #[test]
    fn sanity_check() {
        assert_eq!(log10_ln_approx(1_f32), 0_f32);
        for (x, expected) in TEST_VECTORS {
            let ln_x = log10_ln_approx(*x);
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
