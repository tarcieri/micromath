//! Fast approximation of `1/x`.
//!
//! Method described at: <https://bits.stephan-brumme.com/inverse.html>

use super::F32;

impl F32 {
    /// Fast approximation of `1/x`.
    pub fn inv(self) -> Self {
        // Perform the bit manipulation for the approximation
        // The constant 0x7f00_0000 corresponds to the bit pattern for 1.0 in IEEE 754 format.
        // Subtracting the bits of the original number from this constant effectively inverts the exponent,
        // resulting in an approximation of the reciprocal.
        match 0x7f00_0000_u32.checked_sub(self.0.to_bits()) {
            Some(result) => Self(f32::from_bits(result)),
            // Check if the value is too large for the approximation, NaN (e.g. 0x7fC0_0000) or infinity.
            None => {
                if self.0.is_infinite() {
                    // 1/∞ = 0; by definition.
                    if self.0.is_sign_positive() {
                        Self(0.0)
                    } else {
                        Self(-0.0)
                    }
                } else if self.0.is_nan() {
                    Self(f32::NAN)
                } else {
                    // Values larger than the threshold result in zero for 1/x
                    Self(0.0)
                }
            }
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::F32;

    /// Deviation from the actual value (8%)
    pub(crate) const MAX_ERROR: f32 = 0.08;

    #[test]
    fn sanity_check() {
        for x in 0..100 {
            let x = F32(x as f32);
            let inv_x = x.inv().0;
            let expected = 1.0 / x;
            let allowed_delta = x * MAX_ERROR;
            let actual_delta = inv_x - expected;

            assert!(
                actual_delta <= allowed_delta,
                "delta {} too large: {} vs {}",
                actual_delta,
                inv_x,
                expected
            );
        }
    }

    #[test]
    fn special_floats() {
        assert!(f32::NAN.to_bits() > 0x7f00_0000);

        assert_eq!(F32(f32::from_bits(0x7f00_0001)).inv(), F32(0.0));
        assert!(F32(f32::NAN).inv().is_nan());
        assert_eq!(F32(f32::INFINITY).inv(), F32(0.0));
        assert_eq!(F32(f32::NEG_INFINITY).inv(), F32(-0.0));
        assert!(F32(f32::INFINITY).inv().is_sign_positive());
        assert!(F32(f32::NEG_INFINITY).inv().is_sign_negative());
    }
}
