use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{anyhow, Context, Result};
use log::warn;

use crate::domain::Deployment;

const DATA_DIR: &str = "output";

#[derive(Debug, Clone)]
pub struct Device(PathBuf);

impl Device {
    pub fn sorted_deployments(&self) -> Result<Vec<Deployment>> {
        let data_dir = self.data_dir()?;

        let entries = fs::read_dir(&data_dir)
            .with_context(|| format!("Failed to read data directory with path '{}'", data_dir.display()))?;

        let mut deployments: Vec<Deployment> = Vec::new();
        for entry in entries {
            let entry = entry
                .with_context(|| format!("Failed to read entry in data directory with path '{}'", data_dir.display()))?;

            match Deployment::try_from(entry.path()) {
                Ok(deployment) => deployments.push(deployment),
                Err(err) => warn!("Ignoring entry: {err:?}"),
            }
        }

        deployments.sort_by(|d1, d2| d1.timestamp.cmp(&d2.timestamp));
        Ok(deployments)
    }

    fn data_dir(&self) -> Result<PathBuf> {
        let data_dir = self.0.join(DATA_DIR);
        if data_dir.is_dir() {
            Ok(data_dir)
        } else if data_dir.is_file() {
            Err(anyhow!("data directory with sub-path '{DATA_DIR}' in device '{}' is a file but must be a directory", self.0.display()))
        } else {
            Err(anyhow!("data directory with sub-path '{DATA_DIR}' in device '{}' does not exist", self.0.display()))
        }
    }
}

// todo documentation - FromStr required for compatibility with clap
impl FromStr for Device {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            return Err("path to device must not be empty".to_string());
        }

        let path_buf = PathBuf::from(s);
        if path_buf.is_dir() {
            Ok(Self(path_buf))
        } else if path_buf.is_file() {
            Err(format!("device with path '{s}' is a file but must be a directory"))
        } else {
            Err(format!("device with path '{s}' does not exist"))
        }
    }
}
