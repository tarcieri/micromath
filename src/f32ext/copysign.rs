use super::utils;
/// copy the sign over from another number
use core::f32;
use core::u32;

pub(super) fn copysign(destination: f32, source: f32) -> f32 {
    let source_bits: u32 = source.to_bits();
    let source_sign: u32 = source_bits & utils::SIGN_MASK;
    let signless_destination_bits: u32 = destination.to_bits() & !utils::SIGN_MASK;
    f32::from_bits(signless_destination_bits | source_sign)
}

#[cfg(test)]
mod tests {
    use super::copysign;
    #[test]
    fn sanity_check() {
        assert_eq!(copysign(1.0, -1.0), -1.0);
        assert_eq!(copysign(-1.0, 1.0), 1.0);
        assert_eq!(copysign(1.0, 1.0), 1.0);
        assert_eq!(copysign(-1.0, -1.0), -1.0);
        let large_float: f32 = 100_000_000.13425345345;
        assert_eq!(copysign(large_float, -large_float), -large_float);
        assert_eq!(copysign(-large_float, large_float), large_float);
        assert_eq!(copysign(large_float, large_float), large_float);
        assert_eq!(copysign(-large_float, -large_float), -large_float);
    }
}
