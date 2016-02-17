use cal::local;
use cal::units::{Year, Month};
use util::{self, RangeExt};

/// A month-year pair.
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct YearMonth {
    pub year: Year,
    pub month: Month,
}

impl YearMonth {

    /// Returns the number of days in this month. This can be definitely
    /// known, as the paired year determines whether it’s a leap year, so
    /// there’s no chance of being caught out by February.
    ///
    /// ### Examples
    ///
    /// ```
    /// use datetime::Year;
    /// use datetime::Month::February;
    ///
    /// assert_eq!(Year::from(2000).month(February).day_count(), 29);
    /// assert_eq!(Year::from(1900).month(February).day_count(), 28);
    /// ```
    pub fn day_count(&self) -> i8 {
        self.month.days_in_month(self.year.is_leap_year())
    }

    /// Returns a `local::Date` based on the day of this month.
    ///
    /// This is just a short-cut for the `local::Date::ymd` constructor.
    pub fn day(&self, day: i8) -> local::Result<local::Date> {
        local::Date::ymd(self.year, self.month, day)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct YearMonthDay {
    pub year: Year,
    pub month: Month,
    pub day: i8,
}

impl YearMonthDay {
    pub fn check_ranges(&self) -> Result<(), util::OutOfRange> {
        try!(self.day.check_range(1 .. self.month.days_in_month(self.year.is_leap_year()) + 1));
        Ok(())
    }
}
