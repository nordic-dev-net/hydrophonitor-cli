use std::io;
use std::process::Command;

use clap::Parser;
use dialoguer::Select;
use log::debug;

#[derive(Parser, Debug)]
#[clap(about = "Connects to a device")]
pub struct Connect {}

impl Connect {
    pub fn connect(&mut self) {
        let output = Command::new("lsblk").output().expect("Failed to run lsblk!");
        let output = String::from_utf8_lossy(&output.stdout);
        let devices: Vec<&str> = output.lines().collect();
        debug!("devices: {:?}", devices);
        let mut selected_device = "";
        for device in devices.iter() {
            if device.contains("snap") {
                selected_device = device.split(" ").last().unwrap_or_default();
                break;
            }
        }
        if !selected_device.is_empty() {
            println!("Found potential Hydrophonitor device {selected_device}. Do you want to connect to this device? (y/n)");
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).expect("Failed to read line!");
            match user_input.trim().to_lowercase().as_str() {
                "y" | "yes" => mount_device(selected_device),
                "n" | "no" => manual_connect(&devices),
                _ => println!("Invalid response. Please enter 'y' or 'n'."),
            }
        } else {
            println!("No device found matching the Hydrophonitor disk.");
            manual_connect(&devices);
        }
    }
}

fn manual_connect(devices: &Vec<&str>) {
    let selection = Select::new()
        .with_prompt("Please choose a device from the list:")
        .items(&devices)
        .default(0)
        .interact();
}

fn mount_device(device: &str) {}
