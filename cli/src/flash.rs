use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about = "This command flashes a SD card or USB mass storage with the selected version of the hydrophonitor system.")]
pub struct Flash {
    ///Path to USB mass storage or SD card where the NixOS image will be flashed.
    #[clap(short, long)]
    device: Option<PathBuf>,

    ///  Path to the image that will be flashed to the device.
    #[clap(short, long)]
    image: Option<PathBuf>,
}

impl Flash {
    pub fn flash(&mut self) {
        println!("Flashing device {:?} with image {:?}", &self.device, &self.image);
    }
}
