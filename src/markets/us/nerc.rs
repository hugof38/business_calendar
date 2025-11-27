use chrono::Weekday;

use crate::rules::{HolidayRule, YearRange};
use crate::static_calendar::StaticCalendar;

// NERC: power market, slightly lighter set.
static US_NERC_RULES: &[HolidayRule] = &[
    HolidayRule::WeekendAdjustedFixed {
        month: 1,
        day: 1,
        years: YearRange::always(),
    },
    HolidayRule::NthWeekday {
        month: 5,
        weekday: Weekday::Mon,
        nth: -1,
        years: YearRange::from(1971),
    },
    HolidayRule::WeekendAdjustedFixed {
        month: 7,
        day: 4,
        years: YearRange::always(),
    },
    HolidayRule::NthWeekday {
        month: 9,
        weekday: Weekday::Mon,
        nth: 1,
        years: YearRange::always(),
    },
    HolidayRule::NthWeekday {
        month: 11,
        weekday: Weekday::Thu,
        nth: 4,
        years: YearRange::always(),
    },
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
