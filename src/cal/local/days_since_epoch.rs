use cal::compound::YearMonthDay;
use cal::unit::{Year, Month, Weekday};
use util::split_cycles;


/// Number of days guaranteed to be in four years.
const DAYS_IN_4Y:   i64 = 365 *   4 +  1;

/// Number of days guaranteed to be in a hundred years.
const DAYS_IN_100Y: i64 = 365 * 100 + 24;

/// Number of days guaranteed to be in four hundred years.
const DAYS_IN_400Y: i64 = 365 * 400 + 97;


/// Number of days between  **1st January, 1970** and **1st March, 2000**.
///
/// This might seem like an odd number to calculate, instead of using the
/// 1st of January as a reference point, but it turs out that by having the
/// reference point immediately after a possible leap-year day, the maths
/// needed to calculate the day/week/month of an instant comes out a *lot*
/// simpler!
///
/// The Gregorian calendar operates on a 400-year cycle, so the combination
/// of having it on a year that’s a multiple of 400, and having the leap
/// day at the very end of one of these cycles, means that the calculations
/// are reduced to simple division (of course, with a bit of date-shifting
/// to base a date around this reference point).
///
/// Rust has the luxury of having been started *after* this date. In Win32,
/// the epoch is midnight, the 1st of January, 1601, for much the same
/// reasons - except that it was developed before the year 2000, so they
/// had to go all the way back to the *previous* 400-year multiple.[^win32]
///
/// The only problem is that many people assume the Unix epoch to be
/// midnight on the 1st January 1970, so this value (and any functions that
/// depend on it) aren’t exposed to users of this library.
///
/// [^win32]: http://blogs.msdn.com/b/oldnewthing/archive/2009/03/06/9461176.aspx
///
const EPOCH_DIFFERENCE: i64 = (30 * 365      // 30 years between 2000 and 1970...
                               + 7           // plus seven days for leap years...
                               + 31 + 29);   // plus all the days in January and February in 2000.


/// This rather strange triangle is an array of the number of days elapsed
/// at the end of each month, starting at the beginning of March (the first
/// month after the EPOCH above), going backwards, ignoring February.
const TIME_TRIANGLE: &'static [i64; 11] =
    &[31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 + 30 + 31 + 31,  // January
      31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 + 30 + 31,  // December
      31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 + 30,  // November
      31 + 30 + 31 + 30 + 31 + 31 + 30 + 31,  // October
      31 + 30 + 31 + 30 + 31 + 31 + 30,  // September
      31 + 30 + 31 + 30 + 31 + 31,  // August
      31 + 30 + 31 + 30 + 31,  // July
      31 + 30 + 31 + 30,  // June
      31 + 30 + 31,  // May
      31 + 30,  // April
      31]; // March


pub struct DaysSinceEpoch(i64);

impl From<YearMonthDay> for DaysSinceEpoch {
    fn from(ymd: YearMonthDay) -> DaysSinceEpoch {
        use cal::unit::Month;

        let years = *ymd.year - 2000;
        let (leap_days_elapsed, is_leap_year) = leap_year_calculations(years);

        // Work out the number of days from the start of 1970 to now,
        // which is a multiple of the number of years...
        let days = years * 365

            // Plus the number of leap years that have elapsed between
            // now and the start of 2000...
            + leap_days_elapsed

            // Minus January and February, 2000...
            - (31 + 29)

            // Plus the number of days in all the months leading up to
            // the current month...
            + ymd.month.days_before_start() as i64

            // Plus an extra leap day for *this* year...
            + if is_leap_year && ymd.month >= Month::March { 1 } else { 0 }

            + ymd.day as i64;

        DaysSinceEpoch(days)
    }
}

impl DaysSinceEpoch {
    pub fn new(days_since_unix_epoch: i64) -> DaysSinceEpoch {
        DaysSinceEpoch(days_since_unix_epoch - EPOCH_DIFFERENCE)
    }

    pub fn add(&mut self, days: i64) {
        self.0 += days
    }

    pub fn count(&self) -> i64 {
        self.0 + EPOCH_DIFFERENCE
    }

