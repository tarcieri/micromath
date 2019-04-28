//! Embedded math library featuring fast, safe floating point approximations
//! for common arithmetic operations, 2D and 3D vector types, and statistical
//! analysis.

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
#![doc(html_root_url = "https://docs.rs/micromath/0.0.0")]

mod f32ext;

pub use f32ext::F32Ext;
