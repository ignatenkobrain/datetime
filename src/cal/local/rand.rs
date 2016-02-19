use rand::{Rand, Rng};

use cal::compound::YearMonth;
use super::Date;


impl Rand for Date {
    fn rand<R: Rng>(rng: &mut R) -> Date {
        let ym: YearMonth = rng.gen();
        let day = rng.gen_range(0, ym.day_count()) + 1;

        Date::ymd(ym.year, ym.month, day).unwrap()
    }
}


#[cfg(any(test, feature = "quickcheck_impls"))]
mod quickcheck {
    use quickcheck::{Arbitrary, Gen};
    use super::super::Date;

    impl Arbitrary for Date {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            g.gen()
        }
    }
}
