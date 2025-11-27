use chrono::Weekday;

use crate::rules::{HolidayRule, YearRange};
use crate::static_calendar::StaticCalendar;

// Basic French settlement calendar (public holidays list).
// Nationwide public holidays:
// - New Year's Day, January 1st
// - Easter Monday
// - Labour Day, May 1st
// - Armistice 1945, May 8th
// - Ascension, May 10th (movable feast; approximated here as a fixed date)
// - Pentecôte, May 21st (movable feast; approximated here as a fixed date)
// - Fête nationale, July 14th
// - Assumption, August 15th
// - All Saints' Day, November 1st
// - Armistice 1918, November 11th
// - Christmas Day, December 25th
static FR_SETTLEMENT_RULES: &[HolidayRule] = &[
    // New Year's Day
    HolidayRule::Fixed {
        month: 1,
        day: 1,
        years: YearRange::always(),
    },
    // Easter Monday (Western Easter Monday: offset 0)
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
    // Armistice 1945, May 8th
    HolidayRule::Fixed {
        month: 5,
        day: 8,
        years: YearRange::always(),
    },
    // Ascension, May 10th (simplified as fixed-date holiday)
    HolidayRule::Fixed {
        month: 5,
        day: 10,
        years: YearRange::always(),
    },
    // Pentecôte, May 21st (simplified as fixed-date holiday)
    HolidayRule::Fixed {
        month: 5,
        day: 21,
        years: YearRange::always(),
    },
    // Fête nationale, Bastille Day
    HolidayRule::Fixed {
        month: 7,
        day: 14,
        years: YearRange::always(),
    },
    // Assumption Day
    HolidayRule::Fixed {
        month: 8,
        day: 15,
        years: YearRange::always(),
    },
    // All Saints' Day, November 1st
    HolidayRule::Fixed {
        month: 11,
        day: 1,
        years: YearRange::always(),
    },
    // Armistice 1918, November 11th
    HolidayRule::Fixed {
        month: 11,
        day: 11,
        years: YearRange::always(),
    },
    // Christmas Day
    HolidayRule::Fixed {
        month: 12,
        day: 25,
        years: YearRange::always(),
    },
];

pub static FR_SETTLEMENT: StaticCalendar = StaticCalendar {
    name: "France Settlement",
    weekend: [Weekday::Sat, Weekday::Sun],
    rules: FR_SETTLEMENT_RULES,
};
