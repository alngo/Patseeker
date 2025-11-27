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

    pub fn window(&self) -> &Window {
        &self.window
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

    fn handle(
        &self,
        command: Self::Command,
    ) -> Result<Vec<Self::Event>, super::shared::DomainError> {
        match command {
            MarketCommand::AddCandlestick(candlestick) => {
                if let Some(last) = self.window.candles().last()
                    && candlestick.timestamp() <= last.timestamp()
                {
                    return Err(super::shared::DomainError {
                        message: "Candlestick timestamp must be greater than the last candlestick"
                            .to_string(),
                    });
                }
                Ok(vec![MarketEvent::CandlestickAdded(candlestick)])
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
