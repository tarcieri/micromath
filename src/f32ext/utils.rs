use core::f32;
pub const SIGN_MASK: u32 = 0b10000000_00000000_00000000_00000000;
pub const EXPONENT_MASK: u32 = 0b01111111_10000000_00000000_00000000;
pub const MANTISSA_MASK: u32 = 0b00000000_01111111_11111111_11111111;
pub const EXPONENT_BIAS: u32 = 127;

pub(super) trait FloatComponents<IntType = i32, UIntType = u32> {
    fn extract_sign_bit(self) -> UIntType;
    fn extract_exponent_bits(self) -> UIntType;
    fn extract_mantissa_bits(self) -> UIntType;
    fn extract_exponent_value(self) -> IntType;
    fn without_sign(self) -> Self;
    fn set_exponent(self, exponent: IntType) -> Self;
}
impl FloatComponents for f32 {
    fn extract_sign_bit(self) -> u32 {
        (self.to_bits() & SIGN_MASK).overflowing_shr(32 - 1).0
    }
    fn extract_exponent_bits(self) -> u32 {
        (self.to_bits() & EXPONENT_MASK).overflowing_shr(23).0
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
            .overflowing_shl(23)
            .0;
        f32::from_bits(without_exponent | only_exponent)
    }
}
