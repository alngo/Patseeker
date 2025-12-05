use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(DateTime<Utc>);

impl From<i64> for Timestamp {
    fn from(value: i64) -> Self {
        let datetime = DateTime::from_timestamp(value, 0).expect("Invalid timestamp");
        Timestamp(datetime)
    }
}

impl From<i32> for Timestamp {
    fn from(value: i32) -> Self {
        let datetime = DateTime::from_timestamp(value as i64, 0).expect("Invalid timestamp");
        Timestamp(datetime)
    }
}

impl From<usize> for Timestamp {
    fn from(value: usize) -> Self {
        let datetime = DateTime::from_timestamp(value as i64, 0).expect("Invalid timestamp");
        Timestamp(datetime)
    }
}

impl From<Timestamp> for DateTime<Utc> {
    fn from(timestamp: Timestamp) -> Self {
        timestamp.0
    }
}
