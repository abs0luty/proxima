#[derive(Clone, Copy)]
pub struct CharLocation {
    line: usize,
    column: usize,
    offset: usize,
}

impl CharLocation {
    #[inline]
    #[must_use]
    pub const fn new(line: usize, column: usize, offset: usize) -> Self {
        Self {
            line,
            column,
            offset,
        }
    }

    #[inline]
    #[must_use]
    pub const fn line(&self) -> usize {
        self.line
    }

    #[inline]
    #[must_use]
    pub const fn column(&self) -> usize {
        self.column
    }

    #[inline]
    #[must_use]
    pub const fn offset(&self) -> usize {
        self.offset
    }
}

#[derive(Clone, Copy)]
pub struct SpanLocation {
    start: CharLocation,
    end: CharLocation,
}

impl SpanLocation {
    #[inline]
    #[must_use]
    pub const fn new(start: CharLocation, end: CharLocation) -> Self {
        Self { start, end }
    }

    #[inline]
    #[must_use]
    pub const fn start(&self) -> CharLocation {
        self.start
    }

    #[inline]
    #[must_use]
    pub const fn end(&self) -> CharLocation {
        self.end
    }
}
