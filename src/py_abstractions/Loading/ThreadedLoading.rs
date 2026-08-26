use pyo3::prelude::*;
 
use pyo3_stub_gen::derive::* ;
use pyo3::types::PyDict;
use pyo3::PyResult;

use std::thread;
use crate::engine::PChannel::PChannel;
use crate::py_abstractions::Loading::FileData::FileData;
use crate::py_abstractions::Loading::Loading::{self as load, download_file, load_file, write_to_file};
use crate::py_abstractions::PFuture::{FileDataFuture, Future as FutureP};



/// Namespace for static Download-related functions.
#[gen_stub_pyclass]
#[pyclass]
pub struct Loading;

#[gen_stub_pymethods]
#[pymethods]
impl Loading {
    
    /// downloads a ressource file and saves it at the given filepath.
    /// Does nothing if the given filepath already exists.
    #[cfg(not(any(target_arch = "wasm32", target_os = "ios")))]
    #[staticmethod]
    pub fn download_file_and_save(url: String, filepath: String) -> PyResult<()> {
        use crate::py_abstractions::Loading::Loading::does_file_exist;
        
        if does_file_exist(&filepath) {
            return Ok(());
        }
        
        let data = download_file(&url)?;
        write_to_file(&data, filepath)
    }

    /// downloads a ressource file and saves it at the given filepath.
    /// Does nothing if the given filepath already exists.
    #[cfg(not(any(target_arch = "wasm32", target_os = "ios")))]
    #[staticmethod]
    pub fn download_file_and_save_future(url: String, filepath: String) -> PyResult<FutureP> {
        use crate::{engine::{PChannel::PChannel, PThreading::{limited_thread, thread_pool}}, py_abstractions::Loading::Loading::does_file_exist};


        let (tx, rx) = PChannel::channel();

        thread_pool(crate::engine::PThreading::TaskType::DOWNLOAD, move || {
            let result = (|| -> PyResult<()> {
                if !does_file_exist(&filepath) {
                    let data = download_file(&url)?;
                    write_to_file(&data, filepath)?;
                }
                Ok(())
            })();
            let _ = tx.send(result);
        });

        Ok(FutureP::new(rx))
    }

    /// TODO: do not download if file exists already.
    #[staticmethod]
    pub fn download_file_and_save_and_load(url: String, filepath: String)-> PyResult<FileData>{
        
        let data = download_file(&url)?;
        write_to_file(&data, filepath)?;
        Ok(data)
    }

    
    #[staticmethod]
    fn download_file(url: &str) -> PyResult<FileData> {
        load::download_file(url)
    }

    #[cfg(not(any(target_arch = "wasm32", target_os = "ios")))]
    #[staticmethod]
    fn download_file_future(url: &str) -> PyResult<FileDataFuture> {
        use crate::{engine::{PChannel::PChannel, PThreading::{limited_thread, thread_pool}}, py_abstractions::Loading::Loading::does_file_exist};
        let (tx, rx) = PChannel::channel();

        let url = url.to_string();
        thread_pool(crate::engine::PThreading::TaskType::DOWNLOAD, move || {
            let res = download_file(&url);
            let _ = tx.send(res);
        });

        Ok( FileDataFuture::new(rx) )
    }


    #[staticmethod]
    pub fn load_multiple_files<'py>(py: Python<'py>, paths: &Bound<'_, PyDict>)-> PyResult<Bound<'py, PyDict>>{
        let mut var_names: Vec<String> = Vec::new();
        let mut path_names: Vec<String> = Vec::new();
        
        for (key, value) in paths {
            let var_name: String = key.extract()?;
            let file_path: String = value.extract()?;
            var_names.push(var_name);
            path_names.push(file_path);
        }

        let res: Vec<FileData> = threaded_map(path_names, &|s: String| {
            load_file(&s) 
        })?;

        
        
        if var_names.len() != res.len() {
            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Mismatch between variable names and file load results length."
            ));
        }


        let result_dict = PyDict::new(py);

        for (var_name, file) in var_names.into_iter().zip(res) {
            result_dict.set_item(var_name, file)?;
        }
        Ok(result_dict)
    }




}


fn threaded_map<T, U, F>(
    items: Vec<T>, 
    op: &F
) -> PyResult<Vec<U>> 
where 
    T: Send + 'static,
    U: Send + 'static,
    F: Fn(T) -> PyResult<U> + Send + Sync + 'static + Clone,
{
    let handles: Vec<_> = items.into_iter().map(|item| {
        let op_clone = op.clone();
        let (sender, receiver) = PChannel::channel();
        thread::spawn(move || {
            let _ = sender.send(op_clone(item));
        });
        receiver
    }).collect();
    
    handles.into_iter().map(|handle| {
        
        let res = ||-> PyResult<U> {
            handle.recv()?
        };
        res()

    }).collect()
}


