use crate::domain::{
    market,
    shared::{Aggregate, DomainError}, Symbol, Timeframe,
};

mod context;
mod structure;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
pub use context::Context;
pub use structure::Structure;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait ContextRepository {
    fn from(&self, symbol: Symbol, timeframe: Timeframe, from: DateTime<Utc>);
}

type ContextAdvisorId = uuid::Uuid;

/// State of the ContextAdvisor
/// Active: ContextAdvisor is actively analyzing data.
/// Inactive: ContextAdvisor is not analyzing data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextAdvisorStatus {
    Active,
    Inactive,
    Paused(String), // Paused with a reason
}

/// Aggregates root
/// ContextAdvisor is responsible for analyzing market data and produce context insight.
/// # Fields
/// - id: Unique identifier for the analyzer.
/// - status: Current status of the analyzer (Active/Inactive).
#[derive(Debug)]
pub struct ContextAdvisor {
    id: ContextAdvisorId,
    status: ContextAdvisorStatus,
    structures: Vec<Box<dyn Structure>>,
}

impl ContextAdvisor {
    pub fn new(id: ContextAdvisorId) -> Self {
        Self {
            id,
            status: ContextAdvisorStatus::Inactive,
            structures: Vec::new(),
        }
    }

    pub fn status(&self) -> &ContextAdvisorStatus {
        &self.status
    }
}

pub enum ContextAdvisorCommand<'a> {
    Activate,
    Deactivate,
    Analyze(&'a [market::Candlestick]),
}

pub enum ContextAdvisorEvent {
    Activated,
    Deactivated,
    ContextDetected(Context),
}

impl Aggregate for ContextAdvisor {
    type Command = ContextAdvisorCommand<'static>;
    type Event = ContextAdvisorEvent;

    fn handle(&self, command: Self::Command) -> Result<Vec<Self::Event>, DomainError> {
        match command {
            ContextAdvisorCommand::Activate => {
                if self.status == ContextAdvisorStatus::Active {
                    return Err(DomainError {
                        message: "ContextAdvisor is already active".to_string(),
                    });
                }
                Ok(vec![ContextAdvisorEvent::Activated])
            }
            ContextAdvisorCommand::Deactivate => {
                if self.status == ContextAdvisorStatus::Inactive {
                    return Err(DomainError {
                        message: "ContextAdvisor is already inactive".to_string(),
                    });
                }
                Ok(vec![ContextAdvisorEvent::Deactivated])
            }
            ContextAdvisorCommand::Analyze(candles) => {
                let mut events = Vec::new();
                for structure in &self.structures {
                    if structure.matches(candles) {
                        let context = Context::new(
                            uuid::Uuid::new_v4(),
                            chrono::Utc::now(),
                            format!("Detected structure: {}", structure.name()),
                        );
                        events.push(ContextAdvisorEvent::ContextDetected(context));
                    }
                }
                Ok(events)
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            ContextAdvisorEvent::Activated => {
                self.status = ContextAdvisorStatus::Active;
            }
            ContextAdvisorEvent::Deactivated => {
                self.status = ContextAdvisorStatus::Inactive;
            }
            _ => {}
        }
    }
}
