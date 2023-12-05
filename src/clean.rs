use std::path::PathBuf;

use clap::Parser;
use log::info;

use crate::logging::init_logging;

#[derive(Parser, Debug)]
#[clap(about = "This command removes all deployment data from the given device's /output path")]
pub struct Clean {
    ///Path to USB mass storage or SD card where data will be deleted from.
    #[clap(short, long, required = true)]
    pub device: PathBuf,

    ///Sets the CLI verbosity. Allowed values are 'error', 'warn', 'info', 'debug' and 'trace'.
    #[clap(short, long, default_value = "error")]
    pub verbose: String,
}

impl Clean {
    pub fn clean(&self) {
        init_logging(&self.verbose);
        info!("Cleaning device at {:?}", self.device);
    }
}
