use ps_range::{Range, RangeEnd, RangeStart};

#[test]
fn range_end_orders_by_index_before_variant() {
    assert!(RangeEnd::Exclusive(0) < RangeEnd::Inclusive(100));
    assert!(RangeEnd::Inclusive(4) < RangeEnd::Exclusive(5));
}

#[test]
fn range_end_exclusive_precedes_inclusive_on_equal_indices() {
    assert!(RangeEnd::Exclusive(5) < RangeEnd::Inclusive(5));
    assert!(RangeEnd::Inclusive(5) > RangeEnd::Exclusive(5));
}

#[test]
fn range_end_equal_boundaries_compare_unequal() {
    assert!(RangeEnd::Inclusive(4) < RangeEnd::Exclusive(5));
    assert_ne!(RangeEnd::Inclusive(4), RangeEnd::Exclusive(5));
}

#[test]
fn range_end_partial_cmp_agrees_with_cmp() {
    let ends = [
        RangeEnd::Exclusive(0),
        RangeEnd::Inclusive(0),
        RangeEnd::Exclusive(5),
        RangeEnd::Inclusive(5),
    ];

    for lhs in &ends {
        for rhs in &ends {
            assert_eq!(lhs.partial_cmp(rhs), Some(lhs.cmp(rhs)));
        }
    }
}

#[test]
fn constructors_store_their_bounds() {
    assert_eq!(
        Range::new(2, RangeEnd::Inclusive(7)),
        Range::inclusive(2, 7)
    );
    assert_eq!(
        Range::new(2, RangeEnd::Exclusive(7)),
        Range::exclusive(2, 7)
    );
    assert_eq!(Range::inclusive(2, 7).start(), 2);
}

#[test]
fn range_orders_by_start_then_end() {
    let shorter = Range::exclusive(3, 10);
    let longer = Range::inclusive(3, 10);
    let later = Range::exclusive(4, 5);

    assert!(shorter < longer);
    assert!(longer < later);
}

#[test]
fn std_ranges_convert_by_their_bound_kind() {
    assert_eq!(Range::from(2..7), Range::exclusive(2, 7));
    assert_eq!(Range::from(2..=7), Range::inclusive(2, 7));
}

#[test]
fn reversed_inclusive_ranges_canonicalize_to_an_empty_range_at_the_start() {
    #![allow(clippy::reversed_empty_ranges)]
    assert_eq!(Range::from(5..=3), Range::exclusive(5, 5));
}

#[test]
fn drained_inclusive_ranges_convert_to_an_empty_range() {
    let mut source = 5u8..=7;

    source.by_ref().for_each(drop);

    let converted = Range::from(source);

    assert!(matches!(
        converted,
        Range {
            start,
            end: RangeEnd::Exclusive(end),
        } if start == end
    ));
}
