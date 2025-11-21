mod symbol;
mod timeframe;
mod candlestick;

mod rules;
mod commands;
mod events;

use symbol::Symbol;
use timeframe::Timeframe;
use candlestick::Candlestick;
use rules::*;
use commands::MarketCommand::{AddCandlestick};
use events::MarketEvent::{CandlestickAdded};

use crate::domain::shared::{Aggregate, CheckRule, DomainError};


/// Aggregate entity representing a market with its associated candlestick data.
/// # Fields
/// * `id`: Unique identifier for the market.
/// * `symbol`: The trading symbol associated with the market.
/// * `timeframe`: The timeframe for the candlestick data.
/// * `candlestick`: The last candlestick data for the market.
#[derive(Debug)]
pub struct Market {
    symbol: Symbol,
    timeframe: Timeframe,
    candlestick: Option<Candlestick>
}

impl CheckRule for Market {}

impl Market {
    pub fn new(symbol: Symbol, timeframe: Timeframe) -> Self {
        Market {
            symbol,
            timeframe,
            candlestick: None,
        }
    }

    pub fn add_candlestick(&mut self, candlestick: Candlestick) -> Result<(), DomainError> {
        Self::check_rule(OHLCMustBeCompatibleWithTickSize::new(
            &self.symbol,
            &candlestick
        ))?;
        if self.candlestick.is_some() {
            Self::check_rule(CandlestickTimestampMustBeMoreRecentThanExisting::new(
                self.candlestick.as_ref().unwrap().timestamp(),
                candlestick.timestamp(),
            ))?;
        } 
        self.candlestick = Some(candlestick);
        Ok(())
    }
}

impl Aggregate for Market {
    type Command = commands::MarketCommand;
    type Event = events::MarketEvent;

    fn handle(&self, command: Self::Command) -> Result<Vec<Self::Event>, DomainError> {
        match command {
            AddCandlestick { open, high, low, close, volume, timestamp } => {
                let new_candlestick = Candlestick::new(open, high, low, close, volume, timestamp);

                Self::check_rule(OHLCMustBeCompatibleWithTickSize::new(
                    &self.symbol,
                    &new_candlestick
                ))?;
                if self.candlestick.is_some() {
                    Self::check_rule(CandlestickTimestampMustBeMoreRecentThanExisting::new(
                        self.candlestick.as_ref().unwrap().timestamp(),
                        new_candlestick.timestamp(),
                    ))?;
                } 
                Ok(vec![events::MarketEvent::CandlestickAdded { candlestick: new_candlestick }])
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            CandlestickAdded { candlestick } => {
                self.candlestick = Some(candlestick);
            }
        }
    }
}

#[cfg(test)]
mod market_tests {
    use chrono::{Utc};
    use rust_decimal::{dec, Decimal};
    use crate::domain::{market::candlestick::Volume, shared::{Price, Timestamp}};
    use super::*;

    #[test]
    fn test_add_candlestick_success() {
        let symbol = Symbol::new("BTCUSD".to_string(), Decimal::new(1, 2)); // Tick size of 0.01
        let timeframe = Timeframe::M1;
        let mut market = Market::new(symbol, timeframe);

        let candlestick = Candlestick::new(
            Price::new(dec!(50000)),
            Price::new(dec!(50500)),
            Price::new(dec!(49500)),
            Price::new(dec!(50200)),
            Volume::new(dec!(10)),
            Timestamp::new(Utc::now()),
        );
        let result = market.add_candlestick(candlestick);
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_candlestick_invalid_ohlc() {
        let symbol = Symbol::new("BTCUSD".to_string(), Decimal::new(1, 2)); // Tick size of 0.01
        let timeframe = Timeframe::M1;
        let mut market = Market::new(symbol, timeframe);

        let candlestick = Candlestick::new(
            Price::new(dec!(50000.005)), // Invalid open price
            Price::new(dec!(50500)),
            Price::new(dec!(49500)),
            Price::new(dec!(50200)),
            Volume::new(dec!(10)),
            Timestamp::new(Utc::now()),
        );

        let result = market.add_candlestick(candlestick);
        assert!(result.is_err());
    }

    #[test]
    fn test_add_candlestick_invalid_timestamp() {
        let symbol = Symbol::new("BTCUSD".to_string(), Decimal::new(1, 2)); // Tick size of 0.01
        let timeframe = Timeframe::M1;
        let mut market = Market::new(symbol, timeframe);

        let first_candlestick = Candlestick::new(
            Price::new(dec!(50000)),
            Price::new(dec!(50500)),
            Price::new(dec!(49500)),
            Price::new(dec!(50200)),
            Volume::new(dec!(10)),
            Timestamp::new(Utc::now()),
        );
        market.add_candlestick(first_candlestick).unwrap();

        let second_candlestick = Candlestick::new(
            Price::new(dec!(50100)),
            Price::new(dec!(50600)),
            Price::new(dec!(49600)),
            Price::new(dec!(50300)),
            Volume::new(dec!(15)),
            Timestamp::new(Utc::now() - chrono::Duration::minutes(1)), // Older timestamp
        );

        let result = market.add_candlestick(second_candlestick);
        assert!(result.is_err());
    }
}


