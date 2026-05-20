use std::time::Duration;

use crate::py_abstractions::Textures_and_Images::Image;
use crate::py_abstractions::Loading::FileData::FileData;
use pyo3::prelude::*;
use crate::engine::PChannel::PReceiver;
use pyo3_stub_gen::derive::* ;
use pyo3::PyErr;

/// TODO: maybe add a trait for internal future handling

crate::generate_pfuture!(Image);
crate::generate_pfuture!(FileData);

//#[gen_stub_pyclass]
//#[pyclass]
//pub struct Timeout();

//#[gen_stub_pyclass]
//#[pyclass]
//pub struct EmptyFuture();


use pyo3::prelude::*;
use pyo3_stub_gen::{PyStubType, TypeInfo};
pub struct Timeout();

impl<'py> IntoPyObject<'py> for Timeout {
    type Target = pyo3::types::PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Ok(pyo3::types::PyString::new(py, "Timeout"))
    }
}

impl PyStubType for Timeout {
    fn type_input() -> TypeInfo { TypeInfo::unqualified("typing.Literal['Timeout']") }
    fn type_output() -> TypeInfo { TypeInfo::unqualified("typing.Literal['Timeout']") }
}

pub struct EmptyFuture();

impl<'py> IntoPyObject<'py> for EmptyFuture {
    type Target = pyo3::types::PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Ok(pyo3::types::PyString::new(py, "EmptyFuture"))
    }
}

impl PyStubType for EmptyFuture {
    fn type_input() -> TypeInfo { TypeInfo::unqualified("typing.Literal['EmptyFuture']") }
    fn type_output() -> TypeInfo { TypeInfo::unqualified("typing.Literal['EmptyFuture']") }
}


pub enum FutureWaitResult {
    Timeout(Timeout),
    Empty(EmptyFuture),
}

// Delegate the conversion to the inner ZSTs (Zero-Sized Types)
impl<'py> IntoPyObject<'py> for FutureWaitResult {
    type Target = pyo3::types::PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            FutureWaitResult::Timeout(t) => t.into_pyobject(py),
            FutureWaitResult::Empty(e) => e.into_pyobject(py),
        }
    }
}

// This is what combines them in the Python .pyi file
impl PyStubType for FutureWaitResult {
    fn type_input() -> TypeInfo {
        TypeInfo::unqualified("typing.Literal['Timeout', 'EmptyFuture']")
    }
    fn type_output() -> TypeInfo {
        TypeInfo::unqualified("typing.Literal['Timeout', 'EmptyFuture']")
    }
}



/// Non-async future.
/// this feature is experimental
#[macro_export]
macro_rules! generate_pfuture {
    ($res_type:ty) => {
        paste::paste! {
            #[gen_stub_pyclass]
            #[pyclass]
            pub struct [<$res_type Future>] {
                pub future: std::sync::Mutex<Option<PReceiver<PyResult<$res_type>>>>,
            }

            #[gen_stub_pymethods]
            #[pymethods]
            impl [<$res_type Future>] {
                pub fn result(&self) -> PyResult<Option<$res_type>> {
                    let mut guard = self.future.lock()
                        .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("Mutex poisoned"))?;

                    let rx = guard.as_ref().ok_or_else(|| {
                        pyo3::exceptions::PyRuntimeError::new_err("Result already consumed")
                    })?;

                    match rx.try_recv() {
                        None => Ok(None),
                        Some(res) => {
                            let _ = guard.take(); 
                            Ok(Some(res??))
                        }
                    }
                }

                pub fn result_nowait(&self) -> PyResult<$res_type> {
                    let mut guard = self.future.lock()
                        .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("Mutex poisoned"))?;
                    
                    let rx = guard.as_ref().ok_or_else(|| {
                        pyo3::exceptions::PyRuntimeError::new_err("Result already consumed")
                    })?;
                    
                    let res = rx.recv();
                    let _ = guard.take();
                    Ok(res??)
                }

                /// waits for the result, with timeout in seconds.
                #[pyo3(signature = (timeout = 0.0))]
                pub fn result_nowait_timeout(&self, timeout: f64) -> PyResult<Option<$res_type>> {
                    let mut guard = self.future.lock().map_err(|_| {
                        pyo3::exceptions::PyRuntimeError::new_err("Mutex poisoned")
                    })?;

                    let rx = guard.as_ref().ok_or_else(|| {
                        pyo3::exceptions::PyRuntimeError::new_err("Result already consumed")
                    })?;

                    match rx.recv_timeout(Duration::from_secs_f64(timeout)) {
                        None => Ok(None),
                        
                        Some(res) => {
                            let _ = guard.take();
                            
                            res.map_err( Into::into )
                            .flatten()
                            .map(Some)
                        }
                    }
                }
            }
        }
    };
}




