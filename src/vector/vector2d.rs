//! 2-dimensional vector

use super::{Component, Vector, Vector3d};
use crate::vector::commutative::impl_commutative;
use crate::F32;
use core::ops::{Div, DivAssign};
use core::{
    iter::{FromIterator, Sum},
    ops::{Add, AddAssign, Index, Mul, MulAssign, Sub, SubAssign},
};

/// 2-dimensional XY vector of `i8` values
pub type I8x2 = Vector2d<i8>;

/// 2-dimensional XY vector of `i16` values
pub type I16x2 = Vector2d<i16>;

/// 2-dimensional XY vector of `i32` values
pub type I32x2 = Vector2d<i32>;

/// 2-dimensional XY vector of `u8` values
pub type U8x2 = Vector2d<u8>;

/// 2-dimensional XY vector of `u16` values
pub type U16x2 = Vector2d<u16>;

/// 2-dimensional XY vector of `u32` values
pub type U32x2 = Vector2d<u32>;

/// 2-dimensional XY vector of `f32` values
pub type F32x2 = Vector2d<f32>;

impl_commutative!(Vector2d, i8);
impl_commutative!(Vector2d, i16);
impl_commutative!(Vector2d, i32);
impl_commutative!(Vector2d, u8);
impl_commutative!(Vector2d, u16);
impl_commutative!(Vector2d, u32);
impl_commutative!(Vector2d, f32);
impl_commutative!(Vector2d, F32);

/// 2-dimensional vector
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vector2d<C: Component> {
    /// X component
    pub x: C,

    /// Y component
    pub y: C,
}

impl<C> Vector2d<C>
where
    C: Component,
{
    /// Return a 2-element array containing the coordinates
    // TODO(tarcieri): move this to the `Vector` trait leveraging const generics?
    pub fn to_array(&self) -> [C; 2] {
        [self.x, self.y]
    }

    /// Calculates the inner product.
    pub fn dot(self, rhs: Self) -> C {
        (self.x * rhs.x) + (self.y * rhs.y)
    }

    /// Calculates the perpendicular dot product.
    ///
    /// This value can be understood as the `z` component of the [`cross`](Self::cross) product
    /// between two `Vector2d` instances in 3D space, or the signed area of the parallelogram
    /// formed by the two vectors.
    ///
    /// This value can be used to perform side tests without having to promote the vectors
    /// into [`Vector3d`] instances.
    pub fn perpendicular_dot(self, rhs: Self) -> C {
        (self.x * rhs.y) - (self.y * rhs.x)
    }

    /// Calculates the outer product.
    ///
    /// Note that due to tye type of operation, the result is a [`Vector3d`], not a `Vector2d`.
    /// See also [`perpendicular_dot`](Self::perpendicular_dot) for a simplified version.
    pub fn cross(&self, rhs: Self) -> Vector3d<C> {
        Vector3d::from(*self) * Vector3d::from(rhs)
    }
}

impl<C> FromIterator<C> for Vector2d<C>
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

        assert!(
            iter.next().is_none(),
            "too many items for 2-dimensional vector"
        );

        Self { x, y }
    }
}

impl<C> Vector<C> for Vector2d<C>
where
    C: Component,
{
    const AXES: usize = 2;

    fn get(self, index: usize) -> Option<C> {
        match index {
            0 => Some(self.x),
            1 => Some(self.y),
            _ => None,
        }
    }

    fn dot(self, rhs: Self) -> C {
        self.dot(rhs)
    }
}

impl<C> From<(C, C)> for Vector2d<C>
where
    C: Component,
{
    fn from(vector: (C, C)) -> Self {
        Self {
            x: vector.0,
            y: vector.1,
        }
    }
}

impl<C> From<Vector2d<C>> for (C, C)
where
    C: Component,
{
    fn from(vector: Vector2d<C>) -> (C, C) {
        (vector.x, vector.y)
    }
}

impl<C> From<[C; 2]> for Vector2d<C>
where
    C: Component,
{
    fn from(vector: [C; 2]) -> Self {
        Self {
            x: vector[0],
            y: vector[1],
        }
    }
}

impl<C> From<Vector2d<C>> for [C; 2]
where
    C: Component,
{
    fn from(vector: Vector2d<C>) -> [C; 2] {
        vector.to_array()
    }
}

impl<C> Index<usize> for Vector2d<C>
where
    C: Component,
{
    type Output = C;

    fn index(&self, i: usize) -> &C {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("index out of range"),
        }
    }
}

impl<C> Add for Vector2d<C>
where
    C: Component,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<C> AddAssign for Vector2d<C>
