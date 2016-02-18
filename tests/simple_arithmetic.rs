extern crate datetime;
use datetime::instant::Instant;
use datetime::duration::Duration;


#[test]
fn addition() {
    assert_eq!(Instant::at(10), Instant::at(3) + Duration::of(7))
}

#[test]
fn subtraction() {
    assert_eq!(Instant::at(20), Instant::at(50) - Duration::of(30))
}
