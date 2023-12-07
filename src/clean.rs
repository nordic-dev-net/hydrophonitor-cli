use std::fs;
use std::path::PathBuf;

use clap::Parser;
use log::{error, info};

#[derive(Parser, Debug)]
#[clap(about = "This command removes all deployment data from the given device's /output path")]
pub struct Clean {
    ///Path to USB mass storage or SD card where data will be deleted from.
    #[clap(short, long, required = true)]
    device: PathBuf,
}

impl Clean {
    pub fn clean(&self) {
        info!("Cleaning device at {:?}", &self.device);
        let mut output_folder = self.device.clone();
        output_folder.push("output");

        match fs::remove_dir_all(&output_folder) {
            Ok(_) => {}
            Err(e) => {
                error!("Removing everything in directory {:?} failed: {}", &output_folder, e);
                return;
            }
        };
        match fs::create_dir(output_folder) {
            Ok(_) => {}
            Err(e) => {
                error!("Creating new output directory in {:?} failed: {}", &self.device, e);
                return;
            }
        };
    }
}
