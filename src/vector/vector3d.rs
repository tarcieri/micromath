//! 3-dimensional vector

use super::{commutative::impl_commutative, Component, Vector, Vector2d};
use crate::F32;
use core::ops::{Div, DivAssign};
use core::{
    iter::{FromIterator, Sum},
    ops::{Add, AddAssign, Index, Mul, MulAssign, Sub, SubAssign},
};

/// 3-dimensional XYZ vector of `i8` values
pub type I8x3 = Vector3d<i8>;

/// 3-dimensional XYZ vector of `i16` values
pub type I16x3 = Vector3d<i16>;

/// 3-dimensional XYZ vector of `i32` values
pub type I32x3 = Vector3d<i32>;

/// 3-dimensional XYZ vector of `u8` values
pub type U8x3 = Vector3d<u8>;

/// 3-dimensional XYZ vector of `u16` values
pub type U16x3 = Vector3d<u16>;

/// 3-dimensional XYZ vector of `u32` values
pub type U32x3 = Vector3d<u32>;

/// 3-dimensional XYZ vector of `f32` values
pub type F32x3 = Vector3d<f32>;

impl_commutative!(Vector3d, i8);
impl_commutative!(Vector3d, i16);
impl_commutative!(Vector3d, i32);
impl_commutative!(Vector3d, u8);
impl_commutative!(Vector3d, u16);
impl_commutative!(Vector3d, u32);
impl_commutative!(Vector3d, f32);
impl_commutative!(Vector3d, F32);

/// 3-dimensional vector
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vector3d<C: Component> {
    /// X component
    pub x: C,

    /// Y component
    pub y: C,

    /// Z component
    pub z: C,
}

impl<C> Vector3d<C>
where
    C: Component,
{
    /// Return a 3-element array containing the coordinates
    // TODO(tarcieri): move this to the `Vector` trait leveraging const generics?
    pub fn to_array(&self) -> [C; 3] {
        [self.x, self.y, self.z]
    }

    /// Calculates the inner product.
    pub fn dot(self, rhs: Self) -> C {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    /// Calculates the outer product.
    pub fn cross(&self, rhs: Self) -> Vector3d<C> {
        Self {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.z * rhs.x) - (self.x * rhs.z),
            z: (self.x * rhs.y) - (self.y * rhs.x),
        }
    }
}

impl<C> FromIterator<C> for Vector3d<C>
where
    C: Component,
{
    fn from_iter<T>(into_iter: T) -> Self
    where
        T: IntoIterator<Item = C>,
    {
        let mut iter = into_iter.into_iter();

        let x = iter.next().expect("no x-axis component in slice");
        let y = iter.next().expect("no y-axis component in slice");
        let z = iter.next().expect("no z-axis component in slice");

        assert!(
            iter.next().is_none(),
            "too many items for 3-dimensional vector"
        );

        Self { x, y, z }
    }
}

impl<C> Vector<C> for Vector3d<C>
where
    C: Component,
{
    const AXES: usize = 3;

    fn get(self, index: usize) -> Option<C> {
        match index {
            0 => Some(self.x),
            1 => Some(self.y),
            2 => Some(self.z),
            _ => None,
        }
    }

    fn dot(self, rhs: Self) -> C {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }
}

impl<C> From<Vector2d<C>> for Vector3d<C>
where
    C: Component,
{
    fn from(vector: Vector2d<C>) -> Self {
        let zero = C::default();
        Self {
            x: vector.x,
            y: vector.y,
            z: zero,
        }
    }
}

impl<C> From<(C, C, C)> for Vector3d<C>
where
    C: Component,
{
    fn from(vector: (C, C, C)) -> Self {
        Self {
            x: vector.0,
            y: vector.1,
            z: vector.2,
        }
    }
}

impl<C> From<Vector3d<C>> for (C, C, C)
where
    C: Component,
{
    fn from(vector: Vector3d<C>) -> (C, C, C) {
        (vector.x, vector.y, vector.z)
    }
}

impl<C> From<[C; 3]> for Vector3d<C>
where
    C: Component,
{
    fn from(vector: [C; 3]) -> Self {
        Self {
            x: vector[0],
            y: vector[1],
            z: vector[2],
        }
    }
}

impl<C> From<Vector3d<C>> for [C; 3]
where
    C: Component,
{
    fn from(vector: Vector3d<C>) -> [C; 3] {
        vector.to_array()
    }
}

impl<C> Index<usize> for Vector3d<C>
where
    C: Component,
{
    type Output = C;

    fn index(&self, i: usize) -> &C {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of range"),
        }
    }
}

impl<C> Add for Vector3d<C>
where
    C: Component,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<C> AddAssign for Vector3d<C>
where
    C: Component,
{
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<C> Sub for Vector3d<C>
where
    C: Component,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<C> SubAssign for Vector3d<C>
where
    C: Component,
{
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

/// Compute the cross product of two vectors
impl<C> Mul for Vector3d<C>
where
    C: Component,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        self.cross(rhs)
    }
}

