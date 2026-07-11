use ps_range::{Range, RangeExt};

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
