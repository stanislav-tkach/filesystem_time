use std::fs::Metadata;
use std::path::Path;

/// TODO: FIXME.
pub trait GetTime {
    /// TODO: FIXME.
    fn creation(&self) -> Result<(), ()>;

    /// TODO: FIXME.
    fn last_access(&self) -> Result<(), ()>;

    /// TODO: FIXME.
    fn last_modification(&self) -> Result<(), ()>;
}

impl GetTime for Metadata {
    fn creation(&self) -> Result<(), ()> {
        unimplemented!()
    }

    fn last_access(&self) -> Result<(), ()> {
        unimplemented!()
    }

    fn last_modification(&self) -> Result<(), ()> {
        unimplemented!()
    }
}

impl GetTime for Path {
    fn creation(&self) -> Result<(), ()> {
        unimplemented!()
    }

    fn last_access(&self) -> Result<(), ()> {
        unimplemented!()
    }

    fn last_modification(&self) -> Result<(), ()> {
        unimplemented!()
    }
}

impl GetTime for str {
    fn creation(&self) -> Result<(), ()> {
        unimplemented!()
    }

    fn last_access(&self) -> Result<(), ()> {
        unimplemented!()
    }

    fn last_modification(&self) -> Result<(), ()> {
        unimplemented!()
    }
}
