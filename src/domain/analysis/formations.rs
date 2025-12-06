mod pivot;
mod swing;

pub use swing::Swing;

use crate::domain::{Direction, Structure, analysis::Evaluate};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Formation {
    Dummy,
    Swing(Direction),
}

impl Formation {
    pub fn evaluator(&self) -> Box<dyn Evaluate<Structure>> {
        match self {
            Formation::Swing(_) => Box::new(Swing),
            Formation::Dummy => todo!(),
        }
    }
}

impl std::fmt::Display for Formation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Formation::Swing(dir) => write!(f, "Swing {dir}"),
            Formation::Dummy => todo!(),
        }
    }
}
