use ps_range::{PartialRange, Range};

/// A partially ordered index: values on different branches are incomparable.
#[derive(Clone, Debug, PartialEq)]
struct Branchy(u8, u8);

impl PartialOrd for Branchy {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.0 == other.0).then_some(self.1.cmp(&other.1))
    }
}

impl std::ops::Add for Branchy {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0, self.1 + rhs.1)
    }
}

impl std::ops::Mul for Branchy {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.0, self.1 * rhs.1)
    }
}

impl num_traits::One for Branchy {
    fn one() -> Self {
        Self(0, 1)
    }
}

impl num_traits::CheckedAdd for Branchy {
    fn checked_add(&self, rhs: &Self) -> Option<Self> {
        self.1.checked_add(rhs.1).map(|value| Self(self.0, value))
    }
}

/// An index whose successor is unrepresentable above a cap, even though
/// larger values are constructible.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
struct Capped(u8);

impl std::ops::Add for Capped {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Mul for Capped {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

impl num_traits::One for Capped {
    fn one() -> Self {
        Self(1)
    }
}

impl num_traits::CheckedAdd for Capped {
    fn checked_add(&self, rhs: &Self) -> Option<Self> {
        let sum = self.0.checked_add(rhs.0)?;

        (sum <= 10).then_some(Self(sum))
    }
}

/// An index whose addition saturates at a cap: `checked_add` succeeds at
/// the cap but returns a non-advancing successor. It guards against the
/// iterator yielding the cap forever instead of exhausting.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
struct Sat(u8);

impl std::ops::Add for Sat {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self((self.0 + rhs.0).min(10))
    }
}

impl std::ops::Mul for Sat {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self((self.0 * rhs.0).min(10))
    }
}

impl num_traits::One for Sat {
    fn one() -> Self {
        Self(1)
    }
}

impl num_traits::CheckedAdd for Sat {
    fn checked_add(&self, rhs: &Self) -> Option<Self> {
        Some(Self((self.0 + rhs.0).min(10)))
    }
}

#[test]
fn empty_yields_nothing_and_exhausts() {
    let mut range = PartialRange::Empty { idx: 5usize };

    assert_eq!(range.next(), None);
    assert_eq!(range, PartialRange::Exhausted);
}

#[test]
fn exhausted_yields_nothing_and_stays_exhausted() {
    let mut range = PartialRange::<usize>::Exhausted;

    assert_eq!(range.next(), None);
    assert_eq!(range, PartialRange::Exhausted);
}

#[test]
fn inclusive_ends_exhausted_after_its_last_index() {
    let mut range = PartialRange::Inclusive {
        start: 5usize,
        end: 7,
    };

    assert_eq!(range.by_ref().collect::<Vec<_>>(), vec![5, 6, 7]);
    assert_eq!(range, PartialRange::Exhausted);
    assert_eq!(range.next(), None);
}

#[test]
fn inclusive_yields_the_maximum_index() {
    let mut range = PartialRange::Inclusive {
        start: 254u8,
        end: 255,
    };

    assert_eq!(range.next(), Some(254));
    assert_eq!(range.next(), Some(255));
    assert_eq!(range.next(), None);
    assert_eq!(range, PartialRange::Exhausted);
}

#[test]
fn inclusive_with_reversed_bounds_yields_nothing_and_exhausts() {
    let mut range = PartialRange::Inclusive {
        start: 5usize,
        end: 3,
    };

    assert_eq!(range.next(), None);
    assert_eq!(range, PartialRange::Exhausted);
}

#[test]
fn exclusive_ends_exhausted_after_its_last_index() {
    let mut range = PartialRange::Exclusive { inner: 5usize..8 };

    assert_eq!(range.by_ref().collect::<Vec<_>>(), vec![5, 6, 7]);
    assert_eq!(range, PartialRange::Exhausted);
    assert_eq!(range.next(), None);
}

#[test]
fn exclusive_with_empty_bounds_yields_nothing_and_exhausts() {
    let mut range = PartialRange::Exclusive { inner: 5usize..5 };

    assert_eq!(range.next(), None);
    assert_eq!(range, PartialRange::Exhausted);
}

#[test]
fn from_yields_the_maximum_index_and_exhausts() {
    let mut range = PartialRange::From { inner: 254u8.. };

    assert_eq!(range.next(), Some(254));
    assert_eq!(range.next(), Some(255));
    assert_eq!(range.next(), None);
    assert_eq!(range, PartialRange::Exhausted);
}

#[test]
fn full_domain_iteration_covers_every_index() {
    let indices = PartialRange::from(0u8..).collect::<Vec<_>>();

    assert_eq!(indices.len(), 256);
    assert_eq!(indices.first(), Some(&0));
    assert_eq!(indices.last(), Some(&255));

    let indices = Range::inclusive(0u8, 255).into_iter().collect::<Vec<_>>();

    assert_eq!(indices.len(), 256);
    assert_eq!(indices.last(), Some(&255));
}

#[test]
fn from_with_a_saturating_successor_exhausts_at_the_cap() {
    let mut range = PartialRange::From { inner: Sat(9).. };

    assert_eq!(range.next(), Some(Sat(9)));
    assert_eq!(range.next(), Some(Sat(10)));
    assert_eq!(range.next(), None);
    assert_eq!(range, PartialRange::Exhausted);
}

