use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use pyo3::PyErr;


/// Exactly an 'assert!()' but returning a pyothon error instead of panicing.
#[macro_export]
macro_rules! py_assert {
    ($cond:expr) => {
        if !($cond) {
            return Err($crate::engine::PAssert::py_assert_fail(concat!("Assertion failed: ", stringify!($cond)).to_string()));
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        if !($cond) {
            return Err($crate::engine::PAssert::py_assert_fail(format!($($arg)+)));
        }
    };
}

#[inline(never)]
#[cold]
pub fn py_assert_fail(msg: String) -> pyo3::PyErr {
    pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(msg)
}