use chrono::Weekday;

use crate::rules::{HolidayRule, YearRange};
use crate::static_calendar::StaticCalendar;

// NERC: power market, slightly lighter set.
//
// Holidays covered:
// - 01-01                  : New Year's Day (weekend adjusted)
// - May last Mon from 1971 : Memorial Day
// - 07-04                  : Independence Day (weekend adjusted)
// - Sep 1st Mon            : Labor Day
// - Nov 4th Thu            : Thanksgiving Day
// - 12-25                  : Christmas Day (weekend adjusted)
static US_NERC_RULES: &[HolidayRule] = &[
    // New Year's Day
    HolidayRule::WeekendAdjustedFixed {
        month: 1,
        day: 1,
        years: YearRange::always(),
    },
    // Memorial Day (last Monday in May, from 1971)
    HolidayRule::NthWeekday {
        month: 5,
        weekday: Weekday::Mon,
        nth: -1,
        years: YearRange::from(1971),
    },
    // Independence Day (weekend adjusted)
    HolidayRule::WeekendAdjustedFixed {
        month: 7,
        day: 4,
        years: YearRange::always(),
    },
    // Labor Day (1st Monday in September)
    HolidayRule::NthWeekday {
        month: 9,
        weekday: Weekday::Mon,
        nth: 1,
        years: YearRange::always(),
    },
    // Thanksgiving Day (4th Thursday in November)
    HolidayRule::NthWeekday {
        month: 11,
        weekday: Weekday::Thu,
        nth: 4,
        years: YearRange::always(),
    },
    // Christmas Day (weekend adjusted)
    HolidayRule::WeekendAdjustedFixed {
        month: 12,
        day: 25,
        years: YearRange::always(),
    },
];

pub static US_NERC: StaticCalendar = StaticCalendar {
    name: "US NERC",
    weekend: [Weekday::Sat, Weekday::Sun],
    rules: US_NERC_RULES,
};
