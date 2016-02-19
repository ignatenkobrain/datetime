//! Dates, times, datetimes, months, and weekdays.

mod error;
pub use self::error::{Error, Result};

mod date;
mod date_from_yd;
mod date_from_ymd;
mod date_from_ywd;
pub use self::date::Date;

mod time;
pub use self::time::Time;

mod datetime;
mod datetime_from_instant;
mod datetime_to_instant;
pub use self::datetime::DateTime;

mod days_since_epoch;


/// Number of seconds in a day. As everywhere in this library, leap seconds
/// are simply ignored.
const SECONDS_IN_DAY: i64 = 86400;
