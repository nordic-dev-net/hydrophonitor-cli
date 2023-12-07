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
    pub fn clean(&mut self) {
        info!("Cleaning device at {:?}", &self.device);
        self.device.push("output");

        // Attempt to open the directory
        match self.device.read_dir() {
            Ok(entries) => {
                // Iterate over the entries in the directory
                for entry in entries {
                    if let Ok(entry) = entry {
                        // Print the name of each entry
                        println!("{}", entry.file_name().to_string_lossy());
                    }
                }
            }
            Err(e) => {
                error!("Error opening the output folder of the device at {:?}: {}", &self.device, e);
            }
        }
    }
}
