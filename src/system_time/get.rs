use std::io::Result;
use std::fs::{Metadata, metadata};
use std::time::SystemTime;
use std::path::Path;

/// Provided for consistancy with other traits.
/// Behaves the same as corresponding `std::fs::Metadata` methods.
pub trait GetSystemTime {
    /// TODO: FIXME.
    fn last_modification(&self) -> Result<SystemTime>;

    /// TODO: FIXME.
    fn last_access(&self) -> Result<SystemTime>;

    /// TODO: FIXME.
    fn creation(&self) -> Result<SystemTime>;
}

impl GetSystemTime for Metadata {
    fn last_modification(&self) -> Result<SystemTime> {
        self.modified()
    }

    fn last_access(&self) -> Result<SystemTime> {
        self.accessed()
    }

    fn creation(&self) -> Result<SystemTime> {
        self.created()
    }
}

impl GetSystemTime for Path {
    fn last_modification(&self) -> Result<SystemTime> {
        metadata(self)?.modified()
    }

    fn last_access(&self) -> Result<SystemTime> {
        metadata(self)?.accessed()
    }

    fn creation(&self) -> Result<SystemTime> {
        metadata(self)?.created()
    }
}

impl GetSystemTime for str {
    fn last_modification(&self) -> Result<SystemTime> {
        metadata(self)?.modified()
    }

    fn last_access(&self) -> Result<SystemTime> {
        metadata(self)?.accessed()
    }

    fn creation(&self) -> Result<SystemTime> {
        metadata(self)?.created()
    }
}
