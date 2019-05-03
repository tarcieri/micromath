use super::floor::floor;

/// Floating point ceiling approximation for a single-precision float
pub(super) fn ceil(x: f32) -> f32 {
    -floor(-x)
}

#[cfg(test)]
mod tests {
    use super::ceil;

    #[test]
    fn sanity_check() {
        assert_eq!(ceil(-1.1), -1.0);
        assert_eq!(ceil(-0.1), 0.0);
        assert_eq!(ceil(0.0), 0.0);
        assert_eq!(ceil(1.0), 1.0);
        assert_eq!(ceil(1.1), 2.0);
        assert_eq!(ceil(2.9), 3.0);
    }
}
