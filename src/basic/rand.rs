use rand::{Rand, Rng};
use super::Instant;


impl Rand for Instant {
    fn rand<R: Rng>(rng: &mut R) -> Instant {
        Instant::at(rng.gen())
    }
}


#[cfg(any(test, feature = "quickcheck_impls"))]
mod quickcheck {
    use quickcheck::{Arbitrary, Gen};
    use super::super::Instant;

    impl Arbitrary for Instant {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            g.gen()
        }
    }
}
