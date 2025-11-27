use chrono::Weekday;

use crate::calendar::Calendar;
use crate::date::Date;
use crate::rules::HolidayRule;

/// Simple static calendar over a set of `HolidayRule`s.
#[derive(Debug)]
pub struct StaticCalendar {
    pub name: &'static str,
    pub weekend: [Weekday; 2],
    pub rules: &'static [HolidayRule],
}

impl StaticCalendar {
    /// Returns `true` if the given date is considered a weekend in this calendar.
    #[inline]
    pub fn is_weekend(&self, date: Date) -> bool {
        let wd = date.weekday();
        wd == self.weekend[0] || wd == self.weekend[1]
    }

    /// Returns `true` if any of this calendar's rules marks `date` as a holiday.
    #[inline]
    pub fn is_holiday(&self, date: Date) -> bool {
        let y = date.year();
        for &rule in self.rules {
            if rule.is_holiday(date, y) {
                return true;
            }
        }
        false
    }

    /// Returns `true` if `date` is a business day (non-weekend, non-holiday).
    #[inline]
    pub fn is_business_day(&self, date: Date) -> bool {
        !self.is_weekend(date) && !self.is_holiday(date)
    }
}

impl Calendar for StaticCalendar {
    #[inline]
    fn is_business_day(&self, date: Date) -> bool {
        self.is_business_day(date)
    }
}
