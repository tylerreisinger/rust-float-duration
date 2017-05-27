//! Floating-point duration type `FloatDuration` and helpers.
use std::time;
use std::fmt;
use std::ops;
use std::f64;
use std::u64;
use std::iter::Sum;

#[cfg(feature = "chrono")]
use chrono;
#[cfg(feature = "approx")]
use approx::ApproxEq;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize, Serializer, Deserializer};
#[cfg(feature = "serde")]
use serde::de::{self, Visitor};

use super::error;

/// Number of nanoseconds in a second.
pub const NANOS_PER_SEC: f64 = 1.0e9;
/// Number of microseconds in a second.
pub const MICROS_PER_SEC: f64 = 1.0e6;
/// Number of milliseconds in a second.
pub const MILLIS_PER_SEC: f64 = 1.0e3;
/// Number of seconds in a minute.
pub const SECS_PER_MINUTE: f64 = 60.0;
/// Number of seconds in an hour.
pub const SECS_PER_HOUR: f64 = SECS_PER_MINUTE * 60.0;
/// Number of seconds in a day.
pub const SECS_PER_DAY: f64 = SECS_PER_HOUR * 24.0;
/// Number of seconds in a year.
pub const SECS_PER_YEAR: f64 = SECS_PER_DAY * 365.0;

/// A fallible conversion from one duration representation to another.
///
/// This is very similar to the `std::convert::TryFrom` trait which is currently
/// unstable.
pub trait FromDuration<T>: Sized {
    type Error;
    /// Convert `from` into a `Self` object.
    fn from_duration(from: T) -> Result<Self, Self::Error>;
}

/// A fallible conversion that consumes `self`.
///
/// This is very similar to the `std::convert::TryInto` trait which is currently
/// unstable.
///
/// Similar to `std::convert::Into`, this trait is reflexively implemented for
/// all implementations of `FromDuration` and should not be manually implemented.
pub trait IntoDuration<T>: Sized {
    type Error;
    /// Convert `self` into a `T` object.
    fn into_duration(self) -> Result<T, Self::Error>;
}

impl<T, U> IntoDuration<U> for T
    where U: FromDuration<T>
{
    type Error = U::Error;
    fn into_duration(self) -> Result<U, U::Error> {
        U::from_duration(self)
    }
}

/// A specific point in time.
///
/// Types implementing `TimePoint` can have a `FloatDuration` computed between them
/// via `float_duration_since` in either direction.
pub trait TimePoint<Rhs = Self> {
    /// The type returned if there is an error computing the duration.
    type Error;
    /// The amount of time between two `TimePoint`s.
    fn float_duration_since(self, rhs: Rhs) -> Result<FloatDuration, Self::Error>;
}

/// A time duration stored as a floating point quantity.
///
/// Unlike `std::time::Duration` or `chrono::Duration`, `FloatDuration`
/// aims to be convenient and fast to use in simulation and mathematical expressions
/// rather than to behave like a calendar or perfectly
/// accurately represent precise time scales.
///
/// Internally, a `FloatDuration` stores a single `f64` number of floating-point seconds,
/// thus it is only as precise as the `f64` type.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct FloatDuration {
    secs: f64,
}


impl FloatDuration {
    /// Create a new `FloatDuration` representing a number of years.
    ///
    /// `float_duration` considers one year to be exactly 365 days, with
    /// no consideration of leap years.
    #[inline]
    pub fn years(years: f64) -> FloatDuration {
        FloatDuration { secs: years * SECS_PER_YEAR }
    }
    /// Create a new `FloatDuration` representing a number of days.
    #[inline]
    pub fn days(days: f64) -> FloatDuration {
        FloatDuration { secs: days * SECS_PER_DAY }
    }
    /// Create a new `FloatDuration` representing a number of hours.
    #[inline]
    pub fn hours(hours: f64) -> FloatDuration {
        FloatDuration { secs: hours * SECS_PER_HOUR }
    }
    /// Create a new `FloatDuration` representing a number of minutes.
    #[inline]
    pub fn minutes(mins: f64) -> FloatDuration {
        FloatDuration { secs: mins * SECS_PER_MINUTE }
    }
    /// Create a new `FloatDuration` representing a number of seconds.
    #[inline]
    pub fn seconds(secs: f64) -> FloatDuration {
        FloatDuration { secs: secs }
    }
    /// Create a new `FloatDuration` representing a number of milliseconds.
    #[inline]
    pub fn milliseconds(millis: f64) -> FloatDuration {
        FloatDuration { secs: millis / MILLIS_PER_SEC }
    }
    /// Create a new `FloatDuration` representing a number of microseconds.
    #[inline]
    pub fn microseconds(micros: f64) -> FloatDuration {
        FloatDuration { secs: micros / MICROS_PER_SEC }
    }
    /// Create a new `FloatDuration` representing a number of nanoseconds.
    #[inline]
    pub fn nanoseconds(nanos: f64) -> FloatDuration {
        FloatDuration { secs: nanos / NANOS_PER_SEC }
    }

