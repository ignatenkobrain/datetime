use std::ops::{Deref, DerefMut};

use util::split_cycles;


/// A single year.
///
/// This is just a wrapper around `i64` that performs year-related tests.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct Year(i64);

impl Year {

    /// Returns the year after this year.
    ///
    /// ### Examples
    ///
    /// ```
    /// use datetime::cal::unit::Year;
    ///
    /// assert_eq!(Year::from(1904).next_year(), Year::from(1905));
    /// ```
    pub fn next_year(&self) -> Year {
        Year(self.0 + 1)
    }

    /// Returns the year before this year.
    ///
    /// ### Examples
    ///
    /// ```
    /// use datetime::cal::unit::Year;
    ///
    /// assert_eq!(Year::from(1904).previous_year(), Year::from(1903));
    /// ```
    pub fn previous_year(&self) -> Year {
        Year(self.0 - 1)
    }

    /// Returns whether this year is a leap year.
    ///
    /// ### Examples
    ///
    /// ```
    /// use datetime::cal::unit::Year;
    ///
    /// assert_eq!(Year::from(2000).is_leap_year(), true);
    /// assert_eq!(Year::from(1900).is_leap_year(), false);
    /// ```
    pub fn is_leap_year(&self) -> bool {
        self.leap_year_calculations().1
    }

    /// Performs two related calculations for leap years, returning the
    /// results as a two-part tuple:
    ///
    /// 1. The number of leap years that have elapsed prior to this year;
    /// 2. Whether this year is a leap year or not.
    pub fn leap_year_calculations(&self) -> (i64, bool) {
        let year = self.0 - 2000;

        // This calculation is the reverse of local::Date::from_days_since_epoch.
        let (num_400y_cycles, mut remainder) = split_cycles(year, 400);

        // Standard leap-year calculations, performed on the remainder
        let currently_leap_year = remainder == 0 || (remainder % 100 != 0 && remainder % 4 == 0);

        let num_100y_cycles = remainder / 100;
        remainder -= num_100y_cycles * 100;

        let leap_years_elapsed = remainder / 4
            + 97 * num_400y_cycles  // There are 97 leap years in 400 years
            + 24 * num_100y_cycles  // There are 24 leap years in 100 years
            - if currently_leap_year { 1 } else { 0 };

        (leap_years_elapsed, currently_leap_year)
    }
}

impl From<i64> for Year {
    fn from(year: i64) -> Year {
        Year(year)
    }
}

impl Deref for Year {
    type Target = i64;

    fn deref(&self) -> &i64 {
        &self.0
    }
}

impl DerefMut for Year {
    fn deref_mut(&mut self) -> &mut i64 {
        &mut self.0
    }
}

impl AsRef<i64> for Year {
    fn as_ref(&self) -> &i64 {
        &self.0
    }
}

impl AsMut<i64> for Year {
    fn as_mut(&mut self) -> &mut i64 {
        &mut self.0
    }
}
