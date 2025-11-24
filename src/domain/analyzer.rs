use crate::domain::{
    analyzer::signal::Signal,
    shared::{Aggregate, DomainError},
};

mod entry_point;
mod indicator;
mod pattern;
mod signal;
mod structure;

type AnalyzerId = uuid::Uuid;

/// State of the Analyzer
/// Active: Analyzer is actively analyzing data.
/// Inactive: Analyzer is not analyzing data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnalyzerStatus {
    Active,
    Inactive,
}

/// Aggregates root
/// Analyzer is responsible for analyzing market data and produce signals.
/// # Fields
/// - id: Unique identifier for the analyzer.
/// - status: Current status of the analyzer (Active/Inactive).
/// - structures: Collection of structures used in analysis.
/// - patterns: Collection of patterns used in analysis.
/// - indicators: Collection of indicators used in analysis.
/// - entry_points: Collection of key entry points used in analysis.
/// - signals: Collection of generated signals. ??
#[derive(Debug, Clone, Hash)]
pub struct Analyzer {
    id: AnalyzerId,
    status: AnalyzerStatus,
    structures: Vec<Box<dyn Structure>>,
    patterns: Vec<Box<dyn Pattern>>,
    indicators: Vec<Box<dyn Indicator>>,
    entry_points: Vec<Box<dyn EntryPoint>>,
    signals: Vec<Signal>,
}

impl Analyzer {
    pub fn new(id: AnalyzerId) -> Self {
        Self {
            id,
            status: AnalyzerStatus::Inactive,
            structures: Vec::new(),
            patterns: Vec::new(),
            indicators: Vec::new(),
            entry_points: Vec::new(),
            signals: Vec::new(),
        }
    }
}

/// Analyzer commands and queries
/// - Activate
/// - Deactivate
/// - AddStructure
/// - RemoveStructure
/// - AddPattern
/// - RemovePattern
/// - AddIndicator
/// - RemoveIndicator
/// - AddEntryPoint
/// - RemoveEntryPoint
/// - Analyze
pub enum AnalyzerCommand {
    Activate,
    Deactivate,
    Analyze,
}

/// Analyzer events
/// - Activated
/// - Deactivated
/// - StructureAdded
/// - StructureRemoved
/// - PatternAdded
/// - PatternRemoved
/// - IndicatorAdded
/// - IndicatorRemoved
/// - EntryPointAdded
/// - EntryPointRemoved
/// - SignalGenerated
pub enum AnalyzerEvent {
    Activated,
    Deactivated,
    SignalGenerated(Signal),
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
            AnalyzerCommand::Analyze => {
                // Analysis logic would go here
                let signal = Signal::new(
                    uuid::Uuid::new_v8(self.hash()),
                    chrono::Utc::now(),
                    "Sample analysis reason".to_string(),
                );
                Ok(vec![AnalyzerEvent::SignalGenerated(signal)])
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
            AnalyzerEvent::SignalGenerated(signal) => {
                // do nothing for now
                // signal should not belong to analyzer 
                // but rather persisted in a separate repository
            }
        }
    }
}
