extern crate datetime;
use datetime::local::DateTime;


#[test]
fn test_1970() {
    let date = DateTime::at(0);
    let res = date.to_instant().seconds();

    assert_eq!(res, 0)
}

#[test]
fn test_1971() {
    let date = DateTime::at(86400);
    let res = date.to_instant().seconds();

    assert_eq!(res, 86400)
}

#[test]
fn test_1972() {
    let date = DateTime::at(86400 * 365 * 2);
    let res = date.to_instant().seconds();

    assert_eq!(0, 86400 * 365 * 2 - res)
}

#[test]
fn test_1973() {
    let date = DateTime::at(86400 * (365 * 3 + 1));
    let res = date.to_instant().seconds();

    assert_eq!(0, 86400 * (365 * 3 + 1) - res)
}

#[test]
fn some_date() {
    let date = DateTime::at(1234567890);
    let res = date.to_instant().seconds();

    assert_eq!(1234567890, res)
}

#[test]
fn far_far_future() {
    let date = DateTime::at(54321234567890);
    let res = date.to_instant().seconds();

    assert_eq!(54321234567890, res)
}

#[test]
fn the_distant_past() {
    let date = DateTime::at(-54321234567890);
    let res = date.to_instant().seconds();

    assert_eq!(-54321234567890, res)
}
