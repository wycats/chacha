use chacha_core::ChachaValue;

use crate::{cell::Cell, derived::DerivedWithArg, reactive::Reactive, revision::Revision};

pub struct Timeline {
    current: Revision,
}

impl Timeline {
    pub fn new() -> Timeline {
        Timeline {
            current: Revision::from(1),
        }
    }

    pub fn cell<T>(&self, value: T) -> Cell<T>
    where
        T: ChachaValue,
    {
        Cell::new(value, self.current)
    }

    pub fn derived<F, Ret, Arg>(&self, function: F, arg: Cell<Arg>) -> Reactive<Ret>
    where
        F: Fn(&mut Timeline, Arg) -> Ret + 'static,
        Ret: ChachaValue,
        Arg: ChachaValue,
    {
        Reactive::Derived(Box::new(DerivedWithArg::new(function, arg)))
    }

    pub fn get<'cell, T>(&mut self, reactive: &Reactive<T>) -> T
    where
        T: ChachaValue,
    {
        match reactive {
            Reactive::Cell(c) => c.get(self),
            Reactive::Derived(d) => d.get(self),
        }
    }

    pub fn set<'cell, T>(&mut self, cell: &'cell mut Cell<T>, value: T)
    where
        T: ChachaValue,
    {
        let next = self.current.next();
        self.current = next;
        cell.set(next, value)
    }
}
