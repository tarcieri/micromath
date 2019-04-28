//! Statistical analysis support

mod mean;
mod stddev;
mod variance;

pub use self::{mean::Mean, stddev::StdDev, variance::Variance};
