use std::fmt::Display;

#[allow(dead_code)]
pub trait Present<D> {
    type ViewModel: Display;
    fn present(&self, result: D) -> Self::ViewModel;
}
