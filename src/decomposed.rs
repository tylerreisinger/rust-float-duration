use std::fmt;

/// A duration decomposed into components.
///
/// `DecomposedTime` is mainly provided for a more human-readable and composable
/// representation of a `FloatDuration`. It may be converted back-and-forth
/// between `FloatDuration` at will.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DecomposedTime {
    pub days: u32,
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub fractional_seconds: f64,
    pub sign: i8,
}

impl DecomposedTime {
    pub fn zero() -> DecomposedTime {
        DecomposedTime {
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            fractional_seconds: 0.0,
            sign: 1,
        }
    }

    pub fn from_components(days: u32,
                           hours: u32,
                           minutes: u32,
                           seconds: u32,
                           fractional_seconds: f64)
                           -> DecomposedTime {
        DecomposedTime {
            days,
            hours,
            minutes,
            seconds,
            fractional_seconds,
            sign: 1,
        }
    }

    pub fn negate(mut self) -> DecomposedTime {
        self.sign *= -1;
        self
    }
}

impl fmt::Display for DecomposedTime {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if self.days > 0 {
            write!(fmt, "{}d ", self.days as u64)?;
        }

        if self.sign.is_negative() {
            write!(fmt, "-")?;
        }
        if self.fractional_seconds > 0.0 {
            write!(fmt,
                   "{:02}:{:02}:{}{}",
                   self.hours,
                   self.minutes,
                   if self.seconds < 10 { "0" } else { "" },
                   self.seconds as f64 + self.fractional_seconds)
        } else {
            write!(fmt,
                   "{:02}:{:02}:{:02}",
                   self.hours,
                   self.minutes,
                   self.seconds)
        }
    }
}
