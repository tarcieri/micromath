/// Euclidean reminder for f32
use super::abs;

pub(super) fn rem_euclid(x: f32, y: f32) -> f32 {
    let r = x % y;
    if r < 0.0 {
        r + abs::abs(y)
    } else {
        r
    }
}

#[cfg(test)]
mod tests {
    use super::rem_euclid;
    #[test]
    fn sanity_check() {
        let a = 7.0;
        let b = 4.0;

        assert_eq!(rem_euclid(a, b), 3.0);
        assert_eq!(rem_euclid(-a, b), 1.0);
        assert_eq!(rem_euclid(a, -b), 3.0);
        assert_eq!(rem_euclid(-a, -b), 1.0);
    }
}
