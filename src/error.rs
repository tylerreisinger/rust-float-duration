//! Error handling facilities.
use std::error::Error;
use std::fmt;

#[cfg(feature = "chrono")]
use time;

#[derive(Debug, Clone, Default)]
pub struct OutOfRangeError {}

impl OutOfRangeError {
    pub fn new() -> OutOfRangeError {
        OutOfRangeError {}
    }
}

impl Error for OutOfRangeError {
    fn description(&self) -> &str {
        "The converted duration value is out of range."
    }
}

impl fmt::Display for OutOfRangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

#[cfg(feature = "chrono")]
impl From<time::OutOfRangeError> for OutOfRangeError {
    fn from(_: time::OutOfRangeError) -> OutOfRangeError {
        OutOfRangeError {}
    }
}
