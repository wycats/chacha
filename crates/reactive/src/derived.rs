use chacha_core::{chacha_obj, unit_tests, ChachaValue};

use crate::{cell::Cell, timeline::Timeline};

pub trait Derived<T>
where
    T: ChachaValue,
{
    fn get(&self, timeline: &mut Timeline) -> T;
}

#[chacha_obj]
pub struct DerivedWithArg<F, Ret, Arg>
where
    F: Fn(&mut Timeline, Arg) -> Ret,
    Ret: ChachaValue,
    Arg: ChachaValue,
{
    function: F,
    arg: Cell<Arg>,
}

impl<F, Ret, Arg> Derived<Ret> for DerivedWithArg<F, Ret, Arg>
where
    F: Fn(&mut Timeline, Arg) -> Ret,
    Ret: ChachaValue,
    Arg: ChachaValue,
{
    fn get(&self, timeline: &mut Timeline) -> Ret {
        self.get(timeline)
    }
}

impl<F, Ret, Arg> DerivedWithArg<F, Ret, Arg>
where
    F: Fn(&mut Timeline, Arg) -> Ret,
    Ret: ChachaValue,
    Arg: ChachaValue,
{
    pub fn new(function: F, arg: Cell<Arg>) -> DerivedWithArg<F, Ret, Arg> {
        DerivedWithArg { function, arg }
    }

    pub(crate) fn get(&self, timeline: &mut Timeline) -> Ret {
        let arg = self.arg.get(timeline);

        (self.function)(timeline, arg)
    }
}

unit_tests!(
    all({
        fn uppercase(_: &mut Timeline, string: &'static str) -> &'static str {
            match string {
                "hello world" => "HELLO WORLD",
                _ => "OMG",
            }
        }
    }),
    tests(("Derived#get", {
        let mut timeline = Timeline::new();
        let cell = timeline.cell("hello world");
        let derived = timeline.derived(uppercase, cell);
        assert_eq!(timeline.get(&derived), "HELLO WORLD");
    }))
);
