mod bull_trend_form;

pub use bull_trend_form::BullTrendForm;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Formation {
    BullTrendForm,
    BearTrendForm,
}

impl Formation {
    pub fn evaluator(&self) -> Box<dyn crate::domain::analysis::Evaluate> {
        match self {
            Formation::BullTrendForm => Box::new(BullTrendForm),
            Formation::BearTrendForm => unimplemented!(),
        }
    }
}

impl std::fmt::Display for Formation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Formation::BullTrendForm => write!(f, "Bull Trend Form"),
            Formation::BearTrendForm => write!(f, "Bear Trend Form"),
        }
    }
}
