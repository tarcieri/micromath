use core::f32;
pub const SIGN_MASK: u32 = 0b10000000_00000000_00000000_00000000;
pub const EXPONENT_MASK: u32 = 0b01111111_10000000_00000000_00000000;
pub const MANTISSA_MASK: u32 = 0b00000000_01111111_11111111_11111111;
pub const EXPONENT_BIAS: u32 = 127;

pub(super) trait FloatComponents<IntType=i32,UIntType=u32>{
    fn extract_sign_bit(self) ->UIntType;
    fn extract_exponent_bits(self)->UIntType;
    fn extract_mantissa_bits(self)->UIntType;
    fn extract_exponent_value(self)->IntType;
    fn without_sign(self)->Self;
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
        return (self.extract_exponent_bits() as i32) - EXPONENT_BIAS as i32;
    }
    fn without_sign(self)->f32{
        return f32::from_bits(self.to_bits() & !SIGN_MASK);
    }
}