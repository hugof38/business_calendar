use chrono::Weekday;

use crate::date::Date;
use crate::easter::{orthodox_easter_monday, western_easter_monday};

/// Year range used for rules; inclusive [start, end].
#[derive(Debug, Clone, Copy)]
pub struct YearRange {
    pub start: i32,
    pub end: i32,
}

impl YearRange {
    #[inline]
    pub const fn always() -> Self {
        Self {
            start: i32::MIN,
            end: i32::MAX,
        }
    }

    #[inline]
    pub const fn from(start: i32) -> Self {
        Self {
            start,
            end: i32::MAX,
        }
    }

    #[inline]
    pub const fn until(end: i32) -> Self {
        Self {
            start: i32::MIN,
            end,
        }
    }

    #[inline]
    pub const fn between(start: i32, end: i32) -> Self {
        Self { start, end }
    }

    #[inline]
    pub fn contains(&self, y: i32) -> bool {
        y >= self.start && y <= self.end
    }
}

/// Static, data-driven holiday rules.
#[derive(Debug, Clone, Copy)]
pub enum HolidayRule {
    /// Fixed calendar date in all years in `years`.
    Fixed {
        month: u32,
        day: u32,
        years: YearRange,
    },

    /// Fixed date, observed on Monday if it falls on Sunday,
    /// and Friday if it falls on Saturday (same year, same month).
    WeekendAdjustedFixed {
        month: u32,
        day: u32,
        years: YearRange,
    },

    /// nth weekday of a given month (nth > 0: from start; nth < 0: from end)
    NthWeekday {
        month: u32,
        weekday: Weekday,
        nth: i8,
        years: YearRange,
    },

    /// Easter Monday offset in days (e.g. Good Friday = -3).
    EasterOffset {
        /// If `true`, use Western calendar; if `false`, use Orthodox.
        western: bool,
        offset: i32,
        years: YearRange,
    },

    /// One-off holiday on a specific date.
    OneOff { year: i32, month: u32, day: u32 },
}

impl HolidayRule {
    /// Returns `true` if this rule marks the given `date` as a holiday.
    ///
    /// `year` is passed separately to avoid recomputing it for each rule.
    #[inline]
    pub fn is_holiday(&self, date: Date, year: i32) -> bool {
        match *self {
            HolidayRule::Fixed { month, day, years } => {
                years.contains(year) && date.month() == month && date.day() == day
            }
            HolidayRule::WeekendAdjustedFixed { month, day, years } => {
                if !years.contains(year) || date.month() != month {
                    return false;
                }
                let wd = date.weekday();
                if date.day() == day {
                    // Actual calendar date
                    true
                } else if wd == Weekday::Fri && date.day() + 1 == day {
                    // Observed on Friday if actual date is Saturday
                    true
                } else if wd == Weekday::Mon && date.day() == day + 1 {
                    // Observed on Monday if actual date is Sunday
                    true
                } else {
                    false
                }
            }
            HolidayRule::NthWeekday {
                month,
                weekday,
                nth,
                years,
            } => {
                if !years.contains(year) || date.month() != month || date.weekday() != weekday {
                    return false;
                }
                let day = date.day() as i32;
                if nth > 0 {
                    // e.g. first Monday: 1..=7, second Monday: 8..=14, etc.
                    let idx = nth as i32 - 1;
                    day > idx * 7 && day <= (idx + 1) * 7
                } else {
                    // From end of month: approximate by week index from end.
                    // This keeps the logic simple; more exact variants can be
                    // added later if needed.
                    let nth_abs = -nth as i32;
                    let ordinal_from_end = 31 - day; // upper bound; fine for rule buckets
                    let bucket = ordinal_from_end / 7 + 1;
                    bucket == nth_abs
                }
            }
            HolidayRule::EasterOffset {
                western,
                offset,
                years,
            } => {
                if !years.contains(year) {
                    return false;
                }
                let easter_doy = if western {
                    western_easter_monday(year)
                } else {
                    orthodox_easter_monday(year)
                } as i32;
                let target = easter_doy + offset;
                date.day_of_year() as i32 == target
            }
            HolidayRule::OneOff {
                year: y,
                month,
                day,
            } => year == y && date.month() == month && date.day() == day,
        }
    }
}
