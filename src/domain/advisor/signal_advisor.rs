mod entry_point;
mod pattern;
mod policy;
mod signal;

use async_trait::async_trait;
pub use entry_point::EntryPoint;
pub use pattern::Pattern;
pub use policy::Policy;
pub use signal::Signal;

use crate::domain::{
    market,
    shared::{Aggregate, DomainError},
};

#[async_trait(?Send)]
pub trait SignalRepository {}

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
    status: SignalAdvisorStatus,
    policy: Box<dyn Policy>,
    patterns: Vec<Box<dyn Pattern>>,
    entry_points: Vec<Box<dyn EntryPoint>>,
}

impl SignalAdvisor {
    pub fn new(id: SignalAdvisorId, policy: Box<dyn Policy>) -> Self {
        Self {
            id,
            status: SignalAdvisorStatus::Inactive,
            policy,
            patterns: Vec::new(),
            entry_points: Vec::new(),
        }
    }

    pub fn status(&self) -> &SignalAdvisorStatus {
        &self.status
    }

    pub fn policy(&self) -> &dyn Policy {
        self.policy.as_ref()
    }
}

pub enum SignalAdvisorCommand<'a> {
    Activate,
    Deactivate,
    Analyze(&'a [market::Candlestick]),
}

pub enum SignalAdvisorEvent {
    Activated,
    Deactivated,
    SignalGenerated(Signal),
}

impl Aggregate for SignalAdvisor {
    type Command = SignalAdvisorCommand<'static>;
    type Event = SignalAdvisorEvent;

    fn handle(&self, command: Self::Command) -> Result<Vec<Self::Event>, DomainError> {
        match command {
            SignalAdvisorCommand::Activate => {
                if self.status == SignalAdvisorStatus::Active {
                    return Err(DomainError {
                        message: "SignalAdvisor is already active".to_string(),
                    });
                }
                Ok(vec![SignalAdvisorEvent::Activated])
            }
            SignalAdvisorCommand::Deactivate => {
                if self.status == SignalAdvisorStatus::Inactive {
                    return Err(DomainError {
                        message: "SignalAdvisor is already inactive".to_string(),
                    });
                }
                Ok(vec![SignalAdvisorEvent::Deactivated])
            }
            SignalAdvisorCommand::Analyze(candles) => {
                if self.status != SignalAdvisorStatus::Active {
                    return Err(DomainError {
                        message: "SignalAdvisor is not active".to_string(),
                    });
                }
                let mut events = Vec::new();
                let mut pattern_detected = Vec::new();
                for pattern in &self.patterns {
                    if pattern.matches(candles) {
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
                    events.push(SignalAdvisorEvent::SignalGenerated(signal));
                };

                Ok(events)
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            SignalAdvisorEvent::Activated => {
                self.status = SignalAdvisorStatus::Active;
            }
            SignalAdvisorEvent::Deactivated => {
                self.status = SignalAdvisorStatus::Inactive;
            }
            _ => {}
        }
    }
}
