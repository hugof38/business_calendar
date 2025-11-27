use chrono::Weekday;

use crate::rules::{HolidayRule, YearRange};
use crate::static_calendar::StaticCalendar;

// Government bond market: close to Settlement + Good Friday & a few tweaks.
//
// Base holidays:
// - 01-01                       : New Year's Day (weekend adjusted)
// - Jan 3rd Mon from 1983       : Martin Luther King Jr. Day
// - Feb 3rd Mon from 1971       : Washington's Birthday / Presidents' Day
// - Western Easter offset -3    : Good Friday
// - May last Mon from 1971      : Memorial Day
// - 06-19 from 2022             : Juneteenth National Independence Day (weekend adjusted)
// - 07-04                       : Independence Day (weekend adjusted)
// - Sep 1st Mon                 : Labor Day
// - Oct 2nd Mon                 : Columbus Day / Indigenous Peoples' Day
// - 11-11                       : Veterans Day (weekend adjusted)
// - Nov 4th Thu                 : Thanksgiving Day
// - 12-25                       : Christmas Day (weekend adjusted)
//
// Additional government-bond-specific one-offs:
// - 2018-12-05                  : Bush funeral / national day of mourning
// - 2012-10-30                  : Hurricane Sandy closure
// - 2004-06-11                  : Reagan funeral
pub(crate) static US_GOVERNMENT_BOND_RULES: &[HolidayRule] = &[
    // New Year's Day
    HolidayRule::WeekendAdjustedFixed {
        month: 1,
        day: 1,
        years: YearRange::always(),
    },
    // Martin Luther King Jr. Day (3rd Monday in January, from 1983)
    HolidayRule::NthWeekday {
        month: 1,
        weekday: Weekday::Mon,
        nth: 3,
        years: YearRange::from(1983),
    },
    // Washington's Birthday / Presidents' Day (3rd Monday in February, from 1971)
    HolidayRule::NthWeekday {
        month: 2,
        weekday: Weekday::Mon,
        nth: 3,
        years: YearRange::from(1971),
    },
    // Good Friday (3 days before Western Easter Monday)
    HolidayRule::EasterOffset {
        western: true,
        offset: -3,
        years: YearRange::always(),
    },
    // Memorial Day (last Monday in May, from 1971)
    HolidayRule::NthWeekday {
        month: 5,
        weekday: Weekday::Mon,
        nth: -1,
        years: YearRange::from(1971),
    },
    // Juneteenth National Independence Day (from 2022, weekend adjusted)
    HolidayRule::WeekendAdjustedFixed {
        month: 6,
        day: 19,
        years: YearRange::from(2022),
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
    // Columbus Day / Indigenous Peoples' Day (2nd Monday in October)
    HolidayRule::NthWeekday {
        month: 10,
        weekday: Weekday::Mon,
        nth: 2,
        years: YearRange::always(),
    },
    // Veterans Day (weekend adjusted)
    HolidayRule::WeekendAdjustedFixed {
        month: 11,
        day: 11,
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
    // 2018-12-05: Bush funeral / national day of mourning
    HolidayRule::OneOff {
        year: 2018,
        month: 12,
        day: 5,
    },
    // 2012-10-30: Hurricane Sandy closure
    HolidayRule::OneOff {
        year: 2012,
        month: 10,
        day: 30,
    },
    // 2004-06-11: Reagan funeral
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
