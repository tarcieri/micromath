/// x^n with fractional n approximation for f32
use super::exp;
use super::ln;
use crate::f32ext::utils::FloatComponents;
use core::f32;

pub(super) fn powf_exp_ln_approx(x: f32, n: f32) -> f32 {
    // using x^n = exp(ln(x^n)) = exp(n*ln(x))
    if x < 0.0 {
        if !n.is_integer() {
            f32::NAN
        } else {
            //if n is even, then we know that the result will have no sign, so we can remove it.
            if n.is_even() {
                exp::exp_ln2_approximation(
                    n * ln::ln_1to2_series_approximation(x.without_sign()),
                    4,
                )
            } else {
                //if n isn't even, we need to multiply by -1.0 at the end.
                -exp::exp_ln2_approximation(
                    n * ln::ln_1to2_series_approximation(x.without_sign()),
                    4,
                )
            }
        }
    } else {
        exp::exp_ln2_approximation(n * ln::ln_1to2_series_approximation(x), 4)
    }
}

#[cfg(test)]
mod tests {
    use super::super::abs;
    use super::powf_exp_ln_approx;
    use core::f32;

    //error builds up from both exp and ln approximation, so we double the error allowed.
    pub(crate) const MAX_ERROR: f32 = 0.002;
    ///  powf(3,x) test vectors - `(input, output)`
    pub(crate) const TEST_VECTORS_POW3: &[(f32, f32)] = &[
        (-1e-20, 1.0),
        (-1e-19, 1.0),
        (-1e-18, 1.0),
        (-1e-17, 1.0),
        (-1e-16, 0.9999999999999999),
        (-1e-15, 0.9999999999999989),
        (-1e-14, 0.999999999999989),
        (-1e-13, 0.9999999999998901),
        (-1e-12, 0.9999999999989014),
        (-1e-11, 0.9999999999890139),
        (-1e-10, 0.9999999998901388),
        (-1e-09, 0.9999999989013877),
        (-1e-08, 0.9999999890138772),
        (-1e-07, 0.9999998901387759),
        (-1e-06, 0.9999989013883176),
        (-1e-05, 0.9999890139377381),
        (-1e-04, 0.9998901448084321),
        (-0.001, 0.9989019909127541),
        (-0.01, 0.9890740044150467),
        (-0.1, 0.8959584583740245),
        (-1.0, 0.3333333333333333),
        (-10.0, 1.6935087808430286e-05),
        (-100.0, 1.9403252174826328e-48),
        (-1000.0, 0.0),
        (1e-20, 1.0),
        (1e-19, 1.0),
        (1e-18, 1.0),
        (1e-17, 1.0),
        (1e-16, 1.0),
        (1e-15, 1.000000000000001),
        (1e-14, 1.0000000000000109),
        (1e-13, 1.00000000000011),
        (1e-12, 1.0000000000010987),
        (1e-11, 1.000000000010986),
        (1e-10, 1.0000000001098612),
        (1e-09, 1.0000000010986123),
        (1e-08, 1.000000010986123),
        (1e-07, 1.000000109861236),
        (1e-06, 1.0000010986128893),
        (1e-05, 1.000010986182957),
        (1e-04, 1.0001098672610569),
        (0.001, 1.0010992160364427),
        (0.01, 1.011046691689582),
        (0.1, 1.1161231758610648),
        (1.0, 3.0),
        (10.0, 59049.0),
    ];
    ///  powf(150,x) test vectors - `(input, output)`
    pub(crate) const TEST_VECTORS_POW150: &[(f32, f32)] = &[
        (-1e-20, 1.0),
        (-1e-19, 1.0),
        (-1e-18, 1.0),
        (-1e-17, 1.0),
        (-1e-16, 0.9999999999999994),
        (-1e-15, 0.999999999999995),
        (-1e-14, 0.9999999999999499),
        (-1e-13, 0.999999999999499),
        (-1e-12, 0.9999999999949893),
        (-1e-11, 0.9999999999498936),
        (-1e-10, 0.9999999994989365),
        (-1e-09, 0.9999999949893649),
        (-1e-08, 0.9999999498936486),
        (-1e-07, 0.9999994989365902),
        (-1e-06, 0.9999949893772717),
        (-1e-05, 0.9999498949036271),
        (-1e-04, 0.9994990619946083),
        (-0.001, 0.9950018967618063),
        (-0.01, 0.9511282648985805),
        (-0.1, 0.6058859348946524),
        (-1.0, 0.006666666666666667),
        (-10.0, 1.7341529915832614e-22),
        (-100.0, 2.4596544265798294e-218),
        (-1000.0, 0.0),
        (-10000.0, 0.0),
        (-100000.0, 0.0),
        (-1000000.0, 0.0),
        (-10000000.0, 0.0),
        (-100000000.0, 0.0),
        (-1000000000.0, 0.0),
        (-10000000000.0, 0.0),
        (-100000000000.0, 0.0),
        (-1000000000000.0, 0.0),
        (-10000000000000.0, 0.0),
        (-100000000000000.0, 0.0),
        (-1000000000000000.0, 0.0),
        (-1e+16, 0.0),
        (-1e+17, 0.0),
        (-1e+18, 0.0),
        (-1e+19, 0.0),
        (1e-20, 1.0),
        (1e-19, 1.0),
        (1e-18, 1.0),
        (1e-17, 1.0),
        (1e-16, 1.0000000000000004),
        (1e-15, 1.000000000000005),
        (1e-14, 1.0000000000000502),
        (1e-13, 1.0000000000005012),
        (1e-12, 1.0000000000050107),
        (1e-11, 1.0000000000501064),
        (1e-10, 1.0000000005010636),
        (1e-09, 1.0000000050106352),
        (1e-08, 1.000000050106354),
        (1e-07, 1.0000005010636608),
        (1e-06, 1.0000050106478346),
        (1e-05, 1.0000501076070194),
        (1e-04, 1.0005011890700448),
        (0.001, 1.0050232097591572),
        (0.01, 1.0513829069170084),
        (0.1, 1.6504756793436273),
        (1.0, 150.0),
        (10.0, 5.76650390625e+21),
    ];

