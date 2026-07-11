use ps_range::{PartialRange, PartialRangeExt, Range, RangeEnd, RangeExt};

#[test]
fn end_accessors_report_both_bound_forms() {
    assert_eq!((2usize..8).end_inclusive(), 7);
    assert_eq!((2usize..8).end_exclusive(), Some(8));
    assert_eq!((2usize..=8).end_inclusive(), 8);
    assert_eq!((2usize..=8).end_exclusive(), Some(9));
    assert_eq!((..8usize).end_exclusive(), Some(8));
    assert_eq!((..=8usize).end_exclusive(), Some(9));
}

#[test]
fn end_exclusive_is_none_only_at_the_maximum_inclusive_end() {
    assert_eq!((0..=u8::MAX).end_exclusive(), None);
    assert_eq!((..=u8::MAX).end_exclusive(), None);
    assert_eq!(Range::inclusive(0, u8::MAX).end_exclusive(), None);
    assert_eq!((0..=u8::MAX - 1).end_exclusive(), Some(u8::MAX));
}

#[test]
fn concrete_range_accessors_follow_the_bound_kind() {
    assert_eq!(Range::inclusive(2usize, 8).end_inclusive(), 8);
    assert_eq!(Range::inclusive(2usize, 8).end_exclusive(), Some(9));
    assert_eq!(Range::exclusive(2usize, 8).end_inclusive(), 7);
    assert_eq!(Range::exclusive(2usize, 8).end_exclusive(), Some(8));
}

#[test]
fn clamp_to_binds_both_ends() {
    assert_eq!((5usize..10).clamp_to(0usize, 100usize), 5..10);
    assert_eq!((5usize..10).clamp_to(6usize, 8usize), 6..8);
    assert_eq!((5usize..=10).clamp_to(6usize, 8usize), 6..8);
    assert_eq!(Range::inclusive(5usize, 10).clamp_to(6usize, 8usize), 6..8);
}

#[test]
fn clamp_to_with_a_disjoint_window_is_empty() {
    let clamped = (5usize..10).clamp_to(20usize, 25usize);

    assert!(clamped.is_empty());

    let clamped = (5usize..10).clamp_to(0usize, 3usize);

    assert!(clamped.is_empty());
}

#[test]
fn clamp_to_of_a_maximal_inclusive_range_binds_at_the_window() {
    assert_eq!((0..=u8::MAX).clamp_to(10, 200), 10..200);
    assert_eq!(Range::inclusive(0, u8::MAX).clamp_to(10, 200), 10..200);
}

#[test]
fn clamp_right_lowers_only_the_upper_bound() {
    assert_eq!((..8).clamp_right(6usize), 0..6);
    assert_eq!((2usize..8).clamp_right(6usize), 2..6);
    assert_eq!((2usize..8).clamp_right(100usize), 2..8);
    assert_eq!((..=u8::MAX).clamp_right(200), 0..200);
}

#[test]
fn clamp_right_below_the_start_is_empty() {
    let clamped = (5usize..10).clamp_right(3usize);

    assert!(clamped.is_empty());
}

#[test]
fn drained_inclusive_ranges_do_not_resurrect_through_range_ext() {
    let mut source = 5u8..=7;

    source.by_ref().for_each(drop);

    assert!(source.clamp_to(0u8, 100u8).is_empty());
    assert!(source.clamp_right(100u8).is_empty());
}

#[test]
fn references_forward_range_ext() {
    assert_eq!((2usize..8).end_exclusive(), Some(8));
    assert_eq!((&mut (2usize..8)).clamp_right(6usize), 2..6);
}

#[test]
fn end_bound_reports_the_stored_bound_losslessly() {
    assert_eq!(
        PartialRangeExt::end_bound(&(2usize..8)),
        Some(RangeEnd::Exclusive(8))
    );
    assert_eq!(
        PartialRangeExt::end_bound(&(2usize..=8)),
        Some(RangeEnd::Inclusive(8))
    );
    assert_eq!(
        PartialRangeExt::end_bound(&(0..=usize::MAX)),
        Some(RangeEnd::Inclusive(usize::MAX))
    );
    assert_eq!(PartialRangeExt::end_bound(&(2usize..)), None);
    assert_eq!(PartialRangeExt::<usize>::end_bound(&(..)), None);
}

