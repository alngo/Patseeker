use crate::domain::{market, shared::{Aggregate, DomainError}};
use std::fmt::Debug;

mod structure;
mod entry_point;
mod pattern;
mod policy;
mod signal;

use entry_point::EntryPoint;
use pattern::Pattern;
use policy::Policy;
use signal::Signal;

type AdvisorId = uuid::Uuid;

/// State of the Advisor
/// Active: Advisor is actively analyzing data.
/// Inactive: Advisor is not analyzing data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdvisorStatus {
    Active,
    Inactive,
    Paused(String), // Paused with a reason
}

/// Aggregates root
/// Advisor is responsible for analyzing market data and produce signals.
/// # Fields
/// - id: Unique identifier for the analyzer.
/// - status: Current status of the analyzer (Active/Inactive).
/// - patterns: Collection of patterns used in analysis.
/// - indicators: Collection of indicators used in analysis.
/// - entry_points: Collection of key entry points used in analysis.
#[derive(Debug)]
pub struct Advisor {
    id: AdvisorId,
    status: AdvisorStatus,
    policy: Box<dyn Policy>,
    patterns: Vec<Box<dyn Pattern>>,
    entry_points: Vec<Box<dyn EntryPoint>>,
}

impl Advisor {
    pub fn new(id: AdvisorId, policy: Box<dyn Policy>) -> Self {
        Self {
            id,
            status: AdvisorStatus::Inactive,
            policy,
            patterns: Vec::new(),
            entry_points: Vec::new(),
        }
    }
}

pub enum AdvisorCommand<'a> {
    Activate,
    Deactivate,
    Analyze(&[market::Candlestick]),
}

pub enum AdvisorEvent {
    Activated,
    Deactivated,
    SignalGenerated(Signal),
}

impl Aggregate for Advisor {
    type Command = AdvisorCommand<'static>;
    type Event = AdvisorEvent;

    fn handle(&self, command: Self::Command) -> Result<Vec<Self::Event>, DomainError> {
        match command {
            AdvisorCommand::Activate => {
                if self.status == AdvisorStatus::Active {
                    return Err(DomainError {
                        message: "Advisor is already active".to_string(),
                    });
                }
                Ok(vec![AdvisorEvent::Activated])
            }
            AdvisorCommand::Deactivate => {
                if self.status == AdvisorStatus::Inactive {
                    return Err(DomainError {
                        message: "Advisor is already inactive".to_string(),
                    });
                }
                Ok(vec![AdvisorEvent::Deactivated])
            }
            AdvisorCommand::Analyze(candles) => {
                if self.status != AdvisorStatus::Active {
                    return Err(DomainError {
                        message: "Advisor is not active".to_string(),
                    });
                }
                    let mut pattern_detected = Vec::new();
                    for pattern in &self.patterns {
                        if pattern.matches(&candles) {
                            pattern_detected.push(pattern.name().to_string());
                        }
                    }

                    let mut entry_point_triggered = Vec::new();
                    for entry_point in &self.entry_points {
                        if entry_point.validate(&candles[candles.len() - 1]) {
                            entry_point_triggered.push(entry_point.name().to_string());
                        }
                    }

                    if !pattern_detected.is_empty() && !entry_point_triggered.is_empty() {
                        let signal = Signal::new(
                            uuid::Uuid::new_v4(),
                            chrono::Utc::now(),
                            format!(
                                "Patterns detected: {:?}, Entry points triggered: {:?}",
                                pattern_detected, entry_point_triggered,
                            ),
                        );
                    };
                    return Ok(vec![AdvisorEvent::SignalGenerated(Signal::new(
                        uuid::Uuid::new_v4(),
                        chrono::Utc::now(),
                        "Sample signal".to_string(),
                    ))]);
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            AdvisorEvent::Activated => {
                self.status = AdvisorStatus::Active;
            }
            AdvisorEvent::Deactivated => {
                self.status = AdvisorStatus::Inactive;
            }
            AdvisorEvent::SignalGenerated(_signal) => {
                // Handle signal generation if needed
            }
        }
    }
}
