//! Embedded math library featuring fast, safe floating point approximations
//! for common arithmetic operations, 2D and 3D vector types, and statistical
//! analysis.
//!
//! ## Floating point approximations
//!
//! `micromath` supports approximating many arithmetic operations on `f32`
//! using bitwise operations, providing great performance and small code size
//! at the cost of precision. For use cases like graphics and signal
//! processing, these approximations are often sufficient and the performance
//! gains worth the lost precision.
//!
//! These approximations are provided by an `F32Ext` trait which is impl'd for
//! `f32`, providing a drop-in `std`-compatible (sans lost precision) API.
//!
//! ```
//! use micromath::F32Ext;
//!
//! let n = 2.0.sqrt();
//! assert_eq!(n, 1.5); // close enough
//! ```
//!
//! ### Unused import warnings when linking `std`
//!
//! Since the `F32Ext` trait provides methods which are already defined in
//! `std`, in cases where your crate links `std` the `F32Ext` versions of
//! the same methods will not be used, in which case you will get an unused
//! import warning for `F32Ext`.
//!
//! If you encounter this, add an `#[allow(unused_imports)]` above the import.
//!
//! ```
//! #[allow(unused_imports)]
//! use micromath::F32Ext;
//! ```

#![no_std]
#![deny(
    warnings,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]
#![forbid(unsafe_code)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/NeoBirth/micromath/develop/img/micromath-sq.png",
    html_root_url = "https://docs.rs/micromath/0.2.1"
)]

mod f32ext;
#[cfg(feature = "statistics")]
pub mod statistics;
#[cfg(feature = "vector")]
pub mod vector;

pub use f32ext::F32Ext;
pub use generic_array;
