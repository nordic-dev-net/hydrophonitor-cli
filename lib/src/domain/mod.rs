use std::ffi::OsStr;
use std::path::Path;

use anyhow::Context;

pub use deployment::Deployment;
pub use device::Device;
pub use output::Output;
pub use timestamp::Timestamp;
pub use files::{AudioFile, DepthFile, GpsFile, LogFile, TimestampedFile};

mod deployment;
mod device;
mod output;
mod timestamp;
mod files;

fn get_file_name(path: &Path) -> anyhow::Result<&str> {
    path
        .file_name()
        .and_then(OsStr::to_str)
        .with_context(|| format!("failed to obtain file name for path '{}'", path.display()))
}
