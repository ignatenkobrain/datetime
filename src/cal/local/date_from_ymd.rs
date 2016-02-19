use range_check::Check;

use cal::local::Result;
use cal::unit::{Year, Month};
use cal::compound::YearMonthDay;

use super::date::{days_since_epoch, from_days_since_epoch};
use super::{Date, EPOCH_DIFFERENCE};


impl Date {

    /// Creates a new local date instance from the given year, month, and day
    /// fields.
    ///
    /// The values are checked for validity before instantiation, and
    /// passing in values out of range will return an error.
    ///
    /// ### Examples
    ///
    /// Instantiate the 20th of July 1969 based on its year,
    /// week-of-year, and weekday.
    ///
    /// ```rust
    /// use datetime::cal::local;
    /// use datetime::cal::unit::{Year, Month};
    /// use datetime::cal::DatePiece;
    ///
    /// let date = local::Date::ymd(1969, Month::July, 20).unwrap();
    /// assert_eq!(date.year(), Year::from(1969));
    /// assert_eq!(date.month(), Month::July);
    /// assert_eq!(date.day(), 20);
    ///
    /// assert!(local::Date::ymd(2100, Month::February, 29).is_err());
    /// ```
    ///
    /// ### Overloading
    ///
    /// If you already have a `Year` value, you can pass it in without having
    /// to dereference it to get the actual year number.
    ///
    /// ```
    /// use datetime::cal::DatePiece;
    /// use datetime::cal::local;
    /// use datetime::cal::unit::{Year, Month};
    ///
    /// let year = Year::from(1969);
    /// let date = local::Date::ymd(year, Month::July, 20).unwrap();
    /// assert_eq!(date.year(), year);
    /// ```
    pub fn ymd<Y>(year: Y, month: Month, day: i8) -> Result<Date>
    where Y: Into<Year> {

        // ‘day’ is the only field that needs validating first, but 'year'
        // needs to become a `Year` as well.
        let year = year.into();
        let day = try!(day.check_range(1 .. month.days_in_month(year.is_leap_year()) + 1));

        let ymd  = YearMonthDay { year: year.into(), month: month, day: day };
        let days = days_since_epoch(ymd);

        Ok(from_days_since_epoch(days - EPOCH_DIFFERENCE))
    }
}


#[cfg(test)]
mod test {
    use cal::local;
    use cal::DatePiece;
    use cal::unit::{Year, Month};

    #[test]
    fn the_distant_past() {
        let date = local::Date::ymd(7, Month::April, 1).unwrap();

        assert_eq!(date.year(),  Year::from(7));
        assert_eq!(date.month(), Month::April);
        assert_eq!(date.day(),   1);
    }


    #[test]
    fn the_distant_present() {
        let date = local::Date::ymd(2015, Month::January, 16).unwrap();

        assert_eq!(date.year(),  Year::from(2015));
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(),   16);
    }


    #[test]
    fn the_distant_future() {
        let date = local::Date::ymd(1048576, Month::October, 13).unwrap();

        assert_eq!(date.year(), Year::from(1048576));
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 13);
    }
}
