use chrono::Weekday;

use crate::rules::{HolidayRule, YearRange};
use crate::static_calendar::StaticCalendar;

// Government bond market: close to Settlement + Good Friday & a few tweaks.
pub(crate) static US_GOVERNMENT_BOND_RULES: &[HolidayRule] = &[
    HolidayRule::WeekendAdjustedFixed {
        month: 1,
        day: 1,
        years: YearRange::always(),
    },
    HolidayRule::NthWeekday {
        month: 1,
        weekday: Weekday::Mon,
        nth: 3,
        years: YearRange::from(1983),
    },
    HolidayRule::NthWeekday {
        month: 2,
        weekday: Weekday::Mon,
        nth: 3,
        years: YearRange::from(1971),
    },
    HolidayRule::EasterOffset {
        western: true,
        offset: -3,
        years: YearRange::always(),
    },
    HolidayRule::NthWeekday {
        month: 5,
        weekday: Weekday::Mon,
        nth: -1,
        years: YearRange::from(1971),
    },
    HolidayRule::WeekendAdjustedFixed {
        month: 6,
        day: 19,
        years: YearRange::from(2022),
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
        month: 10,
        weekday: Weekday::Mon,
        nth: 2,
        years: YearRange::always(),
    },
    HolidayRule::WeekendAdjustedFixed {
        month: 11,
        day: 11,
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
    HolidayRule::OneOff {
        year: 2018,
        month: 12,
        day: 5,
    },
    HolidayRule::OneOff {
        year: 2012,
        month: 10,
        day: 30,
    },
    HolidayRule::OneOff {
        year: 2004,
        month: 6,
        day: 11,
    },
];

pub static US_GOVERNMENT_BOND: StaticCalendar = StaticCalendar {
    name: "US Government Bond",
    weekend: [Weekday::Sat, Weekday::Sun],
    rules: US_GOVERNMENT_BOND_RULES,
};
