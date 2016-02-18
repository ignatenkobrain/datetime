use std::fmt;
use std::ops::{Range, RangeFrom, RangeTo, RangeFull};
use std::slice::Iter as SliceIter;

use cal::compound::YearMonth;
use cal::unit::{Year, Month};
use cal::unit::Month::*;


/// Trait for types that contain multiple year-month spans. The obvious
/// example is a year, which contains twelve of these.
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
    /// use datetime::cal::iter::MonthsIter;
    /// use datetime::cal::unit::Month::{April, June};
    /// use datetime::cal::unit::Year;
    ///
    /// let year = Year::from(1999);
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
    year: Year,
    iter: SliceIter<'static, Month>,
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
        write!(f, "YearMonths({}, {:?})", *self.year, self.iter.as_slice())
    }
}



#[cfg(test)]
mod test {
    use super::*;
    use cal::unit::Year;
    use cal::compound::YearMonth;
    use cal::unit::Month::*;

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
