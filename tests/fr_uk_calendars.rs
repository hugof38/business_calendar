use business_calendar::{Calendar, Date};
use business_calendar::markets::fr::{France, FRMarket};
use business_calendar::markets::uk::{UnitedKingdom, UKMarket};

#[test]
fn france_bastille_day_is_holiday() {
    let fr = France::new(FRMarket::Settlement);
    let bastille = Date::ymd(2025, 7, 14);
    assert!(fr.is_holiday(bastille));
    assert!(!fr.is_business_day(bastille));
}

#[test]
fn france_next_day_after_bastille_is_business_day() {
    let fr = France::new(FRMarket::Settlement);
    let next = Date::ymd(2025, 7, 15);
    assert!(fr.is_business_day(next));
}

#[test]
fn uk_early_may_bank_holiday_is_holiday() {
    let uk = UnitedKingdom::new(UKMarket::Settlement);
    // Early May bank holiday 2025: 1st Monday in May is 2025-05-05
    let may_bank = Date::ymd(2025, 5, 5);
    assert!(uk.is_holiday(may_bank));
    assert!(!uk.is_business_day(may_bank));
}

#[test]
fn uk_random_tuesday_is_business_day() {
    let uk = UnitedKingdom::new(UKMarket::Settlement);
    let date = Date::ymd(2025, 3, 4); // A random Tuesday with no holiday
    assert!(uk.is_business_day(date));
}

#[test]
fn uk_exchange_special_ve_day_holidays() {
    let uk = UnitedKingdom::new(UKMarket::Exchange);

    // Early May bank holiday moved to May 8th for VE Day in 1995 and 2020.
    let ve_1995 = Date::ymd(1995, 5, 8);
    let ve_2020 = Date::ymd(2020, 5, 8);

    assert!(uk.is_holiday(ve_1995));
    assert!(!uk.is_business_day(ve_1995));
    assert!(uk.is_holiday(ve_2020));
    assert!(!uk.is_business_day(ve_2020));
}

#[test]
fn uk_exchange_jubilee_and_royal_event_holidays() {
    let uk = UnitedKingdom::new(UKMarket::Exchange);

    // 2002 Golden Jubilee: 3-4 June
    for day in 3..=4 {
        let d = Date::ymd(2002, 6, day);
        assert!(uk.is_holiday(d));
        assert!(!uk.is_business_day(d));
    }

    // 2012 Diamond Jubilee: 4-5 June
    for day in 4..=5 {
        let d = Date::ymd(2012, 6, day);
        assert!(uk.is_holiday(d));
        assert!(!uk.is_business_day(d));
    }

    // 2022 Platinum Jubilee: 2-3 June
    for day in 2..=3 {
        let d = Date::ymd(2022, 6, day);
        assert!(uk.is_holiday(d));
        assert!(!uk.is_business_day(d));
    }

    // Royal Wedding: 29 April 2011
    let royal_wedding = Date::ymd(2011, 4, 29);
    assert!(uk.is_holiday(royal_wedding));
    assert!(!uk.is_business_day(royal_wedding));

    // Queen's Funeral: 19 September 2022
    let funeral = Date::ymd(2022, 9, 19);
    assert!(uk.is_holiday(funeral));
    assert!(!uk.is_business_day(funeral));

    // King Charles III Coronation Bank Holiday: 8 May 2023
    let coronation = Date::ymd(2023, 5, 8);
    assert!(uk.is_holiday(coronation));
    assert!(!uk.is_business_day(coronation));
}

#[test]
fn uk_metals_matches_exchange_for_holidays() {
    let uk_exchange = UnitedKingdom::new(UKMarket::Exchange);
    let uk_metals = UnitedKingdom::new(UKMarket::Metals);

    // Spot check a few well-known holidays across the two calendars.
    let dates = [
        // VE Day special bank holidays
        Date::ymd(1995, 5, 8),
        Date::ymd(2020, 5, 8),
        // Jubilee events
        Date::ymd(2002, 6, 3),
        Date::ymd(2012, 6, 4),
        Date::ymd(2022, 6, 2),
        // Royal Wedding, Queen's Funeral, Coronation BH
        Date::ymd(2011, 4, 29),
        Date::ymd(2022, 9, 19),
        Date::ymd(2023, 5, 8),
    ];

    for d in dates.iter().copied() {
        assert_eq!(uk_exchange.is_holiday(d), uk_metals.is_holiday(d));
        assert_eq!(uk_exchange.is_business_day(d), uk_metals.is_business_day(d));
    }
}
