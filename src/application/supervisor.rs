mod use_cases;

use async_trait::async_trait;
pub use use_cases::run_analysis;

use crate::{
    application::{Service, use_case::UseCase},
    domain::{DataFeed, SignalRepository, StructureRepository, Supervisor},
};

pub struct SupervisorService<'a, D, R> {
    data_feed: &'a D,
    repositories: &'a R,
    supervisor: Supervisor,
}

impl<'a, D, R> SupervisorService<'a, D, R>
where
    D: DataFeed,
    R: StructureRepository + SignalRepository,
{
    pub fn new(data_feed: &'a D, repositories: &'a R) -> Self {
        Self {
            data_feed,
            repositories,
            supervisor: Supervisor::default(),
        }
    }
}

#[async_trait(?Send)]
impl<'a, D, R> Service<run_analysis::Request, run_analysis::Result> for SupervisorService<'a, D, R>
where
    D: DataFeed,
    R: StructureRepository + SignalRepository,
{
    async fn execute(&self, request: run_analysis::Request) -> run_analysis::Result {
        let use_case = run_analysis::RunAnalysis::new(
            self.data_feed,
            self.repositories,
            self.repositories,
            &self.supervisor,
        );

        use_case.execute(request).await
    }
}
