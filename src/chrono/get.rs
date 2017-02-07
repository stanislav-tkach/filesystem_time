use std::fs::Metadata;
use std::path::Path;

/// Provided for consistancy with other traits.
/// Behaves the same as corresponding `std::fs::Metadata` methods.
pub trait GetChronoTime {
    /// TODO: FIXME.
    fn last_modification(&self) -> Result<(), ()>;

    /// TODO: FIXME.
    fn last_access(&self) -> Result<(), ()>;

    /// TODO: FIXME.
    fn creation(&self) -> Result<(), ()>;
}

impl GetChronoTime for Metadata {
    fn last_modification(&self) -> Result<(), ()> {
        unimplemented!()
    }

    fn last_access(&self) -> Result<SystemTime> {
        unimplemented!()
    }

    fn creation(&self) -> Result<SystemTime> {
        unimplemented!()
    }
}

impl GetChronoTime for Path {
    fn last_modification(&self) -> Result<SystemTime> {
        unimplemented!()
    }

    fn last_access(&self) -> Result<SystemTime> {
        unimplemented!()
    }

    fn creation(&self) -> Result<SystemTime> {
        unimplemented!()
    }
}

impl GetChronoTime for str {
    fn last_modification(&self) -> Result<SystemTime> {
        unimplemented!()
    }

    fn last_access(&self) -> Result<SystemTime> {
        unimplemented!()
    }

    fn creation(&self) -> Result<SystemTime> {
        unimplemented!()
    }
}
