use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::domain::{
    Candlestick, DomainError, Symbol, Timeframe,
    analysis::{BearTrendForm, BullTrendForm, Evaluate},
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
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    reason: String,
}

impl Structure {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>, reason: String) -> Self {
        Self { start, end, reason }
    }

    pub fn start(&self) -> &DateTime<Utc> {
        &self.start
    }

    pub fn end(&self) -> &DateTime<Utc> {
        &self.end
    }
}

#[derive(Debug)]
pub struct StructureAdvisor {
    looking_for: Vec<Box<dyn Evaluate>>,
}

impl Default for StructureAdvisor {
    fn default() -> Self {
        Self {
            looking_for: vec![Box::new(BullTrendForm)],
        }
    }
}

impl Advisor for StructureAdvisor {
    fn evaluates(&self, candles: &[Candlestick]) -> Result<Vec<AdvisorEvent>, DomainError> {
        let mut events = Vec::new();
        for form in &self.looking_for {
            if form.evaluates(candles) {
                let structure = Structure::new(
                    chrono::Utc::now(),
                    chrono::Utc::now(),
                    format!("Detected form: {}", form.name()),
                );
                events.push(AdvisorEvent::StructureDetected(structure));
            }
        }
        Ok(events)
    }
}
