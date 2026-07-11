use num_traits::{CheckedAdd, One, SaturatingSub};

use crate::{Range, RangeEnd};

/// A range bounded on both ends.
///
/// The upper bound is exposed in both of its forms:
/// [`end_exclusive`](RangeExt::end_exclusive) returns [`None`] exactly when
/// the exclusive bound would exceed the maximum index value, that is, when
/// the range ends inclusively at the maximum, and
/// [`end_inclusive`](RangeExt::end_inclusive) is meaningful only for a
/// non-empty range, so `(0..0).end_inclusive()` is `0`. The clamping
/// operations consult only [`end_exclusive`](RangeExt::end_exclusive) and
/// compare against bounds that are at most the maximum index value, so no
/// operation converts a bound it cannot represent.
///
/// # Examples
///
/// ```
/// use ps_range::RangeExt;
///
/// assert_eq!((..8).clamp_right(6usize), 0..6);
/// assert_eq!((2..=8usize).end_exclusive(), Some(9));
/// assert_eq!((2..=u8::MAX).end_exclusive(), None);
/// assert_eq!((0..=u8::MAX).clamp_to(10, 200), 10..200);
/// ```
pub trait RangeExt<Idx = usize>: crate::RangeStart<Idx>
where
    Idx: Clone + Ord,
{
    /// Returns the inclusive upper bound.
    ///
    /// The returned bound is meaningful only for a non-empty range: an empty
    /// range has no last index, so `(0..0).end_inclusive()` returns `0`.
    #[must_use]
    fn end_inclusive(&self) -> Idx;

    /// Returns the exclusive upper bound, or [`None`] if it would exceed the
    /// maximum index value, that is, if the range ends inclusively at the
    /// maximum.
    #[must_use]
    fn end_exclusive(&self) -> Option<Idx>;

    /// Restricts the range to the bounds `start..end`.
    ///
    /// The result is slice-safe: its start does not exceed its end, and a
    /// window disjoint from the range yields an empty range anchored at the
    /// computed end.
    #[inline]
    #[must_use]
    fn clamp_to(&self, start: impl Into<Idx>, end: impl Into<Idx>) -> std::ops::Range<Idx> {
        let window_end = end.into();

        // `None` means the exclusive bound exceeds the maximum index value,
        // which the window's end cannot, so the window binds.
        let end = match self.end_exclusive() {
            Some(end) => end.min(window_end),
            None => window_end,
        };
        let start = self.start().max(start.into()).min(end.clone());

        start..end
    }

    /// Lowers the upper bound to at most `end`.
    #[inline]
    #[must_use]
    fn clamp_right(&self, end: impl Into<Idx>) -> std::ops::Range<Idx> {
        self.clamp_to(self.start(), end)
    }
}

impl<Idx> RangeExt<Idx> for std::ops::Range<Idx>
where
    Idx: Clone + Ord + One + SaturatingSub,
{
    #[inline]
    fn end_inclusive(&self) -> Idx {
        self.end.saturating_sub(&Idx::one())
    }

    #[inline]
    fn end_exclusive(&self) -> Option<Idx> {
        Some(self.end.clone())
    }
}

/// The accessors collapse a drained range, whose endpoint values the
/// standard library leaves unspecified, to an empty range at its start, so
/// the clamping operations cannot resurrect it.
impl<Idx> RangeExt<Idx> for std::ops::RangeInclusive<Idx>
where
    Idx: Clone + Ord + One + CheckedAdd,
{
    #[inline]
    fn end_inclusive(&self) -> Idx {
        self.end().clone()
    }

    #[inline]
    fn end_exclusive(&self) -> Option<Idx> {
        if self.is_empty() {
            Some(self.start().clone())
        } else {
            self.end().checked_add(&Idx::one())
        }
    }
}

impl<Idx> RangeExt<Idx> for std::ops::RangeTo<Idx>
where
    Idx: Clone + Ord + num_traits::Zero + One + SaturatingSub,
{
    #[inline]
    fn end_inclusive(&self) -> Idx {
        self.end.saturating_sub(&Idx::one())
    }

    #[inline]
    fn end_exclusive(&self) -> Option<Idx> {
        Some(self.end.clone())
    }
}

impl<Idx> RangeExt<Idx> for std::ops::RangeToInclusive<Idx>
where
    Idx: Clone + Ord + num_traits::Zero + One + CheckedAdd,
{
    #[inline]
    fn end_inclusive(&self) -> Idx {
        self.end.clone()
    }

    #[inline]
    fn end_exclusive(&self) -> Option<Idx> {
        self.end.checked_add(&Idx::one())
    }
}

impl<Idx> RangeExt<Idx> for Range<Idx>
where
    Idx: Clone + Ord + One + CheckedAdd + SaturatingSub,
{
    #[inline]
    fn end_inclusive(&self) -> Idx {
        match &self.end {
            RangeEnd::Inclusive(end) => end.clone(),
            RangeEnd::Exclusive(end) => end.saturating_sub(&Idx::one()),
        }
    }

    #[inline]
    fn end_exclusive(&self) -> Option<Idx> {
        match &self.end {
            RangeEnd::Inclusive(end) => end.checked_add(&Idx::one()),
            RangeEnd::Exclusive(end) => Some(end.clone()),
        }
    }
}

impl<Idx, T> RangeExt<Idx> for &T
where
    Idx: Clone + Ord,
    T: RangeExt<Idx>,
{
    #[inline]
    fn end_inclusive(&self) -> Idx {
        (*self).end_inclusive()
    }

    #[inline]
    fn end_exclusive(&self) -> Option<Idx> {
        (*self).end_exclusive()
    }
}

impl<Idx, T> RangeExt<Idx> for &mut T
where
    Idx: Clone + Ord,
    T: RangeExt<Idx>,
{
    #[inline]
    fn end_inclusive(&self) -> Idx {
        (**self).end_inclusive()
    }

    #[inline]
    fn end_exclusive(&self) -> Option<Idx> {
        (**self).end_exclusive()
    }
}
