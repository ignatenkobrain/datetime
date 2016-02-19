use rand::{Rand, Rng};

use cal::compound::YearMonth;
use super::{Date, Time, DateTime};


impl Rand for Date {
    fn rand<R: Rng>(rng: &mut R) -> Date {
        let ym: YearMonth = rng.gen();
        let day = rng.gen_range(0, ym.day_count()) + 1;

        Date::ymd(ym.year, ym.month, day).unwrap()
    }
}

impl Rand for Time {
    fn rand<R: Rng>(rng: &mut R) -> Time {
        let hours = rng.gen_range(0, 24);
        let minutes = rng.gen_range(0, 60);
        let seconds = rng.gen_range(0, 60);

        Time::hms(hours, minutes, seconds).unwrap()
    }
}

impl Rand for DateTime {
    fn rand<R: Rng>(rng: &mut R) -> DateTime {
        DateTime::new(rng.gen(), rng.gen())
    }
}


#[cfg(any(test, feature = "quickcheck_impls"))]
mod quickcheck {
    use quickcheck::{Arbitrary, Gen};
    use super::super::{Date, Time, DateTime};

    impl Arbitrary for Date {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            g.gen()
        }
    }

    impl Arbitrary for Time {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            g.gen()
        }
    }

    impl Arbitrary for DateTime {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            g.gen()
        }
    }
}
