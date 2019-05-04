/// Fast approximation of `1/x`.
/// Method described at: <https://bits.stephan-brumme.com/inverse.html>
pub(super) fn inv_approx(x: f32) -> f32 {
    f32::from_bits(0x7f00_0000 - x.to_bits())
}

#[cfg(test)]
pub(super) mod tests {
    use super::inv_approx;

    /// Deviation from the actual value (8%)
    pub(crate) const MAX_ERROR: f32 = 0.08;

    #[test]
    fn sanity_check() {
        for x in 0..100 {
            let x = x as f32;
            let inv_x = inv_approx(x);
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
}
