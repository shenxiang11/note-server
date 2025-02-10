use chrono::{DateTime, Utc};
use prost_types::Timestamp;

pub struct PbTimestamp(pub Timestamp);

impl From<PbTimestamp> for DateTime<Utc> {
    fn from(value: PbTimestamp) -> Self {
        DateTime::from_timestamp(value.0.seconds, value.0.nanos as u32).unwrap_or_default()
    }
}

impl From<Timestamp> for PbTimestamp {
    fn from(value: Timestamp) -> Self {
        PbTimestamp(value)
    }
}
