use std::fmt;

use anyhow::{anyhow, Context};
use chrono::{DateTime, FixedOffset};

pub const TIMESTAMP_FORMAT: &str = "%Y-%m-%dT%H_%M_%S%.3f%z";

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Timestamp(DateTime<FixedOffset>);

// todo documentation - TryFrom included in prelude - doesn't require importing std::str::FromStr
impl TryFrom<&str> for Timestamp {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.trim().is_empty() {
            return Err(anyhow!("timestamp must not be empty"));
        }

        let timestamp = DateTime::parse_from_str(s, TIMESTAMP_FORMAT)
            .with_context(|| format!("string '{s}' cannot be parsed as timestamp with format '{TIMESTAMP_FORMAT}'"))?;

        Ok(Self(timestamp))
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.format(TIMESTAMP_FORMAT))
    }
}
