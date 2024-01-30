use std::path::PathBuf;

use anyhow::{anyhow, Context};
use regex::Regex;

pub use audio_file::AudioFile;
pub use depth_file::DepthFile;
pub use gps_file::GpsFile;
pub use log_file::LogFile;
pub use timestamped_file::TimestampedFile;

use crate::domain::{get_file_name, Timestamp};
use crate::domain::timestamp::TIMESTAMP_FORMAT;

mod audio_file;
mod depth_file;
mod gps_file;
mod log_file;
mod timestamped_file;

fn try_from<F, T>(
    path_buf: PathBuf,
    file_type: &str,
    suffix_pattern: &str,
    suffix_plain: &str,
    constructor: F,
) -> anyhow::Result<T>
where
    F: Fn(PathBuf, Timestamp) -> T,
{
    if path_buf.is_file() {
        let file_name = get_file_name(&path_buf)?;

        let timestamp_capture_pattern = r"(?<timestamp>.+)";
        let pattern = format!("{timestamp_capture_pattern}{suffix_pattern}");
        let regex = Regex::new(&pattern)
            .with_context(|| format!("failed to compile regex pattern '{pattern}'"))?;

        let timestamp = {
            match regex.captures(file_name) {
                Some(captures) => Timestamp::try_from(&captures["timestamp"]),
                None => Err(anyhow!("file name does not match pattern")),
            }
        }.with_context(|| format!(
            "file name of {file_type} file with path '{}' must start with a timestamp with format '{}' \
            and end with suffix '{suffix_plain}'", path_buf.display(), TIMESTAMP_FORMAT
        ))?;

        Ok(constructor(path_buf, timestamp))
    } else if path_buf.is_dir() {
        Err(anyhow!("{file_type} file with path '{}' is a directory but must be a file", path_buf.display()))
    } else {
        Err(anyhow!("{file_type} file with path '{}' does not exist", path_buf.display()))
    }
}
