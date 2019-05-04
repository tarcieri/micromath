//! Statistical analysis support.
//!
//! The following traits are available and impl'd for slices and iterators of
//! `f32` (and can be impl'd for other types):
//!
//! - [Mean] - compute arithmetic mean with the `mean()` method
//! - [StdDev] - compute standard deviation with the `stddev()` method
//! - [Variance] - compute variance with the `variance() method
//!
//! [Mean]: https://docs.rs/micromath/latest/micromath/statistics/trait.Mean.html
//! [StdDev]: https://docs.rs/micromath/latest/micromath/statistics/trait.StdDev.html
//! [Variance]: https://docs.rs/micromath/latest/micromath/statistics/trait.Variance.html

mod mean;
mod stddev;
mod variance;

pub use self::{mean::Mean, stddev::StdDev, variance::Variance};
