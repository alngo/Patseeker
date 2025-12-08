use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::domain::{Candlestick, DataFeed, DomainError, SignalRepository, Structure, StructureRepository, Symbol, Timeframe};

pub struct InMemory;

#[async_trait(?Send)]
impl StructureRepository for InMemory {
    async fn save_structures(
        &self,
        symbol: &Symbol,
        timeframe: &Timeframe,
        structures: Vec<Structure>,
    ) -> Result<(), DomainError> {
        let _ = (symbol, timeframe, structures);
        Ok(())
    }

    async fn structures_from(
        &self,
        symbol: &Symbol,
        timeframe: &Timeframe,
        from: DateTime<Utc>,
    ) -> Result<Vec<Structure>, DomainError> {
        let _ = (symbol, timeframe, from);
        Ok(vec![])
    }
}

#[async_trait(?Send)]
impl SignalRepository for InMemory {
}

#[async_trait(?Send)]
impl DataFeed for InMemory {
    async fn candles(
        &self,
        symbol: &Symbol,
        timeframe: &Timeframe,
        len: usize,
    ) -> Result<Vec<Candlestick>, DomainError> {
        let _ = (symbol, timeframe, len);
        Ok(vec![])
    }
}

