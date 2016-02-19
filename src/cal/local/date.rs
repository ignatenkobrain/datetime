use std::cmp::Ordering;
use std::fmt;

use cal::DatePiece;
use cal::fmt::ISO;
use cal::local::DateTime;
use cal::compound::YearMonthDay;
use cal::unit::{Year, Month, Weekday};
use super::days_since_epoch::DaysSinceEpoch;


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


impl Date {

    /// Creates a new datestamp set to the current computer clock's date.
    pub fn today() -> Date {
        DateTime::now().date()
    }
}

impl From<DaysSinceEpoch> for Date {
    fn from(dse: DaysSinceEpoch) -> Date {
        let day = dse.to_yd();

        Date {
            yearday: day.yearday(),
            weekday: dse.weekday(),
            ymd:     day.ymd(),
        }
    }
}
