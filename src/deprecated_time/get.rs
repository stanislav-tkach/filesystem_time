use std::fs::Metadata;
use std::path::Path;

/// TODO: FIXME.
pub trait GetTime {
    /// TODO: FIXME.
    fn last_modification(&self) -> Result<(), ()>;

    /// TODO: FIXME.
    fn last_access(&self) -> Result<(), ()>;

    /// TODO: FIXME.
    fn creation(&self) -> Result<(), ()>;
}

impl GetTime for Metadata {
    fn last_modification(&self) -> Result<(), ()> {
        unimplemented!()
    }

    fn last_access(&self) -> Result<(), ()> {
        unimplemented!()
    }

    fn creation(&self) -> Result<(), ()> {
        unimplemented!()
    }
}

impl GetTime for Path {
    fn last_modification(&self) -> Result<(), ()> {
        unimplemented!()
    }

    fn last_access(&self) -> Result<(), ()> {
        unimplemented!()
    }

    fn creation(&self) -> Result<(), ()> {
        unimplemented!()
    }
}

impl GetTime for str {
    fn last_modification(&self) -> Result<(), ()> {
        unimplemented!()
    }

    fn last_access(&self) -> Result<(), ()> {
        unimplemented!()
    }

    fn creation(&self) -> Result<(), ()> {
        unimplemented!()
    }
}