#[gen_stub_pyclass]
#[pyclass]
pub struct Future {
    pub future: std::sync::Mutex<Option<PReceiver<PyResult<()>>>>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Future {
    // Correctly returns PyResult<Option<EmptyFuture>> -> Stubs: Literal['EmptyFuture'] | None
    pub fn result(&self) -> PyResult<Option<EmptyFuture>> {
        let mut guard = self.future.lock()
            .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("Mutex poisoned"))?;

        let rx = guard.as_ref().ok_or_else(|| {
            pyo3::exceptions::PyRuntimeError::new_err("Result already consumed")
        })?;

        match rx.try_recv() {
            None => Ok(None),
            Some(res) => {
                let _ = guard.take(); 
                let _ = res??;
                Ok(Some( EmptyFuture() ))
            }
        }
    }

    // Correctly returns PyResult<EmptyFuture> -> Stubs: Literal['EmptyFuture']
    pub fn result_nowait(&self) -> PyResult<EmptyFuture> {
        let mut guard = self.future.lock()
            .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("Mutex poisoned"))?;
        
        let rx = guard.as_ref().ok_or_else(|| {
            pyo3::exceptions::PyRuntimeError::new_err("Result already consumed")
        })?;
        
        let res = rx.recv();
        let _ = guard.take();
        let _ = res??;
        Ok(EmptyFuture())
    }

    // Correctly returns PyResult<FutureWaitResult> -> Stubs: Literal['Timeout', 'EmptyFuture']
    /// waits for the result, with timeout in seconds.
    #[pyo3(signature = (timeout = f64::MAX))]
    pub fn result_nowait_timeout(&self, timeout: f64) -> PyResult<FutureWaitResult> {
        let mut guard = self.future.lock().map_err(|_| {
            pyo3::exceptions::PyRuntimeError::new_err("Mutex poisoned")
        })?;

        let rx = guard.as_ref().ok_or_else(|| {
            pyo3::exceptions::PyRuntimeError::new_err("Result already consumed")
        })?;

        match rx.recv_timeout(Duration::from_secs_f64(timeout)) {
            None => Ok(FutureWaitResult::Timeout(Timeout())),
            
            Some(res) => {
                let _ = guard.take();
                let _ = res??;
                Ok(FutureWaitResult::Empty(EmptyFuture()))
            }
        }
    }
}

impl Future {
    pub fn new(receiver: PReceiver<PyResult<()>>) -> Self {
        Future {
            future: std::sync::Mutex::new(Some(receiver)),
        }
    }
}


// #[derive(IntoPyObject)]
// pub enum FutureWaitResult {
//     Timeout(Timeout),
//     Empty(EmptyFuture),
// }
// use pyo3_stub_gen::{PyStubType, TypeInfo};
// impl PyStubType for FutureWaitResult {
//     fn type_input() -> TypeInfo {
//         TypeInfo::unqualified("Timeout | EmptyFuture")
//     }
//     fn type_output() -> TypeInfo {
//         TypeInfo::unqualified("Timeout | EmptyFuture")
//     }
// }



pub trait  FutureTrait<TPlusTimeout, T> {
    fn result_nowait(&self)-> T;
    fn result_timeout(&self, timeout: Duration)-> TPlusTimeout;
    fn result(&self)-> Option<T>;
}