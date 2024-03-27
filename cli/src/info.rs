use std::path::PathBuf;
use clap::Parser;
use log::debug;
use anyhow::Result;

#[derive(Parser, Debug)]
#[clap(about = "Retrieves information abóut the hydrophonitor device")]
pub struct Info {
    ///Path to USB mass storage or SD card of the device.
    #[clap(short, long)]
    device: Option<PathBuf>,
}

impl Info {
    pub fn info(&mut self) -> Result<()>{
        debug!("Retrieving device info");
        Ok(())
    }
    
}
