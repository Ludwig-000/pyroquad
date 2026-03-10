use std::ops::Deref;
use std::sync::Mutex;
use std::sync::MutexGuard;

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




new_key_type! { pub struct FunctionKey; }


pub struct FunctionStorage{
    map: SlotMap<FunctionKey, usize>,
    values: Vec<(usize, // raw object pointer as function input.
         Py<PyAny>,     // owned function
         FunctionKey)>, // reverse lookup
}
impl FunctionStorage {
    pub fn new() -> Self {
        Self {
            map: SlotMap::with_key(),
            values: Vec::new(),
        }
    }

    pub fn add(&mut self, target: Bound<'_, PyAny>, func: Py<PyAny>) -> FunctionKey {
        let ptr = target.as_ptr() as usize;
        let index = self.values.len();
        
        let key = self.map.insert(index);
        
        self.values.push((ptr, func, key));
        key
    }

    pub fn remove(&mut self, key: FunctionKey) {
        if let Some(index) = self.map.remove(key) {
            self.values.swap_remove(index);

            if index < self.values.len() {
                let (_, _, moved_key) = &self.values[index];
                if let Some(idx_ref) = self.map.get_mut(*moved_key) {
                    *idx_ref = index;
                }
            }
        }
    }

    pub fn get(&mut self, key: FunctionKey) -> Option<usize>{
        self.map.get(key).copied()
    }

    
    pub fn execute_all(&self, py: Python<'_>) -> PyResult<()> {
        for (ptr_address, callback, _) in self.values.iter() {
            
            // possible Optimization:
            // call the function directly. no checks. ( did not result in a significant performance boost in measuring. )
            // let result_ptr = ffi::PyObject_CallFunctionObjArgs(
            //     func_ptr, 
            //     arg_ptr, 
            //     std::ptr::null_mut::<ffi::PyObject>() // Sentinel
            // );
            unsafe {
                let arg_ptr = *ptr_address as *mut ffi::PyObject;
                
                let target_bound = Bound::from_borrowed_ptr(py, arg_ptr);
                let func_bound = callback.bind(py);
                
                if let Err(e) = func_bound.call1((target_bound,)) {
                    self.report_error(py, &e, func_bound);
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    
    #[cfg(false)]
    /// Should massively speed up execution in theory, if free threading is enabled.
    /// for now, this is still slower than linear execution
    pub fn execute_all(&self, py: Python<'_>) -> PyResult<()> {
        use rayon::prelude::*;
        
        const BATCH_SIZE: usize = 64;

        py.detach(|| {
            self.values.par_chunks(BATCH_SIZE).for_each(|chunk| {
                
                let err=Python::attach(|py| {
                    for (ptr_address, callback, _) in chunk {
                        unsafe {
                            let raw_ptr = *ptr_address as *mut ffi::PyObject;
                            
                            let target_bound = Bound::from_borrowed_ptr(py, raw_ptr);
                            let func_bound = callback.bind(py);
                            
                            if let Err(e) = func_bound.call1((target_bound,)) {
                                self.report_error(py, &e, func_bound);
                                return Err(e);
                            }
                        }
                    }
                    Ok(())
                });
            });
        });

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