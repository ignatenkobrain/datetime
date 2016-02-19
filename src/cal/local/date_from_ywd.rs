use cal::local::Result;
use cal::unit::{Year, Month, Weekday};
use cal::compound::YearMonthDay;

use super::days_since_epoch::DaysSinceEpoch;
use super::Date;


impl Date {

    /// Creates a new local date instance from the given year, week-of-year,
    /// and weekday values.
    ///
    /// The values are checked for validity before instantiation, and
    /// passing in values out of range will return an error.
    ///
    /// ### Examples
    ///
    /// Instantiate the 11th of September 2015 based on its year,
    /// week-of-year, and weekday.
    ///
    /// ```rust
    /// use datetime::cal::DatePiece;
    /// use datetime::cal::local;
    /// use datetime::cal::unit::{Year, Weekday, Month};
    ///
    /// let date = local::Date::ywd(2015, 37, Weekday::Friday).unwrap();
    /// assert_eq!(date.year(), Year::from(2015));
    /// assert_eq!(date.month(), Month::September);
    /// assert_eq!(date.day(), 11);
    /// assert_eq!(date.weekday(), Weekday::Friday);
    /// ```
    ///
    /// Note that according to the ISO-8601 standard, the year will change
    /// when working with dates early in week 1, or late in week 53:
    ///
    /// ```rust
    /// use datetime::cal::DatePiece;
    /// use datetime::cal::local;
    /// use datetime::cal::unit::{Year, Weekday, Month};
    ///
    /// let date = local::Date::ywd(2009, 1, Weekday::Monday).unwrap();
    /// assert_eq!(date.year(), Year::from(2008));
    /// assert_eq!(date.month(), Month::December);
    /// assert_eq!(date.day(), 29);
    /// assert_eq!(date.weekday(), Weekday::Monday);
    ///
    /// let date = local::Date::ywd(2009, 53, Weekday::Sunday).unwrap();
    /// assert_eq!(date.year(), Year::from(2010));
    /// assert_eq!(date.month(), Month::January);
    /// assert_eq!(date.day(), 3);
    /// assert_eq!(date.weekday(), Weekday::Sunday);
    /// ```
    pub fn ywd<Y>(year: Y, week: i64, weekday: Weekday) -> Result<Date>
    where Y: Into<Year> {
        let year = year.into();

        let jan_4 = YearMonthDay { year: year, month: Month::January, day: 4 };
        let correction = DaysSinceEpoch::from(jan_4)
            .weekday()
            .days_from_monday_as_one() as i64 + 3;

        let yearday = 7 * week + weekday.days_from_monday_as_one() as i64 - correction;

        if yearday <= 0 {
            let days_in_year = if year.previous_year().is_leap_year() { 366 } else { 365 };
            Date::yd(*year - 1, days_in_year + yearday)
        }
        else {
            let days_in_year = if year.is_leap_year() { 366 } else { 365 };

            if yearday >= days_in_year {
                Date::yd(year.next_year(), yearday - days_in_year)
            }
            else {
                Date::yd(year, yearday)
            }
        }
    }
}