mod bear_trend_form;
mod bull_trend_form;

pub use bear_trend_form::BearTrendForm;
pub use bull_trend_form::BullTrendForm;
use chrono::{DateTime, Utc};

use crate::domain::analysis::Evaluate;

pub trait Formation: Evaluate {
    fn start(&self) -> &DateTime<Utc>;
    fn end(&self) -> &DateTime<Utc>;
}
