use std::ops::{Deref, DerefMut};

pub use cal::compounds::{YearMonth};
pub use util::split_cycles;

use self::Month::*;
use self::Weekday::*;


/// A single year.
///
/// This is just a wrapper around `i64` that performs year-related tests.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct Year(pub i64);

impl Year {

    /// Returns whether this year is a leap year.
    ///
    /// ### Examples
    ///
    /// ```
    /// use datetime::Year;
    ///
    /// assert_eq!(Year(2000).is_leap_year(), true);
    /// assert_eq!(Year(1900).is_leap_year(), false);
    /// ```
    pub fn is_leap_year(&self) -> bool {
        self.leap_year_calculations().1
    }

    /// Returns a year-month, pairing this year with the given month.
    ///
    /// ### Examples
    ///
    /// ```
    /// use datetime::{Year, Month};
    ///
    /// let expiry_date = Year(2017).month(Month::February);
    /// assert_eq!(*expiry_date.year, 2017);
    /// assert_eq!(expiry_date.month, Month::February);
    /// ```
    pub fn month(&self, month: Month) -> YearMonth {
        YearMonth {
            year: *self,
            month: month,
        }
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

    fn deref<'a>(&'a self) -> &'a Self::Target {
        &self.0
    }
}

impl DerefMut for Year {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Self::Target {
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


/// A month of the year, starting with January, and ending with December.
///
/// This is stored as an enum instead of just a number to prevent
/// off-by-one errors: is month 2 February (1-indexed) or March (0-indexed)?
/// In this case, it’s 1-indexed, to have January become 1 when you use
/// `as i32` in code.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum Month {
    January =  1, February =  2, March     =  3,
    April   =  4, May      =  5, June      =  6,
    July    =  7, August   =  8, September =  9,
    October = 10, November = 11, December  = 12,
}

impl Month {

    /// Returns the number of days in this month, depending on whether it’s
    /// a leap year or not.
    pub fn days_in_month(&self, leap_year: bool) -> i8 {
        match *self {
            January   => 31, February  => if leap_year { 29 } else { 28 },
            March     => 31, April     => 30,
            May       => 31, June      => 30,
            July      => 31, August    => 31,
            September => 30, October   => 31,
            November  => 30, December  => 31,
        }
    }

    /// Returns the number of days that have elapsed in a year *before* this
    /// month begins, with no leap year check.
    pub fn days_before_start(&self) -> i16 {
        match *self {
            January =>   0, February =>  31, March     =>  59,
            April   =>  90, May      => 120, June      => 151,
            July    => 181, August   => 212, September => 243,
            October => 273, November => 304, December  => 334,
        }
    }

    pub fn months_from_january(&self) -> usize {
        match *self {
            January =>   0, February =>   1, March     =>  2,
            April   =>   3, May      =>   4, June      =>  5,
            July    =>   6, August   =>   7, September =>  8,
            October =>   9, November =>  10, December  => 11,
        }
    }

    /// Returns the month based on a number, with January as **Month 1**,
    /// February as **Month 2**, and so on.
    ///
    /// ```rust
    /// use datetime::Month;
    /// assert_eq!(Month::from_one(5), Some(Month::May));
    /// assert!(Month::from_one(0).is_none());
    /// ```
    pub fn from_one(month: i8) -> Option<Month> {
        Some(match month {
             1 => January,   2 => February,   3 => March,
             4 => April,     5 => May,        6 => June,
             7 => July,      8 => August,     9 => September,
            10 => October,  11 => November,  12 => December,
             _ => return None,
        })
    }

    /// Returns the month based on a number, with January as **Month 0**,
    /// February as **Month 1**, and so on.
    ///
    /// ```rust
    /// use datetime::Month;
    /// assert_eq!(Month::from_zero(5), Some(Month::June));
    /// assert!(Month::from_zero(12).is_none());
    /// ```
    pub fn from_zero(month: i8) -> Option<Month> {
        Some(match month {
            0 => January,   1 => February,   2 => March,
            3 => April,     4 => May,        5 => June,
            6 => July,      7 => August,     8 => September,
            9 => October,  10 => November,  11 => December,
            _ => return None,
        })
    }
}


/// A named day of the week.
///
/// Sunday is Day 0. This seems to be a North American thing? It’s pretty
/// much an arbitrary choice, and as you can’t use the from_zero method,
/// it won’t affect you at all. If you want to change it, the only thing
/// that should be affected is local::Date::days_to_weekday.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Weekday {
    Sunday, Monday, Tuesday, Wednesday, Thursday, Friday, Saturday,
}

// I’m not going to give weekdays an Ord instance because there’s no
// real standard as to whether Sunday should come before Monday, or the
// other way around. Luckily, they don’t need one, as the field is
// ignored when comparing local::Dates.

impl Weekday {
    pub fn days_from_monday_as_one(&self) -> i8 {
        match *self {
            Sunday => 7,   Monday => 1,
            Tuesday => 2,  Wednesday => 3,
            Thursday => 4, Friday => 5,
            Saturday => 6,
        }
    }

    /// Return the weekday based on a number, with Sunday as Day 0, Monday as
    /// Day 1, and so on.
    ///
    /// ```rust
    /// use datetime::Weekday;
    /// assert_eq!(Weekday::from_zero(4), Some(Weekday::Thursday));
    /// assert!(Weekday::from_zero(7).is_none());
    /// ```
    pub fn from_zero(weekday: i8) -> Option<Weekday> {
        Some(match weekday {
            0 => Sunday,     1 => Monday,    2 => Tuesday,
            3 => Wednesday,  4 => Thursday,  5 => Friday,
            6 => Saturday,   _ => return None,
        })
    }

    pub fn from_one(weekday: i8) -> Option<Weekday> {
        Some(match weekday {
            7 => Sunday,     1 => Monday,    2 => Tuesday,
            3 => Wednesday,  4 => Thursday,  5 => Friday,
            6 => Saturday,   _ => return None,
        })
    }
}
