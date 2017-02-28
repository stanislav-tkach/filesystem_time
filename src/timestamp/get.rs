use super::{Timestamp, to_timestamp};
use system_time;

use std::fs::{Metadata, metadata};
use std::path::Path;
use std::io;
use std::time;

error_chain! {
    foreign_links {
        Io(io::Error);
        SystemTime(time::SystemTimeError);
    }
}

/// TODO: FIXME.
pub trait GetTime {
    /// TODO: FIXME.
    fn creation(&self) -> Result<Timestamp>;

    /// TODO: FIXME.
    fn last_access(&self) -> Result<Timestamp>;

    /// TODO: FIXME.
    fn last_modification(&self) -> Result<Timestamp>;
}

impl GetTime for Metadata {
    fn creation(&self) -> Result<Timestamp> {
        let time = to_timestamp(<Self as system_time::GetTime>::creation(self)?)?;
        Ok(time)
    }

    fn last_access(&self) -> Result<Timestamp> {
        let time = to_timestamp(<Self as system_time::GetTime>::last_access(self)?)?;
        Ok(time)
    }

    fn last_modification(&self) -> Result<Timestamp> {
        let time = to_timestamp(<Self as system_time::GetTime>::last_modification(self)?)?;
        Ok(time)
    }
}

impl GetTime for Path {
    fn creation(&self) -> Result<Timestamp> {
        metadata(self)?.creation()
    }

    fn last_access(&self) -> Result<Timestamp> {
        metadata(self)?.last_access()
    }

    fn last_modification(&self) -> Result<Timestamp> {
        metadata(self)?.last_modification()
    }
}

impl GetTime for str {
    fn creation(&self) -> Result<Timestamp> {
        metadata(self)?.creation()
    }

    fn last_access(&self) -> Result<Timestamp> {
        metadata(self)?.last_access()
    }

    fn last_modification(&self) -> Result<Timestamp> {
        metadata(self)?.last_modification()
    }
}
