use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::Command;

use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use log::debug;
use sys_mount::{Mount, unmount, UnmountDrop, UnmountFlags};

lazy_static! {
pub static ref MOUNT_PATH: PathBuf = PathBuf::from("/var/lib/hydrophonitor/device");
static ref TEMP_MOUNT_PATH: PathBuf = PathBuf::from("/var/lib/hydrophonitor/temp_device");
}

//gets all available devices with lsblk
pub fn get_device_list() -> Result<Vec<String>> {
    let output = Command::new("lsblk").arg("-l").output().with_context(|| "Failed to run lsblk!")?;
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
    Ok(devices_cropped)
}

pub fn find_suitable_device(devices: &[String]) -> Result<Option<&String>> {
    create_dir_if_not_existing(&TEMP_MOUNT_PATH);

    //Checking all devices for an output directory
    for device in devices.iter() {
        let device_path = format!("/dev/{device}");
        match Mount::builder()
            .mount_autodrop(&device_path, &*TEMP_MOUNT_PATH, UnmountFlags::DETACH) {
            Ok(_) =>
                {
                    let read_dir_result = fs::read_dir(format!("{}/output", TEMP_MOUNT_PATH.to_str().unwrap()));
                    if read_dir_result.is_ok() {
                        return Ok(Some(device));
                    }
                }
            Err(e) => {
                if e.kind() == ErrorKind::PermissionDenied {
                    return Err(anyhow!("Please execute the command with sudo rights or specify a device path with access rights!"));
                }
                debug!("Mount of device {device_path} failed: {e}")
            }
        }
    }
    Ok(None)
}

pub fn mount_device(device: &String) -> Result<UnmountDrop<Mount>> {
    if unmount(&*MOUNT_PATH, UnmountFlags::empty()).is_ok() { debug!("unmounting previously mounted device at {:?}", &*MOUNT_PATH) }
    create_dir_if_not_existing(&MOUNT_PATH);

    let device_path = format!("/dev/{device}");
    Mount::builder().mount_autodrop(device_path, &*MOUNT_PATH, UnmountFlags::DETACH).with_context(|| "Mount failed")
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