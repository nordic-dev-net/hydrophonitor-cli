use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use nonempty::NonEmpty;

use crate::domain::{AudioFile, DepthFile, get_file_name, GpsFile, LogFile};
use crate::domain::timestamp::Timestamp;

#[derive(Debug, Clone)]
pub struct Deployment {
    path_buf: PathBuf,
    pub timestamp: Timestamp,
}

impl Deployment {
    pub fn sorted_audio_files(&self) -> Result<Option<NonEmpty<AudioFile>>> {
        todo!()
    }

    pub fn sorted_depth_files(&self) -> Result<Option<NonEmpty<DepthFile>>> {
        todo!()
    }

    pub fn sorted_gps_files(&self) -> Result<Option<NonEmpty<GpsFile>>> {
        todo!()
    }

    pub fn sorted_log_files(&self) -> Result<Option<NonEmpty<LogFile>>> {
        todo!()
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
