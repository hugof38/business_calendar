use chrono::{Datelike, Days, NaiveDate, Weekday};

/// Lightweight date wrapper so we can swap out backend if needed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Date(pub NaiveDate);

impl Date {
    /// Construct a `Date` from ISO year-month-day components.
    #[inline]
    pub fn ymd(y: i32, m: u32, d: u32) -> Self {
        Date(NaiveDate::from_ymd_opt(y, m, d).expect("invalid date"))
    }

    /// Year component.
    #[inline]
    pub fn year(&self) -> i32 {
        self.0.year()
    }

    /// Month component (1-12).
    #[inline]
    pub fn month(&self) -> u32 {
        self.0.month()
    }

    /// Day-of-month component (1-31).
    #[inline]
    pub fn day(&self) -> u32 {
        self.0.day()
    }

    /// Weekday of this date.
    #[inline]
    pub fn weekday(&self) -> Weekday {
        self.0.weekday()
    }

    /// Day-of-year (1-based ordinal).
    #[inline]
    pub fn day_of_year(&self) -> u32 {
        self.0.ordinal()
    }

    /// Add `n` days (can be negative) and return the resulting date.
    #[inline]
    pub fn add_days(&self, n: i32) -> Self {
        if n >= 0 {
            Date(self.0 + Days::new(n as u64))
        } else {
            Date(self.0 - Days::new((-n) as u64))
        }
    }
}
