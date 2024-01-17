use std::io;
use std::path::PathBuf;

use clap::Parser;
use log::info;

use hydrophonitor_lib::{clean as clean_lib, connect};

use crate::connect::connect;

#[derive(Parser, Debug)]
#[clap(about = "This command removes all deployment data from the given device's /output path")]
pub struct Clean {
    ///Path to USB mass storage or SD card where data will be deleted from.
    #[clap(short, long)]
    device: Option<PathBuf>,
}

impl Clean {
    pub fn clean(&mut self) {
        //create path to output folder
        let _mount;
        let mut output_dir: PathBuf;
        match &self.device {
            Some(device) => output_dir = device.clone(),
            None => {
                _mount = connect();
                output_dir = connect::MOUNT_PATH.clone();
            }
        }
        output_dir.push("output");

        // Show deployments and ask for confirmation
        match clean_lib::get_deployments_of_device(&output_dir) {
            Some(deployments) => {
                info!("Cleaning device at {:?}", self.device);

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
            }
            None => {
                println!("{:?} is not a valid device! please select a hydrophonitor device with output folder!", output_dir);
                return;
            }
        }

        // Clean device
        clean_lib::clear_directory(&output_dir);
        println!("Successfully cleaned directory!")
    }
}
