/// Square root approximation function for a single-precision float.
/// Method described at: <https://bits.stephan-brumme.com/squareRoot.html>
pub(super) fn sqrt_approx(n: f32) -> f32 {
    let mut n = n.to_bits();
    n += 127 << 23;
    n >>= 1;
    f32::from_bits(n)
}

#[cfg(test)]
mod tests {
    use super::sqrt_approx;

    /// Maximum error as a percentage of the input value (5%)
    const MAX_ERROR: f32 = 0.05;

    #[test]
    fn sanity_check() {
        let sqrt_test_vectors = [
            (1.0, 1.0),
            (2.0, 1.414),
            (3.0, 1.732),
            (4.0, 2.0),
            (5.0, 2.236),
            (10.0, 3.162),
            (100.0, 10.0),
            (250.0, 15.811),
            (500.0, 22.36),
            (1000.0, 31.622),
            (2500.0, 50.0),
            (5000.0, 70.710),
            (1000000.0, 1000.0),
            (2500000.0, 1581.138),
            (5000000.0, 2236.067),
            (10000000.0, 3162.277),
            (25000000.0, 5000.0),
            (50000000.0, 7071.067),
            (100000000.0, 10000.0),
        ];

        for (x, expected) in &sqrt_test_vectors {
            let sqrt_x = sqrt_approx(*x);
            let allowed_delta = x * MAX_ERROR;
            let actual_delta = sqrt_x - expected;

            assert!(
                actual_delta <= allowed_delta,
                "delta {} too large: {} vs {}",
                actual_delta,
                sqrt_x,
                expected
            );
        }
    }
}
