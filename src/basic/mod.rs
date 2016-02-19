mod duration;
pub use self::duration::Duration;

mod instant;
pub use self::instant::Instant;

mod arithmetic;

#[cfg(any(test, feature = "rand_impls"))]
mod rand;
