use std::mem;

use num_traits::{CheckedAdd, One, Zero};

use crate::{Range, RangeEnd, RangeStart};

/// A range that may be unbounded above, empty, or exhausted.
///
/// [`Empty`](PartialRange::Empty) records a range that never contained any
/// indices, anchored at a position; [`Exhausted`](PartialRange::Exhausted)
/// records a range fully consumed by iteration, anchored at no position. The
/// two are empty in the same way but compare as unequal.
///
/// # Examples
///
/// ```
/// use ps_range::PartialRange;
///
/// let mut range = PartialRange::from(5usize..8);
///
/// assert_eq!(range.by_ref().collect::<Vec<_>>(), vec![5, 6, 7]);
/// assert_eq!(range, PartialRange::Exhausted);
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum PartialRange<Idx> {
    /// An empty range that never contained any indices.
    Empty {
        /// The position at which the empty range is anchored.
        idx: Idx,
    },
    /// A range fully consumed by iteration, anchored at no position.
    Exhausted,
    /// A range unbounded above.
    From {
        /// The `start..` range.
        inner: std::ops::RangeFrom<Idx>,
    },
    /// A bounded range that includes its end.
    Inclusive {
        /// The inclusive lower bound.
        start: Idx,
        /// The inclusive upper bound.
        end: Idx,
    },
    /// A bounded range that excludes its end.
    Exclusive {
        /// The `start..end` range.
        inner: std::ops::Range<Idx>,
    },
}

impl<Idx: Clone + Zero> RangeStart<Idx> for PartialRange<Idx> {
    #[inline]
    fn start(&self) -> Idx {
        match self {
            Self::Empty { idx } => idx.clone(),
            Self::Exhausted => Idx::zero(),
            Self::From { inner } => inner.start.clone(),
            Self::Inclusive { start, .. } => start.clone(),
            Self::Exclusive { inner } => inner.start.clone(),
        }
    }
}

/// The size hint is conservative: an empty range reports an exact zero, and
/// a non-empty range reports a lower bound of one with an unknown upper
/// bound, since computing the exact length would require subtraction and a
/// conversion to `usize` beyond the deliberately minimal `Idx` bounds.
///
/// Iteration eventually leaves every range [`Exhausted`](PartialRange::Exhausted):
/// draining the final element transitions there, and so does the first call
/// on a range that has nothing to yield, discarding an
/// [`Empty`](PartialRange::Empty) anchor. Iteration also makes strict
/// progress: when `checked_add` fails, or yields a successor that is not
/// strictly greater than the current index, as with a saturating addition at
/// its cap, the range yields the current index and becomes exhausted instead
/// of overflowing or repeating forever.
impl<Idx> Iterator for PartialRange<Idx>
where
    Idx: CheckedAdd + Clone + One + PartialOrd,
{
    type Item = Idx;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Exhausted => None,

            Self::Empty { .. } => {
                *self = Self::Exhausted;

                None
            }

            Self::From { inner } => match inner.start.checked_add(&Idx::one()) {
                Some(next) if next > inner.start => Some(mem::replace(&mut inner.start, next)),
                _ => {
                    let last = inner.start.clone();

                    *self = Self::Exhausted;

                    Some(last)
                }
            },

            Self::Inclusive { start, end } => {
                if *start <= *end {
                    match start.checked_add(&Idx::one()) {
                        Some(next) if next > *start && next <= *end => {
                            Some(mem::replace(start, next))
                        }
                        _ => {
                            let last = start.clone();

                            *self = Self::Exhausted;

                            Some(last)
                        }
                    }
                } else {
                    *self = Self::Exhausted;

                    None
                }
            }

            Self::Exclusive { inner } => {
                if inner.start < inner.end {
                    match inner.start.checked_add(&Idx::one()) {
                        Some(next) if next > inner.start && next < inner.end => {
                            Some(mem::replace(&mut inner.start, next))
                        }
                        _ => {
                            let last = inner.start.clone();

                            *self = Self::Exhausted;

                            Some(last)
                        }
                    }
                } else {
                    *self = Self::Exhausted;

                    None
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::Empty { .. } | Self::Exhausted => (0, Some(0)),

            Self::From { .. } => (1, None),

            Self::Inclusive { start, end } => {
                if *start <= *end {
                    (1, None)
                } else {
                    (0, Some(0))
                }
            }

            Self::Exclusive { inner } => {
                if inner.start < inner.end {
                    (1, None)
                } else {
                    (0, Some(0))
                }
            }
        }
    }
}

impl<Idx> std::iter::FusedIterator for PartialRange<Idx> where
    Idx: CheckedAdd + Clone + One + PartialOrd
{
}

impl<Idx> From<std::ops::Range<Idx>> for PartialRange<Idx> {
    #[inline]
    fn from(value: std::ops::Range<Idx>) -> Self {
        Self::Exclusive { inner: value }
    }
}

impl<Idx> From<std::ops::RangeFrom<Idx>> for PartialRange<Idx> {
    #[inline]
    fn from(value: std::ops::RangeFrom<Idx>) -> Self {
        Self::From { inner: value }
    }
}

/// Converts the range, canonicalizing an empty source to the
/// [`Empty`](PartialRange::Empty) variant anchored at its start. The
/// endpoint values of a drained [`std::ops::RangeInclusive`] are
/// unspecified, so the resulting anchor is likewise unspecified; only
/// emptiness is guaranteed.
impl<Idx: PartialOrd> From<std::ops::RangeInclusive<Idx>> for PartialRange<Idx> {
    #[inline]
    fn from(value: std::ops::RangeInclusive<Idx>) -> Self {
        let empty = value.is_empty();
        let (start, end) = value.into_inner();

        if empty {
            Self::Empty { idx: start }
        } else {
            Self::Inclusive { start, end }
        }
    }
}

impl<Idx> From<Range<Idx>> for PartialRange<Idx> {
    #[inline]
    fn from(value: Range<Idx>) -> Self {
        match value.end {
            RangeEnd::Inclusive(end) => Self::Inclusive {
                start: value.start,
                end,
            },
            RangeEnd::Exclusive(end) => Self::Exclusive {
                inner: value.start..end,
            },
        }
    }
}

impl<Idx> IntoIterator for Range<Idx>
where
    Idx: CheckedAdd + Clone + One + PartialOrd,
{
    type Item = Idx;
    type IntoIter = PartialRange<Idx>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.into()
    }
}
