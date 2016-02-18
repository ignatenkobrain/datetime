//! Datetimes with a fixed UTC offset.

use std::fmt;

use range_check::{self, Check};

use cal::{DatePiece, TimePiece};
use cal::local;
use cal::fmt::ISO;
use cal::units::{Year, Month, Weekday};
use duration::Duration;


#[derive(PartialEq, Copy, Clone)]
pub struct Offset {
    offset_seconds: Option<i32>,
}

impl Offset {
    fn adjust(&self, local: local::DateTime) -> local::DateTime {
        match self.offset_seconds {
            Some(s) => local + Duration::of(s as i64),
            None    => local,
        }
    }

    pub fn utc() -> Offset {
        Offset { offset_seconds: None }
    }

    pub fn of_seconds(seconds: i32) -> Result<Offset> {
        Ok(Offset {
            offset_seconds: Some(try!(seconds.check_range(-86400..86401))),
        })
    }

    pub fn of_hours_and_minutes(hours: i8, minutes: i8) -> Result<Offset> {
        if (hours.is_positive() && minutes.is_negative())
        || (hours.is_negative() && minutes.is_positive()) {
            Err(Error::SignMismatch)
        }
        else {
            let hours   = try!(hours.check_range(-23..24)) as i32;
            let minutes = try!(minutes.check_range(-59..60)) as i32;
            Offset::of_seconds(hours * (60 * 60) + minutes * 60)
        }
    }

    pub fn transform_date(&self, local: local::DateTime) -> DateTime {
        DateTime {
            local: local,
            offset: self.clone(),
        }
    }

    pub fn is_utc(&self) -> bool {
        self.offset_seconds.is_none()
    }

    pub fn is_negative(&self) -> bool {
        self.hours().is_negative() || self.minutes().is_negative() || self.seconds().is_negative()
    }

    pub fn hours(&self) -> i8 {
        match self.offset_seconds {
            Some(s) => (s / 60 / 60) as i8,
            None => 0,
        }
    }

    pub fn minutes(&self) -> i8 {
        match self.offset_seconds {
            Some(s) => (s / 60 % 60) as i8,
            None => 0,
        }
    }

    pub fn seconds(&self) -> i8 {
        match self.offset_seconds {
            Some(s) => (s % 60) as i8,
            None => 0,
        }
    }
}

impl fmt::Debug for Offset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "offset::Offset({})", self.iso())
    }
}

quick_error! {
    #[derive(PartialEq, Debug, Clone)]
    pub enum Error {
        OutOfRange(err: range_check::Error<i64>) {
            description("offset field out of range")
            display("Field out of range: {}", err)
            cause(err)
        }
        SignMismatch {
            description("sign mismatch")
        }
        Date(err: local::Error) {
            from()
            cause(err)
        }
    }
}

impl<E> From<range_check::Error<E>> for Error
where i64: From<E> {
    fn from(original: range_check::Error<E>) -> Error {
        Error::OutOfRange(original.generify())
    }
}


use std::result;
pub type Result<T> = result::Result<T, Error>;


#[derive(PartialEq, Copy, Clone)]
pub struct DateTime {
    pub local: local::DateTime,
    pub offset: Offset,
}

impl DatePiece for DateTime {
    fn year(&self) -> Year {
        self.offset.adjust(self.local).year()
    }

    fn month(&self) -> Month {
        self.offset.adjust(self.local).month()
    }

    fn day(&self) -> i8 {
        self.offset.adjust(self.local).day()
    }

    fn yearday(&self) -> i16 {
        self.offset.adjust(self.local).yearday()
    }

    fn weekday(&self) -> Weekday {
        self.offset.adjust(self.local).weekday()
    }
}

impl TimePiece for DateTime {
    fn hour(&self) -> i8 {
        self.offset.adjust(self.local).hour()
    }

    fn minute(&self) -> i8 {
        self.offset.adjust(self.local).minute()
    }

    fn second(&self) -> i8 {
        self.offset.adjust(self.local).second()
    }

    fn millisecond(&self) -> i16 {
        self.offset.adjust(self.local).millisecond()
    }
}

impl fmt::Debug for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "offset::DateTime({})", self.iso())
    }
}


#[cfg(test)]
mod test {
    use super::Offset;

    #[test]
    fn fixed_seconds() {
        assert!(Offset::of_seconds(1234).is_ok());
    }

    #[test]
    fn fixed_seconds_panic() {
        assert!(Offset::of_seconds(100_000).is_err());
    }

    #[test]
    fn fixed_hm() {
        assert!(Offset::of_hours_and_minutes(5, 30).is_ok());
    }

    #[test]
    fn fixed_hm_negative() {
        assert!(Offset::of_hours_and_minutes(-3, -45).is_ok());
    }

    #[test]
    fn fixed_hm_err() {
        assert!(Offset::of_hours_and_minutes(8, 60).is_err());
    }

    #[test]
    fn fixed_hm_signs() {
        assert!(Offset::of_hours_and_minutes(-4, 30).is_err());
    }

    #[test]
    fn fixed_hm_signs_zero() {
        assert!(Offset::of_hours_and_minutes(4, 0).is_ok());
    }
}
