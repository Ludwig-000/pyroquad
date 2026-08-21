use std::{sync::{Condvar, LazyLock, Mutex, OnceLock, RwLock, atomic::{AtomicBool, AtomicUsize}}, thread::{self, Thread}, time::Duration};
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};
use crate::engine::PChannel::{PChannel, PReceiver, PSender};
use std::sync::atomic::Ordering;

pub const MAX_GLOBAL_THREADS: usize = 20_000;

/// types of operation and their corresponding thread limit
#[derive(Clone, Copy)]
#[repr(usize)]
pub enum TaskType {
    DOWNLOAD = 300,
    LOAD = 50,
    CPU_HEAVY_TASK = 64,
    GPU_TASK = 1,
}

struct Semipor {
    count: Mutex<usize>,
    cvar: Condvar,
}
impl Semipor {
    const fn new() -> Self {
        Self {
            count: Mutex::new(0),
            cvar: Condvar::new(),
        }
    }

    fn acquire(&self, limit: usize) {
        let mut count = self.count.lock().unwrap();
        while *count >= limit {
            count = self.cvar.wait(count).unwrap();
        }
        *count += 1;
    }

    fn release(&self) {
        let mut count = self.count.lock().unwrap();
        *count -= 1;
        self.cvar.notify_one();
    }
}

pub fn limited_thread<F: FnOnce()->() + Send + 'static>(task: TaskType, fun: F) {

    static GLOBAL_SEM: Semipor = Semipor::new();
    static DOWNLOAD_SEM: Semipor = Semipor::new();
    static LOAD_SEM: Semipor = Semipor::new();
    static CPU_SEM: Semipor = Semipor::new();
    static GPU_SEM: Semipor = Semipor::new();

    let task_sem = match task {
        TaskType::DOWNLOAD => &DOWNLOAD_SEM,
        TaskType::LOAD => &LOAD_SEM,
        TaskType::CPU_HEAVY_TASK => &CPU_SEM,
        TaskType::GPU_TASK => &GPU_SEM,
    };


    GLOBAL_SEM.acquire(MAX_GLOBAL_THREADS);
    task_sem.acquire(task as usize);

    
    std::thread::spawn( move || {

        let result = catch_unwind(AssertUnwindSafe(|| {
            fun();
        }));

        task_sem.release();
        GLOBAL_SEM.release();

        if let Err(err) = result {
            resume_unwind(err);
        }
    });

}


