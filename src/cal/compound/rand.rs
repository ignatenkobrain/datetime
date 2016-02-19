use rand::{Rand, Rng};
use super::YearMonth;


impl Rand for YearMonth {
    fn rand<R: Rng>(rng: &mut R) -> YearMonth {
        YearMonth {
            year: rng.gen(),
            month: rng.gen(),
        }
    }
}


#[cfg(any(test, feature = "quickcheck_impls"))]
mod quickcheck {
    use quickcheck::{Arbitrary, Gen};
    use super::super::YearMonth;

    impl Arbitrary for YearMonth {
        fn arbitrary<G: Gen>(g: &mut G) -> YearMonth {
            g.gen()
        }
    }
}
