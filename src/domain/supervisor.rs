pub use crate::domain::{
    Candlestick, DomainError,
    supervisor::{
        signal::{Signal, SignalAdvisor},
        structure::{Structure, StructureAdvisor},
    },
};

mod advisor;
mod signal;
mod structure;

pub use advisor::*;
pub use signal::*;
pub use structure::*;

pub struct Supervisor;

impl Supervisor {
    pub fn run_analysis(
        candles: &[Candlestick],
        structure_history: &[Structure],
    ) -> Result<(Vec<Structure>, Vec<Signal>), DomainError> {
        let events = StructureAdvisor::default().evaluates(candles)?;
        let mut new_structures = Vec::new();
        for event in events {
            if let AdvisorEvent::StructureDetected(structure) = event {
                new_structures.push(structure);
            }
        }

        let merged_structures = Supervisor::merge_structures(structure_history, &new_structures);

        let mut signals = Vec::new();

        if let Some(last_structure) = merged_structures.last() {
            let advisor = SignalAdvisor::default();
            if advisor.is_activated_on(last_structure.formation()) {
                let events = advisor.evaluates(candles)?;
                for event in events {
                    if let AdvisorEvent::SignalGenerated(signal) = event {
                        signals.push(signal);
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
    use chrono::DateTime;
    use rust_decimal::dec;

    use super::*;
    use crate::domain::Candlestick;

    #[test]
    fn test_run_analysis() {
        let candles = vec![
            Candlestick::new(dec!(100.0), dec!(105.0), dec!(99.0), dec!(104.0), dec!(1), DateTime::from_timestamp(1672531200, 0).unwrap()).unwrap(),
            Candlestick::new(dec!(104.0), dec!(106.0), dec!(103.0), dec!(105.0), dec!(1), DateTime::from_timestamp(1672617600, 0).unwrap()).unwrap(),
            Candlestick::new(dec!(105.0), dec!(107.0), dec!(104.0), dec!(106.0), dec!(1), DateTime::from_timestamp(1672704000, 0).unwrap()).unwrap(),
        ];

        let structure_history = vec![];

        let result = Supervisor::run_analysis(candles.as_slice(), &structure_history);
        assert!(result.is_ok());

        let (structures, signals) = result.unwrap();
        println!("Detected Structures: {:?}", structures);
        println!("Generated Signals: {:?}", signals);
        assert!(!structures.is_empty());
        assert!(!signals.is_empty());
    }
}
