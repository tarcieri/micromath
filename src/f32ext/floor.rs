/// Floating point floor approximation for a single-precision float.
pub(super) fn floor(x: f32) -> f32 {
    let mut x_trunc = (x as i32) as f32;

    if x < x_trunc {
        x_trunc -= 1.0;
    }

    x_trunc
}

#[cfg(test)]
mod tests {
    use super::floor;

    #[test]
    fn sanity_check() {
        assert_eq!(floor(-1.1), -2.0);
        assert_eq!(floor(-0.1), -1.0);
        assert_eq!(floor(0.0), 0.0);
        assert_eq!(floor(1.0), 1.0);
        assert_eq!(floor(1.1), 1.0);
        assert_eq!(floor(2.9), 2.0);
    }
}
