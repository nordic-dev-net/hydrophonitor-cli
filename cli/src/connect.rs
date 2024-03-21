use anyhow::{Context, Result};
use dialoguer::{Confirm, Select};
use sys_mount::{Mount, UnmountDrop};

use hydrophonitor_lib::connect as connect_lib;
use hydrophonitor_lib::device_type::DeviceType;

//Runs the connect wizard to select and mount the hydrophonitor device. It returns a mount object that defines the lifetime of the mount.
pub fn connect() -> Result<UnmountDrop<Mount>> {
    let devices = connect_lib::get_device_list(DeviceType::Part).with_context(|| "Failed to get device list")?;
    let suitable_device = connect_lib::find_suitable_device(&devices).with_context(|| "Getting device failed")?;
    let selected_device = match suitable_device {
        Some(dev) => {
            let connect_to_device = Confirm::new()
                .with_prompt(format!("Found potential Hydrophonitor device '{dev}'. Do you want to connect to this device?"))
                .default(true)
                .interact()?;
            if connect_to_device {
                dev
            } else {
                manual_connect(&devices)
            }
        }
        None => {
            println!("No device found matching the Hydrophonitor disk.");
            manual_connect(&devices)
        }
    };

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