//! Floating point floor approximation for a single-precision float.

use super::F32;

impl F32 {
    /// Returns the largest integer less than or equal to a number.
    #[inline]
    pub fn floor(self) -> Self {
        // Every finite f32 at or above 2^23 in magnitude is integral.
        // Returning the input here also preserves infinities, NaNs and -0.
        let magnitude = self.to_bits() & !super::SIGN_MASK;
        if magnitude == 0 || magnitude >= 0x4b00_0000 {
            return self;
        }

        let mut res = (self.0 as i32) as f32;

        if self.0 < res {
            res -= 1.0;
        }

        Self(res)
    }
}

#[cfg(test)]
mod tests {
    use super::F32;

    #[test]
    fn sanity_check() {
        assert_eq!(F32(-1.1).floor().0, -2.0);
        assert_eq!(F32(-0.1).floor().0, -1.0);
        assert_eq!(F32(0.0).floor().0, 0.0);
        assert_eq!(F32(1.0).floor().0, 1.0);
        assert_eq!(F32(1.1).floor().0, 1.0);
        assert_eq!(F32(2.9).floor().0, 2.0);
    }

    #[test]
    fn integral_and_exceptional_values() {
        for &x in &[
            -0.0_f32,
            0.0,
            -f32::MAX,
            f32::MAX,
            f32::NEG_INFINITY,
            f32::INFINITY,
        ] {
            assert_eq!(F32(x).floor().to_bits(), x.to_bits());
        }
        assert!(F32::NAN.floor().is_nan());
        for &x in &[
            f32::from_bits(1),
            -f32::from_bits(1),
            8_388_607.5,
            -8_388_607.5,
        ] {
            assert_eq!(F32(x).floor().to_bits(), x.floor().to_bits());
        }
    }
}
