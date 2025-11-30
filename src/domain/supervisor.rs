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

pub struct Supervisor;
pub use advisor::*;
pub use signal::*;
pub use structure::*;

impl Supervisor {
    pub fn run_analysis(
        candles: &[Candlestick],
        old_structures: &[Structure],
    ) -> Result<(Vec<Structure>, Vec<Signal>), DomainError> {
        let events = StructureAdvisor::evaluates(candles)?;
        let mut new_structures = Vec::new();
        for event in events {
            match event {
                AdvisorEvent::StructureDetected(structure) => {
                    new_structures.push(structure);
                }
                _ => {}
            }
        }

        let merged_structures = Supervisor::merge_structures(old_structures, &new_structures);

        let events = SignalAdvisor::evaluates(candles)?;
        let mut signals = Vec::new();
        for event in events {
            match event {
                AdvisorEvent::SignalGenerated(signal) => {
                    signals.push(signal);
                }
                _ => {}
            }
        }

        Ok((merged_structures, signals))
    }

    fn merge_structures(old: &[Structure], new: &[Structure]) -> Vec<Structure> {
        // enforce invariants: avoid duplicates, ensure chronological consistency
        let mut merged = old.to_vec();
        merged.extend(new.iter().cloned());
        merged.sort_by_key(|s| *s.start());
        merged
    }
}
