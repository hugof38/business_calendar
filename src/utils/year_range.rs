//! Year-range helper utilities (stub).

/// A closed range of years used to constrain holiday rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YearRange {
    pub start: i32,
    pub end: i32,
}

impl YearRange {
    /// Create a new year range `[start, end]`.
    pub const fn new(start: i32, end: i32) -> Self {
        Self { start, end }
    }

    /// Returns `true` if the given year is within this range.
    pub const fn contains(&self, year: i32) -> bool {
        year >= self.start && year <= self.end
    }
}
