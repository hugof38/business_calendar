use business_calendar::markets::us::{USMarket, UnitedStates};
use business_calendar::{Calendar, Date};

fn main() {
    let cal = UnitedStates::new(USMarket::NYSE);

    let d = Date::ymd(2025, 1, 1); // New Year's Day 2025
    println!(
        "{}-{:02}-{:02} business? {}",
        d.year(),
        d.month(),
        d.day(),
        cal.is_business_day(d)
    );

    let d2 = Date::ymd(2025, 1, 2); // Next trading day
    println!(
        "{}-{:02}-{:02} business? {}",
        d2.year(),
        d2.month(),
        d2.day(),
        cal.is_business_day(d2)
    );
}
