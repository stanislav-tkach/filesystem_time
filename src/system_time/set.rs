use std::io::Result;
use std::fs::File;
use std::time::SystemTime;

/// TODO: FIXME.
pub trait SetSystemTime {
    /// TODO: FIXME.
    fn set_last_modification(&mut self, time: &SystemTime) -> Result<()>;

    /// TODO: FIXME.
    fn set_last_access(&mut self, time: &SystemTime) -> Result<()>;

    /// TODO: FIXME.
    fn set_creation(&mut self, time: &SystemTime) -> Result<()>;
}

impl SetSystemTime for File {
    fn set_last_modification(&mut self, time: &SystemTime) -> Result<()> {
        unimplemented!()
    }

    fn set_last_access(&mut self, time: &SystemTime) -> Result<()> {
        unimplemented!()
    }

    fn set_creation(&mut self, time: &SystemTime) -> Result<()> {
        unimplemented!()
    }
}
