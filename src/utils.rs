use std::cmp::{max, min};
use std::convert::From;
use std::ops::{Add, Index, Range, RangeTo};

use percent_encoding::percent_decode_str;

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct RangeUsize {
    pub(crate) start: usize,
    pub(crate) end: usize,
}

impl RangeUsize {
    pub(crate) fn new(start: usize, end: usize) -> Self {
        RangeUsize { start, end }
    }

    pub(crate) fn range(&self) -> Range<usize> {
        Range {
            start: self.start,
            end: self.end,
        }
    }

    pub(crate) fn range_to(&self) -> RangeTo<usize> {
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

// impl Index<RangeTo<RangeUsize>> for String {
//     type Output = str;

//     fn index(&self, index: RangeTo<RangeUsize>) -> &str {
//         &self[index.start..index.end]
//     }
// }

// pub(crate) fn set_start(start: usize, new_start: Option<usize>, shift: usize) -> usize {
//     if let Some(new_start) = new_start {
//         new_start + shift
//     } else {
//         start
//     }
// }

// pub(crate) fn set_end(end: usize, new_end: Option<usize>, shift: usize) -> usize {
//     if let Some(new_end) = new_end {
//         new_end - shift
//     } else {
//         end
//     }
// }

pub(crate) fn decode(v: &str) -> Option<String> {
    percent_decode_str(v)
        .decode_utf8()
        .map_or(None, |op| Some(op.to_string()))
}
