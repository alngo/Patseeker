use async_trait::async_trait;

use crate::{
    application::shared::{error::ApplicationError, use_case::UseCase},
    domain::{
        DataFeed, Signal, SignalRepository, Structure, StructureRepository, Supervisor, Symbol,
        Timeframe,
    },
};

pub struct Request {
    pub symbol: Symbol,
    pub timeframe: Timeframe,
    pub lookback: usize,
}

pub struct Response {
    pub structures: Vec<Structure>,
    pub signals: Vec<Signal>,
}

pub type Result = core::result::Result<Response, ApplicationError>;

pub struct RunAnalysis<'a, M, C, S> {
    datafeed: &'a M,
    structure_repository: &'a C,
    signal_repository: &'a S,
    supervisor: Supervisor,
}

impl<'a, M, C, S> RunAnalysis<'a, M, C, S>
where
    M: DataFeed,
    C: StructureRepository,
    S: SignalRepository,
{
    pub fn new(
        datafeed: &'a M,
        structure_repository: &'a C,
        signal_repository: &'a S,
        supervisor: Supervisor,
    ) -> Self {
        Self {
            datafeed,
            structure_repository,
            signal_repository,
            supervisor,
        }
    }
}

#[async_trait(?Send)]
impl<'a, M, C, S> UseCase for RunAnalysis<'a, M, C, S>
where
    M: DataFeed,
    C: StructureRepository,
    S: SignalRepository,
{
    type Request = Request;
    type Response = Response;

    async fn execute(&self, request: Request) -> Result {
        let candles = self
            .datafeed
            .candles(&request.symbol, &request.timeframe, request.lookback)
            .await?;

        let from = candles[0].timestamp();
        let structures = self
            .structure_repository
            .structures_from(&request.symbol, &request.timeframe, from.into())
            .await?;

        let (structures, signals) = self
            .supervisor
            .run_analysis(&candles, &structures)
            .map_err(|e| ApplicationError { message: e.message })?;

        Ok(Response {
            structures,
            signals,
        })
    }
}
