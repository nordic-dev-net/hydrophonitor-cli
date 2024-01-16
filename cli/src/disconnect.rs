use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

use clap::Parser;
use lazy_static::lazy_static;
use sys_mount::{unmount, UnmountFlags};

#[derive(Parser, Debug)]
#[clap(about = "Disconnects a device")]
pub struct Disconnect {}

lazy_static! {
static ref MOUNT_PATH: PathBuf = PathBuf::from("/var/lib/hydrophonitor/device");
}


impl Disconnect {
    pub fn disconnect(&mut self) {
        match unmount(&*MOUNT_PATH, UnmountFlags::empty()) {
            Ok(_) => {
                fs::remove_dir(&*MOUNT_PATH).expect("Deleting mount folder failed!");
                println!("unmounted previously mounted device at {:?}!", &*MOUNT_PATH);
            }
            Err(e) => {
                if e.kind() == ErrorKind::NotFound || e.kind() == ErrorKind::InvalidInput {
                    println!("There is currently no device connected!")
                } else {
                    panic!("Unmounting device failed: {}", e)
                }
            }
        }
    }
}