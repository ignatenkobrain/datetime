use std::fmt;

use range_check::Check;

use cal::TimePiece;
use cal::fmt::ISO;
use cal::local::{DateTime, Result};


/// A **local time** is a time on the timeline that recurs once a day,
/// *without a time zone*.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Time {
    hour:   i8,
    minute: i8,
    second: i8,
    millisecond: i16,
}

impl Time {

    /// Computes the number of hours, minutes, and seconds, based on the
    /// number of seconds that have elapsed since midnight.
    pub fn from_seconds_since_midnight(seconds: i64) -> Time {
        Time::from_seconds_and_milliseconds_since_midnight(seconds, 0)
    }

    /// Computes the number of hours, minutes, and seconds, based on the
    /// number of seconds that have elapsed since midnight.
    pub fn from_seconds_and_milliseconds_since_midnight(seconds: i64, millisecond_of_second: i16) -> Time {
        Time {
            hour:   (seconds / 60 / 60) as i8,
            minute: (seconds / 60 % 60) as i8,
            second: (seconds % 60) as i8,
            millisecond: millisecond_of_second,
        }
    }

    /// Returns the time at midnight, with all fields initialised to 0.
    pub fn midnight() -> Time {
        Time { hour: 0, minute: 0, second: 0, millisecond: 0 }
    }

    /// Creates a new timestamp instance with the given hour and minute
    /// fields. The second and millisecond fields are set to 0.
    ///
    /// The values are checked for validity before instantiation, and
    /// passing in values out of range will return an `Err`.
    pub fn hm(hour: i8, minute: i8) -> Result<Time> {
        if hour == 24 && minute == 0 {
            return Ok(Time { hour: hour, minute: minute, second: 0, millisecond: 0 });
        }

        Ok(Time {
            hour: try!(hour.check_range(0..24)),
            minute: try!(minute.check_range(0..60)),
            second: 0,
            millisecond: 0,
        })
    }

    /// Creates a new timestamp instance with the given hour, minute, and
    /// second fields. The millisecond field is set to 0.
    ///
    /// The values are checked for validity before instantiation, and
    /// passing in values out of range will return an `Err`.
    pub fn hms(hour: i8, minute: i8, second: i8) -> Result<Time> {
        if hour == 24 && minute == 0 && second == 0 {
            return Ok(Time { hour: hour, minute: minute, second: second, millisecond: 0 });
        }

        Ok(Time {
            hour: try!(hour.check_range(0..24)),
            minute: try!(minute.check_range(0..60)),
            second: try!(second.check_range(0..60)),
            millisecond: 0,
        })
    }

    /// Creates a new timestamp instance with the given hour, minute,
    /// second, and millisecond fields.
    ///
    /// The values are checked for validity before instantiation, and
    /// passing in values out of range will return an `Err`.
    pub fn hms_ms(hour: i8, minute: i8, second: i8, millisecond: i16) -> Result<Time> {
        Ok(Time {
            hour:        try!(hour.check_range(0..24)),
            minute:      try!(minute.check_range(0..60)),
            second:      try!(second.check_range(0..60)),
            millisecond: try!(millisecond.check_range(0..1000)),
        })
    }

    /// Creates a new timestamp set to the current computer clock’s time.
    pub fn now() -> Time {
        DateTime::now().time()
    }

    /// Calculate the number of seconds since midnight this time is at,
    /// ignoring milliseconds.
    pub fn to_seconds(&self) -> i64 {
        self.hour as i64 * 3600
            + self.minute as i64 * 60
            + self.second as i64
    }
}

impl TimePiece for Time {
    fn hour(&self) -> i8 { self.hour }
    fn minute(&self) -> i8 { self.minute }
    fn second(&self) -> i8 { self.second }
    fn millisecond(&self) -> i16 { self.millisecond }
}

impl fmt::Debug for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "local::Time({})", self.iso())
    }
}
