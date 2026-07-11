//! Generalized range abstractions.
//!
//! This crate provides capability traits over the standard library range
//! types, alongside concrete range types that implement them, so range logic
//! can be written once and reused across every representation.
//!
//! Operations interpret `Idx` as a discrete domain in which `x + one` is the
//! successor of `x`. Bounds are converted between their inclusive and
//! exclusive forms only where the converted bound is provably representable.
#![warn(missing_docs)]

mod range;
mod start;

pub use range::{Range, RangeEnd};
pub use start::RangeStart;
