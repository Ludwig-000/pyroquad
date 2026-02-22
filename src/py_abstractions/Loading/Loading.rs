use pyo3::prelude::*;
use pyo3_stub_gen::derive::* ;
use crate::engine::PError::PError;

use crate::py_abstractions::Loading::FileData::FileData;

use crate::engine::CoreLoop::COMMAND_QUEUE;
use crate::engine::CoreLoop::Command;


/// Loads a file.
#[cfg(not(any(target_arch = "wasm32", target_os = "ios")))]
#[gen_stub_pyfunction]
#[pyfunction]
pub fn load_file(path: &str)-> PyResult<FileData>{
    match std::fs::read(path) {
        Ok(bytes) => Ok(
            FileData { bytes }
        ),
        Err(e) => {
            Err(PError::BasicErr(
                format!("Failed to load file {path}: {e}")
            ).into())
        }
    }
}



/// Loads a file.
#[cfg(any(target_arch = "wasm32", target_os = "ios"))]
#[gen_stub_pyfunction]
#[pyfunction]
pub fn load_file(path: &str)-> PyResult<FileData>{
    use crate::engine::PChannel::PChannel;

    let (tx, rx) = PChannel::sync_channel(1);
    let p_str = path.to_string();
    COMMAND_QUEUE.push( Command::LoadFile { path: p_str, sender: tx } );
    let res = rx.recv()?;
    return match res{
        Ok(r) => Ok(
            FileData { bytes: r }
        ),
        Err(e) => Err(e.into()),
    }
}




/// Downloads a file and returning it's raw data.
#[gen_stub_pyfunction]
#[pyfunction]
pub fn download_file(url: &str) -> PyResult<FileData> {

    let resp = reqwest::blocking::get(url)
        .map_err(|e| PError::BasicErr(format!("Request failed for {url}: {e}")))?;

    let resp = resp.error_for_status()
        .map_err(|e| PError::BasicErr(format!("HTTP error for {url}: {e}")))?;
    
    let bytes = resp.bytes()
        .map_err(|e| PError::BasicErr(format!("Failed to read body for {url}: {e}")))?;
    
    Ok(
        FileData { bytes: bytes.to_vec() }
    )
}



/// Writes raw data to file.
#[cfg(not(any(target_arch = "wasm32", target_os = "ios")))]
#[gen_stub_pyfunction]
#[pyfunction]
pub fn write_to_file(contents: &FileData, path: String) -> PyResult<()> {
    let res = std::fs::write(path.clone(), contents.bytes.clone()).map_err(|e|{
        PError::BasicErr(format!("Failed to write to file {path}: {e}"))
    })?;
    Ok(())
}