use crate::domain::market::{Candlestick, Symbol, Timeframe};

/// Entity to
/// # Fields
/// - symbol: The financial instrument symbol (e.g., EURUSD).
/// - timeframe: The timeframe for the candlesticks (e.g., M5).
/// - lookback: The number of candlesticks to retain in the window.
/// - candles: A vector storing the candlestick data.
#[derive(Debug)]
pub struct Window {
    symbol: Symbol,
    timeframe: Timeframe,
    lookback: usize,
    candles: Vec<Candlestick>,
}

impl Window {
    pub fn new(symbol: Symbol, timeframe: Timeframe, lookback: usize) -> Self {
        Self {
            symbol,
            timeframe,
            lookback,
            candles: Vec::new(),
        }
    }

    pub fn add_candlestick(&mut self, candlestick: Candlestick) {
        self.candles.push(candlestick);
        if self.candles.len() > self.lookback {
            self.candles.remove(0);
        }
    }

    pub fn candles(&self) -> &[Candlestick] {
        &self.candles
    }
}
