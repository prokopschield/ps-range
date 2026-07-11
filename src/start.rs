use num_traits::Zero;

/// A range with a defined inclusive lower bound.
///
/// This is the shared base of the crate's extension traits. The
/// implementations for
/// [`std::ops::RangeTo`], [`std::ops::RangeToInclusive`],
/// [`std::ops::RangeFull`], and [`None`] anchor the start at
/// [`Idx::zero()`](Zero::zero); for a signed index type the negative portion
/// is not represented, so `(..-5)` reports itself empty.
///
/// For [`std::ops::RangeInclusive`], the inherent `start` method shadows this
/// one; call it as `RangeStart::start(&range)`.
///
/// # Examples
///
/// ```
/// use ps_range::RangeStart;
///
/// assert_eq!((2..8).start(), 2);
/// assert_eq!((..8).start(), 0);
/// assert_eq!(RangeStart::start(&(2..=8)), 2);
/// ```
pub trait RangeStart<Idx = usize> {
    /// Returns the inclusive lower bound of the range.
    #[must_use]
    fn start(&self) -> Idx;
}

impl<Idx: Clone> RangeStart<Idx> for std::ops::Range<Idx> {
    #[inline]
    fn start(&self) -> Idx {
        self.start.clone()
    }
}

impl<Idx: Clone> RangeStart<Idx> for std::ops::RangeFrom<Idx> {
    #[inline]
    fn start(&self) -> Idx {
        self.start.clone()
    }
}

impl<Idx: Zero> RangeStart<Idx> for std::ops::RangeFull {
    #[inline]
    fn start(&self) -> Idx {
        Idx::zero()
    }
}

impl<Idx: Clone> RangeStart<Idx> for std::ops::RangeInclusive<Idx> {
    #[inline]
    fn start(&self) -> Idx {
        self.start().clone()
    }
}

impl<Idx: Zero> RangeStart<Idx> for std::ops::RangeTo<Idx> {
    #[inline]
    fn start(&self) -> Idx {
        Idx::zero()
    }
}

impl<Idx: Zero> RangeStart<Idx> for std::ops::RangeToInclusive<Idx> {
    #[inline]
    fn start(&self) -> Idx {
        Idx::zero()
    }
}

impl<Idx, T> RangeStart<Idx> for Option<T>
where
    Idx: Zero,
    T: RangeStart<Idx>,
{
    #[inline]
    fn start(&self) -> Idx {
        self.as_ref().map_or_else(Idx::zero, RangeStart::start)
    }
}

impl<Idx, T: RangeStart<Idx>> RangeStart<Idx> for &T {
    #[inline]
    fn start(&self) -> Idx {
        (*self).start()
    }
}

impl<Idx, T: RangeStart<Idx>> RangeStart<Idx> for &mut T {
    #[inline]
    fn start(&self) -> Idx {
        (**self).start()
    }
}

#[cfg(test)]
mod tests {
    use super::RangeStart;

    #[test]
    fn stored_starts_are_reported() {
        assert_eq!((2usize..8).start(), 2);
        assert_eq!((2usize..).start(), 2);
        assert_eq!(RangeStart::start(&(2usize..=8)), 2);
    }

    #[test]
    fn end_only_ranges_anchor_at_zero() {
        assert_eq!(RangeStart::<i32>::start(&(..)), 0);
        assert_eq!((..8i32).start(), 0);
        assert_eq!((..=-1i32).start(), 0);
    }

    #[test]
    fn none_anchors_at_zero() {
        assert_eq!(None::<std::ops::Range<usize>>.start(), 0);
        assert_eq!(Some(2usize..8).start(), 2);
    }

    #[test]
    fn references_forward() {
        assert_eq!((2usize..8).start(), 2);
        assert_eq!((&mut (2usize..8)).start(), 2);
    }
}
