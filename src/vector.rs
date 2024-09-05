//! Algebraic vector types generic over a component type.
//!
//! All vectors types impl the [Vector] trait, and all vector components
//! impl the [Component] trait.

mod component;
mod iter;
mod vector2d;
mod vector3d;

pub use self::{
    component::Component,
    iter::Iter,
    vector2d::{F32x2, I16x2, I32x2, I8x2, U16x2, U32x2, U8x2, Vector2d},
    vector3d::{F32x3, I16x3, I32x3, I8x3, U16x3, U32x3, U8x3, Vector3d},
};

use core::{fmt::Debug, iter::FromIterator};

#[allow(unused_imports)]
use crate::F32Ext;

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

    /// Returns a normalized version of the vector.
    fn normalized(mut self) -> Self
    where
        Self: FromIterator<C>,
        C: Into<f32> + From<f32>,
    {
        let norm = self.magnitude();
        self.map(|n| C::from(n.into() / norm))
    }

    /// Applies a function to each element of the vector
    /// and returns a new vector of the transformed elements.
    fn map<F>(&mut self, map: F) -> Self
    where
        F: FnMut(C) -> C,
    {
        Self::from_iter(self.iter().map(map))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalized() {
        const ERROR: f32 = 1e-6;
        let vec = Vector3d {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let norm = vec.magnitude();
        assert!((norm - 7.071068).abs() <= ERROR);

        let normalized = vec.normalized();
        assert!((normalized.x - 0.42426407).abs() <= ERROR);
        assert!((normalized.y - 0.56568545).abs() <= ERROR);
        assert!((normalized.z - 0.70710677).abs() <= ERROR);
    }
}
