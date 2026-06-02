use std::ops::Range;

use crate::ir::source::BytePos;

#[derive(Clone, Copy, Debug)]
pub(super) struct Span {
    pub start: BytePos,
    pub end: BytePos,
}

impl Span {
    pub fn new(start: BytePos, end: BytePos) -> Self {
        Self { start, end }
    }

    pub fn dummy() -> Self {
        Self::new(BytePos(0), BytePos(0))
    }

    pub fn to(&self, other: &Self) -> Self {
        assert!(self.end == other.start);
        Self::from(self.start..other.end)
    }
}

impl From<Range<BytePos>> for Span {
    fn from(value: Range<BytePos>) -> Self {
        Self::new(value.start, value.end)
    }
}
