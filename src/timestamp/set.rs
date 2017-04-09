use super::{imp, Timestamp};

use std::fs::File;

/// TODO: FIXME.
pub trait SetTime {
    /// TODO: FIXME.
    fn set_creation(&mut self, time: Timestamp) -> Result<(), ()>;

    /// TODO: FIXME.
    fn set_last_access(&mut self, time: Timestamp) -> Result<(), ()>;

    /// TODO: FIXME.
    fn set_last_modification(&mut self, time: Timestamp) -> Result<(), ()>;
}

impl SetTime for File {
    fn set_creation(&mut self, time: Timestamp) -> Result<(), ()> {
        imp::set_creation_time(&*self, time)
    }

    fn set_last_access(&mut self, time: Timestamp) -> Result<(), ()> {
        imp::set_last_access_time(self, time)
    }

    fn set_last_modification(&mut self, time: Timestamp) -> Result<(), ()> {
        imp::set_last_modification_time(self, time)
    }
}
