use chrono::Weekday;

use crate::date::Date;

/// Simple calendar trait – can be extended with adjust/advance later.
pub trait Calendar {
    /// Returns `true` if the given date is a business day in this calendar.
    fn is_business_day(&self, date: Date) -> bool;

    /// Returns `true` if the given date falls on a weekend for this calendar.
    #[inline]
    fn is_weekend(&self, date: Date) -> bool {
        matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
    }

    /// Returns `true` if the given date is a holiday (non-weekend non-business-day).
    #[inline]
    fn is_holiday(&self, date: Date) -> bool {
        !self.is_business_day(date) && !self.is_weekend(date)
    }
}
