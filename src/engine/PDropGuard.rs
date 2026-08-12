

#[macro_export]
macro_rules! PDropGuard {
    ($msg:expr) => {{
        static IS_EXECUTING: ::std::sync::atomic::AtomicBool =
            ::std::sync::atomic::AtomicBool::new(false);

        struct DropGuard;
        impl ::std::ops::Drop for DropGuard {
            fn drop(&mut self) {
                IS_EXECUTING.store(false, ::std::sync::atomic::Ordering::Release);
            }
        }

        if IS_EXECUTING.swap(true, ::std::sync::atomic::Ordering::AcqRel) {
            return Err(pyo3::exceptions::PyRuntimeError::new_err($msg));
        }

        DropGuard
    }};
}