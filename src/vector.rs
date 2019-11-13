//! Algebraic vector types generic over a number of axes and a component type.
//!
//! The `vector` Cargo feature must be enabled to use this functionality.
//!
//! All vectors types impl the [Vector] trait, and all vector components
//! impl the [Component] trait. The [Vector] trait provides a number of
//! features, including accessing components by `Index<usize>`, iterator
//! support via an [iter()] method which returns an [Iter] type,
//! and a [to_array()] method for returning the vector components as a
//! `GenericArray`.
//!
//! For vectors whose components impl `Into<f32>`, a set of vector geometry
//! extensions are provided by the [VectorExt] trait, including computing
//! the distance between vectors and the magnitude of a vector.
//!
//! [Vector]: https://docs.rs/micromath/latest/micromath/vector/trait.Vector.html
//! [Component]: https://docs.rs/micromath/latest/micromath/vector/trait.Component.html
//! [iter()]: https://docs.rs/micromath/latest/micromath/vector/trait.Vector.html#method.iter
//! [Iter]: https://docs.rs/micromath/latest/micromath/vector/struct.Iter.html
//! [to_array()]: https://docs.rs/micromath/latest/micromath/vector/trait.Vector.html#tymethod.to_array
//! [VectorExt]: https://docs.rs/micromath/latest/micromath/vector/trait.VectorExt.html

#[allow(unused_imports)]
use crate::f32ext::F32Ext;
use core::{
    fmt::Debug,
    ops::{Index, MulAssign},
};
use generic_array::{ArrayLength, GenericArray};

mod component;
mod iter;
mod xy;
mod xyz;

pub use self::{component::*, iter::*, xy::*, xyz::*};

/// Vectors with numeric components
pub trait Vector: Copy + Debug + Default + Index<usize> + PartialEq + Sized + Send + Sync {
    /// Number of axes
    type Axes: ArrayLength<Self::Component>;

    /// Type representing measured acceleration for a particular axis
    type Component: Component;

    /// Smallest value representable by a vector component
    const MIN: Self::Component;

    /// Largest value representable by a vector component
    const MAX: Self::Component;

    /// Instantiate a `Vector` from an iterator over `Self::Component` values.
    ///
    /// Panics of the iterator is not the correct length.
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self::Component>;

    /// Instantiate a vector from a slice of components.
    ///
    /// Panics if the slice is not the right size.
    fn from_slice(slice: &[Self::Component]) -> Self {
        Self::from_iter(slice.iter().cloned())
    }

    /// Get the component value for a particular index
    fn get(self, index: usize) -> Option<Self::Component>;

    /// Iterate over the components of a vector
    fn iter(&self) -> Iter<'_, Self> {
        Iter::new(self)
    }

    /// Obtain an array of the acceleration components for each of the axes
    fn to_array(self) -> GenericArray<Self::Component, Self::Axes>;
}

/// Vector geometry extensions usable on vectors whose components can be
/// converted into `f32`.
pub trait VectorExt: Vector + MulAssign<f32> {
    /// Compute the distance between two vectors
    fn distance(self, other: Self) -> f32;

    /// Compute the magnitude of a vector
    fn magnitude(self) -> f32;
}

impl<V, A, C> VectorExt for V
where
    V: Vector<Axes = A, Component = C> + MulAssign<f32>,
    A: ArrayLength<C>,
    C: Component + Into<f32>,
{
    /// Compute the distance between two vectors
    fn distance(self, other: Self) -> f32 {
        let differences = self
            .iter()
            .zip(other.iter())
            .map(|(a, b)| a.into() - b.into());

        differences.map(|n| n * n).sum::<f32>().sqrt()
    }

    /// Compute the magnitude of a vector
    fn magnitude(self) -> f32 {
        self.iter()
            .map(|n| {
                let n = n.into();
                n * n
            })
            .sum::<f32>()
            .sqrt()
    }
}
