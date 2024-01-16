use std::{fs, process};
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::Command;

use lazy_static::lazy_static;
use log::debug;
use sys_mount::{Mount, Unmount, unmount, UnmountFlags};

lazy_static! {
static ref MOUNT_PATH: PathBuf = PathBuf::from("/var/lib/hydrophonitor/device");
static ref TEMP_MOUNT_PATH: PathBuf = PathBuf::from("/var/lib/hydrophonitor/temp_device");
}

//gets all available devices with lsblk
pub fn get_device_list() -> Vec<String> {
    let output = Command::new("lsblk").arg("-l").output().expect("Failed to run lsblk!");
    let output = String::from_utf8_lossy(&output.stdout);
    let devices: Vec<&str> = output.lines().collect();
    let mut devices_cropped: Vec<String> = Vec::new();
    for device in devices.iter() {
        let mut device_columns = device.split_whitespace();
        let cropped_device = device_columns.next().unwrap_or_default().to_string();
        let device_type = device_columns.nth(4);
        if device_type.unwrap_or_default() == "part" {
            devices_cropped.push(cropped_device);
        }
    }
    devices_cropped
}

pub fn find_suitable_device(devices: &Vec<String>) -> Option<&String> {
    create_dir_if_not_existing(&*TEMP_MOUNT_PATH);

    //Checking all devices for an output directory
    for device in devices.iter() {
        let device_path = format!("/dev/{device}");
        match Mount::builder()
            .mount_autodrop(&device_path, &*TEMP_MOUNT_PATH, UnmountFlags::DETACH) {
            Ok(_) =>
                {
                    let read_dir_result = fs::read_dir(format!("{}/output", TEMP_MOUNT_PATH.to_str().unwrap()));
                    match read_dir_result {
                        Ok(_) => { return Some(device); }
                        Err(_) => {}
                    }
                }
            Err(e) => {
                if e.kind() == ErrorKind::PermissionDenied {
                    println!("Please execute the connect command with sudo rights!");
                    process::exit(1)
                }
                debug!("Mount of device {device_path} failed: {e}")
            }
        }
    }
    None
}


pub fn mount_device(device: &String) {
    match unmount(&*MOUNT_PATH, UnmountFlags::empty()) {
        Ok(_) => debug!("unmounting previously mounted device at {:?}", &*MOUNT_PATH),
        Err(_) => {}
    }
    create_dir_if_not_existing(&*MOUNT_PATH);

    let device_path = format!("/dev/{device}");
    let mount = Mount::builder().mount(&device_path, &*MOUNT_PATH).expect("Mount failed");

    match fs::read_dir(format!("{}/output", MOUNT_PATH.to_str().unwrap())) {
        Ok(_) => {
            println!("successfully connected to device {device}!")
        }
        Err(_) => {
            println! {"The selected device does not have a valid output directory!"};
            mount.into_unmount_drop(UnmountFlags::DETACH);
            return;
        }
    }
}


fn create_dir_if_not_existing(dir_path: &PathBuf) {
    match fs::create_dir_all(dir_path) {
        Ok(_) => {}
        Err(e) => {
            if e.kind() != ErrorKind::AlreadyExists {
                panic!("{}", e)
            }
        }
    }
}