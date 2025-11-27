# business_calendar

`business_calendar` is a small, data-driven Rust crate for working with financial and business day calendars.

It provides:

- A lightweight `Date` wrapper around `chrono::NaiveDate`
- A `Calendar` trait with `is_business_day`, `is_weekend`, and `is_holiday`
- A rule engine (`HolidayRule`) for building static calendars
- Predefined US market calendars (NYSE, settlement, government bond, SOFR, NERC, Federal Reserve)
- Precomputed Western and Orthodox Easter Monday tables (no runtime Easter math)

The design is intentionally simple and allocation-free for common queries.

## Installation

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
business_calendar = { git = "https://github.com/hugof38/business_calendar" }
```

Once the crate is published to crates.io, you will be able to depend on it by version instead:

```toml
[dependencies]
business_calendar = "0.1"
```

## Basic usage

Create a date and query a calendar:

```rust
use business_calendar::{Calendar, Date};
use business_calendar::markets::us::{UnitedStates, USMarket};

fn main() {
    // New York Stock Exchange calendar
    let nyse = UnitedStates::new(USMarket::Nyse);

    // New Year's Day 2025
    let new_years = Date::ymd(2025, 1, 1);
    assert!(nyse.is_holiday(new_years));
    assert!(!nyse.is_business_day(new_years));

    // Next business day
    let next_day = Date::ymd(2025, 1, 2);
    assert!(nyse.is_business_day(next_day));
}
```

There is also an example you can run locally:

```bash
cargo run --example us_nyse
```

## Calendars and rules

At the core of the crate are:

- `Date`: a simple wrapper with helpers like `Date::ymd`, `weekday`, and `add_days`
- `Calendar`: a trait implemented by concrete calendars
- `StaticCalendar`: an implementation of `Calendar` defined by a set of rules
- `HolidayRule`: an enum describing how to match holidays (fixed dates, Nth weekday, Easter offsets, one-off dates, etc.)

US market calendars are exposed under `business_calendar::markets::us` and are backed by `StaticCalendar` instances with immutable rule tables.

## Easter tables

The `business_calendar::easter` module exposes Western and Orthodox Easter Monday tables for years 1901–2199.
These are precomputed as day-of-year values and used by holiday rules (for example, for Good Friday or Easter Monday).

## CI

This repository includes a GitHub Actions workflow that runs on pushes and pull requests to `main`:

- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all`

## License

This project is licensed under the terms of the MIT license. See `LICENSE` for details.