extern crate nix;

use std::{fs, io, process};
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::Command;

use clap::Parser;
use dialoguer::Select;
use dirs::home_dir;
use log::debug;
use nix::mount;
use sys_mount::{Mount, unmount, UnmountFlags};

#[derive(Parser, Debug)]
#[clap(about = "Connects to a device")]
pub struct Connect {}

impl Connect {
    pub fn connect(&mut self) {
        let devices = get_device_list();
        let mut selected_device = &String::new();
        match find_suitable_device(&devices) {
            Some(dev) => {
                println!("Found potential Hydrophonitor device {dev}. Do you want to connect to this device? (y/n)");
                let mut user_input = String::new();
                io::stdin().read_line(&mut user_input).expect("Failed to read line!");
                match user_input.trim().to_lowercase().as_str() {
                    "y" | "yes" => selected_device = dev,
                    "n" | "no" => selected_device = manual_connect(&devices),
                    _ => println!("Invalid response. Please enter 'y' or 'n'."),
                }
            }
            None => {
                println!("No device found matching the Hydrophonitor disk.");
                selected_device = manual_connect(&devices);
            }
        }

        mount_device(selected_device);
    }
}

//gets all available devices with lsblk
fn get_device_list() -> Vec<String> {
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

fn find_suitable_device(devices: &Vec<String>) -> Option<&String> {
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

fn manual_connect(devices: &Vec<String>) -> &String {
    let selection = Select::new()
        .with_prompt("Please choose a device from the list:")
        .items(&devices)
        .default(0)
        .interact();
    &devices[selection.unwrap_or_default()]
}

fn mount_device(device: &String) {
    let mount_path = home_dir().unwrap().join(".hydrophonitor");
    let device_path = PathBuf::from(format!("/dev/{device}"));

    //TODO unmount device if already mounted
    create_dir_if_not_existing(&mount_path);

    mount::mount(
        Some(&device_path),
        &mount_path,
        Some("ext4"),
        mount::MsFlags::empty(),
        None::<&str>,
    ).expect("Failed to mount the device");

    match fs::read_dir(format!("{}/output", mount_path.to_str().unwrap())) {
        Ok(_) => println!("successfully connected to device {device}!"),
        Err(_) => {
            println! {"The selected device does not have a valid output directory!"};
            unmount_device(&device_path);
            return;
        }
    }
}

fn unmount_device(path: &PathBuf) {
    //TODO convert to static file wide variable
    unmount(&path, UnmountFlags::empty()).expect("Failed to unmount device!");
}

fn create_dir_if_not_existing(dir_path: &PathBuf) {
    match fs::create_dir(dir_path) {
        Ok(_) => {}
        Err(e) => {
            if e.kind() != io::ErrorKind::AlreadyExists {
                panic!("{}", e)
            }
        }
    }
}