use num_traits::{One, Zero};

use crate::{PartialRange, Range, RangeEnd, RangeStart};

/// A range that may be unbounded above or empty.
///
/// Implementors provide [`start`](RangeStart::start) via the
/// [`RangeStart`] supertrait and override
/// [`end_bound`](PartialRangeExt::end_bound) when the range is bounded; the
/// default returns [`None`], describing a range unbounded above. An
/// implementor whose emptiness is not derivable from those two accessors,
/// such as a type with hidden exhaustion state like
/// [`std::ops::RangeInclusive`], must also override
/// [`is_empty`](PartialRangeExt::is_empty); every derived operation consults
/// [`is_empty`](PartialRangeExt::is_empty) before the bounds. The standard
/// library leaves the endpoint values of a drained
/// [`std::ops::RangeInclusive`] unspecified, so the position of an empty
/// result derived from one is likewise unspecified; only emptiness is
/// guaranteed.
///
/// The operations interpret `Idx` as a discrete domain in which `x + one` is
/// the successor of `x`, and convert a bound between its inclusive and
/// exclusive forms only where the converted bound is provably at most the
/// window's end. Empty results from the [`PartialRange`]-returning
/// operations anchor at the raised or joint start; the
/// [`std::ops::Range`]- and [`Range`]-returning operations instead lower the
/// start to the computed end, so their results are always slice-safe.
///
/// # Examples
///
/// ```
/// use ps_range::{PartialRange, PartialRangeExt, Range};
///
/// assert_eq!((5usize..=10).clamp_exclusive(0usize, 7usize), 5..7);
/// assert_eq!((5usize..).clamp_inclusive(0usize, 7usize), Range::inclusive(5, 7));
/// assert!((5usize..=10).clamp_exclusive(20usize, 25usize).is_empty());
///
/// assert_eq!(
///     (5usize..=10).intersection(&(8usize..)),
///     PartialRange::Inclusive { start: 8, end: 10 }
/// );
/// ```
pub trait PartialRangeExt<Idx = usize>: RangeStart<Idx>
where
    Idx: Clone + Ord,
{
    /// Returns the upper bound, or [`None`] if the range is unbounded above.
    #[inline]
    #[must_use]
    fn end_bound(&self) -> Option<RangeEnd<Idx>> {
        None
    }

    /// Returns `true` if the range contains no indices.
    #[inline]
    #[must_use]
    fn is_empty(&self) -> bool {
        match self.end_bound() {
            Some(RangeEnd::Exclusive(end)) => end <= self.start(),
            Some(RangeEnd::Inclusive(end)) => end < self.start(),
            None => false,
        }
    }

    /// Restricts the range to the window `start..exclusive_end`.
    ///
    /// The result is slice-safe: its start does not exceed its end, and its
    /// end does not exceed the window's. An empty input yields an empty
    /// range at its start clamped into the window; a disjoint input yields
    /// one at the computed end, which may fall below the window's start.
    #[inline]
    #[must_use]
    fn clamp_exclusive(
        &self,
        start: impl Into<Idx>,
        exclusive_end: impl Into<Idx>,
    ) -> std::ops::Range<Idx>
    where
        Idx: std::ops::Add<Output = Idx> + One,
    {
        let window_start = start.into();
        let window_end = exclusive_end.into();

        if self.is_empty() {
            let anchor = self.start().max(window_start).min(window_end);

            return anchor.clone()..anchor;
        }

        let end = match self.end_bound() {
            None => window_end,
            Some(RangeEnd::Exclusive(end)) => end.min(window_end),
            Some(RangeEnd::Inclusive(end)) => {
                // `end < window_end <= MAX`, so the successor is representable
                // and does not exceed the window's end.
                if end < window_end {
                    end + Idx::one()
                } else {
                    window_end
                }
            }
        };
        let start = self.start().max(window_start).min(end.clone());

        start..end
    }

    /// Restricts the range to the window `start..=inclusive_end`.
    ///
    /// The result is always bounded, so it is expressed as a [`Range`]. An
    /// empty input yields an empty exclusive [`Range`] at its start clamped
    /// into the window; a disjoint input yields one at the computed end,
    /// which may fall below the window's start.
    #[inline]
    #[must_use]
    fn clamp_inclusive(&self, start: impl Into<Idx>, inclusive_end: impl Into<Idx>) -> Range<Idx> {
        let window_start = start.into();
        let window_end = inclusive_end.into();

        if self.is_empty() {
            let anchor = self.start().max(window_start).min(window_end);

            return Range::exclusive(anchor.clone(), anchor);
        }

        let end = match self.end_bound() {
            None => RangeEnd::Inclusive(window_end),
            Some(end) => end.min(RangeEnd::Inclusive(window_end)),
        };
        let start = self.start().max(window_start);

        match end {
            RangeEnd::Inclusive(end) if start <= end => Range::inclusive(start, end),
            RangeEnd::Exclusive(end) if start < end => Range::exclusive(start, end),
            RangeEnd::Inclusive(end) | RangeEnd::Exclusive(end) => {
                Range::exclusive(end.clone(), end)
            }
        }
    }

    /// Caps the upper bound at `inclusive_end`.
    #[inline]
    #[must_use]
    fn clamp_right_inclusive(&self, inclusive_end: impl Into<Idx>) -> Range<Idx> {
        self.clamp_inclusive(self.start(), inclusive_end)
    }

    /// Caps the upper bound at `exclusive_end`.
    #[inline]
    #[must_use]
    fn clamp_right_exclusive(&self, exclusive_end: impl Into<Idx>) -> std::ops::Range<Idx>
    where
        Idx: std::ops::Add<Output = Idx> + One,
    {
        self.clamp_exclusive(self.start(), exclusive_end)
    }

    /// Raises the lower bound to at least `start`, preserving boundedness
    /// above.
    ///
    /// If the range is empty, or `start` is disjoint from it, the result is
    /// the [`Empty`](PartialRange::Empty) variant anchored at the raised
    /// start.
    #[inline]
    #[must_use]
    fn clamp_left(&self, start: impl Into<Idx>) -> PartialRange<Idx> {
        let start = self.start().max(start.into());

        if self.is_empty() {
            return PartialRange::Empty { idx: start };
        }

        match self.end_bound() {
            None => PartialRange::From { inner: start.. },
            Some(RangeEnd::Inclusive(end)) if start <= end => {
                PartialRange::Inclusive { start, end }
            }
            Some(RangeEnd::Exclusive(end)) if start < end => {
                PartialRange::Exclusive { inner: start..end }
            }
            Some(_) => PartialRange::Empty { idx: start },
        }
    }

    /// Returns the overlap between this range and `other`.
    ///
    /// The result is unbounded above only when both ranges are. If either
    /// range is empty, or the ranges are disjoint, the result is the
    /// [`Empty`](PartialRange::Empty) variant anchored at the joint start.
    #[inline]
    #[must_use]
    fn intersection<R>(&self, other: &R) -> PartialRange<Idx>
    where
        R: PartialRangeExt<Idx>,
    {
        let start = self.start().max(other.start());

        if self.is_empty() || other.is_empty() {
            return PartialRange::Empty { idx: start };
        }

        let end = match (self.end_bound(), other.end_bound()) {
            (None, None) => return PartialRange::From { inner: start.. },
            (Some(end), None) | (None, Some(end)) => end,
            (Some(lhs), Some(rhs)) => lhs.min(rhs),
        };

        match end {
            RangeEnd::Inclusive(end) if start <= end => PartialRange::Inclusive { start, end },
            RangeEnd::Exclusive(end) if start < end => {
                PartialRange::Exclusive { inner: start..end }
            }
            _ => PartialRange::Empty { idx: start },
        }
    }
}

