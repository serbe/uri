use std::convert::From;
use std::ops::{Range, Index};

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct RangeUsize {
    start: usize,
    end: usize,
}

impl RangeUsize {
    pub(crate) fn new(start: usize, end: usize) -> Self{
        RangeUsize{start, end}
    }
}

impl From<RangeUsize> for Range<usize> {
    fn from(range_usize: RangeUsize) -> Self {
        Range{start: range_usize.start, end: range_usize.end}
    }
}

impl From<Range<usize>> for RangeUsize {
    fn from(range: Range<usize>) -> Self {
        RangeUsize::new(range.start, range.end)
    }
}

impl Index<RangeUsize> for String {
    type Output = str;

    fn index(&self, index: RangeUsize) -> &str {
        &self[index.start..index.end]
    }
}

pub(crate) fn set_start(start: usize, new_start: Option<usize>, shift: usize) -> usize {
    if let Some(new_start) = new_start {
        new_start + shift
    } else {
        start
    }
}

pub(crate) fn set_end(end: usize, new_end: Option<usize>, shift: usize) -> usize {
    if let Some(new_end) = new_end {
        new_end - shift
    } else {
        end
    }
}