use crate::domain::{
    market::{Candlestick, Symbol, Timeframe},
    shared::DomainError,
};
use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait DataFeed {
    async fn retrieve_candles(
        &self,
        symbol: Symbol,
        timeframe: Timeframe,
        len: usize,
    ) -> Result<Vec<Candlestick>, DomainError>;
}
