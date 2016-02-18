extern crate datetime;
use datetime::cal::DatePiece;
use datetime::cal::local;
use datetime::cal::unit::{Year, Month};


#[test]
fn the_distant_past() {
    let date = local::Date::ymd(7, Month::April, 1).unwrap();

    assert_eq!(date.year(),  Year::from(7));
    assert_eq!(date.month(), Month::April);
    assert_eq!(date.day(),   1);
}


#[test]
fn the_distant_present() {
    let date = local::Date::ymd(2015, Month::January, 16).unwrap();

    assert_eq!(date.year(),  Year::from(2015));
    assert_eq!(date.month(), Month::January);
    assert_eq!(date.day(),   16);
}


#[test]
fn the_distant_future() {
    let date = local::Date::ymd(1048576, Month::October, 13).unwrap();

    assert_eq!(date.year(), Year::from(1048576));
    assert_eq!(date.month(), Month::October);
    assert_eq!(date.day(), 13);
}
