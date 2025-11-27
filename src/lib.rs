//! `business_calendar` – global financial business-day calendars.
//!
//! This crate is under active development. The initial version provides the
//! core abstractions and types that all concrete market calendars will build on.
//!
//! ```rust
//! use business_calendar::Date;
//!
//! let d = Date::ymd(2025, 1, 2);
//! assert_eq!(d.year(), 2025);
//! ```

mod calendar;
mod date;
mod rules;
mod static_calendar;

pub mod easter;
pub mod markets;
pub mod utils;

pub use crate::calendar::Calendar;
pub use crate::date::Date;
