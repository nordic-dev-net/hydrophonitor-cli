use std::error::Error;
use std::fs;
use std::path::PathBuf;

use clap::Parser;
use hound::{WavReader, WavWriter};
use indicatif::ProgressBar;
use walkdir::WalkDir;

const DATA_FOLDER: &str = "home/pi/data";

#[derive(Parser, Debug)]
#[clap(about = "Import audio from an SD card.")]
pub struct Import {
    /// Path to the SD card. You can find the path to the SD card by running
    /// `lsblk` in the terminal.
    #[clap(short, long, required = true)]
    pub device: PathBuf,

    /// Path to the output folder. If not specified, the output folder will be
    /// the current directory.
    #[clap(short, long)]
    pub output: Option<PathBuf>,

    ///Runs a clean after import is complete
    #[clap(long, action)]
    pub clean_imported: bool,

    ///Generates compressed previews of audio files.
    #[clap(long, action)]
    pub audio_previews: bool,

    ///Increases the CLI verbosity
    #[clap(short, long, action)]
    pub verbose: bool,
}

pub fn import_from_sd(sd_card: &mut PathBuf, output_folder: Option<PathBuf>) -> Result<(), Box<dyn Error>> {
    let output_folder = match output_folder {
        Some(output_folder) => output_folder,
        None => std::env::current_dir().unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        })
    };

    sd_card.push(DATA_FOLDER);

    let count = WalkDir::new(sd_card.clone()).into_iter().count();
    let progress_bar = ProgressBar::new(count as u64);

    for entry in WalkDir::new(sd_card.clone()) {
        let entry = entry?;
        let from = entry.path();
        let to = output_folder.join(from.strip_prefix(sd_card.clone())?);

        if entry.file_type().is_dir() {
            fs::create_dir_all(to)?;
        } else if entry.file_type().is_file() {
            fs::copy(from, to)?;
        }
        progress_bar.inc(1);
    }
    progress_bar.finish();
    Ok(())
}

pub fn merge_wavs(input: &std::path::PathBuf, output: &std::path::PathBuf) -> Result<(), Box<dyn Error>> {
    // Read files from input directory
    let mut files = std::fs::read_dir(input)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().ok().map(|t| t.is_file()).unwrap_or(false))
        .filter(|entry| entry.path().extension().unwrap_or_default() == "wav")
        .collect::<Vec<_>>();

    // If there are no wav files, return
    if files.is_empty() {
        println!("No wav files found in {:?}", input);
        return Ok(());
    }
    // Sort files by name
    files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    let output_name = files.first().unwrap().file_name();
    let output_name = output_name.to_str().unwrap();

    // Get wav spec from file
    let spec = WavReader::open(files.first().unwrap().path())?.spec();
    let mut writer = WavWriter::create(output.join(output_name), spec)?;

    let progress_bar = ProgressBar::new(files.len() as u64);
    match spec.sample_format {
        hound::SampleFormat::Float => {
            for file in files {
                let mut reader = WavReader::open(file.path())?;
                for sample in reader.samples::<f32>() {
                    writer.write_sample(sample?)?;
                }
                progress_bar.inc(1);
            }
        }
        hound::SampleFormat::Int => {
            for file in files {
                let mut reader = WavReader::open(file.path())?;
                for sample in reader.samples::<i32>() {
                    writer.write_sample(sample?)?;
                }
                progress_bar.inc(1);
            }
        }
    }
    progress_bar.finish();
    writer.finalize()?;
    Ok(())
}
