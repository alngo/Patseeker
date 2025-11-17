use crate::domain::key_entry_point::KeyEntryPoint;

mod generic;

pub trait Indicator: KeyEntryPoint {
    fn update(&mut self, price: f64);
    fn current_value(&self) -> f64;
}