    /// Return the total number of fractional years represented by the `FloatDuration`.
    ///
    /// `float_duration` considers one year to be exactly 365 days, with
    /// no consideration of leap years.
    #[inline]
    pub fn as_years(&self) -> f64 {
        self.secs / SECS_PER_YEAR
    }
    /// Return the total number of fractional days represented by the `FloatDuration`.
    #[inline]
    pub fn as_days(&self) -> f64 {
        self.secs / SECS_PER_DAY
    }
    /// Return the total number of fractional hours represented by the `FloatDuration`.
    #[inline]
    pub fn as_hours(&self) -> f64 {
        self.secs / SECS_PER_HOUR
    }
    /// Return the total number of fractional minutes represented by the `FloatDuration`.
    #[inline]
    pub fn as_minutes(&self) -> f64 {
        self.secs / SECS_PER_MINUTE
    }
    /// Return the total number of fractional seconds represented by the `FloatDuration`.
    #[inline]
    pub fn as_seconds(&self) -> f64 {
        self.secs
    }
    /// Return the total number of fractional milliseconds represented by the `FloatDuration`.
    #[inline]
    pub fn as_milliseconds(&self) -> f64 {
        self.secs * MILLIS_PER_SEC
    }
    /// Return the total number of fractional microseconds represented by the `FloatDuration`.
    #[inline]
    pub fn as_microseconds(&self) -> f64 {
        self.secs * MICROS_PER_SEC
    }
    /// Return the total number of fractional nanoseconds represented by the `FloatDuration`.
    #[inline]
    pub fn as_nanoseconds(&self) -> f64 {
        self.secs * NANOS_PER_SEC
    }

    /// Compute the absolute value of this duration.
    #[inline]
    pub fn abs(self) -> FloatDuration {
        FloatDuration { secs: self.secs.abs() }
    }
    /// Return a new `FloatDuration` that represents zero elapsed time.
    #[inline]
    pub fn zero() -> FloatDuration {
        FloatDuration { secs: 0.0 }
    }
    /// Returns true is this duration represents zero elapsed time (equals `FloatDuration::zero()`).
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.secs == 0.0
    }
    /// Returns true if the FloatDuration holds a positive amount of time.
    #[inline]
    pub fn is_positive(&self) -> bool {
        self.secs.is_sign_positive()
    }
    /// Returns true if the FloatDuration holds a negative amount of time.
    #[inline]
    pub fn is_negative(&self) -> bool {
        self.secs.is_sign_negative()
    }

    /// Return a new `FloatDuration` with the minimum possible value.
    #[inline]
    pub fn min_value() -> FloatDuration {
        FloatDuration { secs: f64::MIN }
    }
    /// Return a new `FloatDuration` with the maximum possible value.
    #[inline]
    pub fn max_value() -> FloatDuration {
        FloatDuration { secs: f64::MAX }
    }

    /// Create a `std::time::Duration` object from a `FloatDuration`.
    ///
    /// # Errors
    /// `std::time::Duration` does not support negative values or seconds
    /// greater than `std::u64::MAX`. This function will return a
    /// `DurationError::StdOutOfRange` if the `FloatDuration` value is outside
    /// of either of those bounds.
    pub fn to_std(&self) -> Result<time::Duration, error::OutOfRangeError> {
        if self.secs.is_sign_negative() {
            Err(error::OutOfRangeError::new())
        } else {
            let seconds = self.secs.trunc();
            let nanos = self.secs.fract() * NANOS_PER_SEC;

            if seconds > u64::MAX as f64 {
                Err(error::OutOfRangeError::new())
            } else {
                Ok(time::Duration::new(seconds as u64, nanos as u32))
            }
        }
    }

    /// Create a `FloatDuration` object from a `std::time::Duration`.
    #[inline]
    pub fn from_std(duration: time::Duration) -> FloatDuration {
        FloatDuration::seconds((duration.as_secs() as f64) +
                               (duration.subsec_nanos() as f64) / NANOS_PER_SEC)
    }
}

