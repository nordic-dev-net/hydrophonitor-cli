use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::domain::files::try_from;
use crate::domain::Timestamp;

#[derive(Debug, Clone)]
pub struct AudioFile {
    path_buf: PathBuf,
    pub timestamp: Timestamp,
}

impl TryFrom<PathBuf> for AudioFile {
    type Error = anyhow::Error;

    fn try_from(path_buf: PathBuf) -> Result<Self, Self::Error> {
        try_from(
            path_buf,
            "audio",
            r"_audio\.wav",
            "_audio.wav",
            |path_buf, timestamp| Self { path_buf, timestamp },
        )
    }
}

impl AsRef<Path> for AudioFile {
    fn as_ref(&self) -> &Path {
        &self.path_buf
    }
}
