use std::ops::Range;

#[derive(Debug)]
pub struct Span {
    lo: usize,
    hi: usize,
}

impl Span {
    pub fn new(mut lo: usize, mut hi: usize) -> Self {
        if lo > hi {
            std::mem::swap(&mut lo, &mut hi);
        }
        Self { lo, hi }
    }

    pub fn dummy() -> Self {
        Self::new(0, 0)
    }

    pub fn lo(&self) -> usize {
        self.lo
    }

    pub fn hi(&self) -> usize {
        self.hi
    }

    pub fn to(&self, other: &Self) -> Self {
        let lo = self.lo.min(other.lo);
        let hi = self.hi.max(other.hi);

        Self::new(lo, hi)
    }
}

impl From<Range<usize>> for Span {
    fn from(value: Range<usize>) -> Self {
        Self::new(value.start, value.end)
    }
}
