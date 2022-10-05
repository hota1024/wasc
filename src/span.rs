#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn merge(&self, other: &Self) -> Self {
        use std::cmp::{max, min};

        let start = min(self.start, other.start);
        let end = max(self.end, other.end);

        Self::new(start, end)
    }
}
