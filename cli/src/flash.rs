use std::path::PathBuf;

use clap::Parser;

use hydrophonitor_lib::connect;

use crate::connect::connect;

#[derive(Parser, Debug)]
#[clap(about = "This command flashes a SD card or USB mass storage with the selected version of the hydrophonitor system.")]
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
                _mount = connect();
                device_path = &connect::MOUNT_PATH;
            }
        }
        println!("Flashing device {:?} with image {:?}", device_path, &self.image);
    }
}
