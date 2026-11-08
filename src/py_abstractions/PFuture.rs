use std::sync::{Mutex, MutexGuard, PoisonError};
use std::time::Duration;

use pyo3::prelude::*;
use pyo3::sync::PyOnceLock;
use pyo3::IntoPyObjectExt;

use crate::engine::PChannel::{PChannelError, PReceiver};
use crate::py_abstractions::Loading::FileData::FileData;
use crate::py_abstractions::Textures_and_Images::Image;


// pyo3's "experimental-inspect" (stub generation) is only on in builds that pass an abi_* feature.
#[cfg(any(feature = "abi_314", feature = "abi_310", feature = "abi_39"))]
const STATUS_TYPE: pyo3::inspect::PyStaticExpr =
    pyo3::type_hint_identifier!("pyroquad.future_status", "Status");


/// Rust side of `pyroquad.future_status.Status`, a plain Python `Enum`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Status {
    Pending,
    Done,
}

static PENDING: PyOnceLock<Py<PyAny>> = PyOnceLock::new();
static DONE: PyOnceLock<Py<PyAny>> = PyOnceLock::new();

impl<'py> IntoPyObject<'py> for Status {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;

    #[cfg(any(feature = "abi_314", feature = "abi_310", feature = "abi_39"))]
    const OUTPUT_TYPE: pyo3::inspect::PyStaticExpr = STATUS_TYPE;

    fn into_pyobject(self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let (cell, name) = match self {
            Status::Pending => (&PENDING, "Pending"),
            Status::Done => (&DONE, "Done"),
        };
        let member = cell.get_or_try_init(py, || -> PyResult<Py<PyAny>> {
            Ok(py.import("pyroquad.future_status")?.getattr("Status")?.getattr(name)?.unbind())
        })?;
        Ok(member.bind(py).clone())
    }
}

/// `Status.Pending` on its own, for futures that have a value. Typed separately so the stubs
/// say `Image | Literal[Status.Pending]`: after ruling out `Pending`, a type checker knows the
/// result is the value, without also asking about `Status.Done`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Pending;

impl<'py> IntoPyObject<'py> for Pending {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;

    #[cfg(any(feature = "abi_314", feature = "abi_310", feature = "abi_39"))]
    const OUTPUT_TYPE: pyo3::inspect::PyStaticExpr = pyo3::type_hint_subscript!(
        pyo3::type_hint_identifier!("typing", "Literal"),
        pyo3::inspect::PyStaticExpr::Attribute { value: &STATUS_TYPE, attr: "Pending" }
    );

    fn into_pyobject(self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        Status::Pending.into_pyobject(py)
    }
}


/// How long `resolve` may block.
enum Wait {
    Poll,
    Forever,
    Timeout(Duration),
}

impl From<Option<f64>> for Wait {
    fn from(timeout: Option<f64>) -> Self {
        match timeout {
            None => Wait::Forever,
            Some(secs) => Wait::Timeout(
                Duration::try_from_secs_f64(secs.max(0.0)).unwrap_or(Duration::MAX)
            ),
        }
    }
}


/// A `PReceiver<PyResult<T>>` with `T` erased, so `Future` can hold any of them.
trait ErasedReceiver: Send {
    /// None while nothing has arrived.
    fn receive(&self, py: Python<'_>, wait: &Wait) -> Option<Result<PyResult<Py<PyAny>>, PChannelError>>;
}

impl<T> ErasedReceiver for PReceiver<PyResult<T>>
where
    T: for<'py> IntoPyObject<'py> + Send + 'static,
{
    fn receive(&self, py: Python<'_>, wait: &Wait) -> Option<Result<PyResult<Py<PyAny>>, PChannelError>> {
        let received = match wait {
            Wait::Poll => self.try_recv()?,
            Wait::Forever => self.recv(),
            Wait::Timeout(timeout) => self.recv_timeout(*timeout)?,
        };
        Some(received.map(|result| result.and_then(|value| value.into_py_any(py))))
    }
}


enum State {
    Waiting(Box<dyn ErasedReceiver>),
    Ready(Py<PyAny>),
    Failed(PyErr),
}


#[pyclass(subclass, frozen)]
pub struct Future {
    state: Mutex<State>,
}

impl Future {
    fn new<T>(receiver: PReceiver<PyResult<T>>) -> Self
    where
        T: for<'py> IntoPyObject<'py> + Send + 'static,
    {
        Future { state: Mutex::new(State::Waiting(Box::new(receiver))) }
    }

    /// Locks the state, first receiving the outcome if it is there (waiting as `wait` allows).
    fn update(&self, py: Python<'_>, wait: Wait) -> PyResult<MutexGuard<'_, State>> {
        // Nothing in here can leave the state half-written, so a poisoned lock is still usable.
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);

        if let State::Waiting(receiver) = &*state {
            *state = match receiver.receive(py, &wait) {
                None => return Ok(state),
                // The engine was never started: a bug in the caller, and the work may still
                // finish later, so raise without caching it.
                Some(Err(PChannelError::DeadlockError)) => {
                    return Err(PChannelError::DeadlockError.into())
                }
                Some(Err(channel_error)) => State::Failed(channel_error.into()),
                Some(Ok(Err(error))) => State::Failed(error),
                Some(Ok(Ok(value))) => State::Ready(value),
            };
        }
        Ok(state)
    }

