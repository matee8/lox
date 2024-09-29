#[derive(Debug, Clone)]
pub struct Scanner<'src> {
    source: &'src str,
    start: usize,
    current: usize,
    line: i32,
}

impl<'src> Scanner<'src> {
    #[must_use]
    #[inline]
    pub const fn new(source: &'src str) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }
}
