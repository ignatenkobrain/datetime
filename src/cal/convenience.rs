//! Adds convenience functions to some structs.
//!
//! # Example
//! ```
//! # use datetime::local::Date;
//! # use datetime::DatePiece;
//! use datetime::convenience::Today;
//! let today:Date = Date::today();
//! ```
use cal::local::{Date, DateTime};

/// Adds `LocalDate::today() -> LocalDate`
pub trait Today{
    fn today() -> Date;
}

impl Today for Date {
    fn today() -> Date {
        DateTime::now().date()
    }

}

