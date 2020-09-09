use std::cmp::{max, min};
use std::ops::{Add, Index, Range, RangeTo};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RangeUsize {
    pub start: usize,
    pub end: usize,
}

impl RangeUsize {
    pub fn new(start: usize, end: usize) -> Self {
        RangeUsize { start, end }
    }

    pub fn range(&self) -> Range<usize> {
        Range {
            start: self.start,
            end: self.end,
        }
    }

    pub fn range_to(&self) -> RangeTo<usize> {
        RangeTo { end: self.end }
    }

    pub(crate) fn start(&mut self, new_start: usize) {
        self.start = new_start;
    }

    pub(crate) fn shift(&self, shift: usize) -> RangeUsize {
        RangeUsize::new(self.start + shift, self.end + shift)
    }

    pub(crate) fn end(&mut self, new_end: usize) {
        self.end = new_end;
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub(crate) fn len(&self) -> usize {
        self.end - self.start
    }
}

impl Add for RangeUsize {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            start: min(self.start, other.start),
            end: max(self.end, other.end),
        }
    }
}

impl Index<RangeUsize> for str {
    type Output = str;

    fn index(&self, index: RangeUsize) -> &str {
        &self[index.start..index.end]
    }
}

impl Index<&mut RangeUsize> for str {
    type Output = str;

    fn index(&self, index: &mut RangeUsize) -> &str {
        &self[index.start..index.end]
    }
}

impl Index<&&mut RangeUsize> for str {
    type Output = str;

    fn index(&self, index: &&mut RangeUsize) -> &str {
        &self[index.start..index.end]
    }
}

impl Index<RangeUsize> for String {
    type Output = str;

    fn index(&self, index: RangeUsize) -> &str {
        &self[index.start..index.end]
    }
}

impl Index<&RangeUsize> for String {
    type Output = str;

    fn index(&self, index: &RangeUsize) -> &str {
        &self[index.start..index.end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RANGE: RangeUsize  = RangeUsize{start:2,end:4};
    const OTHER_RANGE: RangeUsize  = RangeUsize{start:4,end:2};

    #[test]
    fn compare_with_rangeusize() {
        let expected = RangeUsize::new(2,4);
        assert_eq!(RANGE, expected);
        assert_ne!(OTHER_RANGE, expected);
    }

    #[test]
    fn compare_with_range() {
        let expected = Range{start:2usize, end:4usize};
        assert_eq!(RANGE.range(), expected);
        assert_ne!(OTHER_RANGE.range(), expected);
    }

    #[test]
    fn compare_with_rangeto() {
        let expected = RangeTo{end:4usize};
        assert_eq!(RANGE.range_to(), expected);
        assert_ne!(OTHER_RANGE.range_to(), expected);
    }
}
