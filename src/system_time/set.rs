use std::io::Result;
use std::fs::File;
use std::time::SystemTime;

/// TODO: FIXME.
pub trait SetTime {
    /// TODO: FIXME.
    fn set_creation(&mut self, time: &SystemTime) -> Result<()>;

    /// TODO: FIXME.
    fn set_last_access(&mut self, time: &SystemTime) -> Result<()>;

    /// TODO: FIXME.
    fn set_last_modification(&mut self, time: &SystemTime) -> Result<()>;
}

impl SetTime for File {
    fn set_creation(&mut self, _time: &SystemTime) -> Result<()> {
        unimplemented!()
    }

    fn set_last_access(&mut self, _time: &SystemTime) -> Result<()> {
        unimplemented!()
    }

    fn set_last_modification(&mut self, _time: &SystemTime) -> Result<()> {
        unimplemented!()
    }
}
