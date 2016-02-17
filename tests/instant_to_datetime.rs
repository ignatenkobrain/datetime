extern crate datetime;
use datetime::{Year, Month, DatePiece, TimePiece};
use datetime::local::DateTime;


#[test]
fn a_long_time_ago() {
    let date = DateTime::at(-1_000_000_000);

    assert_eq!(date.year(),   Year::from(1938));
    assert_eq!(date.month(),  Month::April);
    assert_eq!(date.day(),    24);
    assert_eq!(date.hour(),   22);
    assert_eq!(date.minute(), 13);
    assert_eq!(date.second(), 20);
}


#[test]
fn unix_epoch() {
    let date = DateTime::at(0);

    assert_eq!(date.year(),   Year::from(1970));
    assert_eq!(date.month(),  Month::January);
    assert_eq!(date.day(),    01);
    assert_eq!(date.hour(),   00);
    assert_eq!(date.minute(), 00);
    assert_eq!(date.second(), 00);
}


#[test]
fn billennium() {
    let date = DateTime::at(1_000_000_000);

    assert_eq!(date.year(),   Year::from(2001));
    assert_eq!(date.month(),  Month::September);
    assert_eq!(date.day(),    09);
    assert_eq!(date.hour(),   01);
    assert_eq!(date.minute(), 46);
    assert_eq!(date.second(), 40);
}


#[test]
fn numbers() {
    let date = DateTime::at(1_234_567_890);

    assert_eq!(date.year(),   Year::from(2009));
    assert_eq!(date.month(),  Month::February);
    assert_eq!(date.day(),    13);
    assert_eq!(date.hour(),   23);
    assert_eq!(date.minute(), 31);
    assert_eq!(date.second(), 30);
}


#[test]
fn year_2038_problem() {
    let date = DateTime::at(0x7FFF_FFFF);

    assert_eq!(date.year(),   Year::from(2038));
    assert_eq!(date.month(),  Month::January);
    assert_eq!(date.day(),    19);
    assert_eq!(date.hour(),   03);
    assert_eq!(date.minute(), 14);
    assert_eq!(date.second(), 07);
}


#[test]
fn the_end_of_time() {
    let date = DateTime::at(0x7FFF_FFFF_FFFF_FFFF);

    assert_eq!(date.year(),   Year::from(292_277_026_596));
    assert_eq!(date.month(),  Month::December);
    assert_eq!(date.day(),    4);
    assert_eq!(date.hour(),   15);
    assert_eq!(date.minute(), 30);
    assert_eq!(date.second(), 07);
}


#[test]
fn just_some_date() {
    let date = DateTime::at(146096 * 86400);

    assert_eq!(date.year(),   Year::from(2369));
    assert_eq!(date.month(),  Month::December);
    assert_eq!(date.day(),    31);
    assert_eq!(date.hour(),   00);
    assert_eq!(date.minute(), 00);
    assert_eq!(date.second(), 00);
}
