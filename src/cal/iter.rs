use std::fmt;
use std::ops::{Range, RangeFrom, RangeTo, RangeFull};
use std::slice::Iter as SliceIter;

use cal::compounds::YearMonth;
use cal::local;
use cal::units::{Month, Year};
use cal::units::Month::*;


pub trait MonthsIter {

    /// Returns an iterator over a continuous span of months in this year,
    /// returning year-month pairs.
    ///
    /// This method takes one argument that can be of four different types,
    /// depending on the months you wish to iterate over:
    ///
    /// - The `RangeFull` type (such as `..`), which iterates over every
    ///   month;
    /// - The `RangeFrom` type (such as `April ..`), which iterates over
    ///   the months starting from the month given;
    /// - The `RangeTo` type (such as `.. June`), which iterates over the
    ///   months stopping at *but not including* the month given;
    /// - The `Range` type (such as `April .. June`), which iterates over
    ///   the months starting from the left one and stopping at *but not
    ///   including* the right one.
    ///
    /// ### Examples
    ///
    /// ```
    /// use datetime::Year;
    /// use datetime::Month::{April, June};
    /// use datetime::iter::MonthsIter;
    ///
    /// let year = Year(1999);
    /// assert_eq!(year.months(..).count(), 12);
    /// assert_eq!(year.months(April ..).count(), 9);
    /// assert_eq!(year.months(April .. June).count(), 2);
    /// assert_eq!(year.months(.. June).count(), 5);
    /// ```
    fn months<S: MonthSpan>(&self, span: S) -> YearMonths;
}

impl MonthsIter for Year {
    fn months<S: MonthSpan>(&self, span: S) -> YearMonths {
        YearMonths {
            year: *self,
            iter: span.get_slice().iter(),
        }
    }
}

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


pub trait DaysIter {

    /// Returns an iterator over a continuous span of days in this month,
    /// returning `local::Date` values.
    ///
    /// ### Examples
    ///
    /// ```
    /// use datetime::Year;
    /// use datetime::Month::September;
    /// use datetime::iter::DaysIter;
    ///
    /// let ym = Year(1999).month(September);
    /// assert_eq!(ym.days(..).count(), 30);
    /// assert_eq!(ym.days(10 ..).count(), 21);
    /// assert_eq!(ym.days(10 .. 20).count(), 10);
    /// assert_eq!(ym.days(.. 20).count(), 19);
    /// ```
    fn days<S: DaySpan>(&self, span: S) -> MonthDays;
}

impl DaysIter for YearMonth {
    fn days<S: DaySpan>(&self, span: S) -> MonthDays {
        MonthDays {
            ym: *self,
            range: span.get_range(self)
        }
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
    type Item = local::Date;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().and_then(|d| local::Date::ymd(self.ym.year.0, self.ym.month, d).ok())
    }
}

impl DoubleEndedIterator for MonthDays {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range.next_back().and_then(|d| local::Date::ymd(self.ym.year.0, self.ym.month, d).ok())
    }
}
