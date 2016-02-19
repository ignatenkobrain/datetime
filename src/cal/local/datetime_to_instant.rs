use super::{DateTime, SECONDS_IN_DAY};
use super::days_since_epoch::DaysSinceEpoch;
use basic::Instant;
use cal::TimePiece;
use cal::compound::YearMonthDay;


impl DateTime {
    pub fn to_instant(&self) -> Instant {
        // It's easier to do this algorithm in two steps than one: we work out
        // the number of seconds in the days since the Unix epoch, then the
        // number of seconds that have elapsed since midnight, then add them
        // together.
        let ymd: YearMonthDay = self.date().into();
        let seconds_since_unix_epoch = DaysSinceEpoch::from(ymd).count() * SECONDS_IN_DAY;
        let seconds_since_midnight = self.time().to_seconds();

        let seconds = seconds_since_unix_epoch + seconds_since_midnight;
        Instant::at_ms(seconds, self.time().millisecond())
    }
}


#[cfg(test)]
mod unit_test {
    use cal::local;

    #[test]
    fn test_1970() {
        let date = local::DateTime::at(0);
        let res = date.to_instant().seconds();

        assert_eq!(res, 0)
    }

    #[test]
    fn test_1971() {
        let date = local::DateTime::at(86400);
        let res = date.to_instant().seconds();

        assert_eq!(res, 86400)
    }

    #[test]
    fn test_1972() {
        let date = local::DateTime::at(86400 * 365 * 2);
        let res = date.to_instant().seconds();

        assert_eq!(0, 86400 * 365 * 2 - res)
    }

    #[test]
    fn test_1973() {
        let date = local::DateTime::at(86400 * (365 * 3 + 1));
        let res = date.to_instant().seconds();

        assert_eq!(0, 86400 * (365 * 3 + 1) - res)
    }

    #[test]
    fn some_date() {
        let date = local::DateTime::at(1234567890);
        let res = date.to_instant().seconds();

        assert_eq!(1234567890, res)
    }

    #[test]
    fn far_far_future() {
        let date = local::DateTime::at(54321234567890);
        let res = date.to_instant().seconds();

        assert_eq!(54321234567890, res)
    }

    #[test]
    fn the_distant_past() {
        let date = local::DateTime::at(-54321234567890);
        let res = date.to_instant().seconds();

        assert_eq!(-54321234567890, res)
    }
}
