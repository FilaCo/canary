use std::ops::Range;

use crate::ir::source::BytePos;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Span {
    start: BytePos,
    end: BytePos,
}

impl Span {
    pub fn new(mut start: BytePos, mut end: BytePos) -> Self {
        if start > end {
            std::mem::swap(&mut start, &mut end);
        }
        Self { start, end }
    }

    pub fn dummy() -> Self {
        Self::new(BytePos(0), BytePos(0))
    }

    pub fn start(&self) -> BytePos {
        self.start
    }

    pub fn end(&self) -> BytePos {
        self.end
    }

    pub fn to(&self, other: &Self) -> Self {
        let lo = self.start.min(other.start);
        let hi = self.end.max(other.end);

        Self::new(lo, hi)
    }
}

impl From<Range<BytePos>> for Span {
    fn from(value: Range<BytePos>) -> Self {
        Self::new(value.start, value.end)
    }
}
