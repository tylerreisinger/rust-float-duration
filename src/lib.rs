#[cfg(feature = "chrono")]
extern crate chrono;
#[cfg(feature = "time")]
extern crate time;

pub mod duration;
pub mod error;

pub use duration::*;
