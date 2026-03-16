use crate::py_abstractions::Textures_and_Images::Image;
use crate::py_abstractions::Loading::FileData::FileData;
use pyo3::prelude::*;
use crate::engine::PChannel::PReceiver;
use pyo3_stub_gen::derive::* ;
use pyo3::PyErr;


crate::generate_pfuture!(Image);
crate::generate_pfuture!(FileData);

#[gen_stub_pyclass]
#[pyclass]
pub struct Timeout();

#[gen_stub_pyclass]
#[pyclass]
pub struct EmptyFuture();




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
                pub fn result_nowait_timeout(&self, timeout: f32) -> PyResult<Option<$res_type>> {
                    let mut guard = self.future.lock().map_err(|_| {
                        pyo3::exceptions::PyRuntimeError::new_err("Mutex poisoned")
                    })?;

                    let rx = guard.as_ref().ok_or_else(|| {
                        pyo3::exceptions::PyRuntimeError::new_err("Result already consumed")
                    })?;

                    match rx.recv_timeout(timeout) {
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

    /// waits for the result, with timeout in seconds.
    #[pyo3(signature = (timeout = f32::MAX))]
    pub fn result_nowait_timeout(&self, timeout: f32) -> PyResult<FutureWaitResult> {
        let mut guard = self.future.lock().map_err(|_| {
            pyo3::exceptions::PyRuntimeError::new_err("Mutex poisoned")
        })?;

        let rx = guard.as_ref().ok_or_else(|| {
            pyo3::exceptions::PyRuntimeError::new_err("Result already consumed")
        })?;

        match rx.recv_timeout(timeout) {
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


#[derive(IntoPyObject)]
pub enum FutureWaitResult {
    Timeout(Timeout),
    Empty(EmptyFuture),
}
use pyo3_stub_gen::{PyStubType, TypeInfo};
impl PyStubType for FutureWaitResult {
    fn type_input() -> TypeInfo {
        TypeInfo::unqualified("Timeout | EmptyFuture")
    }
    fn type_output() -> TypeInfo {
        TypeInfo::unqualified("Timeout | EmptyFuture")
    }
}