#[test]
fn inclusive_with_a_saturating_successor_exhausts_at_the_cap() {
    let mut range = PartialRange::Inclusive {
        start: Sat(10),
        end: Sat(10),
    };

    assert_eq!(range.next(), Some(Sat(10)));
    assert_eq!(range.next(), None);
    assert_eq!(range, PartialRange::Exhausted);
}

#[test]
fn exclusive_with_a_saturating_successor_exhausts_at_the_cap() {
    let mut range = PartialRange::Exclusive {
        inner: Sat(9)..Sat(15),
    };

    assert_eq!(range.next(), Some(Sat(9)));
    assert_eq!(range.next(), Some(Sat(10)));
    assert_eq!(range.next(), None);
    assert_eq!(range, PartialRange::Exhausted);
}

#[test]
fn exclusive_with_an_unreachable_end_exhausts_at_the_cap() {
    let mut range = PartialRange::Exclusive {
        inner: Capped(9)..Capped(15),
    };

    assert_eq!(range.next(), Some(Capped(9)));
    assert_eq!(range.next(), Some(Capped(10)));
    assert_eq!(range.next(), None);
    assert_eq!(range, PartialRange::Exhausted);
}

#[test]
fn inclusive_with_incomparable_bounds_yields_nothing_and_exhausts() {
    let mut range = PartialRange::Inclusive {
        start: Branchy(0, 5),
        end: Branchy(1, 7),
    };

    assert_eq!(range.size_hint(), (0, Some(0)));
    assert_eq!(range.next(), None);
    assert_eq!(range, PartialRange::Exhausted);
}

#[test]
fn exclusive_with_incomparable_bounds_yields_nothing_and_exhausts() {
    let mut range = PartialRange::Exclusive {
        inner: Branchy(0, 5)..Branchy(1, 7),
    };

    assert_eq!(range.next(), None);
    assert_eq!(range, PartialRange::Exhausted);
}

#[test]
fn size_hint_is_exact_for_empty_ranges() {
    assert_eq!(
        PartialRange::Empty { idx: 5usize }.size_hint(),
        (0, Some(0))
    );
    assert_eq!(PartialRange::<usize>::Exhausted.size_hint(), (0, Some(0)));
    assert_eq!(
        PartialRange::Inclusive {
            start: 5usize,
            end: 3
        }
        .size_hint(),
        (0, Some(0))
    );
    assert_eq!(
        PartialRange::Exclusive { inner: 5usize..5 }.size_hint(),
        (0, Some(0))
    );
}

#[test]
fn size_hint_bounds_non_empty_ranges_below() {
    assert_eq!(
        PartialRange::From { inner: 5usize.. }.size_hint(),
        (1, None)
    );
    assert_eq!(
        PartialRange::Inclusive {
            start: 5usize,
            end: 7
        }
        .size_hint(),
        (1, None)
    );
    assert_eq!(
        PartialRange::Exclusive { inner: 5usize..8 }.size_hint(),
        (1, None)
    );
}

#[test]
fn iterator_is_fused() {
    fn assert_fused<I: std::iter::FusedIterator>(_: &I) {}

    let mut range = PartialRange::Exclusive { inner: 5usize..7 };

    assert_fused(&range);

    range.by_ref().for_each(drop);

    assert_eq!(range.next(), None);
    assert_eq!(range.next(), None);
}

#[test]
fn std_ranges_convert_into_partial_ranges() {
    assert_eq!(
        PartialRange::from(5usize..8),
        PartialRange::Exclusive { inner: 5..8 }
    );
    assert_eq!(
        PartialRange::from(5usize..),
        PartialRange::From { inner: 5.. }
    );
    assert_eq!(
        PartialRange::from(5usize..=8),
        PartialRange::Inclusive { start: 5, end: 8 }
    );
}

#[test]
fn reversed_inclusive_ranges_convert_to_the_empty_variant() {
    #![allow(clippy::reversed_empty_ranges)]
    assert_eq!(
        PartialRange::from(5usize..=3),
        PartialRange::Empty { idx: 5 }
    );
}

#[test]
fn drained_inclusive_ranges_convert_to_the_empty_variant() {
    let mut source = 5u8..=7;

    source.by_ref().for_each(drop);

    assert!(matches!(
        PartialRange::from(source),
        PartialRange::Empty { .. }
    ));
}

#[test]
fn concrete_ranges_convert_by_their_bound_kind() {
    assert_eq!(
        PartialRange::from(Range::inclusive(5usize, 8)),
        PartialRange::Inclusive { start: 5, end: 8 }
    );
    assert_eq!(
        PartialRange::from(Range::exclusive(5usize, 8)),
        PartialRange::Exclusive { inner: 5..8 }
    );
}

#[test]
fn reversed_concrete_inclusive_ranges_behave_empty() {
    let mut range = PartialRange::from(Range::inclusive(5usize, 3));

    assert_eq!(range.next(), None);
    assert_eq!(range, PartialRange::Exhausted);
}
