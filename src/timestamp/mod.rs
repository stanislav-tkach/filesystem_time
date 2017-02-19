//! TODO: FIXME.

use std::convert;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

mod get;
mod set;

pub use self::get::GetTime;
pub use self::set::SetTime;

pub type Timestamp = u64;

pub fn to_timestamp(time: SystemTime) -> Result<Timestamp, SystemTimeError> {
    Ok(time.duration_since(UNIX_EPOCH)?.as_secs())
}

#[cfg(feature = "try_from")]
impl convert::TryFrom<SystemTime> for Timestamp {
    type Err = SystemTimeError;

    fn try_from(time: SystemTime) -> Result<Self, Self::Err> {
        to_timestamp(time)
    }
}
