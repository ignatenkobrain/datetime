use std::cmp::Ordering;
use std::fmt;

use cal::DatePiece;
use cal::fmt::ISO;
use cal::local::DateTime;
use cal::compound::YearMonthDay;
use cal::unit::{Year, Month, Weekday};
use util::split_cycles;


/// A **local date** is a day-long span on the timeline, *without a time
/// zone*.
#[derive(Eq, Clone, Copy)]
pub struct Date {
    ymd:     YearMonthDay,
    yearday: i16,
    weekday: Weekday,
}

impl DatePiece for Date {
    fn year(&self) -> Year { self.ymd.year }
    fn month(&self) -> Month { self.ymd.month }
    fn day(&self) -> i8 { self.ymd.day }
    fn yearday(&self) -> i16 { self.yearday }
    fn weekday(&self) -> Weekday { self.weekday }
}

impl fmt::Debug for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "local::Date({})", self.iso())
    }
}

impl From<Date> for YearMonthDay {
    fn from(input: Date) -> YearMonthDay {
        input.ymd
    }
}

impl PartialEq for Date {
    fn eq(&self, other: &Date) -> bool {
        self.ymd == other.ymd
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Date) -> Option<Ordering> {
        self.ymd.partial_cmp(&other.ymd)
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Date) -> Ordering {
        self.ymd.cmp(&other.ymd)
    }
}



/// Number of days guaranteed to be in four years.
const DAYS_IN_4Y:   i64 = 365 *   4 +  1;

/// Number of days guaranteed to be in a hundred years.
const DAYS_IN_100Y: i64 = 365 * 100 + 24;

/// Number of days guaranteed to be in four hundred years.
const DAYS_IN_400Y: i64 = 365 * 400 + 97;



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


impl Date {

    /// Creates a new datestamp set to the current computer clock's date.
    pub fn today() -> Date {
        DateTime::now().date()
    }
}


/// Computes the weekday, given the number of days that have passed
/// since the EPOCH.
pub fn days_to_weekday(days: i64) -> Weekday {
    // March 1st, 2000 was a Wednesday, so add 3 to the number of days.
    let weekday = (days + 3) % 7;

    // We can unwrap since we’ve already done the bounds checking.
    Weekday::from_zero(if weekday < 0 { weekday + 7 } else { weekday } as i8).unwrap()
}


/// Computes a Date - year, month, day, weekday, and yearday -
/// given the number of days that have passed since the EPOCH.
///
/// This is used by all the other constructor functions.
pub fn from_days_since_epoch(days: i64) -> Date {

    // The Gregorian calendar works in 400-year cycles, which repeat
    // themselves ever after.
    //
    // This calculation works by finding the number of 400-year,
    // 100-year, and 4-year cycles, then constantly subtracting the
    // number of leftover days.
    let (num_400y_cycles, mut remainder) = split_cycles(days, DAYS_IN_400Y);

    // Calculate the numbers of 100-year cycles, 4-year cycles, and
    // leftover years, continually reducing the number of days left to
    // think about.
    let num_100y_cycles = remainder / DAYS_IN_100Y;
    remainder -= num_100y_cycles * DAYS_IN_100Y;  // remainder is now days left in this 100-year cycle

    let num_4y_cycles = remainder / DAYS_IN_4Y;
    remainder -= num_4y_cycles * DAYS_IN_4Y;  // remainder is now days left in this 4-year cycle

    let mut years = remainder / 365;
    remainder -= years * 365;  // remainder is now days left in this year

    // Leap year calculation goes thusly:
    //
    // 1. If the year is a multiple of 400, it’s a leap year.
    // 2. Else, if the year is a multiple of 100, it’s *not* a leap year.
    // 3. Else, if the year is a multiple of 4, it’s a leap year again!
    //
    // We already have the values for the numbers of multiples at this
    // point, and it’s safe to re-use them.
    let days_this_year =
        if years == 0 && !(num_4y_cycles == 0 && num_100y_cycles != 0) { 366 }
                                                                  else { 365 };

    // Find out which number day of the year it is.
    // The 306 here refers to the number of days in a year excluding
    // January and February (which are excluded because of the EPOCH)
    let mut day_of_year = remainder + days_this_year - 306;
    if day_of_year >= days_this_year {
        day_of_year -= days_this_year;  // wrap around for January and February
    }

    // Turn all those cycles into an actual number of years.
    years +=   4 * num_4y_cycles
           + 100 * num_100y_cycles
           + 400 * num_400y_cycles;

    // Work out the month and number of days into the month by scanning
    // the time triangle, finding the month that has the correct number
    // of days elapsed at the end of it.
    // (it’s “11 - index” below because the triangle goes backwards)
    let result = TIME_TRIANGLE.iter()
                              .enumerate()
                              .find(|&(_, days)| *days <= remainder);

    let (mut month, month_days) = match result {
        Some((index, days)) => (11 - index, remainder - *days),
        None => (0, remainder),  // No month found? Then it’s February.
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
    Date {
        yearday: (day_of_year + 1) as i16,
        weekday: days_to_weekday(days),
        ymd: YearMonthDay {
            year:  Year::from(years + 2000),
            month: month_variant,
            day:   (month_days + 1) as i8,
        },
    }
}


/// Assumes the `YearMonthDay` is valid, and will return incorrect answers
/// when given an invalid one.
pub fn days_since_epoch(ymd: YearMonthDay) -> i64 {
    let years = *ymd.year - 2000;
    let (leap_days_elapsed, is_leap_year) = ymd.year.leap_year_calculations();

    // Work out the number of days from the start of 1970 to now,
    // which is a multiple of the number of years...
    years * 365

        // Plus the number of days between the start of 2000 and the
        // start of 1970, to make up the difference because our
        // dates start at 2000 and instants start at 1970...
        + 10958

        // Plus the number of leap years that have elapsed between
        // now and the start of 2000...
        + leap_days_elapsed

        // Plus the number of days in all the months leading up to
        // the current month...
        + ymd.month.days_before_start() as i64

        // Plus an extra leap day for *this* year...
        + if is_leap_year && ymd.month >= Month::March { 1 } else { 0 }

        // Plus the number of days in the month so far! (Days are
        // 1-indexed, so we make them 0-indexed here)
        + (ymd.day - 1) as i64
}
