use range_check;


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
        use self::Month::*;

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
        use self::Month::*;

        match *self {
            January =>   0, February =>  31, March     =>  59,
            April   =>  90, May      => 120, June      => 151,
            July    => 181, August   => 212, September => 243,
            October => 273, November => 304, December  => 334,
        }
    }

    pub fn months_from_january(&self) -> usize {
        use self::Month::*;

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
        use self::Month::*;

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
        use self::Month::*;

        Ok(match month {
            0 => January,   1 => February,   2 => March,
            3 => April,     4 => May,        5 => June,
            6 => July,      7 => August,     8 => September,
            9 => October,  10 => November,  11 => December,
            n => return Err(range_check::Error::new(n, 0..12)),
        })
    }
}