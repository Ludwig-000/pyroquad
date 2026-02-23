use std::sync::atomic::Ordering;
use std::sync::mpsc::{self};
use std::marker::PhantomData;
use std::sync::mpsc::SendError;
use std::time::Duration;

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
pub struct PSyncSender<T> {
    inner: mpsc::SyncSender<Result<T, PChannelError>>,
}
pub struct PReceiver<T> {
    inner: mpsc::Receiver<Result<T, PChannelError>>,
}

impl<T> PChannel<T>{
    pub fn sync_channel(bound: usize) -> (PSyncSender<T>, PReceiver<T>) {
        let (tx, rx) = mpsc::sync_channel(bound);
        (
            PSyncSender { inner: tx },
            PReceiver { inner: rx }
        )
    }
}


impl<T> PSyncSender<T> {
    pub fn send(&self, t: T) -> Result<(), SendError<Result<T, PChannelError>>> {
        self.inner.send(Ok(t))
    }
}
impl<T> Drop for PSyncSender<T>{
    fn drop(&mut self) {
        let _ = self.inner.send(Err(PChannelError::PanicError));
    }
}
impl<T> PReceiver<T> {
    pub fn recv(&self) -> Result<T, PChannelError> {
        loop{
            let res  = self.inner.recv_timeout(Duration::from_millis(100));

            match res {
                Ok(val)=> return val,
                Err(e)=> {
                    if !crate::py_abstractions::py_functions::ENGINE_CURRENTLY_ACTIVE.load(Ordering::Relaxed){ 
                        return Err(PChannelError::DeadlockError)  
                    }
                }
            }

        }
    }

    
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