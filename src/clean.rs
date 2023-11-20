use std::path::PathBuf;

use clap::Parser;
use log::{error, info, trace, warn};

#[derive(Parser, Debug)]
#[clap(about = "This command removes all deployment data from the given device's /output path")]
pub struct Clean {
    ///Path to USB mass storage or SD card where data will be deleted from.
    #[clap(short, long, required = true)]
    pub device: PathBuf,

    ///Increases the CLI verbosity.
    #[clap(short, long, action)]
    pub verbose: bool,
}

impl Clean {
    pub fn clean(&self) {
        trace!("Cleaning device at {:?}", self.device);
    }
}