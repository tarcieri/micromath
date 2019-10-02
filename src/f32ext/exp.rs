/// Exp approximation for f32
use super::abs;
use super::fract;
use super::trunc;
use super::utils;
use crate::f32ext::utils::FloatComponents;
use core::f32;
use core::i32;
use core::u32;

pub(crate) fn exp_smallx(x: f32, iter: u32) -> f32 {
    // if x is between 0.0 and 1.0, we can approximate it with the a series
    // series from here: https://stackoverflow.com/a/6984495, e^x ~= 1 + x(1 + x/2(1 + (x?
    let mut total: f32 = 1.0_f32;
    for i in (1..=iter).rev() {
        total = 1.0_f32 + ((x / (i as f32)) * total);
    }
    total
}

pub(super) fn exp_ln2_approximation(x: f32, partial_iter: u32) -> f32 {
    // idea from# https://stackoverflow.com/a/6985769/2036035
    if x == 0.0_f32 {
        return 1.0;
    }
    if abs::abs(x - 1.0_f32) < f32::EPSILON {
        return f32::consts::E;
    }
    if abs::abs(x - (-1.0_f32)) < f32::EPSILON {
        return 1.0 / f32::consts::E;
    }
    // log base 2(E) == 1/ln(2)
    let ln2_recip: f32 = f32::consts::LOG2_E;
    //x_fract + x_whole = x/ln2_recip
    //ln2*(x_fract + x_whole) = x
    let x_fract = fract::fract_sign(x * ln2_recip);
    let x_trunc = trunc::trunc_sign(x * ln2_recip);
    //guaranteed to be 0 < x < 1.0
    let x_fract = x_fract * f32::consts::LN_2;

    let fract_exp = exp_smallx(x_fract, partial_iter);

    //need the 2^n portion, we can just extract that from the whole number exp portion
    let fract_exponent: i32 = fract_exp
        .extract_exponent_value()
        .saturating_add(x_trunc as i32);

    if fract_exponent < -(utils::EXPONENT_BIAS as i32) {
        return 0.0_f32;
    }
    if fract_exponent > ((utils::EXPONENT_BIAS + 1_u32) as i32) {
        return f32::INFINITY;
    }
    let exp_approx: f32 = fract_exp.set_exponent(fract_exponent);
    exp_approx
}

#[cfg(test)]
mod tests {
    use super::super::abs;
    use super::exp_ln2_approximation;
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
        (-10.0, 4.539993e-05),
    ];

    #[test]
    fn sanity_check() {
        assert_eq!(exp_ln2_approximation(-1000000.0, 4), 0_f32);
        for (x, expected) in TEST_VECTORS {
            let exp_x = exp_ln2_approximation(*x, 4);
            let relative_error = abs::abs(exp_x - *expected) / *expected;

            assert!(
                relative_error <= MAX_ERROR,
                "relative_error {} too large for input {} : {} vs {}",
                relative_error,
                *x,
                exp_x,
                expected
            );
        }
    }
}
