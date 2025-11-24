use crate::domain::shared::{Aggregate, DomainError};

mod policy;
mod entry_point;
mod pattern;

use entry_point::EntryPoint;
use pattern::Pattern;
use policy::Policy;
use std::fmt::Debug;

type AnalyzerId = uuid::Uuid;

/// State of the Analyzer
/// Active: Analyzer is actively analyzing data.
/// Inactive: Analyzer is not analyzing data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnalyzerStatus {
    Active,
    Inactive,
    Paused(String), // Paused with a reason
}

/// Aggregates root
/// Analyzer is responsible for analyzing market data and produce signals.
/// # Fields
/// - id: Unique identifier for the analyzer.
/// - status: Current status of the analyzer (Active/Inactive).
/// - patterns: Collection of patterns used in analysis.
/// - indicators: Collection of indicators used in analysis.
/// - entry_points: Collection of key entry points used in analysis.
#[derive(Debug)]
pub struct Analyzer {
    id: AnalyzerId,
    status: AnalyzerStatus,
    policy: Box<dyn Policy>,
    patterns: Vec<Box<dyn Pattern>>,
    entry_points: Vec<Box<dyn EntryPoint>>,
}

impl Analyzer {
    pub fn new(id: AnalyzerId, policy: Box<dyn Policy>) -> Self {
        Self {
            id,
            status: AnalyzerStatus::Inactive,
            policy,
            patterns: Vec::new(),
            entry_points: Vec::new(),
        }
    }
}

pub enum AnalyzerCommand {
    Activate,
    Deactivate,
}

pub enum AnalyzerEvent {
    Activated,
    Deactivated,
}

impl Aggregate for Analyzer {
    type Command = AnalyzerCommand;
    type Event = AnalyzerEvent;

    fn handle(&self, command: Self::Command) -> Result<Vec<Self::Event>, DomainError> {
        match command {
            AnalyzerCommand::Activate => {
                if self.status == AnalyzerStatus::Active {
                    return Err(DomainError {
                        message: "Analyzer is already active".to_string(),
                    });
                }
                Ok(vec![AnalyzerEvent::Activated])
            }
            AnalyzerCommand::Deactivate => {
                if self.status == AnalyzerStatus::Inactive {
                    return Err(DomainError {
                        message: "Analyzer is already inactive".to_string(),
                    });
                }
                Ok(vec![AnalyzerEvent::Deactivated])
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            AnalyzerEvent::Activated => {
                self.status = AnalyzerStatus::Active;
            }
            AnalyzerEvent::Deactivated => {
                self.status = AnalyzerStatus::Inactive;
            }
        }
    }
}
