/// Compute the absolute value of `n`
/// Method described at: <https://bits.stephan-brumme.com/absFloat.html>
///
/// Constant-time, data-independent implementation.
pub(super) fn abs(n: f32) -> f32 {
    f32::from_bits(n.to_bits() & 0x7FFF_FFFF)
}

#[cfg(test)]
mod tests {
    use super::abs;

    #[test]
    fn sanity_check() {
        assert_eq!(abs(1.0), 1.0);
        assert_eq!(abs(0.0), 0.0);
        assert_eq!(abs(-1.0), 1.0);
    }
}
