use pyo3::prelude::*;
use symphonia;
use hound;

/// Custom error type for this game engine.
/// It collects all possible errors that can be thrown, like MacroquadErr, Hound, Symphonia, Pyo3
/// and allows a conversion to Pyo3 error to be exposed to python.
pub enum PError{
    MacroquadErr(macroquad::Error),
    Pyo3Err(pyo3::PyErr),
    SymphoniaError(symphonia::core::errors::Error),
    HoundError(hound::Error),
    GLTFError(gltf::Error),
    BasicErr(String),
    WithContext(Box<PError>, String),
}
pub struct PyErrorWithMoreContext(pub Box<Option<PError>>, pub String);

impl PError {
    /// adds a string to the end of the error.
    pub fn with_context(self, context: impl Into<String>) -> PError {
        PError::WithContext(Box::new(self), context.into())
    }
}


impl From<PError> for pyo3::PyErr {
    fn from(value: PError) -> pyo3::PyErr {
        regular_extract(value, None)
    }
}


// "extra" is a message added at the end of our error independent of error type.
fn regular_extract(value: PError, extra: Option<&str>) -> pyo3::PyErr {

    let extra = extra.unwrap_or_default();

    match value {
        PError::MacroquadErr(e) => PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{e} || {extra}")),
        PError::Pyo3Err(e) => e,
        PError::SymphoniaError(e) => handle_Symphonia_error(e, extra),
        PError::GLTFError(e) => PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{e} || {extra}")),
        PError::BasicErr(e) => PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{e} || {extra}")),
        PError::HoundError(e) => PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Hound Error: {e} || {extra}")),
        PError::WithContext(err,context ) => {
            let (e,c) = recursively_extract_context(*err, context);
            let ee: pyo3::PyErr = regular_extract(e, Some(&c));
            ee
        }
    }
}

fn recursively_extract_context(err: PError, mut context: String) -> (PError, String) {
    match err {
        PError::WithContext(inner, ctx) => {
            if !context.is_empty() {
                context.push(' ');
            }
            context.push_str(&ctx);
            recursively_extract_context(*inner, context)
        }
        other => (other, context),
    }
}



// "extra" is a message added at the end of our error independent of error type.
fn handle_Symphonia_error(e: symphonia::core::errors::Error, extra: &str ) -> pyo3::PyErr {
    use symphonia::core::errors::*;
    match e {
        Error::DecodeError(e) => PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Symphonia Decode Error: {e} {extra}")),
        Error::IoError(e) => PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Symphonia IoError: {e} {extra}")),
        Error::LimitError(e) => PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Symphonia LimitError: {e} {extra}")),
        Error::ResetRequired => PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Symphonia Reset Required Error {extra}")),
        Error::SeekError(e) => {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Symphonia SeekError: {:?} || {extra}",e))
        },
        Error::Unsupported(e) => PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Symphonia UnsupportedError: {e} {extra}")),

    }
}


impl From<macroquad::Error> for PError {
    fn from(value: macroquad::Error) -> PError {
        PError::MacroquadErr(value)
    }
}

impl From<pyo3::PyErr> for PError {
    fn from(value: pyo3::PyErr) -> PError {
        PError::Pyo3Err(value)
    }
}

impl From<symphonia::core::errors::Error> for PError {
    fn from(value: symphonia::core::errors::Error) -> PError {
        PError::SymphoniaError(value)
    }
}

impl From<hound::Error> for PError {
    fn from(value: hound::Error) -> PError {
        PError::HoundError(value)
    }
}