#[cfg(feature = "chrono")]
impl FloatDuration {
    /// Create a `chrono::Duration` object from a `FloatDuration`.
    ///
    /// # Errors
    /// Presently, the conversion to `chrono::Duration` first goes through
    /// `std::time::Duration` and return an error if `to_std` returns an error.
    pub fn to_chrono(&self) -> Result<chrono::Duration, error::OutOfRangeError> {
        let is_negative = self.is_negative();
        let std_duration = self.abs().to_std()?;
        let chrono_duration = chrono::Duration::from_std(std_duration)?;
        if is_negative {
            Ok(-chrono_duration)
        } else {
            Ok(chrono_duration)
        }
    }

    /// Create a `FloatDuration` object from a `chrono::Duration`.
    ///
    /// `chrono::Duration` does not provide a way to access sub-millisecond
    /// precision if the duration is too large to be entirely represented as a single
    /// value. Thus, if the absolute value of the total number of nanoseconds is
    /// greater than `i64::MAX`, only millisecond precision will be captured.
    #[inline]
    pub fn from_chrono(duration: chrono::Duration) -> FloatDuration {
        if let Some(nanos) = duration.num_nanoseconds() {
            FloatDuration::nanoseconds(nanos as f64)
        } else {
            FloatDuration::milliseconds(duration.num_milliseconds() as f64)
        }
    }
}

#[cfg(feature = "serde")]
struct FloatDurationVisitor;

// We want to serialize a `FloatDuration` as a single f64 instead of a struct.
#[cfg(feature = "serde")]
impl<'de> Visitor<'de> for FloatDurationVisitor {
    type Value = FloatDuration;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a floating-point value")
    }
    fn visit_f32<E>(self, value: f32) -> Result<FloatDuration, E>
        where E: de::Error
    {
        Ok(FloatDuration::seconds(value as f64))
    }
    fn visit_f64<E>(self, value: f64) -> Result<FloatDuration, E>
        where E: de::Error
    {
        Ok(FloatDuration::seconds(value))
    }
}

#[cfg(feature = "serde")]
impl Serialize for FloatDuration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_f64(self.secs)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for FloatDuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_f64(FloatDurationVisitor)
    }
}

#[cfg(feature = "chrono")]
impl<Tz: chrono::TimeZone> TimePoint for chrono::DateTime<Tz> {
    type Error = ();
    #[inline]
    fn float_duration_since(self, since: chrono::DateTime<Tz>) -> Result<FloatDuration, ()> {
        let chrono_duration = self.signed_duration_since(since);
        Ok(FloatDuration::from_chrono(chrono_duration))
    }
}
#[cfg(feature = "chrono")]
impl<Tz: chrono::TimeZone> TimePoint for chrono::Date<Tz> {
    type Error = ();
    #[inline]
    fn float_duration_since(self, since: chrono::Date<Tz>) -> Result<FloatDuration, ()> {
        let chrono_duration = self.signed_duration_since(since);
        Ok(FloatDuration::from_chrono(chrono_duration))
    }
}
#[cfg(feature = "chrono")]
impl TimePoint for chrono::NaiveDate {
    type Error = ();
    #[inline]
    fn float_duration_since(self, since: chrono::NaiveDate) -> Result<FloatDuration, ()> {
        let chrono_duration = self.signed_duration_since(since);
        Ok(FloatDuration::from_chrono(chrono_duration))
    }
}
#[cfg(feature = "chrono")]
impl TimePoint for chrono::NaiveTime {
    type Error = ();
    #[inline]
    fn float_duration_since(self, since: chrono::NaiveTime) -> Result<FloatDuration, ()> {
        let chrono_duration = self.signed_duration_since(since);
        Ok(FloatDuration::from_chrono(chrono_duration))
    }
}
#[cfg(feature = "chrono")]
impl TimePoint for chrono::NaiveDateTime {
    type Error = ();
    #[inline]
    fn float_duration_since(self, since: chrono::NaiveDateTime) -> Result<FloatDuration, ()> {
        let chrono_duration = self.signed_duration_since(since);
        Ok(FloatDuration::from_chrono(chrono_duration))
    }
}

