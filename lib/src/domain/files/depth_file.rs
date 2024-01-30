use std::path::{Path, PathBuf};

use crate::domain::files::timestamped_file::TimestampedFile;
use crate::domain::files::try_from;
use crate::domain::Timestamp;

#[derive(Debug, Clone)]
pub struct DepthFile {
    path_buf: PathBuf,
    pub timestamp: Timestamp,
}

impl TryFrom<PathBuf> for DepthFile {
    type Error = anyhow::Error;

    fn try_from(path_buf: PathBuf) -> anyhow::Result<Self, Self::Error> {
        try_from(
            path_buf,
            "depth",
            r"_depth\.csv",
            "_depth.csv",
            |path_buf, timestamp| Self { path_buf, timestamp },
        )
    }
}

impl AsRef<Path> for DepthFile {
    fn as_ref(&self) -> &Path {
        &self.path_buf
    }
}

impl TimestampedFile for DepthFile {
    fn timestamp(&self) -> &Timestamp {
        &self.timestamp
    }
}