    /// The value once it is there, None while pending. Raises the work's exception if it failed.
    fn resolve(&self, py: Python<'_>, wait: Wait) -> PyResult<Option<Py<PyAny>>> {
        match &*self.update(py, wait)? {
            State::Waiting(_) => Ok(None),
            State::Ready(value) => Ok(Some(value.clone_ref(py))),
            State::Failed(error) => Err(error.clone_ref(py)),
        }
    }
}

#[pymethods]
impl Future {
    /// True once the work has finished, successfully or not. Never blocks, never raises
    /// the work's exception (`poll()` does that).
    #[getter]
    fn done(&self, py: Python<'_>) -> PyResult<bool> {
        Ok(!matches!(&*self.update(py, Wait::Poll)?, State::Waiting(_)))
    }
}


/// Future that has no value to hand back, only whether it is finished.
#[pyclass(extends = Future, frozen)]
pub struct EmptyFuture;

impl EmptyFuture {
    pub fn new(receiver: PReceiver<PyResult<()>>) -> PyResult<Py<Self>> {
        Python::attach(|py| {
            Py::new(py, PyClassInitializer::from(Future::new(receiver)).add_subclass(EmptyFuture))
        })
    }

    fn finish(slf: &Bound<'_, Self>, wait: Wait) -> PyResult<Status> {
        Ok(match slf.as_super().get().resolve(slf.py(), wait)? {
            None => Status::Pending,
            Some(_) => Status::Done,
        })
    }
}

#[pymethods]
impl EmptyFuture {
    /// Returns `Status.Done` or `Status.Pending`. Never blocks.
    /// Raises the work's exception if it failed.
    fn poll(slf: &Bound<'_, Self>) -> PyResult<Status> {
        Self::finish(slf, Wait::Poll)
    }

    /// Blocks until the work is finished, or for at most `timeout` seconds.
    /// Returns `Status.Pending` only if the timeout ran out. Raises the work's exception if it failed.
    #[pyo3(signature = (timeout = None))]
    fn wait(slf: &Bound<'_, Self>, timeout: Option<f64>) -> PyResult<Status> {
        Self::finish(slf, Wait::from(timeout))
    }
}


/// Declares a future class for one value type: `typed_future!(Image)` makes `ImageFuture`.
macro_rules! typed_future {
    ($value:ident) => {
        paste::paste! {
            /// What `poll()` / `wait()` return: the value, or `Status.Pending`.
            #[derive(IntoPyObject)]
            pub enum [<$value Poll>] {
                Ready(Py<$value>),
                Pending(Pending),
            }

            #[doc = "A future to " $value ". It starts working as soon as it is created.\n\n"
                    "`poll()` and `wait()` return the " $value " once it is there, otherwise `Status.Pending`.\n"
                    "If the work failed, they raise the same exception the blocking function would have."]
            #[pyclass(extends = Future, frozen)]
            pub struct [<$value Future>];

            impl [<$value Future>] {
                pub fn new(receiver: PReceiver<PyResult<$value>>) -> PyResult<Py<Self>> {
                    Python::attach(|py| {
                        Py::new(py, PyClassInitializer::from(Future::new(receiver)).add_subclass([<$value Future>]))
                    })
                }

                fn finish(slf: &Bound<'_, Self>, wait: Wait) -> PyResult<[<$value Poll>]> {
                    let py = slf.py();
                    Ok(match slf.as_super().get().resolve(py, wait)? {
                        None => [<$value Poll>]::Pending(Pending),
                        Some(value) => [<$value Poll>]::Ready(value.into_bound(py).cast_into::<$value>()?.unbind()),
                    })
                }
            }

            #[pymethods]
            impl [<$value Future>] {
                #[doc = "Returns the " $value " if it is there, otherwise `Status.Pending`. Never blocks.\n"
                        "Raises the work's exception if it failed."]
                fn poll(slf: &Bound<'_, Self>) -> PyResult<[<$value Poll>]> {
                    Self::finish(slf, Wait::Poll)
                }

                #[doc = "Blocks until the " $value " is there, or for at most `timeout` seconds.\n"
                        "Returns `Status.Pending` only if the timeout ran out. Raises the work's exception if it failed."]
                #[pyo3(signature = (timeout = None))]
                fn wait(slf: &Bound<'_, Self>, timeout: Option<f64>) -> PyResult<[<$value Poll>]> {
                    Self::finish(slf, Wait::from(timeout))
                }
            }
        }
    };
}

typed_future!(Image);
typed_future!(FileData);
