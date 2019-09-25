/// Floating natural log


use core::f32;
use core::u32;
use super::utils::FloatComponents;
use super::utils;

pub(super) fn ln_1to2_series_approximation(x: f32) -> f32 {
    // idea from https://stackoverflow.com/a/44232045/
    // modified to not be restricted to int range and only values of x above 1.0.
    // and got rid of most of the slow conversions,
    // should work for all positive values of x.
    // log base 2(E) == 1/ln(2)
    //let recip_ln2: f32 = f32::consts::LOG2_E;
    if x == 1.0_f32{
        return 0.0_f32;
    }
    let x_less_than_1: bool = x < 1.0;
    //use fast inv here?
    let x_working: f32 = if x_less_than_1 { 1.0 / x } else { x };
    //according to post ln(x) = ln((2^n)*y)= ln(2^n) + ln(y) = ln(2) * n + ln(y)
    //get exponent value
    let base2_exponent: u32 = x_working.extract_exponent_value() as u32;
    let divisor: f32 = f32::from_bits(x_working.to_bits() & utils::EXPONENT_MASK);
    //supposedly normalizing between 1.0 and 2.0
    let x_working: f32 = x_working / divisor;
    //approximate polynomial generated from maple in the post using Remez Algorithm:
    //https://en.wikipedia.org/wiki/Remez_algorithm
    let ln_1to2_polynomial: f32 = -1.7417939_f32
        + (2.8212026_f32 + (-1.4699568_f32
        + (0.44717955_f32 - 0.056570851_f32 * x_working)
        * x_working) * x_working) * x_working;
    // ln(2) * n + ln(y)
    //maybe a faster way to convert exponent?
    let result:f32 = (base2_exponent as f32) * f32::consts::LN_2 +  ln_1to2_polynomial;
    if x_less_than_1{
        return -result;
    }else{
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::ln_1to2_series_approximation;
    use super::super::abs;
    pub(crate) const MAX_ERROR: f32 = 0.001;
    /// Square root test vectors - `(input, output)`
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
        (1e+19, 43.749115)
    ];



    #[test]
    fn sanity_check() {
        assert_eq!(ln_1to2_series_approximation(1_f32), 0_f32);
        for (x, expected) in TEST_VECTORS {
            let ln_x = ln_1to2_series_approximation(*x);
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
