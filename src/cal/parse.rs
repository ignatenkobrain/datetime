use std::str::FromStr;

use iso8601;

use cal::local;
use cal::offset;
use cal::units::{Month, Weekday};


impl FromStr for local::Date {
    type Err = DateError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match iso8601::date(input) {
            Ok(fields)  => fields_to_date(fields).map_err(DateError::Construct),
            Err(e)      => Err(DateError::Parse(e)),
        }
    }
}

impl FromStr for local::Time {
    type Err = DateError;

    fn from_str(input: &str) -> Result<local::Time, Self::Err> {
        match iso8601::time(input) {
            Ok(fields)  => fields_to_time(fields).map_err(DateError::Construct),
            Err(e)      => Err(DateError::Parse(e)),
        }
    }
}

impl FromStr for local::DateTime {
    type Err = DateError;

    fn from_str(input: &str) -> Result<local::DateTime, Self::Err> {
        let fields = match iso8601::datetime(input) {
            Ok(fields)  => fields,
            Err(e)      => return Err(DateError::Parse(e)),
        };

        let date = try!(fields_to_date(fields.date).map_err(DateError::Construct));
        let time = try!(fields_to_time(fields.time).map_err(DateError::Construct));
        Ok(local::DateTime::new(date, time))
    }
}

impl FromStr for offset::DateTime {
    type Err = OffsetError;

    fn from_str(input: &str) -> Result<offset::DateTime, Self::Err> {
        let fields = match iso8601::datetime(input) {
            Ok(fields)  => fields,
            Err(e)      => return Err(OffsetError::Parse(e)),
        };

        let date   = try!(fields_to_date(fields.date).map_err(|e| OffsetError::Construct(offset::Error::Date(e))));
        let time   = try!(fields_to_time(fields.time).map_err(|e| OffsetError::Construct(offset::Error::Date(e))));
        let offset = try!(offset::Offset::of_hours_and_minutes(fields.time.tz_offset_hours as i8, fields.time.tz_offset_minutes as i8).map_err(OffsetError::Construct));
        Ok(offset.transform_date(local::DateTime::new(date, time)))
    }
}


fn fields_to_date(fields: iso8601::Date) -> local::Result<local::Date> {
    match fields {
        iso8601::Date::YMD { year, month, day } => {
            let month_variant = try!(Month::from_one(month as i8));
            local::Date::ymd(year as i64, month_variant, day as i8)
        }

        iso8601::Date::Week { year, ww, d } => {
            let weekday_variant = try!(Weekday::from_one(d as i8));
            local::Date::ywd(year as i64, ww as i64, weekday_variant)
        }

        iso8601::Date::Ordinal { year, ddd } => {
            local::Date::yd(year as i64, ddd as i64)
        }
    }
}

fn fields_to_time(fields: iso8601::Time) -> local::Result<local::Time> {
    let h  = fields.hour as i8;
    let m  = fields.minute as i8;
    let s  = fields.second as i8;
    let ms = fields.millisecond as i16;

    local::Time::hms_ms(h, m, s, ms)
}


quick_error! {
    #[derive(PartialEq, Debug, Clone)]
    pub enum DateError {
        Construct(err: local::Error) {
            description("parsing resulted in an invalid date")
            cause(err)
        }
        Parse(desc: String) {
            description("parse error")
            display("Parse error: {}", desc)
        }
    }
}

quick_error! {
    #[derive(PartialEq, Debug, Clone)]
    pub enum OffsetError {
        Construct(err: offset::Error) {
            description("parsing resulted in an invalid offset date")
            cause(err)
        }
        Parse(desc: String) {
            description("parse error")
            display("Parse error: {}", desc)
        }
    }
}