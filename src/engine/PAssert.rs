use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use pyo3::PyErr;


/// Exactly an 'assert!()' but returning a pyothon error instead of panicing.
#[macro_export]
macro_rules! py_assert {
    ($cond:expr) => {
        if !($cond) {
            return Err( PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(concat!("Assertion failed: ", stringify!($cond))));
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        if !($cond) {
            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!($($arg)+)));
        }
    };
}