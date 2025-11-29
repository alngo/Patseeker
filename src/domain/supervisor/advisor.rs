use crate::domain::{supervisor::{signal::Signal, structure::Structure}, Candlestick, DomainError};

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
    fn evaluate(candles: &[Candlestick]) -> Result<Vec<AdvisorEvent>, DomainError>;
}
