//! Easter-related utilities and lookup tables.

pub mod orthodox;
pub mod western;

pub use orthodox::OrthodoxEasterMondayTable;
pub use western::EasterMondayTable as WesternEasterMondayTable;

/// Day-of-year (1-based) of Western Easter Monday for `year` in 1901–2199.
#[inline]
pub const fn western_easter_monday(year: i32) -> u32 {
    WesternEasterMondayTable::western(year)
}

/// Day-of-year (1-based) of Orthodox Easter Monday for `year` in 1901–2199.
#[inline]
pub const fn orthodox_easter_monday(year: i32) -> u32 {
    OrthodoxEasterMondayTable::orthodox(year)
}
