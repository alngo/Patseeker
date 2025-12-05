use async_trait::async_trait;
use chrono::{DateTime, Utc};

mod advisor;

pub use advisor::*;

use crate::domain::{
    analysis::Formation, DomainError, Location, Symbol, Timeframe, Timestamp
};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait StructureRepository {
    async fn structures_from(
        &self,
        symbol: &Symbol,
        timeframe: &Timeframe,
        from: DateTime<Utc>,
    ) -> Result<Vec<Structure>, DomainError>;
}

#[derive(Debug, Clone)]
pub struct Structure {
    formation: Formation,
    timestamp: Timestamp,
}

impl Structure {
    pub fn new(timestamp: Timestamp, formation: Formation) -> Self {
        Self {
            timestamp,
            formation,
        }
    }

    pub fn timestamp(&self) -> &Timestamp {
        &self.timestamp
    }

    pub fn formation(&self) -> &Formation {
        &self.formation
    }
}
