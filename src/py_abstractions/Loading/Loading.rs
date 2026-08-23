use std::path::Path;
use std::sync::{LazyLock, Mutex};

use pyo3::prelude::*;
use pyo3_stub_gen::derive::* ;
use crate::engine::PChannel::PChannel;
use crate::engine::PError::PError;

use crate::engine::PThreading::thread_pool;
use crate::py_abstractions::Loading::FileData::FileData;
use crate::py_abstractions::PFuture::{FileDataFuture};

pub static PC_ASSET_FOLDER: LazyLock<Mutex<String>> = 
    LazyLock::new(|| Mutex::new(String::new()));

/// Loads a file.
#[cfg(not(any(target_arch = "wasm32", target_os = "ios")))]
#[gen_stub_pyfunction]
#[pyfunction]
pub fn load_file(path: &str)-> PyResult<FileData>{

    let path = {
        let folder = PC_ASSET_FOLDER.lock().unwrap();
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
    let (tx, rx) = PChannel::channel();
    let path_str = path.to_string();

    #[cfg(not(any(target_arch = "wasm32", target_os = "ios")))]
    {
        // On Desktop: Spawn a thread to perform the blocking disk I/O

        use crate::engine::PThreading::{limited_thread, thread_pool};

        thread_pool(crate::engine::PThreading::TaskType::LOAD, move || {
            let result = load_file(&path_str);
            let _ = tx.send(result);
        });
    }

    #[cfg(any(target_arch = "wasm32", target_os = "ios"))]
    {
        thread_pool( crate::engine::PThreading::TaskType::LOAD,move || {
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


use std::thread;
use std::time::Duration;
use reqwest::blocking::Client;
use pyo3::prelude::*;

static CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .user_agent("PyroquadEngine/1.0")
        .pool_max_idle_per_host(32)
        .build()
        .expect("Failed to initialize HTTP client")
});


/// Downloads a file and returning it's raw data.
#[gen_stub_pyfunction]
#[pyfunction]
pub fn download_file(url: &str) -> PyResult<FileData> {
    let max_retries = 4;
    let mut retry_delay = Duration::from_millis(100);

    for attempt in 1..=max_retries {
        let request = CLIENT.get(url).send();

        match request.and_then(|r| r.error_for_status()).and_then(|r| r.bytes()) {
            Ok(bytes) => return Ok(FileData { bytes: bytes.to_vec() }),
            Err(err) => {
                if attempt == max_retries {
                    return Err(PError::BasicErr(format!("Failed to download {url}: {err}")).into());
                }
                thread::sleep(retry_delay);
                retry_delay *= 2;
            }
        }
    }

    Err(PError::BasicErr(format!("Download timed out for {url}")).into())
}


#[gen_stub_pyfunction]
#[pyfunction]
pub fn download_file_future(url: &str) -> PyResult<FileDataFuture> {
    let (tx, rx) = PChannel::channel();
    let url = url.to_string();

    thread_pool(crate::engine::PThreading::TaskType::DOWNLOAD ,move || {
        let result: PyResult<FileData> = download_file(&url);
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
        let folder = PC_ASSET_FOLDER.lock().unwrap();
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
        let folder = PC_ASSET_FOLDER.lock().unwrap();
        if folder.is_empty() {
            path.to_string()
        } else {
            format!("{}/{}", folder, path)
        }
    };
    Path::new(&path).exists()
}