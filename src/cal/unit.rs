use std::ops::{Deref, DerefMut};

use range_check;

use cal::compound::YearMonth;
use util::split_cycles;

use self::Month::*;
use self::Weekday::*;


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


/// A month of the year, starting with January, and ending with December.
///
/// ### Month numbering
///
/// A month is represented by an enum instead of just a number. There have
/// been *no end* of bugs caused by off-by-one-month errors in software: is
/// month #2 February (1-indexed) or March (0-indexed)?
///
/// You can cast a month into a number type to convert it, and the resulting
/// number will be 1-indexed (with January as month #1). However, the methods
/// `from_one` and `months_from_january` do a better job of describing which
/// index type is being used.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum Month {

    /// **January**, the first month.
    ///
    /// Its Latin name *Ianuarius* comes from *Janus*, the Roman god of
    /// beginnings and endings.
    January = 1,

    /// **February**, the second month.
    ///
    /// Its Latin name *Februarius* comes from *Februa*, a purification ritual
    /// held during the month.
    February = 2,

    // insert Intercalaris here

    /// **March**, the third month.
    ///
    /// Its Latin name *Martius* comes from *Mars*, the Roman god of war.
    /// Early Roman calendars actually had this as the first month, as winter
    /// was originally monthless.
    March = 3,

    /// **April**, the fourth month.
    ///
    /// Its Latin name *Aprilis* may come from the verb *aperire* meaning “to
    /// open”, an allusion to when trees flower and buds open. (But this
    /// derivation is not certain.)
    April = 4,

    /// **May**, the fifth month.
    ///
    /// Its Latin name *Maius* may come from the Greek god *Maia*.
    May = 5,

    /// **June**, the sixth month.
    ///
    /// Its Latin name *Junius* comes from the Roman goddess *Juno*,
    June = 6,

    /// **July**, the seventh month.
    ///
    /// Its Latin name *Julius* comes directly from *Julius Caesar* because he
    /// was born in it. Before that, it was named *Quintilis* which literally
    /// means “fifth”.
    July = 7,

    /// **August**, the eighth month.
    ///
    /// Its Latin name comes directly from the general *Augustus*. Before
    /// that, it was named *Sextilis* which literally means “sixth”, and if
    /// you thought these etymologies were getting boring, then don’t read the
    /// next five.
    August = 8,

    /// **September**, the ninth month.
    ///
    /// Its name comes from the Latin *septem* meaning “seven”, even though
    /// it’s the ninth month: January and February weren’t originally at the
    /// start of the year, so the old numbering scheme used to fit.
    September = 9,

    /// **October**, the tenth month.
    ///
    /// Its name comes from the Latin *octo* meaning “eight”.
    October = 10,

    /// **November**, the eleventh month.
    ///
    /// Its name comes from the Latin *novem* meaning “nine”.
    November = 11,

    /// **December**, the twelfth and final month.
    ///
    /// Its name comes from the Latin *decem* meaning “ten”.
    December = 12,
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
    /// use datetime::cal::unit::Month;
    ///
    /// assert_eq!(Month::from_one(5), Ok(Month::May));
    /// assert!(Month::from_one(0).is_err());
    /// ```
    pub fn from_one(month: i8) -> range_check::Result<Month, i8> {
        Ok(match month {
             1 => January,   2 => February,   3 => March,
             4 => April,     5 => May,        6 => June,
             7 => July,      8 => August,     9 => September,
            10 => October,  11 => November,  12 => December,
             n => return Err(range_check::Error::new(n, 1..13)),
        })
    }

    /// Returns the month based on a number, with January as **Month 0**,
    /// February as **Month 1**, and so on.
    ///
    /// ```rust
    /// use datetime::cal::unit::Month;
    ///
    /// assert_eq!(Month::from_zero(5), Ok(Month::June));
    /// assert!(Month::from_zero(12).is_err());
    /// ```
    pub fn from_zero(month: i8) -> range_check::Result<Month, i8> {
        Ok(match month {
            0 => January,   1 => February,   2 => March,
            3 => April,     4 => May,        5 => June,
            6 => July,      7 => August,     8 => September,
            9 => October,  10 => November,  11 => December,
            n => return Err(range_check::Error::new(n, 0..12)),
        })
    }
}


/// A named day of the week.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Weekday {

    /// **Monday**, the first day of the week.
    ///
    /// The name comes from the Anglo-Saxon *monandaeg*, meaning “moon’s day”.
    Monday,

    /// **Tuesday**, the second day of the week.
    ///
    /// Named after the Norse god *Tyr*.
    Tuesday,

    /// **Wednesday**, the third day of the week.
    ///
    /// Named after the Norse god *Odin*.
    Wednesday,

    /// **Thursday**, the fourth day of the week.
    ///
    /// Named after the Norse god *Thor*.
    Thursday,

    /// **Friday**, the fifth day of the week.
    ///
    /// Named after the Norse goddess *Frigg*.
    Friday,

    /// **Saturday**, the sixth day of the week.
    ///
    /// The name comes from the Latin *dies Saturni*, meaning “Saturn’s day”.
    Saturday,

    /// **Sunday**, the seventh and last day of the week.
    ///
    /// The name comes from the Latin *dies solis*, meaning “sun’s day”.
    Sunday,
}

impl Weekday {

    /// Returns the number of days this weekday is from Monday, with Monday as
    /// 1 day, Tuesday as 2 days, and so on until Sunday as 7 days.
    ///
    /// ```rust
    /// use datetime::cal::unit::Weekday;
    ///
    /// assert_eq!(Weekday::Saturday.days_from_monday_as_one(), 6);
    /// assert_eq!(Weekday::Sunday.days_from_monday_as_one(), 7);
    /// ```
    pub fn days_from_monday_as_one(&self) -> i8 {
        match *self {
            Sunday   => 7,  Monday => 1,
            Tuesday  => 2,  Wednesday => 3,
            Thursday => 4,  Friday => 5,
            Saturday => 6,
        }
    }

    /// Returns the weekday based on a number, with Sunday as Day 0, Monday as
    /// Day 1, and so on.
    ///
    /// ```rust
    /// use datetime::cal::unit::Weekday;
    ///
    /// assert_eq!(Weekday::from_zero(4), Ok(Weekday::Thursday));
    /// assert!(Weekday::from_zero(7).is_err());
    /// ```
    pub fn from_zero(weekday: i8) -> range_check::Result<Weekday, i8> {
        Ok(match weekday {
            0 => Sunday,     1 => Monday,    2 => Tuesday,
            3 => Wednesday,  4 => Thursday,  5 => Friday,
            6 => Saturday,
            n => return Err(range_check::Error::new(n, 0..7)),
        })
    }

    /// Returns the weekday based on a number, with Monday as Day 1, and Sunday
    /// as Day 7.
    ///
    /// ```rust
    /// use datetime::cal::unit::Weekday;
    ///
    /// assert_eq!(Weekday::from_one(4), Ok(Weekday::Thursday));
    /// assert!(Weekday::from_one(0).is_err());
    /// ```
    pub fn from_one(weekday: i8) -> range_check::Result<Weekday, i8> {
        Ok(match weekday {   1 => Monday,    2 => Tuesday,
            3 => Wednesday,  4 => Thursday,  5 => Friday,
            6 => Saturday,   7 => Sunday,
            n => return Err(range_check::Error::new(n, 1..8)),
        })
    }
}
