mod swing;
mod pivot;

pub use swing::Swing;

use crate::domain::{analysis::Evaluate, Direction, Structure};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Formation {
    Swing(Direction),
}

impl Formation {
    pub fn evaluator(&self) -> Box<dyn Evaluate<Structure>> {
        match self {
            Formation::Swing(_) => Box::new(Swing),
        }
    }
}

impl std::fmt::Display for Formation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Formation::Swing(dir) => write!(f, "Swing {dir}"),
        }
    }
}
