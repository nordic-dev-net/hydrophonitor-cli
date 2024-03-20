use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use hydrophonitor_lib::import::import;

#[derive(Parser, Debug)]
#[clap(about = "Import audio from an SD card.")]
pub struct Import {
    /// Path to USB mass storage or SD where data will be imported from. You can find the path to
    /// the SD card by running `lsblk` in the terminal.
    #[clap(short, long, required = true)]
    pub device: PathBuf,

    /// Path to the output folder. If not specified, the output folder will be
    /// the current directory.
    #[clap(short, long)]
    pub output: Option<PathBuf>,

    ///Runs a clean after import is complete.
    #[clap(long, action)]
    pub clean_imported: bool,

    ///Generates compressed previews of audio files.
    #[clap(long, action)]
    pub audio_previews: bool,
}

impl Import {
    //TODO old logic; has to be changed to match new commands
    pub fn import(&mut self) -> Result<()> {
        import(&mut self.device, self.output.clone());
        Ok(())
    }
}
