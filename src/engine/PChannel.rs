use std::sync::atomic::Ordering;
use std::sync::mpsc::{self};
use std::marker::PhantomData;
use std::sync::mpsc::{RecvTimeoutError, SendError};
use std::time::{Duration, Instant};

pub enum PChannelError{
    DeadlockError,
    PanicError,
    SendError,
}


/// Pythonic Channel that returns a python error if something goes wrong.
/// will send an error if the sender gets dropped without sending anything.
pub struct PChannel<T> {
    _marker: PhantomData<T>,
}
pub struct PSender<T> {
    inner: mpsc::Sender<Result<T, PChannelError>>,
}
pub struct PReceiver<T> {
    inner: mpsc::Receiver<Result<T, PChannelError>>,
}

impl<T> PChannel<T>{
    pub fn channel() -> (PSender<T>, PReceiver<T>) {
        let (tx, rx) = mpsc::channel();
        (
            PSender { inner: tx },
            PReceiver { inner: rx }
        )
    }
}


impl<T> PSender<T> {
    pub fn send(&self, t: T) -> Result<(), SendError<Result<T, PChannelError>>> {
        self.inner.send(Ok(t))
    }
}
impl<T> Drop for PSender<T>{
    fn drop(&mut self) {
        if std::thread::panicking() {
            let _ = self.inner.send(Err(PChannelError::PanicError));
        }
    }
}
impl<T> PReceiver<T> {
    pub fn recv(&self) -> Result<T, PChannelError> {
        // In the browser there is no other thread to wait for: whoever was going
        // to answer has already had its turn by the time we get here (see
        // `crate::web`), so the reply is either present or never coming. Falling
        // through to a blocking `recv()` would freeze the tab for good.
        #[cfg(target_os = "emscripten")]
        {
            return match self.inner.try_recv() {
                Ok(result) => result,
                Err(_) if !crate::py_abstractions::py_functions::ENGINE_CURRENTLY_ACTIVE
                    .load(Ordering::Relaxed) => Err(PChannelError::DeadlockError),
                Err(_) => Err(PChannelError::SendError),
            };
        }

        #[allow(unreachable_code)]
        {
        // macOS: on the main thread the engine only runs when we let it. See EngineFiber.
        #[cfg(target_os = "macos")]
        while let Some(turn) = self.engine_turn(Duration::from_millis(1)) {
            match turn {
                Ok(result) => return result,
                Err(RecvTimeoutError::Disconnected) => return Err(PChannelError::SendError),
                Err(RecvTimeoutError::Timeout) => {}
            }
        }

        for _ in 0..1_000 { // we busy wait for roughly the first 10 micro seconds.
            match self.inner.try_recv(){
                Ok(result) => return result,
                Err(_) => {}
            }
        }

        if !crate::py_abstractions::py_functions::ENGINE_CURRENTLY_ACTIVE.load(Ordering::Relaxed){
            return Err(PChannelError::DeadlockError)
        }

        blocking(|| self.inner.recv()).unwrap_or(
            Err(PChannelError::SendError)
        )
        }
    }
    /// macOS, main thread: gives the engine a turn, then waits up to `max` for the reply, which
    /// may also come from another thread. None if there is no engine to run here.
    #[cfg(target_os = "macos")]
    fn engine_turn(&self, max: Duration) -> Option<Result<Result<T, PChannelError>, RecvTimeoutError>> {
        if !crate::engine::EngineFiber::run_engine() {
            return None;
        }
        if let Ok(result) = self.inner.try_recv() {
            return Some(Ok(result));
        }
        Some(blocking(|| self.inner.recv_timeout(max)))
    }

    pub fn try_recv(&self) -> Option<Result<T, PChannelError>>{
        // macOS: give the engine a turn first, as if it had kept running on a thread of its own.
        #[cfg(target_os = "macos")]
        crate::engine::EngineFiber::run_engine();

        self.inner.try_recv().ok()
    }

    /// timeout in seconds
    pub fn recv_timeout(&self, timeout: Duration) -> Option<Result<T, PChannelError>> {
        // macOS: on the main thread the engine only runs when we let it. See EngineFiber.
        #[cfg(target_os = "macos")]
        {
            let deadline = Instant::now().checked_add(timeout);
            loop {
                let left = deadline.map_or(Duration::MAX, |d| d.saturating_duration_since(Instant::now()));
                let Some(turn) = self.engine_turn(left.min(Duration::from_millis(1))) else { break };
                match turn {
                    Ok(result) => return Some(result),
                    Err(RecvTimeoutError::Disconnected) => return Some(Err(PChannelError::SendError)),
                    Err(RecvTimeoutError::Timeout) if left.is_zero() => return None,
                    Err(RecvTimeoutError::Timeout) => {}
                }
            }
        }

        if !crate::py_abstractions::py_functions::ENGINE_CURRENTLY_ACTIVE.load(Ordering::Relaxed) {
            return Some(Err(PChannelError::DeadlockError));
        }

        match blocking(|| self.inner.recv_timeout( timeout )) {
            Ok(val) => return Some(val),
            Err(RecvTimeoutError::Timeout) => None,
            Err(RecvTimeoutError::Disconnected) => {
                return Some(Err(PChannelError::SendError))
            }
        }
        
    }
    
}




/// Blocks in `wait`. On macOS without the GIL: the main thread has to keep running Python, since
/// that is what gives the engine its turns there (see EngineFiber).
fn blocking<R>(wait: impl FnOnce() -> R) -> R {
    #[cfg(target_os = "macos")]
    return crate::engine::EngineFiber::without_gil(wait);

    #[cfg(not(target_os = "macos"))]
    wait()
}


use pyo3::PyErr;
impl From<PChannelError> for pyo3::PyErr {
    fn from(value: PChannelError) -> pyo3::PyErr {
        match value{
            PChannelError::DeadlockError=> {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Deadlock error encounterd. Waiting for a blocking channel that may never resolve, since the engine has not yet been initialized.
                    Make sure to call 'activate_engine()' before making engine calls.".to_string()
                )
            }
            PChannelError::PanicError=> {
                PyErr::new::<pyo3::exceptions::PyBaseException, _>(
                    "Fatal error. The engine crashed and could not recover.".to_string()
                )
            }
            // I do not know when this would ever happen.
            PChannelError::SendError=> {
                PyErr::new::<pyo3::exceptions::PyBaseException, _>(
                    "Sender failed to resolve.".to_string()
                )
            }
        }
    }
}