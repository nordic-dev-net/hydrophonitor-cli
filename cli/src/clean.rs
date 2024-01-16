use std::io;
use std::path::PathBuf;

use clap::Parser;
use log::info;

use hydrophonitor_lib::clean as clean_lib;

#[derive(Parser, Debug)]
#[clap(about = "This command removes all deployment data from the given device's /output path")]
pub struct Clean {
    ///Path to USB mass storage or SD card where data will be deleted from.
    #[clap(short, long, default_value = "/var/lib/hydrophonitor/device")]
    device: PathBuf,
}

impl Clean {
    pub fn clean(&mut self) {
        // Checking device for output folder
        info!("Cleaning device at {:?}", self.device);
        let mut output_dir = self.device.clone();
        output_dir.push("output");
        if !output_dir.is_dir() {
            println!("{:?} is not a valid device! please select a hydrophonitor device with output folder!", output_dir);
            return;
        }

        // Showing deployments and asking for confirmation
        let deployments = clean_lib::get_deployments_of_device(&output_dir);
        if !deployments.is_empty() {
            dbg!(deployments);
            println!("Do you really want to delete these deployments? (y/n)");
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).expect("Failed to read line");
            if !(user_input.contains("y") || user_input.contains("Y")) {
                println!("Aborting!");
                return;
            }
        } else {
            println!("The directory is already empty!");
            return;
        }

        // Cleaning device
        clean_lib::clear_directory(&output_dir);
        println!("Successfully cleaned directory!")
    }
}
