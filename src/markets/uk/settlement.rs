use chrono::Weekday;

use crate::rules::{HolidayRule, YearRange};
use crate::static_calendar::StaticCalendar;

// UK settlement calendar: approximate QuantLib UnitedKingdom::Settlement.
// Core holidays (simplified, without all special cases):
// - New Year's Day
// - Good Friday
// - Easter Monday
// - Early May Bank Holiday (1st Mon in May)
// - Spring Bank Holiday (last Mon in May)
// - Summer Bank Holiday (last Mon in August)
// - Christmas Day
// - Boxing Day
static UK_SETTLEMENT_RULES: &[HolidayRule] = &[
    // New Year's Day
    HolidayRule::WeekendAdjustedFixed {
        month: 1,
        day: 1,
        years: YearRange::always(),
    },
    // Good Friday (Western Easter Monday - 3)
    HolidayRule::EasterOffset {
        western: true,
        offset: -3,
        years: YearRange::always(),
    },
    // Easter Monday (Western Easter Monday)
    HolidayRule::EasterOffset {
        western: true,
        offset: 0,
        years: YearRange::always(),
    },
    // Early May Bank Holiday (1st Monday in May)
    HolidayRule::NthWeekday {
        month: 5,
        weekday: Weekday::Mon,
        nth: 1,
        years: YearRange::always(),
    },
    // Spring Bank Holiday (last Monday in May)
    HolidayRule::NthWeekday {
        month: 5,
        weekday: Weekday::Mon,
        nth: -1,
        years: YearRange::always(),
    },
    // Summer Bank Holiday (last Monday in August)
    HolidayRule::NthWeekday {
        month: 8,
        weekday: Weekday::Mon,
        nth: -1,
        years: YearRange::always(),
    },
    // Christmas Day
    HolidayRule::WeekendAdjustedFixed {
        month: 12,
        day: 25,
        years: YearRange::always(),
    },
    // Boxing Day
    HolidayRule::WeekendAdjustedFixed {
        month: 12,
        day: 26,
        years: YearRange::always(),
    },
];

pub static UK_SETTLEMENT: StaticCalendar = StaticCalendar {
    name: "UK Settlement",
    weekend: [Weekday::Sat, Weekday::Sun],
    rules: UK_SETTLEMENT_RULES,
};
