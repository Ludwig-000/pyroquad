use std::path::Path;
use std::sync::Mutex;

use lazy_static::lazy_static;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::* ;
use crate::engine::PChannel::PChannel;
use crate::engine::PError::PError;

use crate::py_abstractions::Loading::FileData::FileData;
use crate::py_abstractions::PFuture::{FileDataFuture};

lazy_static!{
    pub static ref PcAssetFolder: Mutex<String> = Mutex::new("".to_string());
}

/// Loads a file.
#[cfg(not(any(target_arch = "wasm32", target_os = "ios")))]
#[gen_stub_pyfunction]
#[pyfunction]
pub fn load_file(path: &str)-> PyResult<FileData>{

    let path = {
        let folder = PcAssetFolder.lock().unwrap();
        if folder.is_empty() {
            path.to_string()
        } else {
            format!("{}/{}", folder, path)
        }
    };

    match std::fs::read(&path) {
        Ok(bytes) => Ok(
            FileData { bytes }
        ),
        Err(e) => {
            Err(PError::BasicErr(
                format!("Failed to load file {}: {}",&path, e)
            ).into())
        }
    }
}


#[gen_stub_pyfunction]
#[pyfunction]
pub fn load_file_future(path: &str) -> PyResult<FileDataFuture> {
    let (tx, rx) = PChannel::sync_channel(1);
    let path_str = path.to_string();

    #[cfg(not(any(target_arch = "wasm32", target_os = "ios")))]
    {
        // On Desktop: Spawn a thread to perform the blocking disk I/O

        use crate::limited_thread;
        limited_thread!(20_000, move || {
            let result = load_file(&path_str);
            let _ = tx.send(result);
        });
        // std::thread::spawn(move || {
        //     let result = load_file(&path_str);
        //     let _ = tx.send(result);
        // });
    }

    #[cfg(any(target_arch = "wasm32", target_os = "ios"))]
    {
        std::thread::spawn(move || {
            let result = load_file(&path_str);
            let _ = tx.send(result);
        });
    }

    Ok(FileDataFuture {
        future: std::sync::Mutex::new(Some(rx)),
    })
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


#[gen_stub_pyfunction]
#[pyfunction]
pub fn download_file_future(url: &str) -> PyResult<FileDataFuture> {
    let (tx, rx) = PChannel::sync_channel(1);
    let url = url.to_string();

    std::thread::spawn(move || {
        let result: PyResult<FileData> = (|| {
            let resp = reqwest::blocking::get(&url)
                .map_err(|e| PError::BasicErr(format!("Request failed for {url}: {e}")))?;

            let resp = resp.error_for_status()
                .map_err(|e| PError::BasicErr(format!("HTTP error for {url}: {e}")))?;
            
            let bytes = resp.bytes()
                .map_err(|e| PError::BasicErr(format!("Failed to read body for {url}: {e}")))?;
            
            Ok(
                FileData { bytes: bytes.to_vec() }
            )
        })();
        let _ = tx.send(result); 
    });

    Ok(FileDataFuture {
        future: std::sync::Mutex::new(Some(rx)),
    })
}


/// Writes raw data to file.
#[cfg(not(any(target_arch = "wasm32", target_os = "ios")))]
#[gen_stub_pyfunction]
#[pyfunction]
pub fn write_to_file(contents: &FileData, path: String) -> PyResult<()> {
    use std::path::Path;


    let path = {
        let folder = PcAssetFolder.lock().unwrap();
        if folder.is_empty() {
            path.to_string()
        } else {
            format!("{}/{}", folder, path)
        }
    };

    // ensure the asset folder exists.
    let p = Path::new(&path);
    if let Some(parent) = p.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            PError::BasicErr(format!("Failed to create directory {:?}: {}", parent, e))
        })?;
    }

    std::fs::write(path.clone(), contents.bytes.clone()).map_err(|e|{
        PError::BasicErr(format!("Failed to write to file {path}: {e}"))
    })?;
    Ok(())
}



#[cfg(not(any(target_arch = "wasm32", target_os = "ios")))]
pub fn does_file_exist(path: &str) -> bool{

    let path = {
        let folder = PcAssetFolder.lock().unwrap();
        if folder.is_empty() {
            path.to_string()
        } else {
            format!("{}/{}", folder, path)
        }
    };
    Path::new(&path).exists()
}