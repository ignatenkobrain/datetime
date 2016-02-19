use std::fmt;

use basic::Duration;
use cal::{DatePiece, TimePiece};
use cal::fmt::ISO;
use cal::local::{Date, Time};
use cal::unit::{Year, Month, Weekday};
use system::sys_time;

/// A **local date-time** is an exact instant on the timeline, *without a
/// time zone*.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct DateTime {
    date: Date,
    time: Time,
}

impl DateTime {

    /// Creates a new local date time from a local date and a local time.
    pub fn new(date: Date, time: Time) -> DateTime {
        DateTime {
            date: date,
            time: time,
        }
    }

    /// Returns the date portion of this date-time stamp.
    pub fn date(&self) -> Date {
        self.date
    }

    /// Returns the time portion of this date-time stamp.
    pub fn time(&self) -> Time {
        self.time
    }

    /// Creates a new date-time stamp set to the current time.
    pub fn now() -> DateTime {
        let (s, ms) = unsafe { sys_time() };
        DateTime::at_ms(s, ms)
    }

    pub fn add_seconds(&self, seconds: i64) -> DateTime {
        Self::from_instant(self.to_instant() + Duration::of(seconds))
    }
}

impl DatePiece for DateTime {
    fn year(&self) -> Year { self.date.year() }
    fn month(&self) -> Month { self.date.month() }
    fn day(&self) -> i8 { self.date.day() }
    fn yearday(&self) -> i16 { self.date.yearday() }
    fn weekday(&self) -> Weekday { self.date.weekday() }
}

impl TimePiece for DateTime {
    fn hour(&self) -> i8 { self.time.hour() }
    fn minute(&self) -> i8 { self.time.minute() }
    fn second(&self) -> i8 { self.time.second() }
    fn millisecond(&self) -> i16 { self.time.millisecond() }
}

impl fmt::Debug for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "local::DateTime({})", self.iso())
    }
}
