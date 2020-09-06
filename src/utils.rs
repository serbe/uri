use std::convert::From;
use std::ops::{Index, Range, RangeTo};

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct RangeUsize {
    pub(crate) start: usize,
    pub(crate) end: usize,
}

impl RangeUsize {
    pub(crate) fn new(start: usize, end: usize) -> Self {
        RangeUsize { start, end }
    }

    pub(crate) fn to(&self) -> RangeTo<usize> {
        RangeTo { end: self.end }
    }

    pub(crate) fn start(&mut self, new_start: usize) {
        self.start = new_start;
    }

    pub(crate) fn end(&mut self, new_end: usize) {
        self.end = new_end;
    }
}

// impl From<RangeUsize> for Range<usize> {
//     fn from(range_usize: RangeUsize) -> Self {
//         Range {
//             start: range_usize.start,
//             end: range_usize.end,
//         }
//     }
// }

// impl From<&RangeUsize> for Range<usize> {
//     fn from(range_usize: &RangeUsize) -> Self {
//         Range {
//             start: range_usize.start,
//             end: range_usize.end,
//         }
//     }
// }

// impl From<RangeUsize> for RangeTo<usize> {
//     fn from(range_usize: RangeUsize) -> Self {
//         RangeTo {
//             end: range_usize.end,
//         }
//     }
// }

// impl From<Range<usize>> for RangeUsize {
//     fn from(range: Range<usize>) -> Self {
//         RangeUsize::new(range.start, range.end)
//     }
// }

impl Index<&mut RangeUsize> for str {
    type Output = str;

    fn index(&self, index: &mut RangeUsize) -> &str {
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

// impl Index<RangeTo<RangeUsize>> for String {
//     type Output = str;

//     fn index(&self, index: RangeTo<RangeUsize>) -> &str {
//         &self[index.start..index.end]
//     }
// }

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
