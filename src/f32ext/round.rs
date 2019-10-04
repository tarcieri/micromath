use super::copysign::copysign;

pub(super) fn round(x: f32) -> f32 {
    ((x + copysign(0.5, x)) as i32) as f32
}

#[cfg(test)]
mod tests {
    use super::round;
    #[test]
    fn sanity_check() {
        assert_eq!(round(0.0), 0.0);
        assert_eq!(round(-0.0), -0.0);

        assert_eq!(round(0.49999), 0.0);
        assert_eq!(round(-0.49999), -0.0);

        assert_eq!(round(0.5), 1.0);
        assert_eq!(round(-0.5), -1.0);

        assert_eq!(round(9999.499), 9999.0);
        assert_eq!(round(-9999.499), -9999.0);

        assert_eq!(round(9999.5), 10000.0);
        assert_eq!(round(-9999.5), -10000.0);
    }
}
