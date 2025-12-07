use crate::{application::run_analysis, interfaces::shared::Present};

#[derive(Debug, Default)]
pub struct Presenter;

impl Present<run_analysis::Result> for Presenter {
    type ViewModel = String;

    fn present(&self, result: run_analysis::Result) -> Self::ViewModel {
        match result {
            Ok(response) => format!(
                "Analysis completed successfully.\nStructures found: {}\nSignals found: {}",
                response.structures.len(),
                response.signals.len()
            ),
            Err(e) => format!("Analysis failed: {}", e),
        }
    }
}
