/// Inverse square root approximation function for a single-precision float.
/// Method described at: <https://bits.stephan-brumme.com/invSquareRoot.html>
pub(super) fn invsqrt_approx(x: f32) -> f32 {
    f32::from_bits(0x5f37_5a86 - (x.to_bits() >> 1))
}

#[cfg(test)]
mod tests {
    use super::invsqrt_approx;
    use crate::f32ext::sqrt::tests::TEST_VECTORS;

    /// Deviation from the actual value (5%)
    const MAX_ERROR: f32 = 0.05;

    #[test]
    fn sanity_check() {
        for (x, expected) in TEST_VECTORS {
            // The tests vectors are for sqrt(x), so invert the expected value
            let expected = 1.0 / expected;

            let invsqrt_x = invsqrt_approx(*x);
            let allowed_delta = x * MAX_ERROR;
            let actual_delta = invsqrt_x - expected;

            assert!(
                actual_delta <= allowed_delta,
                "delta {} too large: {} vs {}",
                actual_delta,
                invsqrt_x,
                expected
            );
        }
    }
}