impl TimePoint for time::Instant {
    type Error = ();
    #[inline]
    fn float_duration_since(self, since: time::Instant) -> Result<FloatDuration, ()> {
        let std_duration = self.duration_since(since);
        Ok(FloatDuration::from_std(std_duration))
    }
}
impl TimePoint for time::SystemTime {
    type Error = time::SystemTimeError;
    #[inline]
    fn float_duration_since(self,
                            since: time::SystemTime)
                            -> Result<FloatDuration, time::SystemTimeError> {
        let std_duration = self.duration_since(since)?;
        Ok(FloatDuration::from_std(std_duration))
    }
}

impl FromDuration<time::Duration> for FloatDuration {
    type Error = ();
    #[inline]
    fn from_duration(from: time::Duration) -> Result<FloatDuration, ()> {
        Ok(FloatDuration::from_std(from))
    }
}
#[cfg(feature = "chrono")]
impl FromDuration<chrono::Duration> for FloatDuration {
    type Error = ();
    #[inline]
    fn from_duration(from: chrono::Duration) -> Result<FloatDuration, ()> {
        Ok(FloatDuration::from_chrono(from))
    }
}
impl FromDuration<FloatDuration> for time::Duration {
    type Error = error::OutOfRangeError;
    #[inline]
    fn from_duration(from: FloatDuration) -> Result<time::Duration, error::OutOfRangeError> {
        from.to_std()
    }
}
#[cfg(feature = "chrono")]
impl FromDuration<FloatDuration> for chrono::Duration {
    type Error = error::OutOfRangeError;
    #[inline]
    fn from_duration(from: FloatDuration) -> Result<chrono::Duration, error::OutOfRangeError> {
        from.to_chrono()
    }
}

impl From<time::Duration> for FloatDuration {
    fn from(from: time::Duration) -> FloatDuration {
        FloatDuration::from_std(from)
    }
}
#[cfg(feature = "chrono")]
impl From<chrono::Duration> for FloatDuration {
    fn from(from: chrono::Duration) -> FloatDuration {
        FloatDuration::from_chrono(from)
    }
}

impl fmt::Display for FloatDuration {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if self.secs > SECS_PER_YEAR {
            write!(fmt, "{} years", self.as_years())
        } else if self.secs > SECS_PER_DAY {
            write!(fmt, "{} days", self.as_days())
        } else if self.secs > SECS_PER_HOUR {
            write!(fmt, "{} hours", self.as_hours())
        } else if self.secs > SECS_PER_MINUTE {
            write!(fmt, "{} minutes", self.as_minutes())
        } else if self.secs > 1.0 {
            write!(fmt, "{} seconds", self.as_seconds())
        } else if self.secs > 1.0e-3 {
            write!(fmt, "{} milliseconds", self.as_milliseconds())
        } else if self.secs > 1.0e-6 {
            write!(fmt, "{} microseconds", self.as_microseconds())
        } else {
            write!(fmt, "{} nanoseconds", self.as_nanoseconds())
        }
    }
}

impl ops::Neg for FloatDuration {
    type Output = FloatDuration;

    fn neg(self) -> FloatDuration {
        FloatDuration { secs: -self.secs }
    }
}

impl ops::Add<FloatDuration> for FloatDuration {
    type Output = FloatDuration;

    fn add(self, rhs: FloatDuration) -> FloatDuration {
        FloatDuration { secs: self.secs + rhs.secs }
    }
}
impl ops::Sub<FloatDuration> for FloatDuration {
    type Output = FloatDuration;

    fn sub(self, rhs: FloatDuration) -> FloatDuration {
        FloatDuration { secs: self.secs - rhs.secs }
    }
}

