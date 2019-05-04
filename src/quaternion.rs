// Adapted from the madgwick crate: https://github.com/japaric/madgwick
// Copyright (c) 2018 Jorge Aparicio
//
// Original sources dual licensed under your choice of the Apache 2.0
// and/or MIT licenses, which matches this crate's licensing terms.
//
// See toplevel LICENSE-MIT for more information on the MIT license.
// Apache 2.0 license follows:
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Quaternions are a number system that extends the complex numbers which can
//! be used for efficiently computing spatial rotations.
//!
//! They are computed as the quotient of two directed lines in a
//! three-dimensional space, or equivalently as the quotient of two vectors.
//!
//! For given real numbers `a`, `b`, `c`, and `d`, they take the form:
//!
//! `a + bi + cj + dk`
//!
//! where `i`, `j`, and `k` are the fundamental quaternion units:
//!
//! `i² = j² = k² = i*j*k = -1`
//!
//! Quaternion multiplication is noncommutative:
//!
//! | x | 1  | i  | j  | k  |
//! |---|----|----|----|----|
//! | 1 | 1  | i  | j  | k  |
//! | i | i  | -1 | k  | -j |
//! | j | j  | -k | -1 | i  |
//! | k | k  | j  | -i | -1 |

use core::ops::{AddAssign, Mul, MulAssign, SubAssign};

/// Quaternion
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quaternion(pub f32, pub f32, pub f32, pub f32);

impl Quaternion {
    /// Returns the conjugate of this quaternion
    pub fn conj(self) -> Self {
        Quaternion(self.0, -self.1, -self.2, -self.3)
    }

    /// Returns the norm of this quaternion
    pub fn norm(self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2 + self.3 * self.3
    }
}

impl AddAssign<Quaternion> for Quaternion {
    fn add_assign(&mut self, rhs: Quaternion) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
        self.3 += rhs.3;
    }
}

impl MulAssign<f32> for Quaternion {
    fn mul_assign(&mut self, k: f32) {
        self.0 *= k;
        self.1 *= k;
        self.2 *= k;
        self.3 *= k;
    }
}

impl SubAssign<Quaternion> for Quaternion {
    fn sub_assign(&mut self, rhs: Quaternion) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
        self.3 -= rhs.3;
    }
}

impl Mul<Quaternion> for Quaternion {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Quaternion(
            self.0 * other.0 - self.1 * other.1 - self.2 * other.2 - self.3 * other.3,
            self.0 * other.1 + self.1 * other.0 + self.2 * other.3 - self.3 * other.2,
            self.0 * other.2 - self.1 * other.3 + self.2 * other.0 + self.3 * other.1,
            self.0 * other.3 + self.1 * other.2 - self.2 * other.1 + self.3 * other.0,
        )
    }
}

impl Mul<f32> for Quaternion {
    type Output = Self;

    fn mul(self, k: f32) -> Self {
        Quaternion(self.0 * k, self.1 * k, self.2 * k, self.3 * k)
    }
}

impl Mul<Quaternion> for f32 {
    type Output = Quaternion;

    fn mul(self, q: Quaternion) -> Quaternion {
        q * self
    }
}

#[cfg(tests)]
mod tests {
    // TODO: write test!
    #[test]
    fn add_assign() {}

    // TODO: write test!
    #[test]
    fn mul_assign() {}

    // TODO: write test!
    #[test]
    fn sub_assign() {}

    // TODO: write test!
    #[test]
    fn mul_quaternion() {}

    // TODO: write test!
    #[test]
    fn mul_f32() {}
}
