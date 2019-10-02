/// Floating point whole number for f32
use super::copysign;
use super::utils;
use super::utils::FloatComponents;
use core::f32;
use core::u32;

pub(super) fn trunc_sign(x: f32) -> f32 {
    let x_bits: u32 = x.to_bits();
    let exponent: i32 = x.extract_exponent_value();
    //exponent is negative, there is no whole number, just return zero
    if exponent < 0_i32 {
        return copysign::copysign(0.0_f32, x);
    }
    let exponent_clamped = i32::max(exponent, 0_i32) as u32;

    // find the part of the fraction that would be left over
    let fractional_part: u32 = x_bits.overflowing_shl(exponent_clamped).0 & utils::MANTISSA_MASK;
    // if there isn't a fraction we can just return the whole thing.
    if fractional_part == 0_u32 {
        return x;
    }
    let fractional_mask: u32 = fractional_part.overflowing_shr(exponent_clamped).0;
    f32::from_bits(x_bits & !fractional_mask)
}

#[cfg(test)]
mod tests {
    use super::trunc_sign;
    #[test]
    fn sanity_check() {
        assert_eq!(trunc_sign(-1.1), -1.0);
        assert_eq!(trunc_sign(-0.1), -0.0);
        assert_eq!(trunc_sign(0.0), 0.0);
        assert_eq!(trunc_sign(1.0), 1.0);
        assert_eq!(trunc_sign(1.1), 1.0);
        assert_eq!(trunc_sign(2.9), 2.0);

        assert_eq!(trunc_sign(-100_000_000.13425345345), -100_000_000.0);
        assert_eq!(trunc_sign(100_000_000.13425345345), 100_000_000.0);
    }
}
