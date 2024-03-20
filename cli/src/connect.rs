use std::io;

use dialoguer::Select;
use sys_mount::{Mount, UnmountDrop};

use hydrophonitor_lib::connect as connect_lib;
use hydrophonitor_lib::device_type::DeviceType;

// Runs the connect wizard to select and mount the hydrophonitor device.
// The device type specifies which type of devices which should be listed.
// It returns a mount object that defines the lifetime of the mount.
pub fn connect() -> UnmountDrop<Mount> {
    let devices = connect_lib::get_device_list(DeviceType::Part);
    let mut selected_device = &String::new();
    match connect_lib::find_suitable_device(&devices) {
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

    let mount = connect_lib::mount_device(selected_device);
    println!("successfully connected to device {selected_device}!");
    mount
}

pub(crate) fn manual_connect(devices: &[String]) -> &String {
    let selection = Select::new()
        .with_prompt("Please choose a device from the list:")
        .items(devices)
        .default(0)
        .interact();
    &devices[selection.unwrap_or_default()]
    //TODO security question
}