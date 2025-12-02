use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::domain::{
    analysis::{Formation}, supervisor::advisor::{Advisor, AdvisorEvent}, Candlestick, DomainError, Location, Symbol, Timeframe
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
    reason: String,
}

impl Structure {
    pub fn new(location: Location, reason: String) -> Self {
        Self { location, reason }
    }

    pub fn location(&self) -> &Location {
        &self.location
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
        for form in &self.looking_for {
            if let Some(location) = form.evaluator().evaluates(candles) {
                let structure = Structure::new(
                    location,
                    format!("Detected form: {}", form.to_string())
                );
                events.push(AdvisorEvent::StructureDetected(structure));
            }
        }
        Ok(events)
    }
}
