use std::ops::{Range, RangeFrom, RangeTo};
use uri::RangeUsize;

const RANGE: RangeUsize = RangeUsize { start: 2, end: 4 };
const OTHER_RANGE: RangeUsize = RangeUsize { start: 4, end: 2 };

#[test]
fn compare_with_rangeusize() {
    let expected = RangeUsize::new(2, 4);
    assert_eq!(RANGE, expected);
    assert_ne!(OTHER_RANGE, expected);
}

#[test]
fn compare_with_range() {
    let expected = Range {
        start: 2usize,
        end: 4usize,
    };
    assert_eq!(RANGE.range(), expected);
    assert_ne!(OTHER_RANGE.range(), expected);
}

#[test]
fn compare_with_rangeto() {
    let expected = RangeTo { end: 4usize };
    assert_eq!(RANGE.range_to(), expected);
    assert_ne!(OTHER_RANGE.range_to(), expected);
}

#[test]
fn compare_with_rangefrom() {
    let expected = RangeFrom { start: 2usize };
    assert_eq!(RANGE.range_from(), expected);
    assert_ne!(OTHER_RANGE.range_from(), expected);
}
