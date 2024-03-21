use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use dialoguer::Confirm;
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
    pub fn clean(&mut self) -> Result<()> {
        //create path to output folder
        let _mount;
        let mut output_dir: PathBuf;
        match &self.device {
            Some(device) => output_dir = device.clone(),
            None => {
                _mount = connect().with_context(|| "connecting to device failed:")?;
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

                    if !Confirm::new()
                        .with_prompt("Do you really want to delete these deployments? (y/n)")
                        .default(true)
                        .interact()? {
                        println!("Aborting!");
                        return Ok(());
                    }
                } else {
                    println!("The directory is already empty!");
                    return Ok(());
                }
            }
            None => {
                return Err(anyhow!("{output_dir:?} is not a valid device! please select a hydrophonitor device with output folder!"));
            }
        }

        // Clean device
        clean_lib::clear_directory(&output_dir).with_context(|| "Error cleaning directory")?;
        println!("Successfully cleaned directory!");
        Ok(())
    }
}
