mod entry_point;
mod pattern;
mod signal;
mod repository;

pub use entry_point::*;
pub use pattern::*;
use rust_decimal::dec;
pub use signal::Signal;
pub use repository::SignalRepository;

use crate::domain::{
    Advisor, AdvisorEvent, Candlestick,
    shared::DomainError,
};


type SignalAdvisorId = uuid::Uuid;

/// State of the SignalAdvisor
/// Active: SignalAdvisor is actively analyzing data.
/// Inactive: SignalAdvisor is not analyzing data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignalAdvisorStatus {
    Active,
    Inactive,
    Paused(String), // Paused with a reason
}

/// Aggregates root
/// SignalAdvisor is responsible for analyzing market data and produce signals.
/// # Fields
/// - id: Unique identifier for the analyzer.
/// - status: Current status of the analyzer (Active/Inactive).
/// - patterns: Collection of patterns used in analysis.
/// - entry_points: Collection of key entry points used in analysis.
#[derive(Debug)]
pub struct SignalAdvisor {
    id: SignalAdvisorId,
    patterns: Vec<Box<dyn Pattern>>,
    entry_points: Vec<Box<dyn EntryPoint>>,
}

impl SignalAdvisor {
    pub fn new(id: SignalAdvisorId) -> Self {
        Self {
            id,
            patterns: Vec::new(),
            entry_points: Vec::new(),
        }
    }
}

impl Default for SignalAdvisor {
    fn default() -> Self {
        Self { 
            id: Default::default(),
            patterns: vec![Box::new(BullReversalPattern)],
            entry_points: vec![Box::new(Level::new(dec!(1), dec!(2)).unwrap())],
        }
    }
}

impl Advisor for SignalAdvisor {
    fn evaluate(&self, candles: &[Candlestick]) -> Result<Vec<AdvisorEvent>, DomainError> {
        let mut events = Vec::new();
        for pattern in &self.patterns {
            if pattern.matches(candles) {
                for entry_point in &self.entry_points {
                    if entry_point.validate(&candles[candles.len() - 1]) {
                        let signal = Signal::new(
                            uuid::Uuid::new_v4(),
                            chrono::Utc::now(),
                            format!(
                                "Patterns detected"
                            ),
                        );
                        events.push(AdvisorEvent::SignalGenerated(signal));
                    }
                }
            }
        }
        Ok(events)
    }
}