    pub fn weekday(&self) -> Weekday {

        // March 1st, 2000 was a Wednesday, so add 3 to the number of days.
        let weekday = (self.0 + 3) % 7;

        // We can unwrap since we’ve already done the bounds checking.
        Weekday::from_zero(if weekday < 0 { weekday + 7 } else { weekday } as i8).unwrap()
    }

    pub fn to_yd(&self) -> ExactDay {

        // The Gregorian calendar works in 400-year cycles, which repeat
        // themselves ever after.
        //
        // This calculation works by finding the number of 400-year,
        // 100-year, and 4-year cycles, then constantly subtracting the
        // number of leftover days.
        let (num_400y_cycles, mut remainder) = split_cycles(self.0, DAYS_IN_400Y);

        // Calculate the numbers of 100-year cycles, 4-year cycles, and
        // leftover years, continually reducing the number of days left to
        // think about.
        let num_100y_cycles = remainder / DAYS_IN_100Y;
        remainder -= num_100y_cycles * DAYS_IN_100Y;  // remainder is now days left in this 100-year cycle

        let num_4y_cycles = remainder / DAYS_IN_4Y;
        remainder -= num_4y_cycles * DAYS_IN_4Y;  // remainder is now days left in this 4-year cycle

        let mut years = remainder / 365;
        remainder -= years * 365;  // remainder is now days left in this year

        // We already have the values for the numbers of multiples at this
        // point, and it’s safe to re-use them.
        let days_this_year =
            if years == 0 && !(num_4y_cycles == 0 && num_100y_cycles != 0) { 366 }
                                                                      else { 365 };

        // Turn all those cycles into an actual number of years.
        years +=   4 * num_4y_cycles
               + 100 * num_100y_cycles
               + 400 * num_400y_cycles;

        ExactDay {
            year: years,
            remainder: remainder,
            days_this_year: days_this_year,
        }
    }
}


pub struct ExactDay {
    year: i64,
    remainder: i64,
    days_this_year: i64,
}

impl ExactDay {

    pub fn yearday(&self) -> i16 {

        // Find out which number day of the year it is.
        // The 306 here refers to the number of days in a year excluding
        // January and February (which are excluded because of the EPOCH)
        let mut day_of_year = self.remainder + self.days_this_year - 306;
        if day_of_year >= self.days_this_year {
            day_of_year -= self.days_this_year;  // wrap around for January and February
        }

        day_of_year as i16 + 1
    }

    pub fn ymd(&self) -> YearMonthDay {
        let mut years = self.year;

        // Work out the month and number of days into the month by scanning
        // the time triangle, finding the month that has the correct number
        // of days elapsed at the end of it.
        // (it’s “11 - index” below because the triangle goes backwards)
        let result = TIME_TRIANGLE.iter()
                                  .enumerate()
                                  .find(|&(_, days)| *days <= self.remainder);

        let (mut month, month_days) = match result {
            Some((index, days)) => (11 - index, self.remainder - *days),
            None => (0, self.remainder),  // No month found? Then it’s February.
        };

        // Need to add 2 to the month in order to compensate for the EPOCH
        // being in March.
        month += 2;

        if month >= 12 {
            years += 1;   // wrap around for January and February
            month -= 12;  // (yes, again)
        }

        // The check immediately above means we can `unwrap` this, as the
        // month number is guaranteed to be in the range (0..12).
        let month_variant = Month::from_zero(month as i8).unwrap();

        // Finally, adjust the day numbers for human reasons: the first day
        // of the month is the 1st, rather than the 0th, and the year needs
        // to be adjusted relative to the EPOCH.
        YearMonthDay {
            year:  Year::from(years + 2000),
            month: month_variant,
            day:   (month_days + 1) as i8,
        }
    }
}


/// Performs two related calculations for leap years, returning the
/// results as a two-part tuple:
///
/// 1. The number of leap years that have elapsed prior to this year;
/// 2. Whether this year is a leap year or not.
fn leap_year_calculations(year: i64) -> (i64, bool) {
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