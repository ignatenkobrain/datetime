use std::result;

use range_check;


quick_error! {
    #[derive(PartialEq, Debug, Clone)]
    pub enum Error {
        OutOfRange(err: range_check::Error<i64>) {
            description("datetime field out of range")
            display("Field out of range: {}", err)
            cause(err)
        }
    }
}

impl<E> From<range_check::Error<E>> for Error
where i64: From<E> {
    fn from(original: range_check::Error<E>) -> Error {
        Error::OutOfRange(original.generify())
    }
}


pub type Result<T> = result::Result<T, Error>;