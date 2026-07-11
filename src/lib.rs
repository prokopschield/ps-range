//! Generalized range abstractions.
//!
//! This crate provides three capability traits, alongside the concrete range
//! types that implement them:
//!
//! - [`RangeStart`] describes a range with a defined inclusive lower bound;
//!   it is the shared base of the two extension traits.
//! - [`RangeExt`] describes a range bounded on both ends, exposing its upper
//!   bound in both the inclusive and the exclusive form.
//! - [`PartialRangeExt`] describes a range that may be unbounded above or
//!   empty, exposing its upper bound losslessly through [`RangeEnd`] along
//!   with clamping and intersection operations.
//! - [`Range`] is a concrete bounded range whose end is either inclusive or
//!   exclusive, as recorded by [`RangeEnd`].
//! - [`PartialRange`] is a concrete possibly-unbounded range that also
//!   implements [`Iterator`].
//!
//! Each trait is implemented for the standard library range types, for
//! shared and mutable references, and for the concrete types above, wherever
//! the type's shape supports it. [`RangeStart`] and [`PartialRangeExt`] are
//! also implemented for [`Option`], with [`None`] as the empty range, so a
//! possibly-absent range remains usable as a range. The traits' method names
//! are disjoint, so all three can share scope.
//!
//! Operations interpret `Idx` as a discrete domain in which `x + one` is the
//! successor of `x`. A bound is converted between its inclusive and
//! exclusive forms only where the converted bound is provably representable:
//! clamping compares bounds through [`RangeEnd`] or against window ends that
//! are themselves representable, so an inclusive end at the maximum index
//! value is handled faithfully, without saturation or widening.
//!
//! # Examples
//!
//! ```
//! use ps_range::{PartialRange, PartialRangeExt, RangeExt};
//!
//! // Clamp a slice request to a buffer's length.
//! let buffer = [0u8; 10];
//! let requested = 4usize..;
//!
//! assert_eq!(requested.clamp_right_exclusive(buffer.len()), 4..10);
//! assert_eq!(buffer[requested.clamp_right_exclusive(buffer.len())].len(), 6);
//!
//! // Clamping requires only minimal trait bounds on the index type.
//! assert_eq!((..8).clamp_right(6usize), 0..6);
//!
//! // Iteration eventually leaves every range exhausted.
//! let mut range = PartialRange::from(254u8..);
//!
//! assert_eq!(range.by_ref().collect::<Vec<_>>(), vec![254, 255]);
//! assert_eq!(range, PartialRange::Exhausted);
//! ```
#![warn(missing_docs)]

mod partial_range;
mod partial_range_ext;
mod range;
mod range_ext;
mod start;

pub use partial_range::PartialRange;
pub use partial_range_ext::PartialRangeExt;
pub use range::{Range, RangeEnd};
pub use range_ext::RangeExt;
pub use start::RangeStart;
