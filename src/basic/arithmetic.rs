use std::ops::{Add, Sub};

use basic::{Instant, Duration};


impl Add<Duration> for Instant {
    type Output = Instant;

    fn add(self, duration: Duration) -> Instant {
        let (s, ms) = duration.lengths();
        let seconds = self.seconds() + s;
        let milliseconds = self.milliseconds() + ms;

        Instant::at_ms(seconds, milliseconds)
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;

    fn sub(self, duration: Duration) -> Instant {
        let (s, ms) = duration.lengths();
        let seconds = self.seconds() - s;
        let milliseconds = self.milliseconds() - ms;

        Instant::at_ms(seconds, milliseconds)
    }
}


#[cfg(test)]
mod unit_test {
    use basic::{Instant, Duration};

    #[test]
    fn addition() {
        assert_eq!(Instant::at(10), Instant::at(3) + Duration::of(7))
    }

    #[test]
    fn subtraction() {
        assert_eq!(Instant::at(20), Instant::at(50) - Duration::of(30))
    }
}
