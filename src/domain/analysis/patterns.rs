mod bull_reversal_bar;

use bull_reversal_bar::BullReversalBar;

use crate::domain::{Location, analysis::Evaluate};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Pattern {
    BullReversalBar,
    BearReversalBar,
}

impl Pattern {
    pub fn evaluator(&self) -> Box<dyn Evaluate<Location>> {
        match self {
            Pattern::BullReversalBar => Box::new(BullReversalBar),
            Pattern::BearReversalBar => unimplemented!(),
        }
    }
}

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pattern::BullReversalBar => write!(f, "Bull Reversal Bar"),
            Pattern::BearReversalBar => write!(f, "Bear Reversal Bar"),
        }
    }
}
