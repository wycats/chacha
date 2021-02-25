use chacha_core::ChachaValue;

use crate::{cell::Cell, derived::Derived, revision::Revision};

pub enum Reactive<T>
where
    T: ChachaValue,
{
    Cell(Cell<T>),
    Derived(Box<dyn Derived<T>>),
}

impl<T> Reactive<T>
where
    T: ChachaValue,
{
    pub fn is_valid(&self, last_used: Revision) -> bool {
        match self {
            Reactive::Cell(cell) => cell.is_valid(last_used),
            Reactive::Derived(_) => {}
        }
    }
}
