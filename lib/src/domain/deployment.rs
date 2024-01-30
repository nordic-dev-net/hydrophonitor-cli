use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use log::warn;
use nonempty::NonEmpty;

use crate::domain::{AudioFile, DepthFile, get_file_name, GpsFile, LogFile, TimestampedFile};
use crate::domain::timestamp::Timestamp;

const AUDIO_DATA_DIR: &str = "audio";
const DEPTH_DATA_DIR: &str = "depth";
const GPS_DATA_DIR: &str = "gps";
const LOG_DATA_DIR: &str = "log";

#[derive(Debug, Clone)]
pub struct Deployment {
    path_buf: PathBuf,
    pub timestamp: Timestamp,
}

impl Deployment {
    pub fn sorted_audio_files(&self) -> Result<Option<NonEmpty<AudioFile>>> {
        self.sorted_files(AUDIO_DATA_DIR, AudioFile::try_from)
    }

    pub fn sorted_depth_files(&self) -> Result<Option<NonEmpty<DepthFile>>> {
        self.sorted_files(DEPTH_DATA_DIR, DepthFile::try_from)
    }

    pub fn sorted_gps_files(&self) -> Result<Option<NonEmpty<GpsFile>>> {
        self.sorted_files(GPS_DATA_DIR, GpsFile::try_from)
    }

    pub fn sorted_log_files(&self) -> Result<Option<NonEmpty<LogFile>>> {
        self.sorted_files(LOG_DATA_DIR, LogFile::try_from)
    }

    fn sorted_files<F, T>(
        &self,
        sub_dir: &str,
        try_make_file: F,
    ) -> Result<Option<NonEmpty<T>>>
    where
        F: Fn(PathBuf) -> Result<T>,
        T: TimestampedFile,
    {
        let data_dir = match self.data_dir(sub_dir)? {
            Some(data_dir) => data_dir,
            None => return Ok(None),
        };

        let entries = fs::read_dir(&data_dir)
            .with_context(|| format!("Failed to read data directory with path '{}'", data_dir.display()))?;

        let mut files: Vec<T> = Vec::new();
        for entry in entries {
            let entry = entry
                .with_context(|| format!("Failed to read entry in data directory with path '{}'", data_dir.display()))?;

            match try_make_file(entry.path()) {
                Ok(file) => files.push(file),
                Err(err) => warn!("Ignoring entry: {err:?}"),
            }
        }

        files.sort_by(|f1, f2| f1.timestamp().cmp(f2.timestamp()));
        Ok(NonEmpty::from_vec(files))
    }

    fn data_dir(&self, sub_dir: &str) -> Result<Option<PathBuf>> {
        let data_dir = self.path_buf.join(sub_dir);
        if data_dir.is_file() {
            Err(anyhow!("data directory with sub-path '{sub_dir}' in deployment '{}' is a file but must be a directory", self.path_buf.display()))
        } else if data_dir.is_dir() {
            Ok(Some(data_dir))
        } else {
            Ok(None)
        }
    }
}

impl TryFrom<PathBuf> for Deployment {
    type Error = anyhow::Error;

    fn try_from(path_buf: PathBuf) -> Result<Self, Self::Error> {
        if path_buf.is_dir() {
            let file_name = get_file_name(&path_buf)?;

            let timestamp = Timestamp::try_from(file_name)
                .with_context(|| format!("file name of deployment with path '{}' must be a timestamp", path_buf.display()))?;

            Ok(Self { path_buf, timestamp })
        } else if path_buf.is_file() {
            Err(anyhow!("deployment with path '{}' is a file but must be a directory", path_buf.display()))
        } else {
            Err(anyhow!("deployment with path '{}' does not exist", path_buf.display()))
        }
    }
}

impl AsRef<Path> for Deployment {
    fn as_ref(&self) -> &Path {
        &self.path_buf
    }
}
