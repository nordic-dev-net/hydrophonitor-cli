use std::io;
use std::path::PathBuf;

use clap::Parser;

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
    pub fn flash(&mut self) {
        let device_path = match &self.device {
            Some(device) => device.clone(),
            None => select_device()
        };

        //TODO change to use dialoguer
        //Confirmation question
        println!("Do you really want to flash the Hydrophonitor OS to the device {device_path:?}? All data on this device will be lost!");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line!");
        match user_input.trim().to_lowercase().as_str() {
            "y" | "yes" => (),
            "n" | "no" => {
                println!("Aborting!");
                return;
            }
            _ => {
                println!("Invalid response. Please enter 'y' or 'n'.");
                return;
            }
        }

        //Flashing
        println!("Flashing device {:?} with image {:?}, this may take a while...", &device_path, &self.image);
        match flash_lib::flash(&self.image, &device_path) {
            Ok(_) => println!("Flashing finished!"),
            Err(err) => println!("Flashing failed due to error: {}, aborting!", err),
        };
    }
}

fn select_device() -> PathBuf {
    let devices = connect_lib::get_device_list(DeviceType::Disk);
    let selected_device = crate::connect::manual_connect(&devices);
    PathBuf::from(format!("/dev/{selected_device}"))
}


