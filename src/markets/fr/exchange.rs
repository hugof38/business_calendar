use chrono::Weekday;

use crate::rules::{HolidayRule, YearRange};
use crate::static_calendar::StaticCalendar;

// Euronext Paris-style exchange calendar (market-holiday list).
// Core holidays:
// - New Year's Day, January 1st
// - Good Friday
// - Easter Monday
// - Labour Day, May 1st
// - Christmas Eve, December 24th
// - Christmas Day, December 25th
// - Boxing Day, December 26th
// - New Year's Eve, December 31st
static FR_EXCHANGE_RULES: &[HolidayRule] = &[
    // New Year's Day, January 1st
    HolidayRule::Fixed {
        month: 1,
        day: 1,
        years: YearRange::always(),
    },
    // Good Friday (3 days before Western Easter Monday)
    HolidayRule::EasterOffset {
        western: true,
        offset: -3,
        years: YearRange::always(),
    },
    // Easter Monday
    HolidayRule::EasterOffset {
        western: true,
        offset: 0,
        years: YearRange::always(),
    },
    // Labour Day, May 1st
    HolidayRule::Fixed {
        month: 5,
        day: 1,
        years: YearRange::always(),
    },
    // Christmas Eve, December 24th
    HolidayRule::Fixed {
        month: 12,
        day: 24,
        years: YearRange::always(),
    },
    // Christmas Day, December 25th
    HolidayRule::Fixed {
        month: 12,
        day: 25,
        years: YearRange::always(),
    },
    // Boxing Day, December 26th
    HolidayRule::Fixed {
        month: 12,
        day: 26,
        years: YearRange::always(),
    },
    // New Year's Eve, December 31st
    HolidayRule::Fixed {
        month: 12,
        day: 31,
        years: YearRange::always(),
    },
];

pub static FR_EXCHANGE: StaticCalendar = StaticCalendar {
    name: "France Exchange",
    weekend: [Weekday::Sat, Weekday::Sun],
    rules: FR_EXCHANGE_RULES,
};
