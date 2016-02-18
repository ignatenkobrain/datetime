//! Iteration over multiple consecutive days (or other timespans).
//!
//! A common problem is to want to iterate across time one chunk at a time.
//! Examples of doing this include:
//!
//! - Iterating over each day in a year
//! - Iterating over every hour in a day
//! - Iterating over month in a range of years

mod month_into_days;
pub use self::month_into_days::{DaysIter, DaySpan, MonthDays};

mod year_into_months;
pub use self::year_into_months::{MonthsIter, MonthSpan, YearMonths};