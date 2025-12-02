use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::domain::{
    Candlestick, DomainError, Location, Symbol, Timeframe,
    analysis::Formation,
    supervisor::advisor::{Advisor, AdvisorEvent},
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
    location: Location,
    formation: Formation,
    reason: String,
}

impl Structure {
    pub fn new(location: Location, formation: Formation, reason: String) -> Self {
        Self {
            location,
            formation,
            reason,
        }
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn formation(&self) -> &Formation {
        &self.formation
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Debug)]
pub struct StructureAdvisor {
    looking_for: Vec<Formation>,
}

impl Default for StructureAdvisor {
    fn default() -> Self {
        Self {
            looking_for: vec![Formation::BullTrendForm],
        }
    }
}

impl Advisor for StructureAdvisor {
    fn evaluates(&self, candles: &[Candlestick]) -> Result<Vec<AdvisorEvent>, DomainError> {
        let mut events = Vec::new();
        for formation in &self.looking_for {
            if let Some(location) = formation.evaluator().evaluates(candles) {
                let structure = Structure::new(
                    location,
                    *formation,
                    format!("Detected form: {}", formation),
                );
                events.push(AdvisorEvent::StructureDetected(structure));
            }
        }
        Ok(events)
    }
}
