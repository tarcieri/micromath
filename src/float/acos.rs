//! arccos approximation for a single-precision float.
//!
//! Method described at:
//! <https://math.stackexchange.com/questions/2908908/express-arccos-in-terms-of-arctan>

use super::F32;

impl F32 {
    /// Computes `acos(x)` approximation in radians in the range `[0, pi]`.
    pub(crate) fn acos(self) -> Self {
        ((Self::ONE - self * self).sqrt() / self).atan()
    }
}

#[cfg(test)]
mod tests {
    use super::F32;
    use core::f32::consts::FRAC_PI_4;

    const MAX_ERROR: f32 = 0.03;

    #[test]
    fn sanity_check() {
        let difference = F32(FRAC_PI_4).cos().acos() - FRAC_PI_4;
        assert!(difference.abs() <= MAX_ERROR);
    }
}
