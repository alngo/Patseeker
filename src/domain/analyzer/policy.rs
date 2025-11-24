use crate::domain::market::{Direction, Structure};
use std::fmt::Debug;

/// Policy trait defines the behavior for evaluating market structures.
pub trait Policy: Debug {
    fn evaluate(&self, structure: &dyn Structure) -> bool;
}

/// BullPolicy evaluates to true for bullish market structures.
#[derive(Debug)]
pub struct BullPolicy;

impl Policy for BullPolicy {
    fn evaluate(&self, structure: &dyn Structure) -> bool {
        matches!(structure.direction(), Direction::Bullish)
    }
}

/// BearPolicy evaluates to true for bearish market structures.
#[derive(Debug)]
pub struct BearPolicy;

impl Policy for BearPolicy {
    fn evaluate(&self, structure: &dyn Structure) -> bool {
        matches!(structure.direction(), Direction::Bearish)
    }
}

/// RangePolicy evaluates to true for range-bound market structures.
#[derive(Debug)]
pub struct RangePolicy;

impl Policy for RangePolicy {
    fn evaluate(&self, structure: &dyn Structure) -> bool {
        matches!(structure.direction(), Direction::Unknown)
    }
}
