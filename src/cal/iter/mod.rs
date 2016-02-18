//! Iteration over multiple consecutive days (or other timespans).
//!
//! A common problem is to want to iterate across time one chunk at a time.
//! Examples of doing this include:
//!
//! - Iterating over each day in a year
//! - Iterating over every hour in a day
//! - Iterating over month in a range of years

mod month_into_days;
pub use self::month_into_days::{DaySpan, MonthDays};

mod year_into_months;
pub use self::year_into_months::{MonthSpan, YearMonths};


#[cfg(test)]
mod both_test {
    use cal::unit::Year;

    #[test]
    fn entire_year() {
        let count = Year::from(1999).months(..)
                              .flat_map(|m| m.days(..))
                              .count();

        assert_eq!(count, 365);
    }

    #[test]
    fn entire_leap_year() {
        let count = Year::from(2000).months(..)
                              .flat_map(|m| m.days(..))
                              .count();

        assert_eq!(count, 366);
    }
}
