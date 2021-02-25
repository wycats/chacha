use chacha_core::{chacha_obj, unit_tests, ChachaValue};

use crate::{revision::Revision, timeline::Timeline};

#[chacha_obj]
pub struct Cell<T>
where
    T: ChachaValue,
{
    value: T,
    revision: Revision,
}

impl<T> Cell<T>
where
    T: ChachaValue,
{
    pub(crate) fn new(value: impl Into<T>, revision: impl Into<Revision>) -> Cell<T> {
        Cell {
            value: value.into(),
            revision: revision.into(),
        }
    }

    pub(crate) fn get(&self, _timeline: &mut Timeline) -> T {
        self.value
    }

    pub(crate) fn set(&mut self, revision: Revision, new_value: T) {
        self.value = new_value;
        self.revision = revision;
    }

    pub(crate) fn is_valid(&self, last_used: Revision) -> bool {
        last_used.is_newer_than(self.revision)
    }
}

unit_tests!(
    all({
        use crate::reactive::Reactive;
    }),
    tests(
        ("Cell#get", {
            let mut timeline = Timeline::new();
            let cell = timeline.cell("hello world");
            assert_eq!(cell.get(&mut timeline), "hello world");
        }),
        ("Cell#set", {
            let mut timeline = Timeline::new();
            let mut cell = timeline.cell("hello world");

            timeline.set(&mut cell, "goodbye world");
            assert_eq!(timeline.get(&Reactive::Cell(cell)), "goodbye world");
        })
    )
);
