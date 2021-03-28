//! Algebraic vector types generic over a component type.
//!
//! All vectors types impl the [Vector] trait, and all vector components
//! impl the [Component] trait.

use core::{
    fmt::Debug,
    iter::FromIterator,
    marker::PhantomData,
    ops::{Add, AddAssign, Div, Index, Mul, MulAssign, Sub},
};

#[allow(unused_imports)]
use crate::f32ext::F32Ext;

/// Components of numeric vectors.
///
/// All components must be `Copy` + `Sized` types which support a minimal
/// set of arithmeticic operations (`Add`, `Sub`, `Mul`, `Div`), as well as
/// `Default`, `PartialEq` and `PartialOrd`.
///
/// This trait is impl'd for the following primitive types:
///
/// - `i8`, `i16`, `i32`
/// - `u8`, `u16`, `u32`
/// - `f32`
pub trait Component:
    Copy
    + Debug
    + Default
    + PartialEq
    + PartialOrd
    + Send
    + Sized
    + Sync
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
}

impl Component for i8 {}
impl Component for i16 {}
impl Component for i32 {}
impl Component for u8 {}
impl Component for u16 {}
impl Component for u32 {}
impl Component for f32 {}

/// Algebraic vector generic over a given [`Component`] type.
pub trait Vector<C>: Copy + Debug + Default + FromIterator<C> + Send + Sync
where
    C: Component,
{
    /// Number of axes
    const AXES: usize;

    /// Get the component value for a particular index
    fn get(self, index: usize) -> Option<C>;

    /// Compute the dot product of two vectors
    fn dot(self, rhs: Self) -> C;

    /// Instantiate a vector from a slice of components.
    ///
    /// Panics if the slice is not the right size.
    fn from_slice(slice: &[C]) -> Self {
        Self::from_iter(slice.iter().cloned())
    }

    /// Iterate over the components of this vector
    fn iter(&self) -> Iter<'_, Self, C> {
        Iter::new(self)
    }

    /// Compute the distance between two vectors
    fn distance(self, rhs: Self) -> f32
    where
        C: Into<f32>,
    {
        let differences = self
            .iter()
            .zip(rhs.iter())
            .map(|(a, b)| a.into() - b.into());

        differences.map(|n| n * n).sum::<f32>().sqrt()
    }

    /// Compute the magnitude of a vector
    fn magnitude(self) -> f32
    where
        C: Into<f32>,
    {
        self.iter()
            .map(|n| {
                let n = n.into();
                n * n
            })
            .sum::<f32>()
            .sqrt()
    }
}

//
// 2D vector types
//

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
        (self.x * rhs.x) + (self.y * rhs.y)
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

impl<C> MulAssign<C> for Vector2d<C>
where
    C: Component,
{
    fn mul_assign(&mut self, rhs: C) {
        *self = *self * rhs;
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

//
// 3D vector types
//

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

/// Compute the cross product of two vectors
impl<C> Mul for Vector3d<C>
where
    C: Component,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.z * rhs.x) - (self.x * rhs.z),
            z: (self.x * rhs.y) - (self.y * rhs.x),
        }
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

/// Iterator over the components of an algebraic vector
pub struct Iter<'a, V, C>
where
    V: Vector<C>,
    C: Component,
{
    /// Reference to the original vector
    vector: &'a V,

    /// Iteration position within the vector
    position: usize,

    /// Component type
    component: PhantomData<C>,
}

impl<'a, V, C> Iter<'a, V, C>
where
    V: Vector<C>,
    C: Component,
{
    /// Create a new iterator over the vector's components
    pub(super) fn new(vector: &'a V) -> Self {
        Self {
            vector,
            position: 0,
            component: PhantomData,
        }
    }
}

impl<'a, V, C> Iterator for Iter<'a, V, C>
where
    V: Vector<C>,
    C: Component,
{
    type Item = C;

    fn next(&mut self) -> Option<C> {
        let item = self.vector.get(self.position);

        if item.is_some() {
            self.position += 1;
        }

        item
    }
}
