use clap::Parser;

use hydrophonitor_lib::domain::{Device, Output};

#[derive(Parser, Debug)]
#[clap(about = "Import audio from an SD card.")]
pub struct Import {
    /// Path to USB mass storage or SD where data will be imported from. You can find the path to
    /// the SD card by running `lsblk` in the terminal.
    #[arg(short, long, required = true)]
    pub device: Device,

    /// Path to the output folder where a new directory will be created for every imported deployment,
    /// within which the corresponding files will be merged and placed.
    #[arg(short, long)]
    pub output: Output,

    ///Runs a clean after import is complete.
    #[arg(long, action)]
    pub clean_imported: bool,

    ///Generates compressed previews of audio files.
    #[arg(long, action)]
    pub audio_previews: bool,
}

impl Import {
    pub fn import(&mut self) {
        todo!();
    }
}
