use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

use crate::clean::Clean;
use crate::flash::Flash;
use crate::import::Import;
use crate::info::Info;

mod import;
mod clean;
mod connect;
mod flash;
mod info;

#[derive(Subcommand)]
#[clap(about = "A tool to record audio on Linux using the command line.")]
pub enum Commands {
    Import(Import),
    Clean(Clean),
    Flash(Flash),
    Info(Info)
}


#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub commands: Commands,

    #[command(flatten)]
    pub verbose: Verbosity,
}


fn main() {
    let Cli { commands, verbose } = Cli::parse();

    env_logger::builder().filter_level(verbose.log_level_filter()).init();

    let result = match commands {
        Commands::Import(mut import) => import.import(),
        Commands::Clean(mut clean) => clean.clean(),
        Commands::Flash(mut flash) => flash.flash(),
        Commands::Info(mut info) => info.info(),
    };

    if let Err(err) = result {
        println!("{err:?}")
    }
}

