use crate::domain::market::Direction;
use std::fmt::Debug;

/// Policy trait defines the behavior for evaluating market structures.
pub trait Policy: Debug {
    fn evaluate(&self, direction: &Direction) -> bool;
}

/// BullPolicy evaluates to true for bullish market structures.
#[derive(Debug)]
pub struct BullPolicy;

impl Policy for BullPolicy {
    fn evaluate(&self, direction: &Direction) -> bool {
        matches!(direction, Direction::Bullish)
    }
}

/// BearPolicy evaluates to true for bearish market structures.
#[derive(Debug)]
pub struct BearPolicy;

impl Policy for BearPolicy {
    fn evaluate(&self, direction: &Direction) -> bool {
        matches!(direction, Direction::Bullish)
    }
}

/// RangePolicy evaluates to true for range-bound market structures.
#[derive(Debug)]
pub struct RangePolicy;

impl Policy for RangePolicy {
    fn evaluate(&self, direction: &Direction) -> bool {
        matches!(direction, Direction::Bullish)
    }
}
