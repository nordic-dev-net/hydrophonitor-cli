use crate::domain::Timestamp;

pub trait TimestampedFile {
    fn timestamp(&self) -> &Timestamp;
}
