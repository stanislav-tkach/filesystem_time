use std::io::Result;
use std::fs::{Metadata, metadata};
use std::path::Path;
use std::time::SystemTime;

/// Provided for consistency with other traits.
/// Behaves the same as corresponding `std::fs::Metadata` methods.
pub trait GetTime {
    /// TODO: FIXME.
    fn creation(&self) -> Result<SystemTime>;

    /// TODO: FIXME.
    fn last_access(&self) -> Result<SystemTime>;

    /// TODO: FIXME.
    fn last_modification(&self) -> Result<SystemTime>;
}

impl GetTime for Metadata {
    fn creation(&self) -> Result<SystemTime> {
        self.created()
    }

    fn last_access(&self) -> Result<SystemTime> {
        self.accessed()
    }

    fn last_modification(&self) -> Result<SystemTime> {
        self.modified()
    }
}

impl GetTime for Path {
    fn creation(&self) -> Result<SystemTime> {
        metadata(self)?.creation()
    }

    fn last_access(&self) -> Result<SystemTime> {
        metadata(self)?.last_access()
    }

    fn last_modification(&self) -> Result<SystemTime> {
        metadata(self)?.last_modification()
    }
}

impl GetTime for str {
    fn creation(&self) -> Result<SystemTime> {
        metadata(self)?.creation()
    }

    fn last_access(&self) -> Result<SystemTime> {
        metadata(self)?.last_access()
    }

    fn last_modification(&self) -> Result<SystemTime> {
        metadata(self)?.last_modification()
    }
}
