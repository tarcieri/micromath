//! Traits for vector components

use core::ops::{Add, Div, Mul, Sub};

/// Vector components. All components must be `Copy` + `Sized` types which
/// support basic arithmetic (`Add`, `Sub`, `Mul`, `Div`), as well as `Default`,
/// `PartialEq` and `PartialOrd`.
///
/// This trait is impl'd for the following primitive types:
///
/// - `i8`, `i16`, `i32`
/// - `u8`, `u16`, `u32`
/// - `f32`
pub trait Component:
    Copy
    + Default
    + PartialEq
    + PartialOrd
    + Sized
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
