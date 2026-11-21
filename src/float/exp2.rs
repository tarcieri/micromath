//! Base-two exponential for a single-precision float.

use super::F32;
use core::f32::consts::LN_2;

impl F32 {
    /// Approximates 2 raised to this power. Integer powers are exact whenever
    /// representable. The fourth-degree fractional approximation has relative
    /// error below 0.0008 before subnormal rounding.
    #[inline]
    pub fn exp2(self) -> Self {
        let x = self.0;
        if self.abs().0 < 126.0 {
            let (p, k) = split_exp2(x);
            return Self(p * F32::ONE.set_exponent(k).0);
        }
        exp2_limit(x)
    }
}

// Keep the checks and scaling for exceptional and extreme exponents off
// the common path, especially when exp2 is inlined in an inner loop.
#[cold]
fn exp2_limit(x: f32) -> F32 {
    if x >= 128.0 {
        return F32::INFINITY;
    }
    if x <= -150.0 {
        return F32::ZERO;
    }
    if x.is_nan() {
        return F32(x);
    }
    let (p, k) = split_exp2(x);
    F32(scale(p, k))
}

#[inline]
fn split_exp2(x: f32) -> (f32, i32) {
    // At 1.5 * 2^23 the f32 spacing is one. Adding then subtracting this
    // bias rounds x to an integer; its low bits encode that same integer.
    // Recover the floor when rounding went upward.
    const BIAS: f32 = 12_582_912.0;
    let biased = x + BIAS;
    let nearest = biased - BIAS;
    let integer = (biased.to_bits() as i32) - 0x4b40_0000;
    let delta = x - nearest;
    let (k, f) = if delta < 0.0 {
        (integer - 1, delta + 1.0)
    } else {
        (integer, delta)
    };
    // Taylor coefficients for exp(f * ln(2)), on 0 <= f < 1.
    let p = 1.0 + f * (LN_2 + f * (0.240_226_5 + f * (0.055_504_1 + f * 0.009_618_1)));
    (p, k)
}

// Multiply by 2^k for -150 <= k <= 128. Split the two boundary cases
// so underflow and overflow round in the final floating-point operation.
#[inline]
pub(super) fn scale(value: f32, k: i32) -> f32 {
    if k < -126 {
        return (value * F32::ONE.set_exponent(k + 126).0) * f32::MIN_POSITIVE;
    }
    if k == 128 {
        return (value * 2.0) * F32::ONE.set_exponent(127).0;
    }
    value * F32::ONE.set_exponent(k).0
}

#[cfg(test)]
mod tests {
    use super::F32;
    use crate::F32Ext;

    #[test]
    fn powers_and_limits() {
        assert_eq!(F32Ext::exp2(-0.0_f32), 1.0);
        assert_eq!(F32Ext::exp2(f32::NEG_INFINITY), 0.0);
        assert_eq!(F32::INFINITY.exp2(), F32::INFINITY);
        assert!(F32::NAN.exp2().is_nan());
        for k in -150..=128 {
            let reference = f64::from(k).exp2() as f32;
            assert_eq!(F32(k as f32).exp2().to_bits(), reference.to_bits());
        }
        let mut previous = 0.0;
        for i in -38_400..32_768 {
            let x = i as f32 / 256.0;
            let reference = f64::from(x).exp2() as f32;
            let actual = F32(x).exp2().0;
            let tolerance = (0.0008 * reference).max(f32::from_bits(1));
            assert!(
                actual >= previous && (actual - reference).abs() <= tolerance,
                "exp2({}) = {}",
                x,
                actual
            );
            previous = actual;
        }
    }
}