    ///  misc powf(x,n) test vectors - `(base_input, power_input, output)`
    pub(crate) const TEST_VECTORS_MISC: &[(f32, f32, f32)] = &[
        (-0.5881598, 2.0, 0.3459319498370519),
        (-0.5881598, 3.2, f32::NAN),
        (-0.5881598, 3.0, -0.20346326672325524),
        (-1000000.0, 4.0, 1e+24),
    ];

    fn calc_relative_error(experimental: f32, expected: f32) -> f32 {
        let relative_error: f32 = if experimental.is_nan() && expected.is_nan() {
            0.0_f32
        } else if expected != 0.0_f32 {
            abs::abs(experimental - expected) / expected
        } else {
            abs::abs(experimental - expected) / (expected + 1.0e-20_f32)
        };
        relative_error
    }

    #[test]
    fn sanity_check() {
        for (x, expected) in TEST_VECTORS_POW3 {
            let exp_x = powf_exp_ln_approx(3.0, *x);
            let relative_error: f32 = calc_relative_error(exp_x, *expected);

            assert!(
                relative_error <= MAX_ERROR,
                "relative_error {} too large for input {} : {} vs {}",
                relative_error,
                *x,
                exp_x,
                expected
            );
        }

        for (x, expected) in TEST_VECTORS_POW150 {
            let exp_x = powf_exp_ln_approx(150.0, *x);
            let relative_error: f32 = calc_relative_error(exp_x, *expected);

            assert!(
                relative_error <= MAX_ERROR,
                "relative_error {} too large for input {} : {} vs {}",
                relative_error,
                *x,
                exp_x,
                expected
            );
        }

        for (base_input, power_input, expected) in TEST_VECTORS_MISC {
            let exp_x = powf_exp_ln_approx(*base_input, *power_input);
            let relative_error: f32 = calc_relative_error(exp_x, *expected);

            assert!(
                relative_error <= MAX_ERROR,
                "relative_error {} too large for input {}.powf({}) : {} vs {}",
                relative_error,
                *base_input,
                *power_input,
                exp_x,
                expected
            );
        }
    }
}
