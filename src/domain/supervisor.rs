pub use crate::domain::{
    Candlestick, DomainError,
    supervisor::{
        signal::{Signal, SignalRepository},
        structure::{Structure, StructureRepository},
    },
};
use crate::domain::{
    Direction,
    analysis::{Formation, Pattern},
};

pub use signal::SignalAdvisor;
use structure::StructureAdvisor;

mod advisor;
mod signal;
mod structure;

pub use advisor::*;

pub struct Supervisor {
    struct_advisor: StructureAdvisor,
    signal_advisors: Vec<SignalAdvisor>,
}

impl Supervisor {
    pub fn new(signal_advisors: Vec<SignalAdvisor>) -> Result<Self, DomainError> {
        if signal_advisors.is_empty() {
            return Err(DomainError {
                message: "At least one SignalAdvisor must be provided".to_string(),
            });
        }

        let formations = vec![Formation::Swing(Direction::All)];

        let struct_advisor = StructureAdvisor::new(formations);

        Ok(Self {
            struct_advisor,
            signal_advisors,
        })
    }
}

impl Default for Supervisor {
    fn default() -> Self {
        let signal_advisor = SignalAdvisor::new(
            vec![Formation::Swing(Direction::All)],
            vec![Pattern::BullReversalBar],
        );
        Supervisor::new(vec![signal_advisor]).unwrap()
    }
}

impl Supervisor {
    pub fn run_analysis(
        &self,
        candles: &[Candlestick],
        history: &[Structure],
    ) -> Result<(Vec<Structure>, Vec<Signal>), DomainError> {
        if candles.is_empty() {
            return Err(DomainError {
                message: "Candles data cannot be empty".to_string(),
            });
        }

        let events = self.struct_advisor.evaluates(candles)?;
        let mut new_structures = Vec::new();
        for event in events {
            if let AdvisorEvent::StructureDetected(structure) = event {
                new_structures.push(structure);
            }
        }

        let merged_structures = Supervisor::merge_structures(history, &new_structures);

        let mut signals = Vec::new();

        if let Some(last_structure) = merged_structures.last() {
            for advisor in &self.signal_advisors {
                if advisor.is_activated_on(last_structure.formation()) {
                    let events = advisor.evaluates(candles)?;
                    for event in events {
                        if let AdvisorEvent::SignalGenerated(signal) = event {
                            signals.push(signal);
                        }
                    }
                }
            }
        }
        Ok((merged_structures, signals))
    }

    fn merge_structures(old: &[Structure], new: &[Structure]) -> Vec<Structure> {
        let mut merged = old.to_vec();
        for structure in new {
            if !merged
                .iter()
                .any(|s| s.timestamp() == structure.timestamp())
            {
                merged.push(structure.clone());
            }
        }
        merged.sort_by_key(|s| *s.timestamp());
        merged
    }
}

#[cfg(test)]
mod tests {
    use crate::{candle, domain::analysis::Formation};

    use super::*;

    #[test]
    fn test_supervisor_creation_without_signal_advisors() {
        let result = Supervisor::new(vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_supervisor_creation_with_signal_advisors() {
        let signal_advisor = SignalAdvisor::new(vec![], vec![]);
        let result = Supervisor::new(vec![signal_advisor]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_analysis_with_empty_candle() {
        let signal_advisor = SignalAdvisor::new(vec![], vec![]);
        let supervisor = Supervisor::new(vec![signal_advisor]).unwrap();
        let candles = vec![];
        let structure_history = vec![];
        let result = supervisor.run_analysis(&candles, &structure_history);
        assert!(result.is_err());
    }

    #[test]
    fn test_run_analysis_with_valid_data() {
        let signal_advisor = SignalAdvisor::new(vec![], vec![]);
        let supervisor = Supervisor::new(vec![signal_advisor]).unwrap();
        let candles = vec![
            candle!(1672531200, 100.0, 110.0, 90.0, 105.0, 1000),
            candle!(1672531500, 105.0, 115.0, 95.0, 110.0, 1500),
        ];
        let structure_history = vec![];
        let result = supervisor.run_analysis(&candles, &structure_history);
        assert!(result.is_ok());
    }

    #[test]
    fn test_merge_structures() {
        let old_structures = vec![];
        let new_structures = vec![];
        let merged = Supervisor::merge_structures(&old_structures, &new_structures);
        assert!(merged.is_empty());
    }

    #[test]
    fn test_merge_structures_with_data() {
        let old_structures = vec![
            Structure::new(1672531200.into(), Formation::Dummy),
            Structure::new(1672531500.into(), Formation::Dummy),
        ];
        let new_structures = vec![
            Structure::new(1672531800.into(), Formation::Dummy),
            Structure::new(1672532100.into(), Formation::Dummy),
        ];

        let expected_structures = vec![
            Structure::new(1672531200.into(), Formation::Dummy),
            Structure::new(1672531500.into(), Formation::Dummy),
            Structure::new(1672531800.into(), Formation::Dummy),
            Structure::new(1672532100.into(), Formation::Dummy),
        ];
        let merged = Supervisor::merge_structures(&old_structures, &new_structures);
        assert_eq!(merged, expected_structures);
    }

    #[test]
    fn test_merge_structures_with_overlapping_data() {
        let old_structures = vec![
            Structure::new(1672531200.into(), Formation::Dummy),
            Structure::new(1672531500.into(), Formation::Dummy),
        ];
        let new_structures = vec![
            Structure::new(1672531200.into(), Formation::Dummy),
            Structure::new(1672531500.into(), Formation::Dummy),
        ];
        let expected_structures = vec![
            Structure::new(1672531200.into(), Formation::Dummy),
            Structure::new(1672531500.into(), Formation::Dummy),
        ];
        let merged = Supervisor::merge_structures(&old_structures, &new_structures);
        assert_eq!(merged, expected_structures);
    }
}
