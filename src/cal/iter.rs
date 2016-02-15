use std::fmt;
use std::ops::{Range, RangeFrom, RangeTo, RangeFull};
use std::slice::Iter as SliceIter;

use cal::compounds::YearMonth;
use cal::datetime::{LocalDate};
use cal::units::{Month, Year};
use cal::units::Month::*;


/// A span of months, which gets used to construct a `YearMonths` iterator.
///
/// See the `months` method of `Year` for more information.
pub trait MonthSpan {

    /// Returns a static slice of `Month` values contained by this span.
    fn get_slice(&self) -> &'static [Month];
}

static MONTHS: &'static [Month] = &[
    January,  February,  March,
    April,    May,       June,
    July,     August,    September,
    October,  November,  December,
];

impl MonthSpan for RangeFull {
    fn get_slice(&self) -> &'static [Month] {
        MONTHS
    }
}

impl MonthSpan for RangeFrom<Month> {
    fn get_slice(&self) -> &'static [Month] {
        &MONTHS[self.start.months_from_january() ..]
    }
}

impl MonthSpan for RangeTo<Month> {
    fn get_slice(&self) -> &'static [Month] {
        &MONTHS[.. self.end.months_from_january()]
    }
}

impl MonthSpan for Range<Month> {
    fn get_slice(&self) -> &'static [Month] {
        &MONTHS[self.start.months_from_january() .. self.end.months_from_january()]
    }
}


/// An iterator over a continuous span of months in a year.
///
/// Use the `months` method on `Year` to create instances of this iterator.
pub struct YearMonths {
    pub year: Year,
    pub iter: SliceIter<'static, Month>,
}

impl Iterator for YearMonths {
    type Item = YearMonth;

    fn next(&mut self) -> Option<YearMonth> {
        self.iter.next().map(|m| YearMonth {
            year: self.year,
            month: *m,
        })
    }
}

impl DoubleEndedIterator for YearMonths {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|m| YearMonth {
            year: self.year,
            month: *m,
        })
    }
}

impl fmt::Debug for YearMonths {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "YearMonths({}, {:?})", self.year.0, self.iter.as_slice())
    }
}


/// A span of days, which gets used to construct a `MonthDays` iterator.
pub trait DaySpan {

    /// Returns a `Range` of the day numbers specified for the given year-month pair.
    fn get_range(&self, ym: &YearMonth) -> Range<i8>;
}

impl DaySpan for RangeFull {
    fn get_range(&self, ym: &YearMonth) -> Range<i8> {
        1 .. ym.day_count() + 1
    }
}

impl DaySpan for RangeFrom<i8> {
    fn get_range(&self, ym: &YearMonth) -> Range<i8> {
        self.start .. ym.day_count() + 1
    }
}

impl DaySpan for RangeTo<i8> {
    fn get_range(&self, _ym: &YearMonth) -> Range<i8> {
        1 .. self.end
    }
}

impl DaySpan for Range<i8> {
    fn get_range(&self, _ym: &YearMonth) -> Range<i8> {
        self.clone()
    }
}


/// An iterator over a continuous span of days in a month.
///
/// Use the `days` method on `YearMonth` to create instances of this iterator.
#[derive(PartialEq, Debug)]
pub struct MonthDays {
    pub ym: YearMonth,
    pub range: Range<i8>,
}

impl Iterator for MonthDays {
    type Item = LocalDate;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().and_then(|d| LocalDate::ymd(self.ym.year.0, self.ym.month, d).ok())
    }
}

impl DoubleEndedIterator for MonthDays {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range.next_back().and_then(|d| LocalDate::ymd(self.ym.year.0, self.ym.month, d).ok())
    }
}
