extern crate datetime;
use datetime::local::DateTime;
use datetime::Duration;


#[test]
fn addition() {
    let date = DateTime::at(10000);
    assert_eq!(DateTime::at(10001), date + Duration::of(1))
}

#[test]
fn subtraction() {
    let date = DateTime::at(100000000);
    assert_eq!(DateTime::at(99999999), date - Duration::of(1))
}
