/// Floating point fractional number for f32
use super::copysign;
use super::utils;
use super::utils::FloatComponents;

use core::f32;
use core::u32;

pub(super) fn fract_sign(x: f32) -> f32 {
    let x_bits: u32 = x.to_bits();
    let exponent: i32 = x.extract_exponent_value();
    // we know it is *only* fraction
    if exponent < 0_i32 {
        return x;
    }
    // find the part of the fraction that would be left over
    let fractional_part: u32 = x_bits.overflowing_shl(exponent as u32).0 & utils::MANTISSA_MASK;
    // if there isn't a fraction we can just return 0
    if fractional_part == 0_u32 {
        // most people don't actually care about -0.0, so would it be better to just not copysign?
        return copysign::copysign(0.0_f32, x);
    }
    // Note: alternatively this could use -1.0, but it's assumed subtraction would be more costly
    // example: 'let new_exponent_bits = 127_u32.overflowing_shl(23_u32).0)) - 1.0_f32'
    let exponent_shift: u32 =
        (fractional_part.leading_zeros() - (32_u32 - utils::MANTISSA_BITS)) + 1;
    let fractional_normalized: u32 =
        fractional_part.overflowing_shl(exponent_shift).0 & utils::MANTISSA_MASK;

    let new_exponent_bits = (utils::EXPONENT_BIAS - (exponent_shift))
        .overflowing_shl(utils::MANTISSA_BITS)
        .0;
    copysign::copysign(f32::from_bits(fractional_normalized | new_exponent_bits), x)
}

#[cfg(test)]
mod tests {
    use super::fract_sign;

    #[test]
    fn sanity_check() {
        //fraction check actually won't be the same, though technically exactly accurate
        //so we test by adding back the number removed.
        assert_eq!(fract_sign(2.9) + 2.0, 2.9_f32);
        assert_eq!(fract_sign(-1.1) - 1.0, -1.1_f32);
        assert_eq!(fract_sign(-0.1), -0.1);
        assert_eq!(fract_sign(0.0), 0.0);
        assert_eq!(fract_sign(1.0) + 1.0, 1.0);
        assert_eq!(fract_sign(1.1) + 1.0, 1.1);

        assert_eq!(fract_sign(-100_000_000.13425345345), -0.0);
        assert_eq!(fract_sign(100_000_000.13425345345), 0.0);
    }
}
