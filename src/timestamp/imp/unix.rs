use super::Timestamp;

use std::os::unix::io::AsRawFd;

pub fn set_creation_time(_file: &File, _time: Timestamp) -> Result<(), ()>{
    unimplemented!()
}

pub fn set_last_access_time(_file: &File, _time: Timestamp) -> Result<(), ()> {
    unimplemented!()
}

pub fn set_last_modification_time(_file: &File, _time: Timestamp) -> Result<(), ()> {
    unimplemented!()
}
