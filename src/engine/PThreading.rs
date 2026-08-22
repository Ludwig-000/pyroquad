use core::panic;
use std::{collections::VecDeque, sync::{Condvar, LazyLock, Mutex, OnceLock, RwLock, atomic::{AtomicBool, AtomicUsize}, mpsc}, thread::{self, Thread}, time::{Duration, Instant}};
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};
use crossbeam::queue::SegQueue;

use crate::engine::{FrameInfo::MOUSE_POSITION, PChannel::{PChannel, PReceiver, PSender}};
use std::sync::atomic::Ordering;

pub const MAX_GLOBAL_THREADS: usize = 20_000;



/// types of operation and their corresponding thread limit
#[derive(Clone, Copy)]
#[repr(usize)]
pub enum TaskType {
    DOWNLOAD = 300,
    LOAD = 50,
    CPU_TASK = 64,
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

    /// There is just one tiiiiny bug here. since thready may wait for the result of other threads, we need to weak these threads up sequentially.
    fn release(&self) {
        let mut count = self.count.lock().unwrap();
        *count -= 1;
        self.cvar.notify_one();
    }
}


/// 🔥🚀🧵
pub fn limited_thread<F: FnOnce()->() + Send + 'static>(task: TaskType, fun: F) {
    static CORE_COUNT: LazyLock<usize> = LazyLock::new(|| thread::available_parallelism().map(|a|a.get()).unwrap_or(4));
    static GLOBAL_SEM: Semipor = Semipor::new();
    static DOWNLOAD_SEM: Semipor = Semipor::new();
    static LOAD_SEM: Semipor = Semipor::new();
    static CPU_SEM: Semipor = Semipor::new();
    static GPU_SEM: Semipor = Semipor::new();

    let task_sem = match task {
        TaskType::DOWNLOAD => &DOWNLOAD_SEM,
        TaskType::LOAD => &LOAD_SEM,
        TaskType::CPU_TASK => &CPU_SEM,
        TaskType::GPU_TASK => &GPU_SEM,
    };


    GLOBAL_SEM.acquire(MAX_GLOBAL_THREADS);
    
    std::thread::spawn( move || {

        task_sem.acquire(task as usize);

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













const THREAD_POOL_SIZE: usize = 500;

/// Thread pool
struct Pool{
    last_task_dispatch: Instant,
    threads_currently_executing: usize,
    thread_count: usize,
    task_queue: VecDeque< Box< dyn FnOnce()->() + Send + 'static>>
}



impl Pool{
    pub fn new() -> Pool{
        Pool{
            last_task_dispatch: Instant::now(),
            threads_currently_executing: 0,
            thread_count: 0,
            task_queue: VecDeque::new(),
        }
    }
    pub fn dispatch_new_thread() {

        let (tx,rx) = mpsc::channel();
        
        let _ = thread::spawn( move || {

            {
                let mut pool = ThreadPool.lock().unwrap();
                pool.last_task_dispatch = Instant::now();
            }

            let _ = tx.send(()); // thread sucessfully initialized.

            
            loop { // aquire new task / wait / delete thread.
                let fun = {
                    let mut pool = ThreadPool.lock().unwrap();
                    let task  = pool.task_queue.pop_front();
                    if task.is_none(){ // no task found. groom thread pool.
                        
                        if Instant::now().duration_since( pool.last_task_dispatch ).as_micros() > 10_000  
                            && (pool.thread_count as f32 * 0.7) as usize > pool.threads_currently_executing {
                            pool.thread_count -= 1;
                            
                            break;
                        }
                        
                    } else {
                        pool.threads_currently_executing +=1;
                    }
                    task
                };

                if let Some(fun) = fun{
                    fun();

                    {
                       let mut pool = ThreadPool.lock().unwrap();
                       pool.threads_currently_executing -=1;
                       pool.last_task_dispatch = Instant::now();
                    }
                } else {
                    thread::sleep(Duration::from_millis(1));
                }
            }

        });

        let _ = rx.recv(); // we wait for the thread to initialize


    }
}



static ThreadPool: LazyLock<Mutex<Pool>> = LazyLock::new( || Mutex::new(Pool::new()));


pub fn thread_pool<F: FnOnce()->() + Send + 'static>(task: TaskType, fun: F) {


    let dispatch = {
        let boxed_function = Box::new(fun);
        let mut pool = ThreadPool.lock().unwrap();
        pool.task_queue.push_back(boxed_function);

        
        if pool.thread_count < THREAD_POOL_SIZE && pool.thread_count == pool.threads_currently_executing{
            pool.thread_count += 1;
            true
        } else {
            false
        }
    };


    // groom thread pool
    if dispatch {
        Pool::dispatch_new_thread();
    }

}