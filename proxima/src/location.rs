#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
    pub const fn of_first_byte() -> Self {
        Self::new(1, 0, 0)
    }

    #[inline]
    #[must_use]
    pub const fn line(&self) -> usize {
        self.line
    }

    #[inline]
    pub fn set_line(&mut self, line: usize) {
        self.line = line;
    }

    #[inline]
    #[must_use]
    pub const fn column(&self) -> usize {
        self.column
    }

    #[inline]
    pub fn set_column(&mut self, column: usize) {
        self.column = column;
    }

    #[inline]
    #[must_use]
    pub const fn offset(&self) -> usize {
        self.offset
    }

    #[inline]
    pub fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }

    #[inline]
    #[must_use]
    pub const fn next_byte_location(&self) -> Self {
        Self::new(self.line, self.column + 1, self.offset + 1)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Location {
    start: CharLocation,
    end: CharLocation,
}

impl Location {
    #[inline]
    #[must_use]
    pub const fn new(start: CharLocation, end: CharLocation) -> Self {
        Self { start, end }
    }

    #[inline]
    #[must_use]
    pub const fn of_first_byte() -> Self {
        Self::new(
            CharLocation::of_first_byte(),
            CharLocation::of_first_byte().next_byte_location(),
        )
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

pub trait HasLocation {
    #[must_use]
    fn location(&self) -> Location;
}
