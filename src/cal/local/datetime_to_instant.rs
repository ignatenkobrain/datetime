use super::{DateTime, SECONDS_IN_DAY};
use super::date::days_since_epoch;
use basic::Instant;
use cal::TimePiece;


impl DateTime {
    pub fn to_instant(&self) -> Instant {
        let seconds = days_since_epoch(self.date().into()) * SECONDS_IN_DAY
            + self.time().to_seconds();

        Instant::at_ms(seconds, self.time().millisecond())
    }
}



#[cfg(test)]
mod test {
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