impl ops::Mul<f64> for FloatDuration {
    type Output = FloatDuration;

    fn mul(self, rhs: f64) -> FloatDuration {
        FloatDuration { secs: self.secs * rhs }
    }
}
impl ops::Mul<FloatDuration> for f64 {
    type Output = FloatDuration;

    fn mul(self, rhs: FloatDuration) -> FloatDuration {
        FloatDuration { secs: self * rhs.secs }
    }
}
impl ops::Div<f64> for FloatDuration {
    type Output = FloatDuration;

    fn div(self, rhs: f64) -> FloatDuration {
        FloatDuration { secs: self.secs / rhs }
    }
}
impl ops::Div<FloatDuration> for FloatDuration {
    type Output = f64;

    fn div(self, rhs: FloatDuration) -> f64 {
        self.secs / rhs.secs
    }
}

impl ops::AddAssign<FloatDuration> for FloatDuration {
    fn add_assign(&mut self, rhs: FloatDuration) {
        self.secs += rhs.secs;
    }
}
impl ops::SubAssign<FloatDuration> for FloatDuration {
    fn sub_assign(&mut self, rhs: FloatDuration) {
        self.secs -= rhs.secs;
    }
}

impl ops::MulAssign<f64> for FloatDuration {
    fn mul_assign(&mut self, rhs: f64) {
        self.secs *= rhs;
    }
}
impl ops::DivAssign<f64> for FloatDuration {
    fn div_assign(&mut self, rhs: f64) {
        self.secs /= rhs;
    }
}
impl Default for FloatDuration {
    fn default() -> FloatDuration {
        FloatDuration::zero()
    }
}
impl Sum for FloatDuration {
    fn sum<I>(iter: I) -> FloatDuration
        where I: Iterator<Item = FloatDuration>
    {
        iter.fold(FloatDuration::zero(), |a, b| a + b)
    }
}
impl<'a> Sum<&'a FloatDuration> for FloatDuration {
    fn sum<I>(iter: I) -> FloatDuration
        where I: Iterator<Item = &'a FloatDuration>
    {
        iter.fold(FloatDuration::zero(), |a, &b| a + b)
    }
}

#[cfg(feature = "approx")]
impl ApproxEq for FloatDuration {
    type Epsilon = f64;

