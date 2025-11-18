use chrono::{DateTime, TimeZone, Utc};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    pub fn new(value: DateTime<Utc>) -> Self {
        Timestamp(value)
    }

    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}

impl From<DateTime<Utc>> for Timestamp {
    fn from(value: DateTime<Utc>) -> Self {
        Timestamp::new(value)
    }
}

impl From<Timestamp> for DateTime<Utc> {
    fn from(value: Timestamp) -> Self {
        value.0
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
        value.0.timestamp()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_timestamp_from_datetime() {
        let datetime = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
        let timestamp = Timestamp::from(datetime);
        assert_eq!(timestamp.value(), datetime);
    }

    #[test]
    fn test_timestamp_to_datetime() {
        let datetime = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
        let timestamp = Timestamp::new(datetime);
        let converted_datetime: DateTime<Utc> = timestamp.into();
        assert_eq!(converted_datetime, datetime);
    }

    #[test]
    fn test_timestamp_from_i64() {
        let unix_time: i64 = 1700000000; // Corresponds to some date
        let timestamp = Timestamp::from(unix_time);
        assert_eq!(timestamp.value().timestamp(), unix_time);
    }

    #[test]
    fn test_timestamp_to_i64() {
        let unix_time: i64 = 1700000000; // Corresponds to some date
        let datetime = Utc.timestamp_opt(unix_time, 0).unwrap();
        let timestamp = Timestamp::new(datetime);
        let converted_unix_time: i64 = timestamp.into();
        assert_eq!(converted_unix_time, unix_time);
    }
}
