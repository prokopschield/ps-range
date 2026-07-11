use crate::RangeStart;

/// The upper bound of a [`Range`], either inclusive or exclusive.
///
/// Ordering is by boundary position: bounds compare by their index first, and
/// on equal indices `Exclusive(n) < Inclusive(n)`, since an inclusive bound
/// extends one element further. `Inclusive(n)` and `Exclusive(n + 1)` denote
/// the same boundary but compare as unequal, with `Inclusive(n)` ordered
/// first, consistent with [`Eq`].
///
/// # Examples
///
/// ```
/// use ps_range::RangeEnd;
///
/// assert!(RangeEnd::Exclusive(5) < RangeEnd::Inclusive(5));
/// assert!(RangeEnd::Inclusive(4) < RangeEnd::Exclusive(5));
/// ```
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum RangeEnd<Idx> {
    /// The end index is part of the range.
    Inclusive(Idx),
    /// The end index is not part of the range.
    Exclusive(Idx),
}

const fn index<Idx>(end: &RangeEnd<Idx>) -> &Idx {
    match end {
        RangeEnd::Inclusive(index) | RangeEnd::Exclusive(index) => index,
    }
}

const fn rank<Idx>(end: &RangeEnd<Idx>) -> u8 {
    match end {
        RangeEnd::Exclusive(_) => 0,
        RangeEnd::Inclusive(_) => 1,
    }
}

impl<Idx: Ord> Ord for RangeEnd<Idx> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        index(self)
            .cmp(index(other))
            .then_with(|| rank(self).cmp(&rank(other)))
    }
}

impl<Idx: PartialOrd> PartialOrd for RangeEnd<Idx> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        index(self)
            .partial_cmp(index(other))
            .map(|ordering| ordering.then_with(|| rank(self).cmp(&rank(other))))
    }
}

/// A range bounded on both ends, with an inclusive start and an end whose
/// inclusivity is recorded by [`RangeEnd`].
///
/// Ordering is lexicographic: by start first, then by end.
///
/// # Examples
///
/// ```
/// use ps_range::Range;
///
/// assert_eq!(Range::from(2..7), Range::exclusive(2, 7));
/// assert_ne!(Range::inclusive(2, 6), Range::exclusive(2, 7));
/// ```
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Range<Idx> {
    /// The inclusive lower bound.
    pub start: Idx,
    /// The upper bound, either inclusive or exclusive.
    pub end: RangeEnd<Idx>,
}

impl<Idx> Range<Idx> {
    /// Creates a range from a start index and an upper bound.
    #[inline]
    #[must_use]
    pub const fn new(start: Idx, end: RangeEnd<Idx>) -> Self {
        Self { start, end }
    }

    /// Creates the range `start..=end`.
    #[inline]
    #[must_use]
    pub const fn inclusive(start: Idx, end: Idx) -> Self {
        Self::new(start, RangeEnd::Inclusive(end))
    }

    /// Creates the range `start..end`.
    #[inline]
    #[must_use]
    pub const fn exclusive(start: Idx, end: Idx) -> Self {
        Self::new(start, RangeEnd::Exclusive(end))
    }
}

impl<Idx: Clone> RangeStart<Idx> for Range<Idx> {
    #[inline]
    fn start(&self) -> Idx {
        self.start.clone()
    }
}

impl<Idx: Clone + Ord> crate::PartialRangeExt<Idx> for Range<Idx> {
    #[inline]
    fn end_bound(&self) -> Option<RangeEnd<Idx>> {
        Some(self.end.clone())
    }
}

impl<Idx> From<std::ops::Range<Idx>> for Range<Idx> {
    #[inline]
    fn from(value: std::ops::Range<Idx>) -> Self {
        Self::exclusive(value.start, value.end)
    }
}

/// Converts the range, canonicalizing an empty source to an empty exclusive
/// range at its start. The endpoint values of a drained
/// [`std::ops::RangeInclusive`] are unspecified, so the position of the
/// resulting empty range is likewise unspecified; only emptiness is
/// guaranteed.
impl<Idx: Clone + PartialOrd> From<std::ops::RangeInclusive<Idx>> for Range<Idx> {
    #[inline]
    fn from(value: std::ops::RangeInclusive<Idx>) -> Self {
        let empty = value.is_empty();
        let (start, end) = value.into_inner();

        if empty {
            Self::exclusive(start.clone(), start)
        } else {
            Self::inclusive(start, end)
        }
    }
}
