use std::path::{Path, PathBuf};

use crate::domain::files::timestamped_file::TimestampedFile;
use crate::domain::files::try_from;
use crate::domain::Timestamp;

#[derive(Debug, Clone)]
pub struct GpsFile {
    path_buf: PathBuf,
    pub timestamp: Timestamp,
}

impl TryFrom<PathBuf> for GpsFile {
    type Error = anyhow::Error;

    fn try_from(path_buf: PathBuf) -> anyhow::Result<Self, Self::Error> {
        try_from(
            path_buf,
            "gps",
            r"_gps\.json",
            "_gps.json",
            |path_buf, timestamp| Self { path_buf, timestamp },
        )
    }
}

impl AsRef<Path> for GpsFile {
    fn as_ref(&self) -> &Path {
        &self.path_buf
    }
}

impl TimestampedFile for GpsFile {
    fn timestamp(&self) -> &Timestamp {
        &self.timestamp
    }
}
