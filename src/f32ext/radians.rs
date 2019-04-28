use core::f32::consts::PI;

/// Assuming `n` is normalized between `[0, 4)`, compute radians
pub(super) fn radians_norm(n: f32) -> f32 {
    PI / 2.0 * if n > 2.0 { n - 4.0 } else { n }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn radians_norm_test() {
        assert_eq!(radians_norm(0.0), 0.0);
        assert_eq!(radians_norm(1.0), PI / 2.0);
        assert_eq!(radians_norm(2.0), PI);
        assert_eq!(radians_norm(3.0), -PI / 2.0);
        assert_eq!(radians_norm(4.0), 0.0);
    }
}
