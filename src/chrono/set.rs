use std::fs::File;

/// TODO: FIXME.
pub trait SetChronoTime {
    /// TODO: FIXME.
    fn set_last_modification(&mut self, time: u32) -> Result<(), ()>;

    /// TODO: FIXME.
    fn set_last_access(&mut self, time: u32) -> Result<(), ()>;

    /// TODO: FIXME.
    fn set_creation(&mut self, time: u32) -> Result<(), ()>;
}

impl SetChronoTime for File {
    fn set_last_modification(&mut self, _time: u32) -> Result<(), ()> {
        unimplemented!()
    }

    fn set_last_access(&mut self, _time: u32) -> Result<(), ()> {
        unimplemented!()
    }

    fn set_creation(&mut self, _time: u32) -> Result<(), ()> {
        unimplemented!()
    }
}
