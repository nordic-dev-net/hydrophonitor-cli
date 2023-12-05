use clap::{Parser, Subcommand};

use crate::clean::Clean;
use crate::import::Import;

mod import;
mod clean;
mod logging;

#[derive(Subcommand)]
#[clap(about = "A tool to record audio on Linux using the command line.")]
pub enum Commands {
    Import(Import),
    Clean(Clean),
}


#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub commands: Commands,
}


fn main() {
    let commands = Cli::parse();


    match commands.commands {
        Commands::Import(mut import) => {
            import.import() }
        Commands::Clean(mut clean) => { clean.clean() }
    }
}

