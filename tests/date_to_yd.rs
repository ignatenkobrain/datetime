extern crate datetime;
use datetime::cal::DatePiece;
use datetime::cal::local;
use datetime::cal::unit::Month;


#[test]
fn start_of_year_day() {
    let date = local::Date::ymd(2015, Month::January, 1).unwrap();
    assert_eq!(date.yearday(), 1);
}


#[test]
fn end_of_year_day() {
    let date = local::Date::ymd(2015, Month::December, 31).unwrap();
    assert_eq!(date.yearday(), 365);
}


#[test]
fn end_of_leap_year_day() {
    let date = local::Date::ymd(2016, Month::December, 31).unwrap();
    assert_eq!(date.yearday(), 366);
}


#[test]
fn yearday() {
    for year in 1..2058 {
        assert_eq!( local::Date::ymd(year, Month::January, 31).unwrap().yearday() + 1,
                    local::Date::ymd(year, Month::February, 01).unwrap().yearday());
        assert_eq!( local::Date::ymd(year, Month::March, 31).unwrap().yearday() + 1,
                    local::Date::ymd(year, Month::April, 01).unwrap().yearday());
        assert_eq!( local::Date::ymd(year, Month::April, 30).unwrap().yearday() + 1,
                    local::Date::ymd(year, Month::May, 01).unwrap().yearday());
        assert!(    local::Date::ymd(year, Month::December, 31).unwrap().yearday() > 0);
    }
    assert_eq!( local::Date::ymd(1600, Month::February, 29).unwrap().yearday() + 1, // leap year
                local::Date::ymd(1600, Month::March, 01).unwrap().yearday());
    assert_eq!( local::Date::ymd(1601, Month::February, 28).unwrap().yearday() + 1, // no leap year
                local::Date::ymd(1601, Month::March, 01).unwrap().yearday());
}