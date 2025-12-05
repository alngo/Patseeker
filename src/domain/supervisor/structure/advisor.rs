use mockall::mock;

use crate::domain::{Advisor, AdvisorEvent, Candlestick, DomainError, analysis::Formation};

#[derive(Debug)]
pub struct StructureAdvisor {
    look_for: Vec<Formation>,
}

impl StructureAdvisor {
    pub fn new(look_for: Vec<Formation>) -> Self {
        Self { look_for }
    }

    pub fn look_for(&self) -> &Vec<Formation> {
        &self.look_for
    }
}

impl Advisor for StructureAdvisor {
    fn evaluates(&self, candles: &[Candlestick]) -> Result<Vec<AdvisorEvent>, DomainError> {
        let mut events = Vec::new();
        for formation in &self.look_for {
            let structures = formation.evaluator().evaluates(candles);
            for structure in structures {
                events.push(AdvisorEvent::StructureDetected(structure));
            }
        }
        Ok(events)
    }
}

mock! {
    pub StructureAdvisor {
        pub fn new(look_for: Vec<Formation>) -> Self;

        pub fn look_for(&self) -> &Vec<Formation>;
    }

    impl Advisor for StructureAdvisor {
        fn evaluates(&self, candles: &[Candlestick]) -> Result<Vec<AdvisorEvent>, DomainError>;
    }
}
