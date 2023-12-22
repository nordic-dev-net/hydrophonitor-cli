use std::fs;
use std::io::ErrorKind;

use clap::Parser;
use dirs::home_dir;
use sys_mount::{unmount, UnmountFlags};

#[derive(Parser, Debug)]
#[clap(about = "Disconnects a device")]
pub struct Disconnect {}

impl Disconnect {
    pub fn disconnect(&mut self) {
        let mount_path = home_dir().unwrap().join(".hydrophonitor");
        match unmount(&mount_path, UnmountFlags::empty()) {
            Ok(_) => {
                fs::remove_dir(&mount_path).expect("Deleting mount folder failed!");
                println!("unmounted previously mounted device at {:?}!", mount_path);
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