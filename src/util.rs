//! Misc stuff.

use std::ops::Range;


pub trait RangeExt: Sized {

    /// Returns whether this value exists within the given range of values.
    fn is_within(&self, range: Range<Self>) -> bool;

    fn check_range(&self, range: Range<Self>) -> Result<(), OutOfRange> {
        if self.is_within(range) {
            Ok(())
        }
        else {
            Err(OutOfRange)
        }
    }
}

// Define RangeExt on *anything* that can be compared, though it’s only
// really ever used for numeric ranges...

impl<T> RangeExt for T where T: PartialOrd<T> {
    fn is_within(&self, range: Range<Self>) -> bool {
        *self >= range.start && *self < range.end
    }
}


pub struct OutOfRange;


/// Split a number of years into a number of year-cycles, and the number
/// of years left over that don’t fit into a cycle. This is also used
/// for day-cycles.
///
/// This is essentially a division operation with the result and the
/// remainder, with the difference that a negative value gets ‘wrapped
/// around’ to be a positive value, owing to the way the modulo operator
/// works for negative values.
pub fn split_cycles(number_of_periods: i64, cycle_length: i64) -> (i64, i64) {
    let mut cycles    = number_of_periods / cycle_length;
    let mut remainder = number_of_periods % cycle_length;

    if remainder < 0 {
        remainder += cycle_length;
        cycles    -= 1;
    }

    (cycles, remainder)
}