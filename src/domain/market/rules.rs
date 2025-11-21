use crate::domain::{market::{candlestick::Candlestick, symbol::Symbol}, shared::{Rule, Timestamp}};

pub struct OHLCMustBeCompatibleWithTickSize<'a> {
    symbol: &'a Symbol,
    candlestick: &'a Candlestick,
}

impl<'a> OHLCMustBeCompatibleWithTickSize<'a> {
    pub fn new(symbol: &'a Symbol, candlestick: &'a Candlestick) -> Self {
        OHLCMustBeCompatibleWithTickSize { symbol, candlestick }
    }

}

impl Rule for OHLCMustBeCompatibleWithTickSize<'_> {
    fn is_valid(&self) -> bool {
        let tick_size = self.symbol.tick_size();
        self.candlestick.open().is_compatible(tick_size)
            && self.candlestick.high().is_compatible(tick_size)
            && self.candlestick.low().is_compatible(tick_size)
            && self.candlestick.close().is_compatible(tick_size)
    }

    fn message(&self) -> String {
        format!(
            "OHLC prices must be compatible with the tick size of the symbol {}",
            self.symbol.ticker()
        )
    }
}

pub struct CandlestickTimestampMustBeMoreRecentThanExisting<'a> {
    cur_timestamp: &'a Timestamp,
    new_timestamp: &'a Timestamp,
}

impl<'a> CandlestickTimestampMustBeMoreRecentThanExisting<'a> {
    pub fn new(
        cur_timestamp: &'a Timestamp,
        new_timestamp: &'a Timestamp,
    ) -> Self {
        CandlestickTimestampMustBeMoreRecentThanExisting {
            cur_timestamp,
            new_timestamp,
        }
    }
}

impl Rule for CandlestickTimestampMustBeMoreRecentThanExisting<'_> {
    fn is_valid(&self) -> bool {
        self.new_timestamp > self.cur_timestamp
    }

    fn message(&self) -> String {
        format!("New candlestick timestamp {} must be more recent than the existing one {}",
            self.new_timestamp, self.cur_timestamp)
    }
}
