use chrono::{DateTime, Utc};

use crate::domain::{analysis::{BearTrendForm, BullTrendForm}, supervisor::advisor::{Advisor, AdvisorEvent}, Candlestick, DomainError};

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
pub struct StructureAdvisor;

impl Advisor for StructureAdvisor {
    fn evaluate(candles: &[Candlestick]) -> Result<Vec<AdvisorEvent>, DomainError> {
        let forms =  vec![Box::new(BullTrendForm), Box::new(BearTrendForm)];
        let mut events = Vec::new();
        for form in forms {
            if form.matches(candles) {
                let structure = Structure::new(
                    uuid::Uuid::new_v4(),
                    chrono::Utc::now(),
                    format!("Detected form: {}", form.name()),
                );
                events.push(AdvisorEvent::StructureDetected(structure));
            }
        }
        Ok(events)
    }
}
