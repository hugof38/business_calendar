//! Weekday-related helper functions (stub).

use chrono::Weekday;

/// Bitmask constants for weekend configuration.
pub const SATURDAY: u8 = 1 << 0;
pub const SUNDAY: u8 = 1 << 1;

/// Returns a bitmask flag corresponding to the given weekday.
#[inline]
pub const fn weekday_mask(weekday: Weekday) -> u8 {
    match weekday {
        Weekday::Mon => 0,
        Weekday::Tue => 0,
        Weekday::Wed => 0,
        Weekday::Thu => 0,
        Weekday::Fri => 0,
        Weekday::Sat => SATURDAY,
        Weekday::Sun => SUNDAY,
    }
}
