use super::{Timestamp, to_timestamp};
use system_time;

use std::fs::Metadata;
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
    fn last_modification(&self) -> Result<Timestamp>;

    /// TODO: FIXME.
    fn last_access(&self) -> Result<Timestamp>;

    /// TODO: FIXME.
    fn creation(&self) -> Result<Timestamp>;
}

impl GetTime for Metadata {
    fn last_modification(&self) -> Result<Timestamp> {
        Ok(to_timestamp(<Self as system_time::GetTime>::last_modification(self)?)?)
    }

    fn last_access(&self) -> Result<Timestamp> {
        unimplemented!()
    }

    fn creation(&self) -> Result<Timestamp> {
        unimplemented!()
    }
}

impl GetTime for Path {
    fn last_modification(&self) -> Result<Timestamp> {
        unimplemented!()
    }

    fn last_access(&self) -> Result<Timestamp> {
        unimplemented!()
    }

    fn creation(&self) -> Result<Timestamp> {
        unimplemented!()
    }
}

impl GetTime for str {
    fn last_modification(&self) -> Result<Timestamp> {
        unimplemented!()
    }

    fn last_access(&self) -> Result<Timestamp> {
        unimplemented!()
    }

    fn creation(&self) -> Result<Timestamp> {
        unimplemented!()
    }
}

//fn convert_time() -> Result<Timestamp, > {
//
//}
