#[cfg(feature = "chrono")]
extern crate chrono;
#[cfg(feature = "time")]
extern crate time;
#[cfg(feature = "approx")]
extern crate approx;

pub mod duration;
pub mod error;

pub use duration::{FloatDuration, TimePoint};
pub use duration::{NANOS_PER_SEC, MICROS_PER_SEC, MILLIS_PER_SEC, SECS_PER_MINUTE, SECS_PER_DAY,
                   SECS_PER_YEAR};
