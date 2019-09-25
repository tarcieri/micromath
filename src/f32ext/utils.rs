use core::f32;
pub const SIGN_MASK: u32 = 0b10000000_00000000_00000000_00000000;
pub const EXPONENT_MASK: u32 = 0b01111111_10000000_00000000_00000000;
pub const MANTISSA_MASK: u32 = 0b00000000_01111111_11111111_11111111;

pub(super) trait FloatComponents<IntType=i32,UIntType=u32>{
    fn extract_sign_bit(self) ->UIntType;
    fn extract_exponent_bits(self)->UIntType;
    fn extract_mantissa_bits(self)->UIntType;
    fn extract_exponent_value(self)->IntType;
}
impl FloatComponents for f32 {
    fn extract_sign_bit(self) ->u32{
        return (self.to_bits()& SIGN_MASK).overflowing_shr(32 - 1).0;
    }
    fn extract_exponent_bits(self)->u32{
        return (self.to_bits()& EXPONENT_MASK).overflowing_shr(23).0;
    }
    fn extract_mantissa_bits(self)->u32{
        return self.to_bits()& MANTISSA_MASK;
    }
    fn extract_exponent_value(self)->i32{
            return (self.extract_exponent_bits() as i32) - 127_i32;
    }
}