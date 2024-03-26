use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use log::warn;

pub fn get_deployments_of_device(output_dir: &PathBuf) -> Option<Vec<String>> {
    match fs::read_dir(output_dir) {
        Ok(dir) => {
            let mut entries = Vec::new();
            // Iterate over the entries in the directory
            for entry in dir {
                entries.push(entry.unwrap().file_name().into_string().unwrap());
            }
            Some(entries)
        }
        Err(e) => {
            warn!("Error opening the directory {:?}: {}", output_dir, e);
            None
        }
    }
}

pub fn clear_directory(output_dir: &PathBuf) -> Result<()> {
    fs::remove_dir_all(output_dir).with_context(|| format!("Removing everything in directory {:?} failed", &output_dir))?;
    fs::create_dir(output_dir).with_context(|| format!("Creating new empty output directory in {:?} failed", &output_dir))?;
    Ok(())
}
