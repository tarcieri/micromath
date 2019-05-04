//! Traits for vector components

use core::ops::{Add, Div, Mul, Sub};

/// Vector components
pub trait Component:
    Copy
    + Default
    + Sized
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + PartialOrd
    + PartialEq
{
}

impl Component for i8 {}
impl Component for i16 {}
impl Component for i32 {}
impl Component for u8 {}
impl Component for u16 {}
impl Component for u32 {}
impl Component for f32 {}
