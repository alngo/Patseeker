mod bull_trend_form;
mod swing;

pub use bull_trend_form::BullTrendForm;
pub use swing::Swing;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Formation {
    Swing,
    BullTrendForm,
    BearTrendForm,
}

impl Formation {
    pub fn evaluator(&self) -> Box<dyn crate::domain::analysis::Evaluate> {
        match self {
            Formation::Swing => Box::new(Swing),
            Formation::BullTrendForm => Box::new(BullTrendForm),
            Formation::BearTrendForm => unimplemented!(),
        }
    }
}

impl std::fmt::Display for Formation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Formation::Swing => write!(f, "Swing"),
            Formation::BullTrendForm => write!(f, "Bull Trend Form"),
            Formation::BearTrendForm => write!(f, "Bear Trend Form"),
        }
    }
}
