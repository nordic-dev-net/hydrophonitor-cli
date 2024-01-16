use std::{fs, process};
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::Command;

use dirs::home_dir;
use log::debug;
use sys_mount::{Mount, Unmount, unmount, UnmountFlags};

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
    let mount_path = home_dir().unwrap().join(".temp_hydrophonitor");
    create_dir_if_not_existing(&mount_path);

    //Checking all devices for an output directory
    for device in devices.iter() {
        let device_path = format!("/dev/{device}");
        match Mount::builder()
            .mount_autodrop(&device_path, &mount_path, UnmountFlags::DETACH) {
            Ok(_) =>
                {
                    let read_dir_result = fs::read_dir(format!("{}/output", mount_path.to_str().unwrap()));
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
    let mount_path = home_dir().unwrap().join(".hydrophonitor");

    match unmount(&mount_path, UnmountFlags::empty()) {
        Ok(_) => debug!("unmounting previously mounted device at {:?}", mount_path),
        Err(_) => {}
    }
    create_dir_if_not_existing(&mount_path);

    let device_path = format!("/dev/{device}");
    let mount = Mount::builder().mount(&device_path, &mount_path).expect("Mount failed");

    match fs::read_dir(format!("{}/output", mount_path.to_str().unwrap())) {
        Ok(_) => println!("successfully connected to device {device}!"),
        Err(_) => {
            println! {"The selected device does not have a valid output directory!"};
            mount.into_unmount_drop(UnmountFlags::DETACH);
            return;
        }
    }
}


fn create_dir_if_not_existing(dir_path: &PathBuf) {
    match fs::create_dir(dir_path) {
        Ok(_) => {}
        Err(e) => {
            if e.kind() != ErrorKind::AlreadyExists {
                panic!("{}", e)
            }
        }
    }
}
