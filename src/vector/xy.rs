//! 2-dimensional vectors (X,Y)

use super::Vector;
use core::ops::{Index, MulAssign};
use generic_array::{arr, typenum::U2, GenericArray};

macro_rules! impl_2d_vector {
    ($vector:ident, $component:tt, $doc:expr) => {
        #[doc=$doc]
        #[derive(Copy, Clone, Debug, Default, PartialEq)]
        pub struct $vector {
            /// X component
            pub x: $component,

            /// Y component
            pub y: $component,
        }

        impl $vector {
            /// Instantiate from X and Y components
            pub fn new(x: $component, y: $component) -> Self {
                $vector { x, y }
            }
        }

        impl Vector for $vector {
            type Component = $component;
            type Axes = U2;

            const MIN: $component = core::$component::MIN;
            const MAX: $component = core::$component::MAX;

            fn from_iter<I>(into_iter: I) -> Self
            where
                I: IntoIterator<Item = Self::Component>
            {
                let mut iter = into_iter.into_iter();

                let x = iter.next().expect("no x-axis component in slice");
                let y = iter.next().expect("no y-axis component in slice");
                debug_assert!(iter.next().is_none(), "too many items in 2-dimensional component slice");

                Self::new(x, y)
            }

            fn get(self, i: usize) -> Option<Self::Component> {
                if i <= 1 {
                    Some(self[i])
                } else {
                    None
                }
            }

            fn to_array(self) -> GenericArray<$component, U2> {
                arr![$component; self.x, self.y]
            }
        }

        impl From<($component, $component)> for $vector {
            fn from(vector: ($component, $component)) -> Self {
                $vector::new(vector.0, vector.1)
            }
        }

        impl Index<usize> for $vector {
            type Output = $component;

            fn index(&self, i: usize) -> &$component {
                match i {
                    0 => &self.x,
                    1 => &self.y,
                    _ => panic!("index out of range")
                }
            }
        }
    }
}

macro_rules! impl_2d_vector_ext {
    ($vector:ident, $component:tt, $doc:expr) => {
        impl_2d_vector!($vector, $component, $doc);

        impl MulAssign<f32> for $vector {
            #[allow(trivial_numeric_casts)]
            fn mul_assign(&mut self, n: f32) {
                self.x = (f32::from(self.x) * n) as $component;
                self.y = (f32::from(self.y) * n) as $component;
            }
        }
    };
}

impl_2d_vector_ext!(I8x2, i8, "2-dimensional XY vector of `i8` values");
impl_2d_vector_ext!(I16x2, i16, "2-dimensional XY vector of `i16` values");
impl_2d_vector!(I32x2, i32, "2-dimensional XY vector of `i32` values");
impl_2d_vector_ext!(U8x2, u8, "2-dimensional XY vector of `u8` values");
impl_2d_vector_ext!(U16x2, u16, "2-dimensional XY vector of `u16` values");
impl_2d_vector!(U32x2, u32, "2-dimensional XY vector of `u32` values");
impl_2d_vector_ext!(F32x2, f32, "2-dimensional XY vector of `f32` values");

impl MulAssign<i8> for I8x2 {
    fn mul_assign(&mut self, n: i8) {
        self.x *= n;
        self.y *= n;
    }
}

impl MulAssign<i16> for I16x2 {
    fn mul_assign(&mut self, n: i16) {
        self.x *= n;
        self.y *= n;
    }
}

impl MulAssign<u8> for U8x2 {
    fn mul_assign(&mut self, n: u8) {
        self.x *= n;
        self.y *= n;
    }
}

impl MulAssign<u16> for U16x2 {
    fn mul_assign(&mut self, n: u16) {
        self.x *= n;
        self.y *= n;
    }
}

impl From<I8x2> for F32x2 {
    fn from(vector: I8x2) -> F32x2 {
        F32x2::new(vector.x.into(), vector.y.into())
    }
}

impl From<I16x2> for F32x2 {
    fn from(vector: I16x2) -> F32x2 {
        F32x2::new(vector.x.into(), vector.y.into())
    }
}

impl From<U8x2> for F32x2 {
    fn from(vector: U8x2) -> F32x2 {
        F32x2::new(vector.x.into(), vector.y.into())
    }
}

impl From<U16x2> for F32x2 {
    fn from(vector: U16x2) -> F32x2 {
        F32x2::new(vector.x.into(), vector.y.into())
    }
}
