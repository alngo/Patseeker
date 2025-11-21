use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};

/// Value object representing a timestamp.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    pub fn new(datetime: DateTime<Utc>) -> Self {
        Self(datetime)
    }

    pub fn now() -> Self {
        Self(Utc::now())
    }
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
