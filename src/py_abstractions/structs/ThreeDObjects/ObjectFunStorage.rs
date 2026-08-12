
/// new design idea:
/// Atomic lock that prevents acessing the mutex ONLY when we are executing functions.
///            if we are executiong ATM, we throw the request into a queue.
///                 both before and after we are done executing, the queue gets cleared.
///             if we are NOT executing functions, we simply wait out the mutex lock.
/// this way, 
///     1) we achieve consistency.
///     2) do not need to fucking clone our functions
///     3) get slower push / pop speed
///     4) we might need some unsafe code though me thinks >_<
///     5) we need to verify pointers in case of "during execution" though, since we could have been destructed by now. 
///                 allthought it may be good to just validate pointers in any case for simplicity.
///                 Either way, we cannot use raw ptr anymore :(
///     6) NEED THE STRICTEST POSSIBLE ATOMIC LOCK, since we ABSOLUTELY cannot mix up instructions.
/// WAIT FUCK
/// adding a function returns a key. we cannot aquire a key without aquiring the storage, therefore we deadlock :skull:


use std::ops::Deref;
use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::Mutex;
use std::sync::MutexGuard;
use pyo3::types::PyWeakref;
use pyo3::types::PyWeakrefReference;
use slotmap::SlotMap;
use pyo3::prelude::*;
use pyo3::ffi;
use std::sync::{OnceLock};
use slotmap::{new_key_type};
use crossbeam::queue::SegQueue;

use crate::engine::PChannel::PSyncSender;


pub fn add_function(target: &Bound<'_, PyAny>, func: Py<PyAny>) -> PyResult<FunctionKey> {
    let weak_target: Bound<'_, PyWeakrefReference> = PyWeakrefReference::new(target)?;
    let weak_target: Bound<'_, PyWeakref> = weak_target.cast_into()?;
    let weak_target: Py<PyWeakref> = weak_target.unbind();


    let mut map = FUN_STORAGE.map.lock().unwrap();

    let key = map.insert( usize::MAX ); // we reserve a key

    FUNCTION_COMMAND_STORAGE.push(
        StorageCommand2::Push(weak_target, func, key)
    );
    Ok(key)
}

pub fn remove_function(key: FunctionKey) {
    FUNCTION_COMMAND_STORAGE.push(
        StorageCommand2::Pop(key)
    );
}

pub fn execute_all_functions(py: Python<'_>) -> PyResult<()> {
    let _guard = ReentrancyGuard::acquire()?; // prevents recursive calls

    { // flush built up commands.
        FunctionStorage::execute_command_storage();
    }

    FunctionStorage::execute_all(py)
    
}





new_key_type! { pub struct FunctionKey; }

enum StorageCommand2{
    Pop(FunctionKey),
    Push(Py<PyWeakref>, Py<PyAny>, FunctionKey),
}



static FUNCTION_COMMAND_STORAGE: LazyLock<SegQueue<StorageCommand2>> = 
    LazyLock::new(SegQueue::new);


static FUN_STORAGE: LazyLock<FunctionStorage> = 
    LazyLock::new( FunctionStorage::new );


struct FunctionStorage{
    map: Mutex<SlotMap<FunctionKey, usize>>,
    values: Mutex<Vec<(
        Py<PyWeakref>, // object
        Py<PyAny>, // function
        FunctionKey // reverse lookup
    )>>,
}
impl FunctionStorage {
    fn new() -> Self {
        Self {
            map: Mutex::new(SlotMap::with_key()),
            values: Mutex::new(Vec::new()),
        }
    }

    #[inline(never)]
    fn execute_command_storage() {

        let mut map = FUN_STORAGE.map.lock().unwrap();
        let mut values = FUN_STORAGE.values.lock().unwrap();

        while let Some(command) = FUNCTION_COMMAND_STORAGE.pop() {
            match command {
                StorageCommand2::Push(target, func, key) => {

                    if let Some(slot) = map.get_mut(key) {
                        let index = values.len();
                        values.push((target, func, key));
                        *slot = index;
                    } else {
                        panic!("how did we get here?")
                    }

                }
                StorageCommand2::Pop(key) => {
                    if let Some(index) = map.remove(key) {
                        if index != usize::MAX && index < values.len() {
                            values.swap_remove(index);
                            if index < values.len() {
                                let swapped_key = values[index].2;
                                if let Some(slot) = map.get_mut(swapped_key) {
                                    *slot = index;
                                }
                            }
                        }
                    } else {
                        panic!("how did we get here?")
                    }
                }
            }
        }
    }


    #[inline(never)]
    fn execute_all(py: Python<'_>) -> PyResult<()> {
        let tasks = FUN_STORAGE.values.lock().unwrap();
    
        for (target_object, callback, _) in tasks.iter() {
            
            if let Some(target_bound) = &target_object.bind(py).upgrade(){
                let func_bound: &Bound<'_, PyAny> = callback.bind(py);

                if let Err(e) = func_bound.call1((target_bound,)) {
                    FunctionStorage::report_error(py, &e, func_bound);
                    return Err(e);
                }
            }
        }
    
        Ok(())
    }

}






impl FunctionStorage {
    fn report_error(py: Python<'_>, err: &PyErr, func: &Bound<'_, PyAny>) {
        println!("\n--- SCRIPT ERROR ---");

        let func_name = func.getattr("__name__")
            .and_then(|n| n.extract::<String>())
            .unwrap_or_else(|_| "unknown_func".into());

        if let Some(tb) = err.traceback(py) {
            let mut last_tb = tb.into_any();

            while let Ok(next) = last_tb.getattr("tb_next") {
                if next.is_none() { break; }
                last_tb = next;
            }

            let lineno: i32 = last_tb.getattr("tb_lineno").and_then(|l| l.extract()).unwrap_or(0);
            
            if let Ok(frame) = last_tb.getattr("tb_frame")
                && let Ok(code) = frame.getattr("f_code")
                    && let Ok(filename) = code.getattr("co_filename").and_then(|f| f.extract::<String>()) {
                        
                        println!("Function: {}", func_name);
                        println!("Location: {} (Line {})", filename, lineno);

                        if let Ok(linecache) = py.import("linecache")
                             && let Ok(line) = linecache.call_method1("getline", (filename, lineno)) {
                                println!("Code:     > {}", line.to_string().trim());
                        }
                        
                    
                }
            
        }

        println!("Error:    {}", err.value(py));
        println!("-----------------------\n");
    }
}




use std::sync::atomic::{AtomicBool, Ordering};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

static IS_EXECUTING: AtomicBool = AtomicBool::new(false);

struct ReentrancyGuard;

impl ReentrancyGuard {
    fn acquire() -> PyResult<Self> {
        if IS_EXECUTING.swap(true, Ordering::AcqRel) {
            Err(PyRuntimeError::new_err(
                "'next_frame' cannot be called inside a tick function, since this would deadlock the engine.",
            ))
        } else {
            Ok(Self)
        }
    }
}

impl Drop for ReentrancyGuard {
    fn drop(&mut self) {
        IS_EXECUTING.store(false, Ordering::Release);
    }
}