impl<Idx: Clone + Ord> PartialRangeExt<Idx> for std::ops::Range<Idx> {
    #[inline]
    fn end_bound(&self) -> Option<RangeEnd<Idx>> {
        Some(RangeEnd::Exclusive(self.end.clone()))
    }
}

impl<Idx: Clone + Ord> PartialRangeExt<Idx> for std::ops::RangeFrom<Idx> {}

impl<Idx: Clone + Ord + Zero> PartialRangeExt<Idx> for std::ops::RangeFull {}

impl<Idx: Clone + Ord> PartialRangeExt<Idx> for std::ops::RangeInclusive<Idx> {
    #[inline]
    fn end_bound(&self) -> Option<RangeEnd<Idx>> {
        Some(RangeEnd::Inclusive(self.end().clone()))
    }

    #[inline]
    fn is_empty(&self) -> bool {
        Self::is_empty(self)
    }
}

impl<Idx: Clone + Ord + Zero> PartialRangeExt<Idx> for std::ops::RangeTo<Idx> {
    #[inline]
    fn end_bound(&self) -> Option<RangeEnd<Idx>> {
        Some(RangeEnd::Exclusive(self.end.clone()))
    }
}

impl<Idx: Clone + Ord + Zero> PartialRangeExt<Idx> for std::ops::RangeToInclusive<Idx> {
    #[inline]
    fn end_bound(&self) -> Option<RangeEnd<Idx>> {
        Some(RangeEnd::Inclusive(self.end.clone()))
    }
}

/// [`None`] is the empty range, anchored at [`Idx::zero()`](Zero::zero), so
/// a possibly-absent range remains usable as a range.
impl<Idx, T> PartialRangeExt<Idx> for Option<T>
where
    Idx: Clone + Ord + Zero,
    T: PartialRangeExt<Idx>,
{
    #[inline]
    fn end_bound(&self) -> Option<RangeEnd<Idx>> {
        self.as_ref().map_or_else(
            || Some(RangeEnd::Exclusive(Idx::zero())),
            PartialRangeExt::end_bound,
        )
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.as_ref().is_none_or(PartialRangeExt::is_empty)
    }
}

impl<Idx, T> PartialRangeExt<Idx> for &T
where
    Idx: Clone + Ord,
    T: PartialRangeExt<Idx>,
{
    #[inline]
    fn end_bound(&self) -> Option<RangeEnd<Idx>> {
        (*self).end_bound()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        (*self).is_empty()
    }
}

impl<Idx, T> PartialRangeExt<Idx> for &mut T
where
    Idx: Clone + Ord,
    T: PartialRangeExt<Idx>,
{
    #[inline]
    fn end_bound(&self) -> Option<RangeEnd<Idx>> {
        (**self).end_bound()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        (**self).is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::PartialRangeExt;
    use crate::PartialRange;

    struct StartOnlyRange;

    impl crate::RangeStart<usize> for StartOnlyRange {
        fn start(&self) -> usize {
            42
        }
    }

    impl PartialRangeExt<usize> for StartOnlyRange {}

    #[test]
    fn a_start_only_implementor_is_unbounded_above() {
        let range = StartOnlyRange;

        assert_eq!(range.end_bound(), None);
        assert!(!range.is_empty());
        assert_eq!(range.clamp_right_exclusive(50usize), 42..50);
        assert_eq!(
            range.clamp_left(50usize),
            PartialRange::From { inner: 50.. }
        );
    }
}
