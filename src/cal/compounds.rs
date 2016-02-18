use cal::units::{Year, Month};


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

    /// Returns a `YearMonthDay` based on this year and month, along with the
    /// given day.
    ///
    /// This is just a short-cut for creating `YearMonthDay` values, and
    /// doesn’t do anything special.
    pub fn day(&self, day: i8) -> YearMonthDay {
        YearMonthDay {
            year: self.year,
            month: self.month,
            day: day,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct YearMonthDay {
    pub year: Year,
    pub month: Month,
    pub day: i8,
}
