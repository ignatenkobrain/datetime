use std::str::FromStr;

use iso8601;

use cal::local;
use cal::offset;
use cal::units::{Month, Weekday};


impl FromStr for local::Date {
    type Err = DateError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let fields = try!(iso8601::date(input));
        let date = try!(fields_to_date(fields));
        Ok(date)
    }
}

impl FromStr for local::Time {
    type Err = DateError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let fields = try!(iso8601::time(input));
        let time = try!(fields_to_time(fields));
        Ok(time)
    }
}

impl FromStr for local::DateTime {
    type Err = DateError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let fields = try!(iso8601::datetime(input));
        let date = try!(fields_to_date(fields.date));
        let time = try!(fields_to_time(fields.time));
        Ok(local::DateTime::new(date, time))
    }
}

impl FromStr for offset::DateTime {
    type Err = OffsetError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let fields = try!(iso8601::datetime(input));

        let date   = try!(fields_to_date(fields.date));
        let time   = try!(fields_to_time(fields.time));

        let hours   = fields.time.tz_offset_hours as i8;
        let minutes = fields.time.tz_offset_minutes as i8;

        let offset = try!(offset::Offset::of_hours_and_minutes(hours, minutes));
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
            from()
            description("parsing resulted in an invalid date")
            cause(err)
        }
        Parse(desc: String) {
            from()
            description("parse error")
            display("Parse error: {}", desc)
        }
    }
}

quick_error! {
    #[derive(PartialEq, Debug, Clone)]
    pub enum OffsetError {
        Construct(err: offset::Error) {
            from()
            from(e: local::Error) -> (e.into())
            description("parsing resulted in an invalid offset date")
            cause(err)
        }
        Parse(desc: String) {
            from()
            description("parse error")
            display("Parse error: {}", desc)
        }
    }
}