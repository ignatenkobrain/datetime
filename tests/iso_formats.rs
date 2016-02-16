extern crate datetime;
pub use datetime::ISO;
pub use std::string::ToString;

mod datetimes {
    use super::*;
    use datetime::Month;
    use datetime::local::{Date, Time, DateTime};

    #[test]
    fn recently() {
        let date = Date::ymd(1600, Month::February, 28).unwrap();
        let debugged = date.iso().to_string();

        assert_eq!(debugged, "1600-02-28");
    }

    #[test]
    fn just_then() {
        let date = Date::ymd(-753, Month::December, 1).unwrap();
        let debugged = date.iso().to_string();

        assert_eq!(debugged, "-0753-12-01");
    }

    #[test]
    fn far_far_future() {
        let date = Date::ymd(10601, Month::January, 31).unwrap();
        let debugged = date.iso().to_string();

        assert_eq!(debugged, "+10601-01-31");
    }

    #[test]
    fn midday() {
        let time = Time::hms(12, 0, 0).unwrap();
        let debugged = time.iso().to_string();

        assert_eq!(debugged, "12:00:00.000");
    }

    #[test]
    fn ascending() {
        let then = DateTime::new(
                    Date::ymd(2009, Month::February, 13).unwrap(),
                    Time::hms(23, 31, 30).unwrap());

        let debugged = then.iso().to_string();

        assert_eq!(debugged, "2009-02-13T23:31:30.000");
    }
}

mod offsets {
    use super::*;
    use datetime::offset::Offset;

    #[test]
    fn zulu() {
        let offset = Offset::utc();
        let debugged = offset.iso().to_string();
        assert_eq!(debugged, "Z");
    }

    #[test]
    fn hours() {
        let offset = Offset::of_hours_and_minutes(1, 0).unwrap();
        let debugged = offset.iso().to_string();
        assert_eq!(debugged, "+01");
    }

    #[test]
    fn hours_minutes() {
        let offset = Offset::of_hours_and_minutes(1, 30).unwrap();
        let debugged = offset.iso().to_string();
        assert_eq!(debugged, "+01:30");
    }

    #[test]
    fn dublin_mean_time() {
        let offset = Offset::of_seconds(-25 * 60 - 21).unwrap();
        let debugged = offset.iso().to_string();
        assert_eq!(debugged, "-00:25:21");
    }

    #[test]
    fn offset_date_time() {
        use datetime::Month;
        use datetime::local::{Date, Time, DateTime};

        let offset = Offset::of_seconds(25 * 60 + 21).unwrap();

        let then = DateTime::new(
                    Date::ymd(2009, Month::February, 13).unwrap(),
                    Time::hms(23, 31, 30).unwrap());

        let debugged = offset.transform_date(then).iso().to_string();
        assert_eq!(debugged, "2009-02-13T23:31:30.000+00:25:21");
    }
}
