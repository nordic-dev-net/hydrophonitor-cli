use std::path::{Path, PathBuf};

use crate::domain::files::try_from;
use crate::domain::Timestamp;

#[derive(Debug, Clone)]
pub struct LogFile {
    path_buf: PathBuf,
    pub timestamp: Timestamp,
}

impl TryFrom<PathBuf> for LogFile {
    type Error = anyhow::Error;

    fn try_from(path_buf: PathBuf) -> anyhow::Result<Self, Self::Error> {
        try_from(
            path_buf,
            "log",
            r"_journalctl\.txt",
            "_journalctl.txt",
            |path_buf, timestamp| Self { path_buf, timestamp },
        )
    }
}

impl AsRef<Path> for LogFile {
    fn as_ref(&self) -> &Path {
        &self.path_buf
    }
}
