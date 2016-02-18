extern crate datetime;
use datetime::cal::local;
use datetime::Duration;


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
