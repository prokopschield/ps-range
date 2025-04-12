use std::ops::{self, RangeToInclusive};

use num_traits::{One, SaturatingAdd, Zero};

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct OpenRange<Idx> {
    pub start: Idx,
    pub end: Option<Idx>,
}

pub trait PartialRange<Idx: Clone + Ord = usize> {
    #[must_use]
    fn start(&self) -> Idx;

    #[must_use]
    fn end(&self) -> Option<Idx>;

    #[inline]
    #[must_use]
    fn clamp(&self, start: impl Into<Idx>, end: impl Into<Idx>) -> ops::Range<Idx> {
        let lhs_start = start.into();
        let lhs_end = end.into();
        let lhs = lhs_start..lhs_end.clone();

        let rhs_start = self.start();
        let rhs_end = self.end().unwrap_or(lhs_end);
        let rhs = rhs_start..rhs_end;

        Range::intersection(&lhs, &rhs)
    }

    #[inline]
    #[must_use]
    fn clamp_left(&self, start: impl Into<Idx>) -> OpenRange<Idx> {
        OpenRange {
            start: self.start().max(start.into()),
            end: self.end(),
        }
    }

    #[inline]
    #[must_use]
    fn clamp_right(&self, end: impl Into<Idx>) -> ops::Range<Idx> {
        let end = match self.end() {
            Some(rhs_end) => end.into().min(rhs_end),
            None => end.into(),
        };

        let start = self.start().min(end.clone());

        start..end
    }

    #[inline]
    #[must_use]
    fn intersection<T, R>(&self, other: R) -> OpenRange<Idx>
    where
        T: Clone + Ord + Into<Idx>,
        R: PartialRange<T>,
    {
        other.end().map_or_else(
            || self.clamp_left(other.start()),
            |end| self.clamp(other.start(), end).to_open_range(),
        )
    }
}

impl<Idx: Clone + Ord> PartialRange<Idx> for OpenRange<Idx> {
    #[inline]
    #[must_use]
    fn start(&self) -> Idx {
        self.start.clone()
    }

    #[inline]
    #[must_use]
    fn end(&self) -> Option<Idx> {
        self.end.clone()
    }
}

pub trait Range<Idx: Clone + Ord = usize> {
    fn start(&self) -> Idx;
    fn end(&self) -> Idx;

    #[inline]
    #[must_use]
    fn clamp(&self, start: impl Into<Idx>, end: impl Into<Idx>) -> ops::Range<Idx> {
        let end = self.end().min(end.into());
        let start = self.start().max(start.into()).min(end.clone());

        start..end
    }

    #[inline]
    #[must_use]
    fn clamp_left(&self, start: impl Into<Idx>) -> ops::Range<Idx> {
        self.clamp(start, self.end())
    }

    #[inline]
    #[must_use]
    fn clamp_right(&self, end: impl Into<Idx>) -> ops::Range<Idx> {
        self.clamp(self.start(), end)
    }

    #[inline]
    #[must_use]
    fn intersection<T, R>(&self, other: &R) -> ops::Range<Idx>
    where
        T: Clone + Ord + Into<Idx>,
        R: Range<T>,
    {
        self.clamp(other.start(), other.end())
    }

    #[inline]
    #[must_use]
    fn to_open_range(&self) -> OpenRange<Idx> {
        OpenRange {
            start: self.start(),
            end: Some(self.end()),
        }
    }
}

impl<Idx: Clone + Ord> Range<Idx> for ops::Range<Idx> {
    fn start(&self) -> Idx {
        self.start.clone()
    }

    fn end(&self) -> Idx {
        self.end.clone()
    }
}

impl<Idx: Clone + Ord> PartialRange<Idx> for ops::Range<Idx> {
    fn start(&self) -> Idx {
        self.start.clone()
    }

    fn end(&self) -> Option<Idx> {
        Some(self.end.clone())
    }
}

impl<Idx: Clone + Ord> PartialRange<Idx> for ops::RangeFrom<Idx> {
    fn start(&self) -> Idx {
        self.start.clone()
    }

    fn end(&self) -> Option<Idx> {
        None
    }
}

impl<Idx: Clone + Ord + Zero> PartialRange<Idx> for ops::RangeFull {
    fn start(&self) -> Idx {
        Zero::zero()
    }

    fn end(&self) -> Option<Idx> {
        None
    }
}

impl<Idx: Copy + One + Ord + SaturatingAdd> Range<Idx> for ops::RangeInclusive<Idx> {
    fn start(&self) -> Idx {
        *self.start()
    }

    fn end(&self) -> Idx {
        self.end().clone().saturating_add(&One::one())
    }
}

impl<Idx: Copy + One + Ord + SaturatingAdd> PartialRange<Idx> for ops::RangeInclusive<Idx> {
    fn start(&self) -> Idx {
        *self.start()
    }

    fn end(&self) -> Option<Idx> {
        Some(self.end().clone().saturating_add(&One::one()))
    }
}

impl<Idx: Copy + Ord + Zero> Range<Idx> for ops::RangeTo<Idx> {
    fn start(&self) -> Idx {
        Zero::zero()
    }

    fn end(&self) -> Idx {
        self.end
    }
}

impl<Idx: Copy + Ord + Zero> PartialRange<Idx> for ops::RangeTo<Idx> {
    fn start(&self) -> Idx {
        Zero::zero()
    }

    fn end(&self) -> Option<Idx> {
        Some(self.end)
    }
}

impl<Idx: Clone + One + Ord + SaturatingAdd + Zero> Range<Idx> for RangeToInclusive<Idx> {
    fn start(&self) -> Idx {
        Zero::zero()
    }

    fn end(&self) -> Idx {
        self.end.saturating_add(&One::one())
    }
}

impl<Idx: Clone + One + Ord + SaturatingAdd + Zero> PartialRange<Idx> for RangeToInclusive<Idx> {
    fn start(&self) -> Idx {
        Zero::zero()
    }

    fn end(&self) -> Option<Idx> {
        Some(self.end.saturating_add(&One::one()))
    }
}
