use chrono::Weekday;

use business_calendar::markets::us::{USMarket, UnitedStates};
use business_calendar::{Calendar, Date};

#[test]
fn nyse_new_years_day_is_holiday() {
    let cal = UnitedStates::new(USMarket::NYSE);
    let d = Date::ymd(2025, 1, 1); // Wednesday, observed holiday
    assert!(!cal.is_business_day(d));
}

#[test]
fn nyse_following_day_is_business() {
    let cal = UnitedStates::new(USMarket::NYSE);
    let d = Date::ymd(2025, 1, 2);
    assert!(cal.is_business_day(d));
}

#[test]
fn settlement_juneteenth_2024_is_holiday() {
    let cal = UnitedStates::new(USMarket::Settlement);
    let d = Date::ymd(2024, 6, 19);
    assert!(!cal.is_business_day(d));
}

#[test]
fn settlement_weekend_is_not_business() {
    let cal = UnitedStates::new(USMarket::Settlement);
    // 2025-01-04 is a Saturday
    let d = Date::ymd(2025, 1, 4);
    assert!(!cal.is_business_day(d));
}

#[test]
fn libor_impact_override_around_july4() {
    let cal = UnitedStates::new(USMarket::LiborImpact);

    // July 3, 2015 was a Friday; around July 4 handling should treat as business day
    let d = Date::ymd(2015, 7, 3);
    assert_eq!(d.weekday(), Weekday::Fri);
    assert!(cal.is_business_day(d));
}
