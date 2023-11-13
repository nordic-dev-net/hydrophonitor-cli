use clap::{Parser, Subcommand};

use crate::import::Import;

mod import;
mod clean;

#[derive(Subcommand)]
#[clap(about = "A tool to record audio on Linux using the command line.")]
pub enum Commands {
    Import(Import),
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
        Commands::Import(mut import) => { import.import() }
    }
}
