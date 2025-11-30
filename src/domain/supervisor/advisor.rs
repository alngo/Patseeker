use crate::domain::{
    Candlestick, DomainError,
    supervisor::{signal::Signal, structure::Structure},
};

#[cfg(test)]
use mockall::automock;

pub enum AdvisorEvent {
    Activated,
    Deactivated,
    Paused(String),
    StructureDetected(Structure),
    SignalGenerated(Signal),
}

#[cfg_attr(test, automock)]
pub trait Advisor {
    fn evaluates(&self, candles: &[Candlestick]) -> Result<Vec<AdvisorEvent>, DomainError>;
}
