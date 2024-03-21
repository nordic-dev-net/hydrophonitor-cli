use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use dialoguer::Confirm;

use hydrophonitor_lib::connect as connect_lib;
use hydrophonitor_lib::device_type::DeviceType;
use hydrophonitor_lib::flash as flash_lib;

#[derive(Parser, Debug)]
#[clap(about = "This command flashes a SD card or USB mass storage with the selected version of the Hydrophonitor system.")]
pub struct Flash {
    ///Path to USB mass storage or SD card where the NixOS image will be flashed.
    #[clap(short, long)]
    device: Option<PathBuf>,

    ///  Path to the image that will be flashed to the device.
    #[clap(short, long, required = true)]
    image: PathBuf,
}

impl Flash {
    pub fn flash(&mut self) -> Result<()> {
        let device_path = match &self.device {
            Some(device) => device.clone(),
            None => select_device().with_context(|| "Failed to select device")?
        };

        //Confirmation question
        let use_device = Confirm::new()
            .with_prompt(format!("Do you really want to flash the Hydrophonitor OS to the device {device_path:?}? All data on this device will be lost!"))
            .default(true)
            .interact()?;
        if !use_device {
            println!("Aborting!");
            return Ok(());
        }

        //Flashing
        println!("Flashing device {:?} with image {:?}, this may take a while...", &device_path, &self.image);
        flash_lib::flash(&self.image, &device_path).with_context(|| "Flashing device failed")?;
        Ok(())
    }
}

fn select_device() -> Result<PathBuf> {
    let devices = connect_lib::get_device_list(DeviceType::Disk).with_context(|| "Failed to get device list")?;
    let selected_device = crate::connect::manual_connect(&devices);
    Ok(PathBuf::from(format!("/dev/{selected_device}")))
}


