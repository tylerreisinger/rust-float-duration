use std::error::Error;
use std::fmt;

#[cfg(feature = "chrono")]
use time;

#[derive(Debug, Clone)]
pub struct OutOfRangeError {}

impl OutOfRangeError {
    pub fn new() -> OutOfRangeError {
        OutOfRangeError {}
    }
}

impl Error for OutOfRangeError {
    fn description(&self) -> &str {
        "The conversion between duration representations yielded
        produced an out-of-range value"
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
