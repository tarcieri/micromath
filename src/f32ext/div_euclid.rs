/// Euclidean division for f32
use super::trunc;

pub(super) fn div_euclid(x: f32, y: f32) -> f32 {
    let q = trunc::trunc_sign(x / y);
    if x % y < 0.0 {
        return if y > 0.0 { q - 1.0 } else { q + 1.0 };
    }
    q
}

#[cfg(test)]
mod tests {
    use super::div_euclid;
    #[test]
    fn sanity_check() {
        let a = 7.0;
        let b = 4.0;

        assert_eq!(div_euclid(a, b), 1.0);
        assert_eq!(div_euclid(-a, b), -2.0);
        assert_eq!(div_euclid(a, -b), -1.0);
        assert_eq!(div_euclid(-a, -b), 2.0);
    }
}