/// Multiply vector by a scalar component
impl<C> Mul<C> for Vector3d<C>
where
    C: Component,
{
    type Output = Self;

    fn mul(self, rhs: C) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<C> MulAssign<C> for Vector3d<C>
where
    C: Component,
{
    fn mul_assign(&mut self, rhs: C) {
        *self = *self * rhs;
    }
}

impl<C> Div<C> for Vector3d<C>
where
    C: Component,
{
    type Output = Self;

    fn div(self, rhs: C) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<C> DivAssign<C> for Vector3d<C>
where
    C: Component,
{
    fn div_assign(&mut self, rhs: C) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}

impl<C> Sum<Vector3d<C>> for Vector3d<C>
where
    C: Component,
{
    /// Method which takes an iterator and generates `Self` from the elements by
    /// "summing up" the items.
    ///
    /// ## Example
    /// ```
    /// use micromath::vector::Vector3d;
    /// let vectors = [
    ///     Vector3d { x: 1.0, y: 0.0, z: -2.0 },
    ///     Vector3d { x: 0.0, y: 2.0, z: -1.0 },
    /// ];
    /// let sum: Vector3d<f32> = vectors.iter().copied().sum();
    /// assert_eq!(sum.x, 1.0);
    /// assert_eq!(sum.y, 2.0);
    /// assert_eq!(sum.z, -3.0);
    /// ```
    fn sum<I: Iterator<Item = Vector3d<C>>>(iter: I) -> Self {
        iter.fold(Vector3d::default(), |prev, current| prev + current)
    }
}

impl<'a, C> Sum<&'a Vector3d<C>> for Vector3d<C>
where
    C: Component + 'a,
{
    /// Method which takes an iterator and generates `Self` from the elements by
    /// "summing up" the items.
    ///
    /// ## Example
    /// ```
    /// use micromath::vector::Vector3d;
    /// let vectors = [
    ///     Vector3d { x: 1.0, y: 0.0, z: -2.0 },
    ///     Vector3d { x: 0.0, y: 2.0, z: -1.0 },
    /// ];
    /// let sum: Vector3d<f32> = vectors.iter().copied().sum();
    /// assert_eq!(sum.x, 1.0);
    /// assert_eq!(sum.y, 2.0);
    /// assert_eq!(sum.z, -3.0);
    /// ```
    fn sum<I: Iterator<Item = &'a Vector3d<C>>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl From<I8x3> for F32x3 {
    fn from(vector: I8x3) -> F32x3 {
        Self {
            x: vector.x.into(),
            y: vector.y.into(),
            z: vector.z.into(),
        }
    }
}

impl From<I16x3> for F32x3 {
    fn from(vector: I16x3) -> F32x3 {
        Self {
            x: vector.x.into(),
            y: vector.y.into(),
            z: vector.z.into(),
        }
    }
}

impl From<U8x3> for F32x3 {
    fn from(vector: U8x3) -> F32x3 {
        Self {
            x: vector.x.into(),
            y: vector.y.into(),
            z: vector.z.into(),
        }
    }
}

impl From<U16x3> for F32x3 {
    fn from(vector: U16x3) -> F32x3 {
        Self {
            x: vector.x.into(),
            y: vector.y.into(),
            z: vector.z.into(),
        }
    }
}

impl From<Vector3d<F32>> for F32x3 {
    fn from(vector: Vector3d<F32>) -> F32x3 {
        Self {
            x: vector.x.into(),
            y: vector.y.into(),
            z: vector.z.into(),
        }
    }
}

#[cfg(feature = "defmt")]
#[cfg_attr(docsrs, doc(cfg(feature = "defmt")))]
impl<C> defmt::Format for Vector3d<C>
where
    C: Component + defmt::Format,
{
    fn format(&self, fmt: defmt::Formatter<'_>) {
        defmt::write!(fmt, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_tuple() {
        let vec: Vector3d<_> = (1, 2, 3).into();
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);
        assert_eq!(vec[2], 3);

        let (x, y, z) = vec.into();
        assert_eq!(x, 1);
        assert_eq!(y, 2);
        assert_eq!(z, 3);
    }

    #[test]
    fn from_array() {
        let vec: Vector3d<_> = [1, 2, 3].into();
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);
        assert_eq!(vec[2], 3);

        let arr: [_; 3] = vec.into();
        assert_eq!(arr[0], 1);
        assert_eq!(arr[1], 2);
        assert_eq!(arr[2], 3);
    }

    #[test]
    fn cross() {
        let lhs = Vector3d { x: 1, y: 2, z: 3 };
        let rhs = Vector3d { x: 4, y: 5, z: 6 };
        let cross = lhs.cross(rhs);
        assert_eq!(cross.x, -3);
        assert_eq!(cross.y, 6);
        assert_eq!(cross.z, -3);

        let mul = lhs * rhs;
        assert_eq!(mul, cross);
    }

    #[test]
    fn dot() {
        let lhs = Vector3d { x: 1, y: 2, z: 3 };
        let rhs = Vector3d { x: 4, y: 5, z: 6 };
        let dot = lhs.dot(rhs);
        assert_eq!(dot, 32);
    }

    #[test]
    fn div() {
        let vec = Vector3d {
            x: 10,
            y: 20,
            z: 30,
        };
        let result = vec / 2;
        assert_eq!(result.x, 5);
        assert_eq!(result.y, 10);
        assert_eq!(result.z, 15);
    }

    #[test]
    fn div_assign() {
        let mut vec = Vector3d {
            x: 10,
            y: 20,
            z: 30,
        };
        vec /= 2;
        assert_eq!(vec.x, 5);
        assert_eq!(vec.y, 10);
        assert_eq!(vec.z, 15);
    }
}
