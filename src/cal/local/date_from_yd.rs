use range_check::Check;

use cal::local::Result;
use cal::unit::{Year, Month};
use cal::compound::YearMonthDay;

use super::date::{days_since_epoch, from_days_since_epoch};
use super::{Date, EPOCH_DIFFERENCE};


impl Date {

    /// Creates a new local date instance from the given year and day-of-year
    /// values.
    ///
    /// The values are checked for validity before instantiation, and
    /// passing in values out of range will return an error.
    ///
    /// ### Examples
    ///
    /// Instantiate the 13th of September 2015 based on its year
    /// and day-of-year.
    ///
    /// ```rust
    /// use datetime::cal::DatePiece;
    /// use datetime::cal::local;
    /// use datetime::cal::unit::{Year, Weekday, Month};
    ///
    /// let date = local::Date::yd(2015, 0x100).unwrap();
    /// assert_eq!(date.year(), Year::from(2015));
    /// assert_eq!(date.month(), Month::September);
    /// assert_eq!(date.day(), 13);
    /// ```
    ///
    /// ### Overloading
    ///
    /// If you already have a `Year` value, you can pass it in without having
    /// to dereference it to get the actual year number.
    ///
    /// ```
    /// use datetime::cal::unit::{Year, Month};
    /// use datetime::cal::DatePiece;
    /// use datetime::cal::local;
    ///
    /// let year = Year::from(2015);
    /// let date = local::Date::yd(year, 0x100).unwrap();
    /// assert_eq!(date.year(), year);
    /// ```
    pub fn yd<Y>(year: Y, yearday: i64) -> Result<Date>
    where Y: Into<Year> {
        let year = year.into();

        let days_in_year = if year.is_leap_year() { 367 } else { 366 };
        let yearday = try!(yearday.check_range(0..days_in_year));

        let jan_1 = YearMonthDay { year: year, month: Month::January, day: 1 };
        let days = days_since_epoch(jan_1);
        Ok(from_days_since_epoch(days + yearday - 1 - EPOCH_DIFFERENCE))
    }
}


#[cfg(test)]
mod test {
    use cal::local;
    use cal::DatePiece;
    use cal::unit::{Year, Month};

    #[test]
    fn day_start_of_year() {
        let date = local::Date::yd(2015, 1).unwrap();
        assert_eq!(Year::from(2015), date.year());
        assert_eq!(Month::January, date.month());
        assert_eq!(1, date.day());
    }


    #[test]
    fn from_yearday() {
        for date in vec![
            //Date::ymd(1970, 01 , 01).unwrap(),
            local::Date::ymd(1971, Month::from_one(01).unwrap(), 01).unwrap(),
            local::Date::ymd(1973, Month::from_one(01).unwrap(), 01).unwrap(),
            local::Date::ymd(1977, Month::from_one(01).unwrap(), 01).unwrap(),
            local::Date::ymd(1989, Month::from_one(11).unwrap(), 10).unwrap(),
            local::Date::ymd(1990, Month::from_one( 7).unwrap(),  8).unwrap(),
            local::Date::ymd(2014, Month::from_one( 7).unwrap(), 13).unwrap(),
            local::Date::ymd(2001, Month::from_one( 2).unwrap(), 03).unwrap(),
        ]{
            let new_date = local::Date::yd(date.year(), date.yearday() as i64).unwrap();
            assert_eq!(new_date, date);
            assert!(local::Date::yd(2002, 1).is_ok());

            assert_eq!(new_date.yearday(), date.yearday());
        }
    }}
