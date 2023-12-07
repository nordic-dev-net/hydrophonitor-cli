use std::{fs, io};
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
        match fs::read_dir(&self.device) {
            Ok(entries) => {
                // Iterate over the entries in the directory
                let mut entries_found = false;
                for entry in entries {
                    if let Ok(entry) = entry {
                        // Print the name of each entry
                        println!("{}", entry.file_name().to_string_lossy());
                        entries_found = true;
                    }
                }
                if entries_found {
                    println!("Do you really want to delete these entries? (y/n)");
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
            }
            Err(e) => {
                error!("Error opening the directory {:?}: {}", &self.device, e);
                return;
            }
        }

        match fs::remove_dir_all(&self.device) {
            Ok(_) => {}
            Err(e) => {
                error!("Removing everything in directory {:?} failed: {}", &self.device, e);
                return;
            }
        };
        match fs::create_dir(&self.device) {
            Ok(_) => {}
            Err(e) => {
                error!("Creating new output directory in {:?} failed: {}", &self.device, e);
                return;
            }
        };

        println!("Successfully cleaned directory!")
    }
}
