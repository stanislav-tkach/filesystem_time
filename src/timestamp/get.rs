use std::fs::Metadata;
use std::path::Path;
use std::time::SystemTime;
use super::Timestamp;

/// TODO: FIXME.
pub trait GetTime {
    /// TODO: FIXME.
    fn last_modification(&self) -> Result<Timestamp, ()>;

    /// TODO: FIXME.
    fn last_access(&self) -> Result<Timestamp, ()>;

    /// TODO: FIXME.
    fn creation(&self) -> Result<Timestamp, ()>;
}

impl GetTime for Metadata {
    fn last_modification(&self) -> Result<Timestamp, ()> {
        unimplemented!()
    }

    fn last_access(&self) -> Result<Timestamp, ()> {
        unimplemented!()
    }

    fn creation(&self) -> Result<Timestamp, ()> {
        unimplemented!()
    }
}

impl GetTime for Path {
    fn last_modification(&self) -> Result<Timestamp, ()> {
        unimplemented!()
    }

    fn last_access(&self) -> Result<Timestamp, ()> {
        unimplemented!()
    }

    fn creation(&self) -> Result<Timestamp, ()> {
        unimplemented!()
    }
}

impl GetTime for str {
    fn last_modification(&self) -> Result<Timestamp, ()> {
        unimplemented!()
    }

    fn last_access(&self) -> Result<Timestamp, ()> {
        unimplemented!()
    }

    fn creation(&self) -> Result<Timestamp, ()> {
        unimplemented!()
    }
}

fn convert_time(time: &SystemTime) {
}
