use chrono::{TimeZone, DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Timestamp {
    value: DateTime<Utc>,
}

impl Timestamp {
    pub fn new(value: DateTime<Utc>) -> Self {
        Timestamp { value }
    }

    pub fn value(&self) -> DateTime<Utc> {
        self.value
    }
}

impl From<DateTime<Utc>> for Timestamp {
    fn from(value: DateTime<Utc>) -> Self {
        Timestamp::new(value)
    }
}

impl From<Timestamp> for DateTime<Utc> {
    fn from(value: Timestamp) -> Self {
        value.value()
    }
}

impl From<i64> for Timestamp {
    fn from(value: i64) -> Self {
        let datetime = Utc.timestamp_opt(value, 0).unwrap();
        Timestamp::new(datetime)
    }
}

impl From<Timestamp> for i64 {
    fn from(value: Timestamp) -> Self {
        value.value.timestamp()
    }
}
