mod candlestick;
mod window;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    Bullish,
    Bearish,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Symbol {
    EURUSD,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Timeframe {
    M5,
}

pub use candlestick::Candlestick;
pub use window::Window;

use crate::domain::shared::Aggregate;

pub struct Market {
    pub symbol: Symbol,
    pub timeframe: Timeframe,
    window: Window,
}

impl Market {
    pub fn new(symbol: Symbol, timeframe: Timeframe, lookback: usize) -> Self {
        Self {
            symbol: symbol.clone(),
            timeframe: timeframe.clone(),
            window: Window::new(symbol, timeframe, lookback),
        }
    }
}

pub enum MarketCommand {
    AddCandlestick(Candlestick),
}

pub enum MarketEvent {
    CandlestickAdded(Candlestick),
}

impl Aggregate for Market {
    type Command = MarketCommand;
    type Event = MarketEvent;

    fn handle(&self, command: Self::Command) -> Result<Vec<Self::Event>, super::shared::DomainError> {
        match command {
            MarketCommand::AddCandlestick(_candlestick) => {
                Ok(vec![MarketEvent::CandlestickAdded(_candlestick)])
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            MarketEvent::CandlestickAdded(candlestick) => {
                self.window.add_candlestick(candlestick);
            }
        }
    }
}
