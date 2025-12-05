use crate::domain::{DataFeed, SignalRepository, StructureRepository};

pub struct SupervisorController<'a, D, R> {
    datafeed: &'a D,
    repositories: &'a R,
}

impl<'a, D, R> SupervisorController<'a, D, R>
where
    D: DataFeed,
    R: StructureRepository + SignalRepository,
{
    pub fn new(datafeed: &'a D, repositories: &'a R) -> Self {
        Self {
            datafeed,
            repositories,
        }
    }
}
