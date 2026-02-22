use pyo3::prelude::*;
 
use pyo3_stub_gen::derive::* ;
use pyo3::types::PyDict;
use pyo3::PyResult;

use std::thread;
use crate::py_abstractions::Loading::FileData::FileData;
use crate::py_abstractions::Loading::Loading::{self as load, download_file, load_file, write_to_file};


/// Namespace for static Download-related functions.
#[gen_stub_pyclass]
#[pyclass]
pub struct Loading;

#[gen_stub_pymethods]
#[pymethods]
impl Loading {
    
    /// downloads a ressource file and saves it at the given filepath.
    /// Does nothing if the given filepath already exists.
    #[staticmethod]
    pub fn download_file_and_save(url: String, filepath: String)-> PyResult<()>{
        let data = download_file(&url)?;
        write_to_file(&data, filepath)
    }


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

        let res: Vec<FileData> = threaded_map(path_names, |s: String| {
            load_file(&s) 
        })?;

        
        
        if var_names.len() != res.len() {
            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Mismatch between variable names and file load results length."
            ));
        }


        let result_dict = PyDict::new(py);

        for (var_name, file) in var_names.into_iter().zip(res.into_iter()) {
            result_dict.set_item(var_name, file)?;
        }
        Ok(result_dict)
    }




}


fn threaded_map<T, U, F>(
    items: Vec<T>, 
    op: F
) -> PyResult<Vec<U>> 
where 
    T: Send + 'static,
    U: Send + 'static,
    F: Fn(T) -> PyResult<U> + Send + Sync + 'static + Clone,
{
    let handles: Vec<_> = items.into_iter().map(|item| {
        let op_clone = op.clone();
        thread::spawn(move || {
            op_clone(item)
        })
    }).collect();
    
    handles.into_iter().map(|handle| {
        
       handle.join().map_err(|panic_payload| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                format!("Worker thread panicked: {:?}", panic_payload)
            )
        })?

    }).collect()
}




/// all the structs to make from_dict work
/// 
/// 
#[derive(FromPyObject)]
struct VarFileUrl {
    var_name: String,
    file_path: String,
    url: String,
}
#[derive(FromPyObject)]
struct VarFile {
    var_name: String,
    file_path: String,
}

#[derive(FromPyObject)]
struct FileUrl {
    file_path: String,
    url: String,
}