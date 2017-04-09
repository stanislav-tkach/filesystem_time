//! TODO: FIXME.

use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};
#[cfg(feature = "try_from")]
use std::convert::TryFrom;

mod get;
mod set;

#[cfg(unix)]
#[path = "imp/unix.rs"]
mod imp;

#[cfg(windows)]
#[path = "imp/windows.rs"]
mod imp;

#[cfg(target_os = "redox")]
#[path = "imp/redox.rs"]
mod imp;

pub use self::get::GetTime;
pub use self::set::SetTime;

pub type Timestamp = u64;

pub fn to_timestamp(time: SystemTime) -> Result<Timestamp, SystemTimeError> {
    Ok(time.duration_since(UNIX_EPOCH)?.as_secs())
}

#[cfg(feature = "try_from")]
impl TryFrom<SystemTime> for Timestamp {
    type Err = SystemTimeError;

    fn try_from(time: SystemTime) -> Result<Self, Self::Err> {
        to_timestamp(time)
    }
}
