use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

use crate::clean::Clean;
use crate::connect::Connect;
use crate::disconnect::Disconnect;
use crate::import::Import;

mod import;
mod clean;
mod connect;
mod disconnect;

#[derive(Subcommand)]
#[clap(about = "A tool to record audio on Linux using the command line.")]
pub enum Commands {
    Import(Import),
    Clean(Clean),
    Connect(Connect),
    Disconnect(Disconnect),
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

    match commands {
        Commands::Import(mut import) => import.import(),
        Commands::Clean(mut clean) => clean.clean(),
        Commands::Connect(mut connect) => connect.connect(),
        Commands::Disconnect(mut disconnect) => disconnect.disconnect(),
    }
}

