use std::ops::{Add, Sub};

use basic::Duration;
use cal::local;


impl Add<Duration> for local::DateTime {
    type Output = local::DateTime;

    fn add(self, duration: Duration) -> local::DateTime {
        local::DateTime::from_instant(self.to_instant() + duration)
    }
}

impl Sub<Duration> for local::DateTime {
    type Output = local::DateTime;

    fn sub(self, duration: Duration) -> local::DateTime {
        local::DateTime::from_instant(self.to_instant() - duration)
    }
}


#[cfg(test)]
mod test {
    use basic::Duration;
    use cal::local;

    #[test]
    fn addition() {
        let date = local::DateTime::at(10000);
        assert_eq!(local::DateTime::at(10001), date + Duration::of(1))
    }

    #[test]
    fn subtraction() {
        let date = local::DateTime::at(100000000);
        assert_eq!(local::DateTime::at(99999999), date - Duration::of(1))
    }
}