#[test]
fn emptiness_follows_the_bound_kind() {
    #![allow(clippy::reversed_empty_ranges)]
    assert!(PartialRangeExt::is_empty(&(0usize..0)));
    assert!(PartialRangeExt::is_empty(&(5usize..=3)));
    assert!(PartialRangeExt::is_empty(&(..-5i32)));
    assert!(!PartialRangeExt::is_empty(&(0..=usize::MAX)));
    assert!(!PartialRangeExt::is_empty(&(5usize..)));
    assert!(!PartialRangeExt::is_empty(&(0usize..=0)));
}

#[test]
fn clamp_exclusive_binds_both_ends() {
    assert_eq!((5usize..10).clamp_exclusive(0usize, 100usize), 5..10);
    assert_eq!((5usize..10).clamp_exclusive(6usize, 8usize), 6..8);
    assert_eq!((5usize..10).clamp_exclusive(0usize, 7usize), 5..7);
    assert_eq!((5usize..=10).clamp_exclusive(0usize, 7usize), 5..7);
    assert_eq!((5usize..).clamp_exclusive(0usize, 8usize), 5..8);
    assert_eq!(
        PartialRangeExt::<usize>::clamp_exclusive(&(..), 5usize, 8usize),
        5..8
    );
    assert_eq!(
        Range::inclusive(5usize, 10).clamp_exclusive(6usize, 8usize),
        6..8
    );
}

#[test]
fn clamp_exclusive_converts_the_inclusive_end_only_below_the_window() {
    assert_eq!((5usize..=6).clamp_exclusive(0usize, 100usize), 5..7);
    assert_eq!((0..=u8::MAX).clamp_exclusive(0u8, u8::MAX), 0..u8::MAX);
    assert_eq!(
        Range::inclusive(0, u8::MAX).clamp_exclusive(10u8, 200u8),
        10..200
    );
}

#[test]
fn clamp_exclusive_with_a_zero_width_window_is_empty() {
    assert_eq!((0usize..5).clamp_exclusive(0usize, 0usize), 0..0);
}

#[test]
fn clamp_exclusive_with_a_disjoint_window_is_empty() {
    assert!((5usize..10).clamp_exclusive(20usize, 25usize).is_empty());
    assert!((5usize..10).clamp_exclusive(0usize, 3usize).is_empty());
    assert!((5usize..5).clamp_exclusive(0usize, 100usize).is_empty());
}

#[test]
fn clamp_inclusive_binds_both_ends() {
    assert_eq!(
        (5usize..=10).clamp_inclusive(0usize, 7usize),
        Range::inclusive(5, 7)
    );
    assert_eq!(
        (5usize..).clamp_inclusive(0usize, 7usize),
        Range::inclusive(5, 7)
    );
    assert_eq!(
        (5usize..10).clamp_inclusive(0usize, 100usize),
        Range::exclusive(5, 10)
    );
    assert_eq!(
        (5usize..10).clamp_inclusive(0usize, 7usize),
        Range::inclusive(5, 7)
    );
}

#[test]
fn clamp_inclusive_represents_a_maximal_end_faithfully() {
    assert_eq!(
        (5usize..).clamp_inclusive(0usize, usize::MAX),
        Range::inclusive(5, usize::MAX)
    );
    assert_eq!(
        (0..=u8::MAX).clamp_inclusive(10u8, u8::MAX),
        Range::inclusive(10, u8::MAX)
    );
}

#[test]
fn clamp_inclusive_with_a_disjoint_window_is_empty() {
    assert!((5usize..=10).clamp_inclusive(20usize, 25usize).is_empty());
    assert!((5usize..=10).clamp_inclusive(0usize, 3usize).is_empty());
    assert!((5usize..5).clamp_inclusive(0usize, 100usize).is_empty());
}

#[test]
fn right_clamps_delegate_to_the_windowed_clamps() {
    assert_eq!((5usize..10).clamp_right_exclusive(8usize), 5..8);
    assert_eq!((5usize..).clamp_right_exclusive(8usize), 5..8);
    assert_eq!(
        (5usize..10).clamp_right_inclusive(7usize),
        Range::inclusive(5, 7)
    );
    assert_eq!(
        (5usize..).clamp_right_inclusive(usize::MAX),
        Range::inclusive(5, usize::MAX)
    );
    assert!((5usize..10).clamp_right_exclusive(3usize).is_empty());
    assert!((5usize..10).clamp_right_inclusive(3usize).is_empty());
}