where
    C: Component,
{
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<C> Sub for Vector2d<C>
where
    C: Component,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<C> SubAssign for Vector2d<C>
where
    C: Component,
{
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl<C> Mul<C> for Vector2d<C>
where
    C: Component,
{
    type Output = Self;

    fn mul(self, rhs: C) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<C> Mul<Vector2d<C>> for Vector2d<C>
where
    C: Component,
{
    type Output = Vector3d<C>;

    fn mul(self, rhs: Vector2d<C>) -> Vector3d<C> {
        self.cross(rhs)
    }
}

impl<C> MulAssign<C> for Vector2d<C>
where
    C: Component,
{
    fn mul_assign(&mut self, rhs: C) {
        *self = *self * rhs;
    }
}

impl<C> Div<C> for Vector2d<C>
where
    C: Component,
{
    type Output = Self;

    fn div(self, rhs: C) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<C> DivAssign<C> for Vector2d<C>
where
    C: Component,
{
    fn div_assign(&mut self, rhs: C) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
    }
}

impl<C> Sum<Vector2d<C>> for Vector2d<C>
where
    C: Component,
{
    /// Method which takes an iterator and generates `Self` from the elements by
    /// "summing up" the items.
    ///
    /// ## Example
    /// ```
    /// use micromath::vector::Vector2d;
    /// let vectors = [
    ///     Vector2d { x: 1.0, y: 0.0 },
    ///     Vector2d { x: 0.0, y: 2.0 },
    /// ];
    /// let sum: Vector2d<f32> = vectors.iter().copied().sum();
    /// assert_eq!(sum.x, 1.0);
    /// assert_eq!(sum.y, 2.0);
    /// ```
    fn sum<I: Iterator<Item = Vector2d<C>>>(iter: I) -> Self {
        iter.fold(Vector2d::default(), |prev, current| prev + current)
    }
}

impl<'a, C> Sum<&'a Vector2d<C>> for Vector2d<C>
where
    C: Component + 'a,
{
    /// Method which takes an iterator and generates `Self` from the elements by
    /// "summing up" the items.
    ///
    /// ## Example
    /// ```
    /// use micromath::vector::Vector2d;
    /// let vectors = [
    ///     Vector2d { x: 1.0, y: 0.0 },
    ///     Vector2d { x: 0.0, y: 2.0 },
    /// ];
    /// let sum: Vector2d<f32> = vectors.iter().sum();
    /// assert_eq!(sum.x, 1.0);
    /// assert_eq!(sum.y, 2.0);
    /// ```
    fn sum<I: Iterator<Item = &'a Vector2d<C>>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl From<I8x2> for F32x2 {
    fn from(vector: I8x2) -> F32x2 {
        Self {
            x: vector.x.into(),
            y: vector.y.into(),
        }
    }
}

impl From<I16x2> for F32x2 {
    fn from(vector: I16x2) -> F32x2 {
        Self {
            x: vector.x.into(),
            y: vector.y.into(),
        }
    }
}

impl From<U8x2> for F32x2 {
    fn from(vector: U8x2) -> F32x2 {
        Self {
            x: vector.x.into(),
            y: vector.y.into(),
        }
    }
}

impl From<U16x2> for F32x2 {
    fn from(vector: U16x2) -> F32x2 {
        Self {
            x: vector.x.into(),
            y: vector.y.into(),
        }
    }
}

#[cfg(feature = "defmt")]
#[cfg_attr(docsrs, doc(cfg(feature = "defmt")))]
impl<C> defmt::Format for Vector2d<C>
where
    C: Component + defmt::Format,
{
    fn format(&self, fmt: defmt::Formatter<'_>) {
        defmt::write!(fmt, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_tuple() {
        let vec: Vector2d<_> = (1, 2).into();
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);

        let (x, y) = vec.into();
        assert_eq!(x, 1);
        assert_eq!(y, 2);
    }

    #[test]
    fn from_array() {
        let vec: Vector2d<_> = [1, 2].into();
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);

        let arr: [_; 2] = vec.into();
        assert_eq!(arr[0], 1);
        assert_eq!(arr[1], 2);
    }

    #[test]
    fn cross() {
        let lhs = Vector2d { x: 1, y: 2 };
        let rhs = Vector2d { x: 3, y: 4 };
        let cross = lhs.cross(rhs);
        assert_eq!(cross.x, 0);
        assert_eq!(cross.y, 0);
        assert_eq!(cross.z, -2);

        let mul = lhs * rhs;
        assert_eq!(mul, cross);

        let perp_dot = lhs.perpendicular_dot(rhs);
        assert_eq!(perp_dot, cross.z);
    }

    #[test]
    fn dot() {
        let lhs = Vector2d { x: 1, y: 2 };
        let rhs = Vector2d { x: 3, y: 4 };
        let dot = lhs.dot(rhs);
        assert_eq!(dot, 11);
    }

    #[test]
    fn div() {
        let vec = Vector2d { x: 10, y: 20 };
        let result = vec / 2;
        assert_eq!(result.x, 5);
        assert_eq!(result.y, 10);
    }

    #[test]
    fn div_assign() {
        let mut vec = Vector2d { x: 10, y: 20 };
        vec /= 2;
        assert_eq!(vec.x, 5);
        assert_eq!(vec.y, 10);
    }
}
