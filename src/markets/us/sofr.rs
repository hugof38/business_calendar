use chrono::Weekday;

use crate::rules::HolidayRule;
use crate::static_calendar::StaticCalendar;

use super::gov_bond::US_GOVERNMENT_BOND_RULES;

// SOFR: like GovBond – Good Friday is always a holiday (SOFR doesn't fix on it).
static US_SOFR_RULES: &[HolidayRule] = US_GOVERNMENT_BOND_RULES;

pub static US_SOFR: StaticCalendar = StaticCalendar {
    name: "US SOFR",
    weekend: [Weekday::Sat, Weekday::Sun],
    rules: US_SOFR_RULES,
};
