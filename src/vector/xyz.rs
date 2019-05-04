//! 3-dimensional vectors (X,Y,Z)

use super::Vector;
use core::ops::{Index, MulAssign};
use generic_array::{arr, typenum::U3, GenericArray};

macro_rules! impl_3d_vector {
    ($vector:ident, $component:tt, $doc:expr) => {
        #[doc=$doc]
        #[derive(Copy, Clone, Debug, Default, PartialEq)]
        pub struct $vector {
            /// X component
            pub x: $component,

            /// Y component
            pub y: $component,

            /// Z component
            pub z: $component,
        }

        impl $vector {
            /// Instantiate from X,Y,Z components
            pub fn new(x: $component, y: $component, z: $component) -> Self {
                $vector { x, y, z }
            }
        }

        impl Vector for $vector {
            type Component = $component;
            type Axes = U3;

            const MIN: $component = core::$component::MIN;
            const MAX: $component = core::$component::MAX;

            fn from_iter<I>(into_iter: I) -> Self
            where
                I: IntoIterator<Item = Self::Component>
            {
                let mut iter = into_iter.into_iter();

                let x = iter.next().expect("no x-axis component in slice");
                let y = iter.next().expect("no y-axis component in slice");
                let z = iter.next().expect("no z-axis component in slice");
                debug_assert!(iter.next().is_none(), "too many items in 3-axis component slice");

                Self::new(x, y, z)
            }

            fn get(self, i: usize) -> Option<Self::Component> {
                if i <= 2 {
                    Some(self[i])
                } else {
                    None
                }
            }

            fn to_array(self) -> GenericArray<$component, U3> {
                arr![$component; self.x, self.y, self.z]
            }
        }

        impl From<($component, $component, $component)> for $vector {
            fn from(vector: ($component, $component, $component)) -> Self {
                $vector::new(vector.0, vector.1, vector.2)
            }
        }

        impl Index<usize> for $vector {
            type Output = $component;

            fn index(&self, i: usize) -> &$component {
                match i {
                    0 => &self.x,
                    1 => &self.y,
                    2 => &self.z,
                    _ => panic!("index out of range")
                }
            }
        }
    }
}

macro_rules! impl_3d_vector_ext {
    ($vector:ident, $component:tt, $doc:expr) => {
        impl_3d_vector!($vector, $component, $doc);

        impl MulAssign<f32> for $vector {
            #[allow(trivial_numeric_casts)]
            fn mul_assign(&mut self, n: f32) {
                self.x = (f32::from(self.x) * n) as $component;
                self.y = (f32::from(self.y) * n) as $component;
                self.z = (f32::from(self.z) * n) as $component;
            }
        }
    };
}

impl_3d_vector_ext!(I8x3, i8, "3-dimensional XYZ vector of `i8` values");
impl_3d_vector_ext!(I16x3, i16, "3-dimensional XYZ vector of `i16` values");
impl_3d_vector!(I32x3, i32, "3-dimensional XYZ vector of `i32` values");
impl_3d_vector_ext!(U8x3, u8, "3-dimensional XYZ vector of `u8` values");
impl_3d_vector_ext!(U16x3, u16, "3-dimensional XYZ vector of `u16` values");
impl_3d_vector!(U32x3, u32, "3-dimensional XYZ vector of `u16` values");
impl_3d_vector_ext!(F32x3, f32, "3-dimensional XYZ vector of `f32` values");

impl From<I8x3> for F32x3 {
    fn from(vector: I8x3) -> F32x3 {
        F32x3::new(vector.x.into(), vector.y.into(), vector.z.into())
    }
}

impl From<I16x3> for F32x3 {
    fn from(vector: I16x3) -> F32x3 {
        F32x3::new(vector.x.into(), vector.y.into(), vector.z.into())
    }
}

impl From<U8x3> for F32x3 {
    fn from(vector: U8x3) -> F32x3 {
        F32x3::new(vector.x.into(), vector.y.into(), vector.z.into())
    }
}

impl From<U16x3> for F32x3 {
    fn from(vector: U16x3) -> F32x3 {
        F32x3::new(vector.x.into(), vector.y.into(), vector.z.into())
    }
}
