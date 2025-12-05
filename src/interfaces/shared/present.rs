#[allow(dead_code)]
pub trait Present<D> {
    type ViewModel;
    fn present(&self, result: D) -> Self::ViewModel;
}
