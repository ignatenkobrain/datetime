use cal::unit::{Year, Month};
use super::YearMonth;


#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct YearMonthDay {
    pub year: Year,
    pub month: Month,
    pub day: i8,
}


impl YearMonth {

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