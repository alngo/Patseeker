use async_trait::async_trait;

use crate::{
    application::shared::{error::ApplicationError},
    domain::{Context, ContextRepository, DataFeed, Signal, SignalRepository, Symbol, Timeframe},
};

pub struct Request {
    pub symbol: Symbol,
    pub timeframe: Timeframe,
    pub lookback: usize,
}

pub struct Response {
    pub contexts: Vec<Context>,
    pub signals: Vec<Signal>,
}

pub type Result = core::result::Result<Response, ApplicationError>;

pub struct RunAnalysis<'a, M, C, S> {
    datafeed: &'a M,
    context_repository: &'a C,
    signal_repository: &'a S,
}

impl<'a, M, C, S> RunAnalysis<'a, M, C, S>
where
    M: DataFeed,
    C: ContextRepository,
    S: SignalRepository,
{
    pub fn new(datafeed: &'a M, context_repository: &'a C, signal_repository: &'a S) -> Self{

        Self {
            datafeed,
            context_repository,
            signal_repository,
        }

    }
}

#[async_trait(?Send)]
impl<'a, M, C, S> UseCase for RunAnalysis<'a, M, C, S>
where
    M: DataFeed,
    C: ContextRepository,
    S: SignalRepository,
{
    type Request = Request;
    type Response = Response;

    async fn execute(&self, request: Request) -> Result {
        let candles = self.datafeed.retrieve_candles(request.symbol, request.timeframe, request.lookback);

        // get last contexts
        // evaluate context
        // merge context
        //
        // pause advisors based on their "act on(Structure)"
        // evaluate signals
        Ok(Response {contexts: Vec::new(), signals: Vec::new()})
    }
}
