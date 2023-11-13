use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use walkdir::WalkDir;

use crate::import::{Import, import_from_sd, merge_wavs};

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
        Commands::Import(mut import) => {
            println!("Importing audio from SD card at {:?}", import.device);

            if let Some(output_folder) = &import.output {
                println!("Output folder: {:?}", output_folder);
                import_from_sd(&mut import.device, Some(output_folder.clone())).unwrap_or_else(|err| {
                    eprintln!("Error: {}", err);
                    std::process::exit(1);
                });
            } else {
                println!("Output folder: current directory");
                import_from_sd(&mut import.device, None).unwrap_or_else(|err| {
                    eprintln!("Error: {}", err);
                    std::process::exit(1);
                });
            }

            // Iterate folders inside output folder. Inside each iterated folder there is
            // a folder called "audio" which contains the wav files. Merge them into a single
            // wav file and delete the "audio" folder.
            let output_folder = match import.output {
                Some(output_folder) => output_folder,
                None => std::env::current_dir().unwrap_or_else(|err| {
                    eprintln!("Error: {}", err);
                    std::process::exit(1);
                })
            };

            for entry in WalkDir::new(output_folder.clone()) {
                let entry = entry.unwrap_or_else(|err| {
                    eprintln!("Error: {}", err);
                    std::process::exit(1);
                });
                let path = entry.path();
                if path.is_dir() {
                    let audio_folder = path.join("audio");
                    if audio_folder.exists() {
                        merge_wavs(&audio_folder, &PathBuf::from(path)).unwrap_or_else(|err| {
                            eprintln!("Error: {}", err);
                            std::process::exit(1);
                        });
                        fs::remove_dir_all(audio_folder).unwrap_or_else(|err| {
                            eprintln!("Error: {}", err);
                            std::process::exit(1);
                        });
                    }
                }
            }
        }
    }
}
