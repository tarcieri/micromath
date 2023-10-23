//! Takes the reciprocal (inverse) of a number, `1/x`.

use super::F32;

impl F32 {
    /// Returns the reciprocal (inverse) of a number, `1/x`.
    pub fn recip(self) -> Self {
        1.0 / self
    }
}

#[cfg(test)]
mod tests {
    use super::F32;

    #[test]
    fn sanity_check() {
        assert_eq!(F32(0.0).recip(), F32::INFINITY);
        assert_eq!(F32(-0.0).recip(), F32::NEG_INFINITY);
        assert_eq!(F32(1.0).recip(), F32(1.0));
        assert_eq!(F32(2.0).recip(), F32(0.5));
    }
}
