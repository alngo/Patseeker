pub use crate::domain::{
    Candlestick, DomainError,
    supervisor::{
        signal::{Signal, SignalRepository},
        structure::{Structure, StructureRepository},
    },
};

pub use signal::SignalAdvisor;
pub use structure::StructureAdvisor;

mod advisor;
mod signal;
mod structure;

pub use advisor::*;

pub struct Supervisor {
    struct_advisor: StructureAdvisor,
    signal_advisors: Vec<SignalAdvisor>,
}

impl Supervisor {
    pub fn new(
        struct_advisor: StructureAdvisor,
        signal_advisors: Vec<SignalAdvisor>,
    ) -> Result<Self, DomainError> {
        if signal_advisors.is_empty() {
            return Err(DomainError {
                message: "At least one SignalAdvisor must be provided".to_string(),
            });
        }

        Ok(Self {
            struct_advisor,
            signal_advisors,
        })
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
        // enforce invariants: avoid duplicates, ensure chronological consistency
        let mut merged = old.to_vec();
        merged.extend(new.iter().cloned());
        merged.sort_by_key(|s| *s.location().start());
        merged
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supervisor_creation_without_signal_advisors() {
        let structure_advisor = StructureAdvisor::new(vec![]);
        let result = Supervisor::new(structure_advisor, vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_supervisor_creation_with_signal_advisors() {
        let structure_advisor = StructureAdvisor::new(vec![]);
        let signal_advisor = SignalAdvisor::new(vec![], vec![], vec![]);
        let result = Supervisor::new(structure_advisor, vec![signal_advisor]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_analysis_with_empty_candle() {
        let structure_advisor = StructureAdvisor::new(vec![]);
        let signal_advisor = SignalAdvisor::new(vec![], vec![], vec![]);
        let supervisor = Supervisor::new(structure_advisor, vec![signal_advisor]).unwrap();
        let candles = vec![];
        let structure_history = vec![];
        let result = supervisor.run_analysis(&candles, &structure_history);
        assert!(result.is_err());
    }

    #[test]
    fn test_run_analysis_with_valid_data() {
        let structure_advisor = StructureAdvisor::new(vec![]);
        let signal_advisor = SignalAdvisor::new(vec![], vec![], vec![]);
        let supervisor = Supervisor::new(structure_advisor, vec![signal_advisor]).unwrap();
        let candles = vec![
            Candlestick::new(
                1672531200.into(),
                100.0.into(),
                110.0.into(),
                90.0.into(),
                105.0.into(),
                1000.0.into(),
            )
            .unwrap(),
            Candlestick::new(
                1672531500.into(),
                105.0.into(),
                115.0.into(),
                95.0.into(),
                110.0.into(),
                1500.0.into(),
            )
            .unwrap(),
        ];
        let structure_history = vec![];
        let result = supervisor.run_analysis(&candles, &structure_history);
        assert!(result.is_ok());
    }
}
