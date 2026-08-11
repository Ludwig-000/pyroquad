use std::ops::Deref;
use std::sync::Mutex;
use std::sync::MutexGuard;
use pyo3::types::PyWeakref;
use slotmap::SlotMap;

/// stores functions to be executed by Python each frame.
/// 
/// 
/// 
use pyo3::prelude::*;
use pyo3::ffi;
use std::sync::{OnceLock};
use slotmap::{new_key_type};



static FUN_STORAGE: OnceLock<Mutex<FunctionStorage>> = OnceLock::new();

pub fn get_fun_storage() -> MutexGuard<'static, FunctionStorage> {
    
    let mutex = FUN_STORAGE.get_or_init(|| {
        Mutex::new(FunctionStorage::new())
    });
    
    mutex.lock().unwrap()
}


// The deferred command queue
pub enum StorageCommand {
    // We store Py<PyAny> instead of Bound. It is safe to hold out-of-scope.
    Add { target_weak: Py<PyAny>, func: Py<PyAny>, key: FunctionKey },
    Remove(FunctionKey),
}


new_key_type! { pub struct FunctionKey; }


pub struct FunctionStorage{
    map: SlotMap<FunctionKey, usize>,
    values: Vec<(Py<PyAny>, Py<PyAny>, FunctionKey)>,

    pending_commands: Vec<StorageCommand>,
    is_executing: bool,
}
impl FunctionStorage {
    pub fn new() -> Self {
        Self {
            map: SlotMap::with_key(),
            values: Vec::new(),
            pending_commands: Vec::new(),
            is_executing: false,
        }
    }

    pub fn add(&mut self, target: Bound<'_, PyAny>, func: Py<PyAny>) -> PyResult<FunctionKey> {
        let py = target.py();
    
        // 1. Create weakref safely using Python's standard `weakref.ref`
        let weakref_mod = py.import("weakref")?;
        let weak_bound = weakref_mod.call_method1("ref", (target,))?;
        let target_weak = weak_bound.unbind(); 
        
        let key = self.map.insert(usize::MAX); 
    
        if self.is_executing {
            self.pending_commands.push(StorageCommand::Add { target_weak, func, key });
        } else {
            let index = self.values.len();
            self.map[key] = index;
            self.values.push((target_weak, func, key));
        }
    
        Ok(key)
    }

    pub fn remove(&mut self, key: FunctionKey) {
        if self.is_executing {
            // Defer if we are mid-frame (e.g. from Python Drop/__del__ or arr.clear())
            self.pending_commands.push(StorageCommand::Remove(key));
        } else {
            self.apply_remove(key);
        }
    }

    fn apply_remove(&mut self, key: FunctionKey) {
        if let Some(index) = self.map.remove(key) {
            // If it was usize::MAX, it was added and removed in the same frame!
            if index == usize::MAX { return; } 

            self.values.swap_remove(index);
            if index < self.values.len() {
                let (_, _, moved_key) = &self.values[index];
                if let Some(idx_ref) = self.map.get_mut(*moved_key) {
                    *idx_ref = index;
                }
            }
        }
    }

    fn flush_commands(&mut self) {
        let commands = std::mem::take(&mut self.pending_commands);
        for cmd in commands {
            match cmd {
                StorageCommand::Add { target_weak, func, key } => {
                    // Check if it's still in the map (wasn't removed while pending)
                    if let Some(idx_ref) = self.map.get_mut(key) {
                        let index = self.values.len();
                        *idx_ref = index;
                        self.values.push((target_weak, func, key));
                    }
                }
                StorageCommand::Remove(key) => {
                    self.apply_remove(key);
                }
            }
        }
    }

    pub fn get(&self, key: FunctionKey) -> Option<usize> {
        self.map.get(key).copied()
    }

    pub fn execute_all(py: Python<'_>) -> PyResult<()> {
        // Snapshot tasks safely with clone_ref
        let tasks: Vec<(Py<PyAny>, Py<PyAny>, FunctionKey)> = {
            let mut storage = get_fun_storage();
            storage.is_executing = true;
            storage.values
                .iter()
                .map(|(w, f, k)| (w.clone_ref(py), f.clone_ref(py), *k))
                .collect()
        }; // Lock dropped here
    
        let mut dead_keys = Vec::new();
        let mut run_error = None;
    
        for (weak_target, func, key) in tasks {
            // 2. Explicit type annotation fixes E0282
            let weak_bound: &Bound<'_, PyAny> = weak_target.bind(py);
            
            // Calling the weakref object returns the target or None if GC collected it
            if let Ok(target_bound) = weak_bound.call0() {
                if !target_bound.is_none() {
                    let func_bound = func.bind(py);
                    if let Err(e) = func_bound.call1((target_bound,)) {
                        run_error = Some(e);
                        break;
                    }
                } else {
                    // Object died on the Python side
                    dead_keys.push(key);
                }
            }
        }
    
        // Re-lock to sweep dead keys and apply queued commands
        let mut storage = get_fun_storage();
        storage.is_executing = false;
        
        for key in dead_keys {
            storage.apply_remove(key);
        }
        
        storage.flush_commands();
    
        if let Some(e) = run_error {
            println!("--- SCRIPT ERROR ---");
            println!("Error: {}", e.value(py));
            return Err(e);
        }
    
        Ok(())
    }
}




impl FunctionStorage {
    fn report_error(&self, py: Python<'_>, err: &PyErr, func: &Bound<'_, PyAny>) {
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