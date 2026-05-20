use std::sync::{Mutex, Condvar, OnceLock};


pub const MAX_GLOBAL_THREADS: usize = 20_000;
pub static ACTIVE_THREADS: Mutex<usize> = Mutex::new(0);
pub static CVAR: Condvar = Condvar::new();



/// Creates a thread with a local, aswell as a global thread-count limit
#[macro_export]
macro_rules! limited_thread {
    ($limit:expr, $func:expr) => {{
        use std::sync::{OnceLock, Mutex, Condvar};

        struct ConcurrencyState {
            active_threads: Mutex<usize>,
            cvar: Condvar,
        }

        static STATE: OnceLock<ConcurrencyState> = OnceLock::new();
        
        let state = STATE.get_or_init(|| ConcurrencyState {
            active_threads: Mutex::new(0),
            cvar: Condvar::new(),
        });

        // LOCAL Gatekeeper (Wait until THIS call-site has a slot)
        let mut local_count = state.active_threads.lock().unwrap();
        while *local_count >= $limit {
            local_count = state.cvar.wait(local_count).unwrap();
        }
        *local_count += 1;
        drop(local_count); 

        // GLOBAL Gatekeeper (Wait until the WHOLE SYSTEM has a slot)
        // We use the full path to reach the statics defined above
        let mut global_count = $crate::engine::PThreading::ACTIVE_THREADS.lock().unwrap();
        while *global_count >= $crate::engine::PThreading::MAX_GLOBAL_THREADS {
            global_count = $crate::engine::PThreading::CVAR.wait(global_count).unwrap();
        }
        *global_count += 1;
        drop(global_count);

        // Panic-Safe Guard (Releases both slots even if $func panics)
        struct SlotGuard<'a> {
            local_state: &'a ConcurrencyState,
        }
        
        impl<'a> Drop for SlotGuard<'a> {
            fn drop(&mut self) {
                // Release Global Slot First
                {
                    let mut g_count = $crate::engine::PThreading::ACTIVE_THREADS.lock().unwrap();
                    *g_count -= 1;
                    $crate::engine::PThreading::CVAR.notify_one(); 
                }
                
                // Release Local Slot Second
                {
                    let mut l_count = self.local_state.active_threads.lock().unwrap();
                    *l_count -= 1;
                    self.local_state.cvar.notify_one(); 
                }
            }
        }

        let _guard = SlotGuard { local_state: state };

        // Execute the task
        ($func)()
    }};
}