#[test]
fn clamp_left_preserves_the_bound_representation() {
    assert_eq!(
        (5usize..10).clamp_left(7usize),
        PartialRange::Exclusive { inner: 7..10 }
    );
    assert_eq!(
        (5usize..=10).clamp_left(7usize),
        PartialRange::Inclusive { start: 7, end: 10 }
    );
    assert_eq!(
        (5usize..).clamp_left(7usize),
        PartialRange::From { inner: 7.. }
    );
    assert_eq!(
        (5usize..10).clamp_left(3usize),
        PartialRange::Exclusive { inner: 5..10 }
    );
}

#[test]
fn clamp_left_anchors_empty_results_at_the_raised_start() {
    assert_eq!(
        (5usize..10).clamp_left(20usize),
        PartialRange::Empty { idx: 20 }
    );
    assert_eq!(
        (5usize..5).clamp_left(3usize),
        PartialRange::Empty { idx: 5 }
    );
}

#[test]
fn clamp_left_of_a_maximal_inclusive_range_stays_bounded() {
    assert_eq!(
        (0..=usize::MAX).clamp_left(5usize),
        PartialRange::Inclusive {
            start: 5,
            end: usize::MAX
        }
    );
}

#[test]
fn intersection_preserves_the_tighter_bound() {
    assert_eq!(
        (5usize..=10).intersection(&(8usize..)),
        PartialRange::Inclusive { start: 8, end: 10 }
    );
    assert_eq!(
        (5usize..10).intersection(&(8usize..)),
        PartialRange::Exclusive { inner: 8..10 }
    );
    assert_eq!(
        (5usize..10).intersection(&(0usize..=8)),
        PartialRange::Inclusive { start: 5, end: 8 }
    );
    assert_eq!(
        (5usize..=8).intersection(&(0usize..10)),
        PartialRange::Inclusive { start: 5, end: 8 }
    );
    assert_eq!(
        (5usize..).intersection(&(8usize..)),
        PartialRange::From { inner: 8.. }
    );
}

#[test]
fn intersection_anchors_empty_results_at_the_joint_start() {
    assert_eq!(
        (5usize..10).intersection(&(20usize..30)),
        PartialRange::Empty { idx: 20 }
    );
    assert_eq!(
        (0usize..0).intersection(&(0usize..10)),
        PartialRange::Empty { idx: 0 }
    );
}

#[test]
fn intersection_of_a_maximal_inclusive_range_stays_bounded() {
    assert_eq!(
        (0..=usize::MAX).intersection(&(5usize..)),
        PartialRange::Inclusive {
            start: 5,
            end: usize::MAX
        }
    );
}

#[test]
fn none_behaves_as_the_empty_range() {
    let none = None::<std::ops::Range<usize>>;

    assert!(PartialRangeExt::is_empty(&none));
    assert!(none.clamp_exclusive(0usize, 100usize).is_empty());
    assert_eq!(none.clamp_left(5usize), PartialRange::Empty { idx: 5 });
    assert_eq!(
        (5usize..10).intersection(&none),
        PartialRange::Empty { idx: 5 }
    );
}

#[test]
fn some_delegates_to_the_inner_range() {
    let some = Some(5usize..10);

    assert!(!PartialRangeExt::is_empty(&some));
    assert_eq!(some.clamp_exclusive(6usize, 8usize), 6..8);
    assert_eq!(
        some.clamp_left(7usize),
        PartialRange::Exclusive { inner: 7..10 }
    );
}

#[test]
fn references_forward_partial_range_ext() {
    assert_eq!(
        PartialRangeExt::end_bound(&&(2usize..8)),
        Some(RangeEnd::Exclusive(8))
    );
    assert_eq!((5usize..10).clamp_exclusive(6usize, 8usize), 6..8);
    assert_eq!((&mut (5usize..10)).clamp_exclusive(6usize, 8usize), 6..8);
}

#[test]
fn concrete_ranges_clamp_through_the_partial_trait() {
    assert_eq!(
        Range::inclusive(5usize, 10).clamp_left(7usize),
        PartialRange::Inclusive { start: 7, end: 10 }
    );
    assert_eq!(
        Range::exclusive(5usize, 10).intersection(&(7usize..)),
        PartialRange::Exclusive { inner: 7..10 }
    );
    assert_eq!(
        PartialRange::from(5usize..10).clamp_exclusive(6usize, 8usize),
        6..8
    );
    assert_eq!(
        PartialRange::from(5usize..).clamp_inclusive(0usize, 7usize),
        Range::inclusive(5, 7)
    );
}
