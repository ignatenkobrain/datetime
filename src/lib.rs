#![crate_name = "datetime"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

#![warn(missing_copy_implementations)]
//#![warn(missing_debug_implementations)]
//#![warn(missing_docs)]

#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(unused_results)]

extern crate iso8601;
extern crate libc;
extern crate locale;
extern crate num;
extern crate pad;
extern crate range_check;

#[macro_use]
extern crate quick_error;

mod basic;
pub use basic::{Instant, Duration};

pub mod cal;
pub mod system;
mod util;
