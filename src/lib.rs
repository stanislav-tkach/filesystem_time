//! Get/set file time.

#[cfg(feature = "chrono")]
extern crate chrono;
#[cfg(feature = "time")]
extern crate time;

pub mod system_time;
pub mod timestamp;

#[cfg(feature = "chrono")]
pub mod chrono_time;

#[cfg(feature = "time")]
pub mod deprecated_time;
