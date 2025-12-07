use crate::{
    application::{self, run_analysis},
    domain::{Symbol, Timeframe},
    interfaces::shared::Present,
};

pub struct SupervisorController<'a, S, P> {
    supervisor_service: &'a S,
    presenter: &'a P,
}

impl<'a, S, P> SupervisorController<'a, S, P>
where
    S: application::Service<run_analysis::Request, run_analysis::Result>,
    P: Present<run_analysis::Result>,
{
    pub fn new(supervisor_service: &'a S, presenter: &'a P) -> Self {
        Self {
            supervisor_service,
            presenter,
        }
    }

    pub async fn run_analysis(
        &self,
        _symbol: String,
        _timeframe: String,
        lookback: usize,
    ) -> <P as Present<run_analysis::Result>>::ViewModel {
        let request = run_analysis::Request {
            symbol: Symbol::EURUSD,
            timeframe: Timeframe::M5,
            lookback,
        };
        let result = self.supervisor_service.execute(request).await;
        self.presenter.present(result)
    }
}
