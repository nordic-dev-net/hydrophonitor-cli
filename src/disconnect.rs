use dirs::home_dir;
use std::fs;

use clap::Parser;
use sys_mount::{unmount, UnmountFlags};

#[derive(Parser, Debug)]
#[clap(about = "Disconnects a device")]
pub struct Disconnect {}

impl Disconnect {
    pub fn disconnect(&mut self) {
        let mount_path = home_dir().unwrap().join(".hydrophonitor");
        unmount(&mount_path, UnmountFlags::empty()).expect("Unmounting device failed!");
        fs::remove_dir(&mount_path).expect("Deleting mount folder failed!");
        println!("unmounted previously mounted device at {:?}!", mount_path);
    }
}