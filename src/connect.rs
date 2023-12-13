use std::{fs, io};
use std::fs::File;
use std::io::Write;
use std::process::Command;

use clap::Parser;
use dialoguer::Select;
use dirs::home_dir;

#[derive(Parser, Debug)]
#[clap(about = "Connects to a device")]
pub struct Connect {}

impl Connect {
    pub fn connect(&mut self) {
        let devices = get_device_list();
        let mut selected_device = &String::new();
        for device in devices.iter() {
            if device.to_lowercase().contains("nixos") {
                selected_device = device;
                break;
            }
        }
        if !selected_device.is_empty() {
            println!("Found potential Hydrophonitor device {selected_device}. Do you want to connect to this device? (y/n)");
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).expect("Failed to read line!");
            match user_input.trim().to_lowercase().as_str() {
                "y" | "yes" => {}
                "n" | "no" => selected_device = manual_connect(&devices),
                _ => println!("Invalid response. Please enter 'y' or 'n'."),
            }
        } else {
            println!("No device found matching the Hydrophonitor disk.");
            selected_device = manual_connect(&devices);
        }

        save_device(selected_device);
    }
}

//gets all available devices with lsblk
fn get_device_list() -> Vec<String> {
    let output = Command::new("lsblk").output().expect("Failed to run lsblk!");
    let output = String::from_utf8_lossy(&output.stdout);
    let devices: Vec<&str> = output.lines().collect();
    let mut devices_cropped: Vec<String> = Vec::new();
    for device in devices.iter() {
        let cropped_device = device.split(" ").last().unwrap_or_default().to_string();
        devices_cropped.push(cropped_device);
    }
    devices_cropped
}

fn manual_connect(devices: &Vec<String>) -> &String {
    let selection = Select::new()
        .with_prompt("Please choose a device from the list:")
        .items(&devices)
        .default(0)
        .interact();
    &devices[selection.unwrap_or_default()]
}

fn save_device(device: &String) {
    let file_path = home_dir().unwrap().join(".hydrophonitor");

    println!("connecting_to_device device {device}!");
    match fs::read_dir(format!("{device}/output")) {
        Ok(_) => {}
        Err(_) => {
            println! {"The selected device does not have a valid output directory!"};
            return;
        }
    }

    //TODO mount device

    let mut file = File::create(file_path).expect("Error while creating the file!");
    file.write_all(device.as_ref()).expect("Error while writing device path to file!");
}