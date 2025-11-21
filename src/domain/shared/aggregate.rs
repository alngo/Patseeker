use super::error::DomainError;

#[allow(dead_code)]
pub trait Aggregate {
    type Command;
    type Event;

    fn handle(&self, command: Self::Command) -> Result<Vec<Self::Event>, DomainError>;
    fn apply(&mut self, event: Self::Event);
}
