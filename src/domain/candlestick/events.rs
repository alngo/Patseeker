pub enum Event {
    CandlestickCreated {
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
        timestamp: u64,
    },
}
