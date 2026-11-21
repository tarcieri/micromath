//! Tangent approximation for a single-precision float.

use super::F32;
use core::f32::consts::{FRAC_PI_2, FRAC_PI_4};

impl F32 {
    /// Approximates the tangent of an angle in radians.
    #[inline]
    pub fn tan(self) -> Self {
        let x = self.0;
        let a = self.abs().0;
        if a <= FRAC_PI_4 {
            return Self(tan_reduced(x));
        }
        let t = if a <= 3.0 * FRAC_PI_4 {
            // π/2 rounded down, followed by the residual. Subtract before
            // adding the low part so adjacent floats at the pole keep their sign.
            let half_pi_hi = f32::from_bits(0x3fc9_0fda);
            let distance = (half_pi_hi - a) + 7.549_789_4e-8;
            1.0 / tan_reduced(distance)
        } else if a.is_finite() {
            let (r, odd) = reduce_quadrant(a);
            let t = tan_reduced(r);
            if odd {
                -1.0 / t
            } else {
                t
            }
        } else {
            return Self::NAN;
        };
        Self(if x.is_sign_negative() { -t } else { t })
    }
}

// On 0 <= z <= (π/4)^2, interpolate (tan(sqrt(z)) - sqrt(z)) / z^(3/2)
// at seven Chebyshev nodes. The odd form preserves tiny arguments and ±0.
#[inline]
fn tan_reduced(r: f32) -> f32 {
    let z = r * r;
    let high = 0.009_962_147 + z * (0.001_185_321_7 + z * 0.003_843_139_9);
    let mid = 0.053_994_466 + z * (0.021_621_127 + z * high);
    let p = 0.333_333_34 + z * (0.133_332_31 + z * mid);
    r + r * z * p
}

// For a > 3π/4, return (a - nπ/2, n mod 2), where n is the closest
// integer to a * 2/π. Multiplying the 24-bit significand by floor(2^192
// * 2/π) keeps at least 64 fractional bits even at the largest f32 exponent.
#[cold]
#[inline(never)]
fn reduce_quadrant(a: f32) -> (f32, bool) {
    const TWO_OVER_PI: [u32; 6] = [
        0x3c43_9041,
        0xdb62_9599,
        0xf534_ddc0,
        0xfc27_57d1,
        0x4e44_1529,
        0xa2f9_836e,
    ];
    let bits = a.to_bits();
    let significand = u64::from((bits & super::MANTISSA_MASK) | 0x0080_0000);
    let mut product = [0_u32; 7];
    let mut carry = 0;
    for (out, word) in product.iter_mut().zip(TWO_OVER_PI) {
        let p = u64::from(word) * significand + carry;
        *out = p as u32;
        carry = p >> 32;
    }
    product[6] = carry as u32;
    // The binary point is 192 + 23 - unbiased_exponent bits from the right.
    let shift = 342 - (bits >> 23) as usize;
    let word = |start: usize| {
        let index = start / 32;
        let pair = u64::from(product[index]) | (u64::from(product[index + 1]) << 32);
        (pair >> (start % 32)) as u32
    };
    let fraction = (u64::from(word(shift - 32)) << 32) | u64::from(word(shift - 64));
    let round_up = fraction >> 63 != 0;
    let integer_odd = product[shift / 32] >> (shift % 32) & 1 != 0;
    // Take the distance to the nearest integer before converting to f32.
    // Subtracting two rounded floats here loses the residual near a pole.
    let distance = if round_up {
        fraction.wrapping_neg()
    } else {
        fraction
    };
    let r = distance as f32 * (FRAC_PI_2 / 18_446_744_073_709_551_616.0);
    (if round_up { -r } else { r }, integer_odd ^ round_up)
}

#[cfg(test)]
mod tests {
    use super::F32;

    // The original inputs and expected values below are rounded to three decimals.
    const MAX_ERROR: f32 = 0.6;

    /// Tangent test vectors - `(input, output)`
    const TEST_VECTORS: &[(f32, f32)] = &[
        (0.000, 0.000),
        (0.140, 0.141),
        (0.279, 0.287),
        (0.419, 0.445),
        (0.559, 0.625),
        (0.698, 0.839),
        (0.838, 1.111),
        (0.977, 1.483),
        (1.117, 2.050),
        (1.257, 3.078),
        (1.396, 5.671),
        (1.536, 28.636),
        (1.676, -9.514),
        (1.815, -4.011),
        (1.955, -2.475),
        (2.094, -1.732),
        (2.234, -1.280),
        (2.374, -0.966),
        (2.513, -0.727),
        (2.653, -0.532),
        (2.793, -0.364),
        (2.932, -0.213),
        (3.072, -0.070),
        (3.211, 0.070),
        (3.351, 0.213),
        (3.491, 0.364),
        (3.630, 0.532),
        (3.770, 0.727),
        (3.910, 0.966),
        (4.049, 1.280),
        (4.189, 1.732),
        (4.328, 2.475),
        (4.468, 4.011),
        (4.608, 9.514),
        (4.747, -28.636),
        (4.887, -5.671),
        (5.027, -3.078),
        (5.166, -2.050),
        (5.306, -1.483),
        (5.445, -1.111),
        (5.585, -0.839),
        (5.725, -0.625),
        (5.864, -0.445),
        (6.004, -0.287),
        (6.144, -0.141),
        (6.283, 0.000),
    ];

    #[test]
    fn sanity_check() {
        for &(x, expected) in TEST_VECTORS {
            let tan_x = F32(x).tan();
            let delta = (tan_x - expected).abs();

            assert!(
                delta <= MAX_ERROR,
                "delta {} too large: {} vs {}",
                delta,
                tan_x,
                expected
            );
        }
    }

    #[test]
    fn zero() {
        assert_eq!(F32::ZERO.tan(), F32::ZERO);
    }

    #[test]
    fn nan() {
        assert!(F32::NAN.tan().is_nan());
    }

    #[test]
    fn reduction_and_poles() {
        let check = |x: f32| {
            let reference = f64::from(x).tan() as f32;
            let t = F32(x).tan().0;
            assert!(
                (t - reference).abs() <= 5e-7 * reference.abs(),
                "tan({:e}) = {}, expected {}",
                x,
                t,
                reference
            );
            assert_eq!(F32(-x).tan().to_bits(), (-t).to_bits());
        };
        for &x in &[
            0.0,
            f32::from_bits(1),
            f32::from_bits(0x392c_2fcb),
            f32::MAX,
        ] {
            check(x);
        }
        assert!(F32::INFINITY.tan().is_nan() && F32::NEG_INFINITY.tan().is_nan());
        for k in [1, 2, 3, 11, 121, 4097, 65_537, 1_000_001] {
            let center = ((k as f64) * core::f64::consts::FRAC_PI_2) as f32;
            for bits in center.to_bits() - 2..=center.to_bits() + 2 {
                check(f32::from_bits(bits));
            }
        }
        // Exercise the integer reducer across all binary exponents.
        for exponent in 0..255 {
            for mantissa in [0, 1, 0x123456, 0x400001, 0x7fffff] {
                check(f32::from_bits((exponent << 23) | mantissa));
            }
        }
    }
}
