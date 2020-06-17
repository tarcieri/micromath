use core::f32;
use core::i32;
pub const SIGN_MASK: u32 = 0b10000000_00000000_00000000_00000000;
pub const EXPONENT_MASK: u32 = 0b01111111_10000000_00000000_00000000;
pub const MANTISSA_MASK: u32 = 0b00000000_01111111_11111111_11111111;
pub const EXPONENT_BIAS: u32 = 127;
// MANTISSA_DIGITS is availible in core::f32, but the actual bits taken up are 24 - 1
pub const MANTISSA_BITS: u32 = 23;

pub(super) trait FloatComponents<IntType = i32, UIntType = u32> {
    fn extract_sign_bit(self) -> UIntType;
    fn extract_exponent_bits(self) -> UIntType;
    fn extract_mantissa_bits(self) -> UIntType;
    fn extract_exponent_value(self) -> IntType;
    fn without_sign(self) -> Self;
    fn set_exponent(self, exponent: IntType) -> Self;
    fn is_integer(&self) -> bool;
    fn is_even(&self) -> bool;
}
impl FloatComponents for f32 {
    fn extract_sign_bit(self) -> u32 {
        (self.to_bits() & SIGN_MASK).overflowing_shr(32 - 1).0
    }
    fn extract_exponent_bits(self) -> u32 {
        (self.to_bits() & EXPONENT_MASK)
            .overflowing_shr(MANTISSA_BITS)
            .0
    }
    fn extract_mantissa_bits(self) -> u32 {
        self.to_bits() & MANTISSA_MASK
    }
    fn extract_exponent_value(self) -> i32 {
        (self.extract_exponent_bits() as i32) - EXPONENT_BIAS as i32
    }
    fn without_sign(self) -> f32 {
        f32::from_bits(self.to_bits() & !SIGN_MASK)
    }
    fn set_exponent(self, exponent: i32) -> f32 {
        debug_assert!(exponent <= 127 && exponent >= -128);
        let without_exponent: u32 = self.to_bits() & !EXPONENT_MASK;
        let only_exponent: u32 = ((exponent + EXPONENT_BIAS as i32) as u32)
            .overflowing_shl(MANTISSA_BITS)
            .0;
        f32::from_bits(without_exponent | only_exponent)
    }
    fn is_integer(&self) -> bool {
        let exponent: i32 = self.extract_exponent_value();
        let self_bits = self.to_bits();
        //if exponent is negative we shouldn't remove anything, this stops an opposite shift.
        let exponent_clamped = i32::max(exponent, 0_i32) as u32;
        // find the part of the fraction that would be left over
        let fractional_part: u32 = (self_bits).overflowing_shl(exponent_clamped).0 & MANTISSA_MASK;
        // if fractional part contains anything, we know it *isn't* an integer.
        // if zero there will be nothing in the fractional part
        // if it is whole, there will be nothing in the fractional part
        fractional_part == 0_u32
    }
    fn is_even(&self) -> bool {
        // any floating point value that doesn't fit in an i32 range is even,
        // and will loose 1's digit precision at exp values of 23+
        if self.extract_exponent_value() >= 31 {
            true
        } else {
            (*self as i32) % 2 == 0
        }
    }
}
