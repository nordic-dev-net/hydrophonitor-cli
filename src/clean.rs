use std::path::PathBuf;

use clap::Parser;
use log::info;

#[derive(Parser, Debug)]
#[clap(about = "This command removes all deployment data from the given device's /output path")]
pub struct Clean {
    ///Path to USB mass storage or SD card where data will be deleted from.
    #[clap(short, long, required = true)]
    pub device: PathBuf,
}

impl Clean {
    pub fn clean(&self) {
        info!("Cleaning device at {:?}", self.device);
    }
}
