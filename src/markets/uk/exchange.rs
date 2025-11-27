use chrono::Weekday;

use crate::rules::{HolidayRule, YearRange};
use crate::static_calendar::StaticCalendar;

// UK exchange (London Stock Exchange): closer to QuantLib UnitedKingdom::Exchange
// including special bank holidays and moved May bank holidays.
//
// Additional rules vs settlement:
// - Early May Bank Holiday moved to May 8 in 1995 and 2020 (VE Day)
// - Spring Bank Holiday moved / replaced around Jubilees with extra days
//   * 2002: Spring BH moved to Mon 3 Jun, extra Tue 4 Jun
//   * 2012: Spring BH moved to Mon 4 Jun, extra Tue 5 Jun
//   * 2022: Spring BH moved to Thu 2 Jun, extra Fri 3 Jun
// - Royal Wedding bank holiday: Fri 29 Apr 2011
// - Queen's Funeral: Mon 19 Sep 2022
// - King Charles III Coronation BH: Mon 8 May 2023
static UK_EXCHANGE_RULES: &[HolidayRule] = &[
    // New Year's Day (same rule as settlement)
    HolidayRule::WeekendAdjustedFixed {
        month: 1,
        day: 1,
        years: YearRange::always(),
    },
    // Good Friday
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
    // Early May Bank Holiday (1st Monday in May), excluding VE-day years
    HolidayRule::NthWeekday {
        month: 5,
        weekday: Weekday::Mon,
        nth: 1,
        years: YearRange::between(i32::MIN, 1994),
    },
    HolidayRule::NthWeekday {
        month: 5,
        weekday: Weekday::Mon,
        nth: 1,
        years: YearRange::between(1996, 2019),
    },
    HolidayRule::NthWeekday {
        month: 5,
        weekday: Weekday::Mon,
        nth: 1,
        years: YearRange::from(2021),
    },
    // VE Day moves of Early May Bank Holiday to May 8 (1995, 2020)
    HolidayRule::OneOff {
        year: 1995,
        month: 5,
        day: 8,
    },
    HolidayRule::OneOff {
        year: 2020,
        month: 5,
        day: 8,
    },
    // Spring Bank Holiday (last Monday in May), excluding Jubilee special years
    HolidayRule::NthWeekday {
        month: 5,
        weekday: Weekday::Mon,
        nth: -1,
        years: YearRange::between(i32::MIN, 2001),
    },
    HolidayRule::NthWeekday {
        month: 5,
        weekday: Weekday::Mon,
        nth: -1,
        years: YearRange::between(2003, 2011),
    },
    HolidayRule::NthWeekday {
        month: 5,
        weekday: Weekday::Mon,
        nth: -1,
        years: YearRange::between(2013, 2021),
    },
    HolidayRule::NthWeekday {
        month: 5,
        weekday: Weekday::Mon,
        nth: -1,
        years: YearRange::from(2023),
    },
    // 2002 Golden Jubilee: 3-4 June (Mon-Tue)
    HolidayRule::OneOff {
        year: 2002,
        month: 6,
        day: 3,
    },
    HolidayRule::OneOff {
        year: 2002,
        month: 6,
        day: 4,
    },
    // 2012 Diamond Jubilee: 4-5 June (Mon-Tue)
    HolidayRule::OneOff {
        year: 2012,
        month: 6,
        day: 4,
    },
    HolidayRule::OneOff {
        year: 2012,
        month: 6,
        day: 5,
    },
    // 2022 Platinum Jubilee: 2-3 June (Thu-Fri)
    HolidayRule::OneOff {
        year: 2022,
        month: 6,
        day: 2,
    },
    HolidayRule::OneOff {
        year: 2022,
        month: 6,
        day: 3,
    },
    // Summer Bank Holiday (last Monday in August)
    HolidayRule::NthWeekday {
        month: 8,
        weekday: Weekday::Mon,
        nth: -1,
        years: YearRange::always(),
    },
    // Royal Wedding, 29 April 2011
    HolidayRule::OneOff {
        year: 2011,
        month: 4,
        day: 29,
    },
    // Queen's Funeral, 19 September 2022
    HolidayRule::OneOff {
        year: 2022,
        month: 9,
        day: 19,
    },
    // Coronation Bank Holiday, 8 May 2023
    HolidayRule::OneOff {
        year: 2023,
        month: 5,
        day: 8,
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
    // December 31st, 1999 only
    HolidayRule::OneOff {
        year: 1999,
        month: 12,
        day: 31,
    },
];

pub static UK_EXCHANGE: StaticCalendar = StaticCalendar {
    name: "UK Exchange",
    weekend: [Weekday::Sat, Weekday::Sun],
    rules: UK_EXCHANGE_RULES,
};
