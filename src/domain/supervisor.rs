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

        let events = SignalAdvisor::default().evaluates(candles)?;
        let mut signals = Vec::new();
        for event in events {
            if let AdvisorEvent::SignalGenerated(signal) = event {
                signals.push(signal);
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
