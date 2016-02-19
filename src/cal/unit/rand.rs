use rand::{Rand, Rng};
use super::{Year, Month, Weekday};


impl Rand for Year {
    fn rand<R: Rng>(rng: &mut R) -> Year {
        let min = i64::min_value() / 365 + 1;
        let max = i64::max_value() / 365 - 1;

        let num = rng.gen_range(min, max);
        Year::from(num)
    }
}

impl Rand for Month {
    fn rand<R: Rng>(rng: &mut R) -> Month {
        let num = rng.gen_range(0, 12);
        Month::from_zero(num).unwrap()
    }
}

impl Rand for Weekday {
    fn rand<R: Rng>(rng: &mut R) -> Weekday {
        let num = rng.gen_range(0, 7);
        Weekday::from_zero(num).unwrap()
    }
}


#[cfg(any(test, feature = "quickcheck_impls"))]
mod quickcheck {
    use quickcheck::{Arbitrary, Gen};
    use super::super::{Year, Month, Weekday};

    impl Arbitrary for Year {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            g.gen()
        }
    }

    impl Arbitrary for Month {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            g.gen()
        }
    }

    impl Arbitrary for Weekday {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            g.gen()
        }
    }
}
