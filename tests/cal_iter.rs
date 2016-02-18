extern crate datetime;
pub use datetime::{YearMonth, Year};
pub use datetime::iter::{DaysIter, MonthsIter};

mod months {
    use super::*;
    use datetime::Month::*;

    #[test]
    fn range_full() {
        let year = Year::from(2013);
        let months: Vec<_> = year.months(..).collect();
        assert_eq!(months, vec![
            year.month(January),
            year.month(February),
            year.month(March),
            year.month(April),
            year.month(May),
            year.month(June),
            year.month(July),
            year.month(August),
            year.month(September),
            year.month(October),
            year.month(November),
            year.month(December),
        ]);
    }

    #[test]
    fn range_from() {
        let year = Year::from(2013);
        let months: Vec<_> = year.months(July..).collect();
        assert_eq!(months, vec![
            year.month(July),
            year.month(August),
            year.month(September),
            year.month(October),
            year.month(November),
            year.month(December),
        ]);
    }

    #[test]
    fn range_to() {
        let year = Year::from(2013);
        let months: Vec<_> = year.months(..July).collect();
        assert_eq!(months, vec![
            year.month(January),
            year.month(February),
            year.month(March),
            year.month(April),
            year.month(May),
            year.month(June),
        ]);
    }

    #[test]
    fn range() {
        let year = Year::from(2013);
        let months: Vec<_> = year.months(April..July).collect();
        assert_eq!(months, vec![
            year.month(April),
            year.month(May),
            year.month(June),
        ]);
    }

    #[test]
    fn range_empty() {
        let year = Year::from(2013);
        let months: Vec<_> = year.months(August..August).collect();
        assert!(months.is_empty());
    }

    #[test]
    fn range_singular() {
        let year = Year::from(2013);
        let months = year.month(April);
        assert_eq!(months, year.month(April));
    }
}

mod days {
    use super::*;
    use datetime::local::Date;
    use datetime::Month::*;

    #[test]
    fn range_full() {
        let year = Year::from(2013).month(February);
        let days: Vec<_> = year.days(..).collect();
        let results: Vec<_> = (1..29).map(|d| Date::ymd(2013, February, d).unwrap()).collect();
        assert_eq!(days, results);
    }

    #[test]
    fn range_full_leap_year() {
        let year = Year::from(2000).month(February);
        let days: Vec<_> = year.days(..).collect();
        let results: Vec<_> = (1..30).map(|d| Date::ymd(2000, February, d).unwrap()).collect();
        assert_eq!(days, results);
    }

    #[test]
    fn range() {
        let year = Year::from(2008).month(March);
        let days: Vec<_> = year.days(10..20).collect();
        let results: Vec<_> = (10..20).map(|d| Date::ymd(2008, March, d).unwrap()).collect();
        assert_eq!(days, results);
    }
}

#[test]
fn entire_year() {
    let count = Year::from(1999).months(..)
                          .flat_map(|m| m.days(..))
                          .count();

    assert_eq!(count, 365);
}

#[test]
fn entire_leap_year() {
    let count = Year::from(2000).months(..)
                          .flat_map(|m| m.days(..))
                          .count();

    assert_eq!(count, 366);
}
