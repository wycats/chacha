use std::fmt::Debug;
use std::hash::Hash;

pub trait ChachaValue: Debug + Copy + Clone + Hash + Eq + Ord + Send + 'static {}

impl<T> ChachaValue for T where T: Debug + Copy + Clone + Hash + Eq + Ord + Send + 'static {}

pub trait ChachaObject: Debug + Clone + Hash + Eq + Ord + Send + 'static {}

impl<T> ChachaObject for T where T: Debug + Clone + Hash + Eq + Ord + Send + 'static {}

pub use chacha_macros::{chacha_obj, chacha_value, unit_test};
pub use pretty_assertions;

#[macro_export]
macro_rules! unit_tests {
    (all({ $($all:tt)* }), $tests:ident($(( $name:tt, { $($tokens:tt)* } )),*)) => {
        #[cfg(test)]
        mod $tests {
            #[allow(unused)]
            use super::*;

            use $crate::unit_test;
            use $crate::pretty_assertions::assert_eq;

            $($all)*

            $(
                $crate::unit_test! { $name, || { $($tokens)* } }
            )*
        }
    };

    ($tests:ident($($tokens:tt)*)) => {
        unit_tests!(all({}), $tests($($tokens)*));
    }
}
