/// Represents the unit of time for a timeframe.
/// Possible values are Minute, Hour, and Day.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimeUnit {
    Minute,
    Hour,
    Day,
}

/// Represents a timeframe with a specific length and unit.
/// # Fields
/// - `length`: The length of the timeframe (must be positive).
/// - `unit`: The unit of time for the timeframe (Minute, Hour, Day).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Timeframe {
    length: u32,
    unit: TimeUnit,
}

impl Timeframe {
    pub fn new(length: u32, unit: TimeUnit) -> Result<Self, String> {
        if length == 0 {
            return Err("Timeframe length must be positive".into());
        }
        Ok(Self { length, unit })
    }

    pub fn code(&self) -> String {
        match self.unit {
            TimeUnit::Minute => format!("M{}", self.length),
            TimeUnit::Hour   => format!("H{}", self.length),
            TimeUnit::Day    => format!("D{}", self.length),
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn unit(&self) -> &TimeUnit {
        &self.unit
    }
}

#[cfg(test)]
mod tests {
    use super::{TimeUnit, Timeframe};

    #[test]
    fn test_timeframe_creation_success() {
        let timeframe = Timeframe::new(15, TimeUnit::Minute).unwrap();
        assert_eq!(timeframe.length(), 15);
        assert_eq!(timeframe.unit(), &TimeUnit::Minute);
        assert_eq!(timeframe.code(), "M15");
    }

    #[test]
    fn test_timeframe_creation_failure_zero_length() {
        let result = Timeframe::new(0, TimeUnit::Hour);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Timeframe length must be positive");
    }

    #[test]
    fn test_timeframe_code_generation() {
        let tf_minute = Timeframe::new(5, TimeUnit::Minute).unwrap();
        assert_eq!(tf_minute.code(), "M5");

        let tf_hour = Timeframe::new(2, TimeUnit::Hour).unwrap();
        assert_eq!(tf_hour.code(), "H2");

        let tf_day = Timeframe::new(1, TimeUnit::Day).unwrap();
        assert_eq!(tf_day.code(), "D1");
    }
}