    #[inline]
    fn default_epsilon() -> f64 {
        f64::default_epsilon()
    }
    #[inline]
    fn default_max_relative() -> f64 {
        f64::default_max_relative()
    }
    #[inline]
    fn default_max_ulps() -> u32 {
        f64::default_max_ulps()
    }
    #[inline]
    fn relative_eq(&self, other: &FloatDuration, epsilon: f64, max_relative: f64) -> bool {
        self.secs.relative_eq(&other.secs, epsilon, max_relative)
    }
    #[inline]
    fn ulps_eq(&self, other: &FloatDuration, epsilon: f64, max_ulps: u32) -> bool {
        self.secs.ulps_eq(&other.secs, epsilon, max_ulps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time;

    #[test]
    fn test_construct() {
        let duration1 = FloatDuration::hours(3.0);
        assert_eq!(duration1.as_hours(), 3.0);
        assert_eq!(duration1.as_minutes(), 180.0);
        assert_eq!(duration1.as_seconds(), 180.0 * 60.0);
        assert_eq!(duration1.as_days(), 3.0 / 24.0);
        assert_eq!(duration1.as_milliseconds(), 180.0 * 60.0 * 1000.0);
        assert!(duration1.is_positive());

        let duration2 = FloatDuration::milliseconds(55.0);
        assert_eq!(duration2.as_seconds(), 0.055);
        assert_eq!(duration2.as_milliseconds(), 55.0);
        assert_eq!(duration2.as_microseconds(), 55000.0);
        assert_eq!(duration2.as_nanoseconds(), 55000000.0);
        assert!(!duration2.is_zero());

        let duration3 = FloatDuration::zero();
        assert!(duration3.is_zero());
        assert_eq!(duration3.as_minutes(), 0.0);
        assert_eq!(duration3.as_nanoseconds(), 0.0);

        assert_eq!(FloatDuration::days(1.5), FloatDuration::hours(36.0));
        assert_eq!(FloatDuration::minutes(30.0), FloatDuration::hours(0.5));
        assert_eq!(FloatDuration::seconds(180.0), FloatDuration::minutes(3.0));
        assert_eq!(FloatDuration::seconds(3.5),
                   FloatDuration::milliseconds(3500.0));
        assert_eq!(FloatDuration::microseconds(300.0),
                   FloatDuration::milliseconds(0.30));
        assert_eq!(FloatDuration::nanoseconds(1000.0),
                   FloatDuration::microseconds(1.0));

        let duration4 = FloatDuration::minutes(-3.0);
        assert_eq!(duration4.as_minutes(), -3.0);
        assert_eq!(duration4.as_hours(), -0.05);
        assert!(duration4.is_negative());

        assert_eq!(FloatDuration::years(2.0), FloatDuration::days(365.0 * 2.0));
    }

    #[test]
    fn test_arithmetic() {
        assert_eq!(FloatDuration::minutes(5.0) + FloatDuration::seconds(30.0),
                   FloatDuration::seconds(330.0));

        assert_eq!(FloatDuration::hours(3.0) * 2.5, FloatDuration::hours(7.5));

        assert_eq!(FloatDuration::days(3.0) / 3.0 - FloatDuration::hours(2.0),
                   FloatDuration::hours(22.0));

        assert_eq!(FloatDuration::zero() + FloatDuration::milliseconds(500.0) +
                   FloatDuration::microseconds(500.0),
                   FloatDuration::microseconds(500500.0));

        assert_eq!(2.0 * FloatDuration::milliseconds(150.0),
                   FloatDuration::milliseconds(300.0));

        assert_eq!(FloatDuration::minutes(10.0) / FloatDuration::seconds(60.0),
                   10.0);
        assert_eq!(FloatDuration::minutes(5.0),
                   (-FloatDuration::minutes(5.0)) * -1.0);

        assert_eq!(FloatDuration::seconds(10.0) - FloatDuration::minutes(1.0),
                   FloatDuration::seconds(-50.0));

        let inf = FloatDuration::seconds(10.0) / 0.0;

        assert!(inf.as_seconds().is_infinite());
        assert!(inf.as_years().is_infinite());
        assert!(inf.as_microseconds().is_infinite());
        assert!(FloatDuration::hours(10.0) / FloatDuration::minutes(0.0) == f64::INFINITY);
    }

    #[test]
    fn test_std_conversion() {
        let duration1 = FloatDuration::minutes(5.0);
        let std_duration1 = duration1.to_std().unwrap();
        assert!(duration1.is_positive());
        assert_eq!(std_duration1, time::Duration::new(300, 0));
        assert_eq!(FloatDuration::from_std(std_duration1), duration1);

        let duration2 = FloatDuration::hours(-2.0);
        assert!(duration2.is_negative());
        assert!(!duration2.to_std().is_ok());
        let std_duration2 = (-duration2).to_std().unwrap();
        assert_eq!(std_duration2, time::Duration::new(3600 * 2, 0));
        assert_eq!(FloatDuration::from_std(std_duration2), -duration2);

        assert_eq!(FloatDuration::zero().to_std().unwrap(),
                   time::Duration::new(0, 0));
        assert!(FloatDuration::nanoseconds(-1.0).to_std().is_err());
        assert!(FloatDuration::max_value().to_std().is_err());

        assert_eq!(FloatDuration::from_std(time::Duration::new(0, 1)),
                   FloatDuration::nanoseconds(1.0));
        assert_eq!(FloatDuration::from_std(time::Duration::new(1, 1)),
                   FloatDuration::seconds(1.0) + FloatDuration::nanoseconds(1.0));
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", FloatDuration::minutes(3.5)), "3.5 minutes");
        assert_eq!(format!("{}", FloatDuration::days(3.0) + FloatDuration::hours(12.0)),
                   "3.5 days");
        assert_eq!(format!("{}", FloatDuration::seconds(12.7)), "12.7 seconds");
        assert_eq!(format!("{}", FloatDuration::default()), "0 nanoseconds");

        assert_eq!(format!("{}", FloatDuration::microseconds(100.0)),
                   "100 microseconds");
        assert_eq!(format!("{}", FloatDuration::milliseconds(12.5)),
                   "12.5 milliseconds");

        assert_eq!(format!("{}", FloatDuration::days(325.0) + FloatDuration::hours(6.0)),
                   "325.25 days");
        assert_eq!(format!("{}",
                           FloatDuration::milliseconds(50.0) + FloatDuration::microseconds(500.0)),
                   "50.5 milliseconds");

        assert_eq!(format!("{}", FloatDuration::nanoseconds(25.25)),
                   "25.25 nanoseconds");
        assert_eq!(format!("{}", FloatDuration::minutes(90.0)), "1.5 hours");
        assert_eq!(format!("{}", FloatDuration::years(2.5)), "2.5 years");
    }

    #[test]
    fn test_sum() {
        let zero: [FloatDuration; 0] = [];

        assert_eq!(zero.iter().sum::<FloatDuration>(), FloatDuration::zero());

        assert_eq!([FloatDuration::milliseconds(50.0),
                    FloatDuration::milliseconds(30.0),
                    FloatDuration::zero()]
                       .iter()
                       .sum::<FloatDuration>(),
                   FloatDuration::milliseconds(80.0));
        assert_eq!([FloatDuration::days(2.0)].iter().sum::<FloatDuration>(),
                   FloatDuration::days(2.0));
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn test_chrono_conversion() {
        assert_eq!(FloatDuration::from_chrono(chrono::Duration::minutes(10)),
                   FloatDuration::minutes(10.0));
        assert_eq!(FloatDuration::from_chrono(chrono::Duration::hours(72)),
                   FloatDuration::days(3.0));
        assert_eq!(FloatDuration::from_chrono(chrono::Duration::nanoseconds(500)),
                   FloatDuration::nanoseconds(500.0));
        assert_eq!(FloatDuration::from_chrono(chrono::Duration::microseconds(-20000)),
                   FloatDuration::milliseconds(-20.0));
        assert_eq!(FloatDuration::from_chrono(chrono::Duration::zero()),
                   FloatDuration::zero());
        assert_eq!(FloatDuration::from_chrono(chrono::Duration::hours(10000)),
                   FloatDuration::hours(10000.0));
        assert_eq!(FloatDuration::from_chrono(chrono::Duration::milliseconds(1i64 << 62)),
                   FloatDuration::milliseconds((1i64 << 62) as f64));

        assert_eq!(FloatDuration::minutes(2.5).to_chrono().unwrap(),
                   chrono::Duration::seconds(150));
        assert_eq!(FloatDuration::milliseconds(250.050).to_chrono().unwrap(),
                   chrono::Duration::microseconds(250050));
        assert!(FloatDuration::max_value().to_chrono().is_err());
        assert_eq!(FloatDuration::nanoseconds(-20.0).to_chrono().unwrap(),
                   chrono::Duration::nanoseconds(-20));

    }

    #[cfg(feature = "chrono")]
    #[test]
    fn test_chrono_timepoint() {
        use chrono::{TimeZone, UTC, Local};

        let date1 = UTC.ymd(2017, 5, 25).and_hms(10, 0, 0);
        let date2 = UTC.ymd(2017, 5, 26).and_hms(12, 0, 0);

        assert_eq!(date2.float_duration_since(date1).unwrap(),
                   FloatDuration::days(1.0) + FloatDuration::hours(2.0));

        let date3 = Local::now();
        assert_eq!(date3.float_duration_since(date3).unwrap(),
                   FloatDuration::zero());
        assert!(date3.float_duration_since(Local::now()).unwrap() < FloatDuration::zero());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde() {
        use serde_test::{Token, assert_tokens};

        let duration = FloatDuration::seconds(1.5);
        assert_tokens(&duration, &[Token::F64(duration.as_seconds())]);

        assert_tokens(&FloatDuration::zero(), &[Token::F64(0.0)]);

        let duration2 = FloatDuration::hours(3.0);
        assert_tokens(&duration2, &[Token::F64(3.0 * SECS_PER_HOUR)]);

        let duration3 = FloatDuration::days(5.0) + FloatDuration::minutes(35.2) +
                        FloatDuration::milliseconds(100.0);
        assert_tokens(&duration3, &[Token::F64(duration3.as_seconds())]);
    }
}
