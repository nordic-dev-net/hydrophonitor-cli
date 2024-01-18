use std::io;

use anyhow::{Context, Result};
use dialoguer::Select;
use sys_mount::{Mount, UnmountDrop};

use hydrophonitor_lib::connect as connect_lib;

//Runs the connect wizard to select and mount the hydrophonitor device. It returns a mount object that defines the lifetime of the mount.
pub fn connect() -> Result<UnmountDrop<Mount>> {
    let devices = connect_lib::get_device_list().with_context(|| "Failed to get device list")?;
    let mut selected_device = &String::new();
    let suitable_device = connect_lib::find_suitable_device(&devices).with_context(|| "Getting device failed")?;
    match suitable_device {
        Some(dev) => {
            println!("Found potential Hydrophonitor device {dev}. Do you want to connect to this device? (y/n)");
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).with_context(|| "Failed to read line!")?;
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

    let mount = connect_lib::mount_device(selected_device).with_context(|| "Mounting device failed")?;
    println!("successfully connected to device {selected_device}!");
    Ok(mount)
}

fn manual_connect(devices: &[String]) -> &String {
    let selection = Select::new()
        .with_prompt("Please choose a device from the list:")
        .items(devices)
        .default(0)
        .interact();
    &devices[selection.unwrap_or_default()]
}