use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Output(PathBuf);

// todo documentation - FromStr required for compatibility with clap
impl FromStr for Output {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            return Err("path to output must not be empty".to_string());
        }

        let path_buf = PathBuf::from(s);
        if path_buf.is_dir() {
            Ok(Self(path_buf))
        } else if path_buf.is_file() {
            Err(format!("output with path '{s}' is a file but must be a directory"))
        } else {
            Err(format!("output with path '{s}' does not exist"))
        }
    }
}

impl AsRef<Path> for Output {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}
