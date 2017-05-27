//! Utilities for iteration with duration objects.

use duration::FloatDuration;
use std::iter;

/// An iterator over an evenly spaced lattice of `FloatDuration`s.
///
/// This type is returned by `subdivide` and friends, and it not meant to be
/// instantiated directly.
#[derive(Debug, Clone)]
pub struct Subdivide {
    start: FloatDuration,
    step_size: FloatDuration,
    len: usize,
    index: usize,
}

impl Subdivide {
    fn new(start: FloatDuration, end: FloatDuration, steps: usize) -> Subdivide {
        assert!(steps >= 2, "subdivide requires at least two steps");
        let step_size = (end - start) / (steps - 1) as f64;

        Subdivide {
            start: start,
            step_size: step_size,
            len: steps,
            index: 0,
        }
    }

    /// The distance between steps in the iteration.
    pub fn step_size(&self) -> FloatDuration {
        self.step_size
    }
}

impl Iterator for Subdivide {
    type Item = FloatDuration;

    #[inline]
    fn next(&mut self) -> Option<FloatDuration> {
        if self.index >= self.len {
            None
        } else {
            let index = self.index;
            self.index += 1;
            Some(self.start + self.step_size * (index as f64))
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let left = self.len - self.index;
        (left, Some(left))
    }
}

impl DoubleEndedIterator for Subdivide {
    fn next_back(&mut self) -> Option<FloatDuration> {
        if self.index >= self.len {
            None
        } else {
            self.len -= 1;
            let index = self.len;
            Some(self.start + self.step_size * (index as f64))
        }
    }
}

impl ExactSizeIterator for Subdivide {}

/// Subdivide the distance between two duration into `steps` evenly spaced points.
///
/// `subdivide` returns an iterator that lazily computes and returns exactly `steps`
/// evenly spaced
/// points between `begin` and `end`. This iterator is *inclusive* in that it
/// returns `begin` as the first element and `end` as the final element.
///
/// The returned iterator [`Subdivide`](struct.Subdivide.html) implements
/// `DoubleEndedIterator`, and thus can be reversed or consumed from both sides.
///
/// ```rust
/// use float_duration::FloatDuration;
/// use float_duration::iter::subdivide;
///
/// fn cost_function(t: &FloatDuration) -> f64 {
///     return 0.5*t.as_seconds()*t.as_seconds()
/// }
/// fn main() {
///     let start = FloatDuration::zero();
///     let end = FloatDuration::minutes(10.0);
///     let total: f64 = subdivide(start, end, 100).map(|x| cost_function(&x)).sum();
/// }
/// ```
///
/// # Panics
/// This function panics if `steps < 2` as this would violate the property
/// that the iterator visits both endpoints.
pub fn subdivide(begin: FloatDuration, end: FloatDuration, steps: usize) -> Subdivide {
    Subdivide::new(begin, end, steps)
}

/// Subdivide the distance between two duration into `steps` evenly spaced points
/// and include a timestep.
///
/// `subdivide_with_step` is equivalent to [`subdivide`](fn.subdivide.html)
/// except that it returns the
/// step size with the current time in each iteration. It is mainly a convenience
/// function for the common case of running a simulation over discrete time steps.
///
/// It is exactly equivalent to:
///
/// ```rust
/// # use float_duration::FloatDuration;
/// # use float_duration::iter::subdivide;
/// use std::iter;
///
/// let steps = 100;
/// # let begin = FloatDuration::zero();
/// # let end = FloatDuration::minutes(5.0);
///
/// let sub = subdivide(begin, end, steps);
/// let step_size = sub.step_size();
/// let my_iter = sub.zip(iter::repeat(step_size));
/// ```
///
/// Example usage in a simulation:
///
/// ```rust
/// use float_duration::FloatDuration;
/// use float_duration::iter::subdivide_with_step;
///
/// let start = FloatDuration::zero();
/// let end = FloatDuration::hours(1.0);
///
/// let mut x = 5.0;
/// let mut v = 0.0;
///
/// for (t, dt) in subdivide_with_step(start, end, 100) {
///      let a = x*x - v*x;
///      let v = a*dt.as_seconds();
///      let x = v*dt.as_seconds();
///
///      println!("Position: {}", x);
/// }
/// ```
///
/// # Panics
/// This function panics if `steps < 2` as this would violate the property
/// that the iterator visits both endpoints.
///
pub fn subdivide_with_step(begin: FloatDuration,
                           end: FloatDuration,
                           steps: usize)
                           -> iter::Zip<Subdivide, iter::Repeat<FloatDuration>> {
    let sub = subdivide(begin, end, steps);
    let step_size = sub.step_size();
    sub.zip(iter::repeat(step_size))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subdivide() {
        let s = subdivide(FloatDuration::zero(), FloatDuration::minutes(1.0), 3);
        let s_rev = s.clone().rev();
        assert_eq!(s.collect::<Vec<_>>(),
                   vec![FloatDuration::zero(),
                        FloatDuration::seconds(30.0),
                        FloatDuration::minutes(1.0)]);
        assert_eq!(s_rev.collect::<Vec<_>>(),
                   vec![FloatDuration::minutes(1.0),
                        FloatDuration::seconds(30.0),
                        FloatDuration::zero()]);

    }
}
