use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use log::{debug};

use hydrophonitor_lib::clean as clean_lib;
use hydrophonitor_lib::connect;

use crate::connect::connect;

#[derive(Parser, Debug)]
#[clap(about = "Retrieves information abóut the hydrophonitor device")]
pub struct Info {
    ///Path to USB mass storage or SD card of the device.
    #[clap(short, long)]
    device: Option<PathBuf>,
}

impl Info {
    pub fn info(&mut self) -> Result<()> {
        debug!("Retrieving device info");

        // Creating path to output folder
        let _mount;     //device is mounted as long as this variable is in scope
        let mut output_dir: PathBuf;
        match &self.device {
            Some(device) => output_dir = device.clone(),
            None => {
                _mount = connect().with_context(|| "connecting to device failed:")?;
                output_dir = connect::MOUNT_PATH.clone();
            }
        }
        output_dir.push("output");

        // Printing deployments
        match clean_lib::get_deployments_of_device(&output_dir) {
            Some(deployments) => {
                println!("DEPLOYMENTS:");
                dbg!(deployments);
            }
            None => {
                return Err(anyhow!("{output_dir:?} is not a valid device! please select a hydrophonitor device with output folder!"));
            }
        }

        Ok(())
    }
}
