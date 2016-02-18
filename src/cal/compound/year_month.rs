use cal::unit::{Year, Month};


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
    /// use datetime::cal::unit::{Year, Month};
    ///
    /// assert_eq!(Year::from(2000).month(Month::February).day_count(), 29);
    /// assert_eq!(Year::from(1900).month(Month::February).day_count(), 28);
    /// ```
    pub fn day_count(&self) -> i8 {
        self.month.days_in_month(self.year.is_leap_year())
    }
}


impl Year {

    /// Returns a year-month, pairing this year with the given month.
    ///
    /// ### Examples
    ///
    /// ```
    /// use datetime::cal::unit::{Year, Month};
    ///
    /// let expiry_date = Year::from(2017).month(Month::February);
    /// assert_eq!(*expiry_date.year, 2017);
    /// assert_eq!(expiry_date.month, Month::February);
    /// ```
    pub fn month(&self, month: Month) -> YearMonth {
        YearMonth {
            year: *self,
            month: month,
        }
    }
}