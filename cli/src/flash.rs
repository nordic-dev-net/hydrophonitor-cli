use std::path::PathBuf;

use clap::Parser;
use log::error;

use hydrophonitor_lib::connect;
use hydrophonitor_lib::device_type::DeviceType;
use hydrophonitor_lib::flash as flash_lib;

use crate::connect::connect;

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
    pub fn flash(&mut self) {
        let _mount;
        let device_path;
        match &self.device {
            Some(device) => device_path = device,
            None => {
                _mount = connect(DeviceType::Disk);
                device_path = &connect::MOUNT_PATH;
            }
        }
        println!("Flashing device {:?} with image {:?}, this may take a while...", device_path, &self.image);
        flash_lib::flash(&self.image, device_path).unwrap_or_else(|err| {
            error!("Error: {}", err);
            std::process::exit(1);
        });
        println!("Flashing finished!");
    }
}


