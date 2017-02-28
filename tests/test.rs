extern crate filesystem_time;
extern crate tempdir;
extern crate chrono;
extern crate time;

use filesystem_time::system_time::GetTime as SystemGet;
use filesystem_time::system_time::SetTime as SystemSet;
use filesystem_time::timestamp::GetTime as TimestampGet;
use filesystem_time::timestamp::SetTime as TimestampSet;
use filesystem_time::chrono_time::GetTime as ChronoGet;
use filesystem_time::chrono_time::SetTime as ChronoSet;
use filesystem_time::deprecated_time::GetTime as TimeGet;
use filesystem_time::deprecated_time::SetTime as TimeSet;
use filesystem_time::timestamp::to_timestamp;

use tempdir::TempDir;

use std::fs::{File, Metadata};
use std::path::Path;
use std::error::Error;

#[test]
fn test() {
    let temp_dir = TempDir::new("time_check_dir").expect("Unable to create temporary directory");
    let path = temp_dir.path().join("time_check_file");
    let mut file = File::create(&path).expect("Unable to create temp file");

    check(&path);
}

fn check(path: &Path) {
    let metadata = path.metadata().expect("Unable to get metadata for the temp file");

    check_created(&metadata);
}

fn check_created(metadata: &Metadata) {
    let time = match metadata.created() {
        Ok(val) => val,
        Err(err) => {
            println!("Unable to get creation time: {}", err.description());

            assert!(<Metadata as SystemGet>::creation(&metadata).is_err());
            assert!(<Metadata as TimestampGet>::creation(&metadata).is_err());
            assert!(<Metadata as ChronoGet>::creation(&metadata).is_err());
            assert!(<Metadata as TimeGet>::creation(&metadata).is_err());

            return;
        }
    };

    assert_eq!(time, <Metadata as SystemGet>::creation(&metadata).unwrap());

    let timestamp = to_timestamp(time).unwrap();
    assert_eq!(timestamp, <Metadata as TimestampGet>::creation(&metadata).unwrap());
    //assert_eq!(timestamp, <Metadata as ChronoGet>::creation(&metadata).unwrap());
}

//fn check_path(path: &Path) {
//    let metadata = path.metadata().expect("Unable to get metadata for the temp file");
//
//    let standard_time = metadata.created();
//}
//
//fn check_path_creation(path: &Path) {
//    //let standard_time = path.metadata().
//    //let system_time = <Path as system_time::GetTime>::created(&path);
//}
//
//fn check_path_access(path: &Path) {}
//
//fn check_path_modification(path: &Path) {}
//
//fn check_file(file: &File) {}
//
//fn check_file_creation(path: &Path) {}
//
//fn check_file_access(path: &Path) {}
//
//fn check_file_modification(path: &Path) {}
