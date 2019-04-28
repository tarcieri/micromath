//! Algebraic vector types generic over a number of axes and a component type

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
pub trait Vector:
    Copy + Debug + Default + Index<usize> + MulAssign<f32> + PartialEq + Sized + Send + Sync
{
    /// Type representing measured acceleration for a particular axis
    type Component: Component;

    /// Number of axes
    type Axes: ArrayLength<Self::Component>;

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
    fn iter(&self) -> Iter<Self> {
        Iter::new(self)
    }

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

    /// Obtain an array of the acceleration components for each of the axes
    fn to_array(self) -> GenericArray<Self::Component, Self::Axes>;
}
