use std::{
    cmp::{max, min},
    ops::{Add, Index, Range, RangeFrom, RangeTo},
};

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

    pub fn range_from(&self) -> RangeFrom<usize> {
        RangeFrom { start: self.start }
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

    const RANGE: RangeUsize = RangeUsize { start: 2, end: 4 };

    #[test]
    fn range_usize_len() {
        let s = "test string;";
        assert_eq!(&s[RANGE], "st");
        assert_eq!(s[RANGE].len(), RANGE.len());
    }

    #[test]
    fn range_usize_is_empty() {
        assert!(RangeUsize::new(2, 2).is_empty());
        assert!(!RANGE.is_empty());
    }

    #[test]
    fn range_usize_start() {
        let expected = RangeUsize::new(1, 4);
        let mut input = RANGE;
        input.start(1);
        assert_eq!(input, expected);
        let expected = RangeUsize::new(5, 4);
        input.start(5);
        assert_eq!(input, expected);
    }

    #[test]
    fn range_usize_shift() {
        let expected = RangeUsize::new(5, 7);
        assert_eq!(RANGE.shift(3), expected);
    }

    #[test]
    fn range_usize_end() {
        let expected = RangeUsize::new(2, 3);
        let mut input = RANGE;
        input.end(3);
        assert_eq!(input, expected);
    }
}
