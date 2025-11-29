use crate::domain::{
    supervisor::{signal::Signal, structure::{Structure, StructureAdvisor}}, Candlestick
};

mod advisor;
mod signal;
mod structure;

pub struct Supervisor;

impl Supervisor {
    pub fn run_analysis(
        candles: &[Candlestick],
        old_structures: &[Structure],
    ) -> (Vec<Structure>, Vec<Signal>) {
        let new_structures = StructureAdvisor::evaluate(candles);
        let merged_structures = Supervisor::merge_structures(old_structures, &new_structures);


        let signals = SignalAdvisor::evaluate(candles);
        (merged_structures, signals)
    }

    fn merge_structures(old: &[Structure], new: &[Structure]) -> Vec<Structure> {
        // enforce invariants: avoid duplicates, ensure chronological consistency
        let mut merged = old.to_vec();
        merged.extend(new.iter().cloned());
        merged.sort_by_key(|s| s.start());
        merged
    }
}
