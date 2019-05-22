[float_duration](https://docs.rs/float_duration) 0.3.3
======================
[![Build Status](https://travis-ci.org/tylerreisinger/rust-float-duration.svg?branch=master)](https://travis-ci.org/tylerreisinger/rust-float-duration)
[![float_duration on docs.rs][docsrs-image]][docsrs]
[![float_duration on crates.io][crates-image]][crates]

[docsrs-image]: https://docs.rs/float_duration/badge.svg?version=0.3.3
[docsrs]: https://docs.rs/float_duration/0.3.3
[crates-image]: https://img.shields.io/crates/v/float_duration.svg
[crates]: https://crates.io/crates/float_duration

Temporal quantification using floating-point quantities.

This crate provides quantification for spans of time. Unlike the standard
[`std::time`](https://doc.rust-lang.org/std/time/index.html) or the
[chrono](https://crates.io/crates/chrono) crate, this crate aims
to provide a fully featured Duration type which exposes and uses floating-point
values instead of integer ones.

The existing solutions for time spans are high precision and good for displaying
and storing time values, but they are less than convenient to use in simulations or
mathematical contexts where a single fractional value in some unit is needed. Thus,
this crate was developed in order to fill that need while being maximally compatible
with the existing time libraries and not re-inventing the wheel. The interface aims to
be very similar to these libraries, just using floating point values so it may be
used as a near drop-in replacement for existing programs that
would benefit from floating point durations.

The goal is to provide a type with the same set of features and flexibility as
`std::chrono::duration` in C++ while being "Rusty" in design.

# Usage
Put this in your `Cargo.toml`:

```
[dependencies]
float_duration = "0.3.3"
```
# Overview

This crate provides a single primary type:
[`FloatDuration`](duration/struct.FloatDuration.html) which represents an
arbitrary distance in time with no defined start or end point.
Internally, it stores a single `f64` holding the number of seconds the duration
represents, which can be negative for a "backward" duration. It provides accessors
methods to create and read the value in various units, as well as `impl`s for many
arithmetic operators in `std::ops`.

```rust
let timespan = FloatDuration::hours(2.5) + FloatDuration::seconds(30.0);
assert_eq!(timespan, FloatDuration::seconds(9030.0));
assert_eq!(timespan, FloatDuration::minutes(150.5));
```
Additionally, a [`TimePoint`](duration/trait.TimePoint.html) trait is provided
for computing a `FloatDuration` between two objects representing a point in time.

# Example Usage

Compute the number of blocks in a larger interval:

```rust
use float_duration::FloatDuration;

let time_block = FloatDuration::minutes(5.0);
let blocks_per_hour = FloatDuration::hours(1.0) / time_block;

assert_eq!(blocks_per_hour, 12.0);
```

Perform a basic numerical integration of a mass on a spring:

```rust
use float_duration::FloatDuration;

fn acceleration(m: f64, x: f64, t: FloatDuration) -> f64 {
    0.5*m*x*x
}

fn main() {
    let mut sim_time = FloatDuration::zero();
    let end_time = FloatDuration::minutes(2.0);
    let dt = FloatDuration::milliseconds(50.0);

    let mut x = 2.0;
    let mut v = 0.0;
    let m = 1.0;

    while sim_time < end_time {
        let acc = acceleration(m, x, sim_time);
        v += acc*dt.as_seconds();
        x += v*dt.as_seconds();
        sim_time += dt;
    }
}
```

Run a function on 100 evenly spaced durations:

```rust
use float_duration::{FloatDuration, subdivide};

fn compute_value(t: FloatDuration) -> f64 {
    t.as_seconds()*t.as_seconds()
}

for time in subdivide(FloatDuration::zero(), FloatDuration::hours(1.0), 100) {
    println!("{}", compute_value(time));
}
```

# Library Support

Currently `float_duration` can be compiled without any dependencies, but it
provides optional features for interfacing with other libraries.

## std::time
The `std::time` module is supported and `FloatDuration`
can be used directly with `SystemTime` and `Instant`:

```rust
// TimePoint needed for `float_duration_since`.
use float_duration::{FloatDuration, TimePoint};
use std::time::{Instant, SystemTime};


let start_time = Instant::now();
// Do lengthy operation...
let end_time = Instant::now();

println!("Took {}.", end_time.float_duration_since(start_time).unwrap());
```

`FloatDuration` may also be converted to/from `std::time::Duration` via the
`to_std` and `from_std` methods.

## [approx](https://crates.io/crates/approx)
`FloatDuration` provides an implementation of `approx::ApproxEq`
for near-equality comparisons of `FloatDuration` if the `approx` feature is enabled.
Since `FloatDuration` uses floating point values, this should be the
preferred way to establish equality between two duration objects.

## [chrono](https://crates.io/crates/chrono)

Similar to `std::time`, computing a `FloatDuration` between any two of the same type of
date or time objects
is supported via `TimePoint` trait impls, assuming the feature
"chrono" is enabled.
Additionally, `FloatDuration` objects can be converted to/from
`chrono::Duration` objects via the `to_chrono` and `from_chrono` methods.

Note: if the `chrono` feature is enabled, the `time` feature must also be enabled
as `chrono` directly relies on some types defined in `time`.

```rust
use chrono::{UTC, TimeZone};
use float_duration::{TimePoint, FloatDuration};

let date1 = UTC.ymd(2017, 5, 25).and_hms(10, 0, 0);
let date2 = UTC.ymd(2017, 5, 26).and_hms(12, 0, 0);

assert_eq!(date2.float_duration_since(date1).unwrap(), FloatDuration::days(1.0) +
    FloatDuration::hours(2.0));
```

## [serde](https://crates.io/crates/serde)

`FloatDuration` supports serialization with `serde`. Presently, a `FloatDuration`
is serialized to a single `f64` value representing the number of seconds in the
duration.
