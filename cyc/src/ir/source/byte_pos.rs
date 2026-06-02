use std::ops::AddAssign;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BytePos(pub usize);

impl From<BytePos> for usize {
    fn from(value: BytePos) -> Self {
        value.0
    }
}

impl AddAssign<BytePos> for BytePos {
    fn add_assign(&mut self, rhs: BytePos) {
        self.0 += rhs.0
    }
}
