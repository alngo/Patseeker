use async_trait::async_trait;

use crate::{
    application::shared::{error::ApplicationError, use_case::UseCase},
    domain::{
        Advisor, AdvisorEvent, DataFeed, Signal, SignalAdvisor, SignalRepository, Structure,
        StructureAdvisor, StructureRepository, Symbol, Timeframe,
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
}

impl<'a, M, C, S> RunAnalysis<'a, M, C, S>
where
    M: DataFeed,
    C: StructureRepository,
    S: SignalRepository,
{
    pub fn new(datafeed: &'a M, structure_repository: &'a C, signal_repository: &'a S) -> Self {
        Self {
            datafeed,
            structure_repository,
            signal_repository,
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
        let mut structures = self
            .structure_repository
            .from(&request.symbol, &request.timeframe, from)
            .await?;

        let structures_detected = StructureAdvisor::default().evaluate(&candles)?;

        for event in structures_detected {
            if let AdvisorEvent::StructureDetected(structure) = event {
                structures.push(structure);
            }
        }

        let signals_generated = SignalAdvisor::default().evaluate(&candles)?;

        let mut signals = Vec::new();
        for event in signals_generated {
            if let AdvisorEvent::SignalGenerated(signal) = event {
                signals.push(signal);
            }
        }

        Ok(Response {
            structures,
            signals,
        })
    }
}
