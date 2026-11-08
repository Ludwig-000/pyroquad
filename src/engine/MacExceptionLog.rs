//! macOS only: prints every Objective-C exception to stderr the moment it is thrown.
//!
//! AppKit reports misuse (for example creating a window off the main thread) by throwing an
//! Objective-C exception. When that exception unwinds into Rust, a `catch_unwind` turns it into
//! "Rust cannot catch foreign exceptions, aborting", and its name and reason are lost: the
//! Objective-C runtime only prints them for exceptions that nobody catches.
//!
//! The runtime's exception preprocessor is called on every throw, before any unwinding starts,
//! so it sees every exception, caught or not. We print it and hand it on unchanged. An exception
//! that something catches and handles is logged too, which is why the message says "thrown"
//! rather than "crashed".

// objc 0.2's msg_send!/class! test a `feature = "cargo-clippy"` cfg that this crate doesn't declare
#![allow(unexpected_cfgs)]

use std::ffi::CStr;
use std::io::Write;
use std::os::raw::c_char;
use std::panic::{self, AssertUnwindSafe};
use std::sync::{Once, OnceLock};

use objc::runtime::{BOOL, NO, Object};
use objc::{class, msg_send, sel, sel_impl};

type Id = *mut Object;
type Preprocessor = unsafe extern "C" fn(Id) -> Id;

unsafe extern "C" {
    // <objc/objc-exception.h>; libobjc is linked by the objc crate.
    fn objc_setExceptionPreprocessor(preprocessor: Option<Preprocessor>) -> Option<Preprocessor>;
}

/// Stack frames printed per exception.
const MAX_FRAMES: usize = 40;

static INSTALL: Once = Once::new();
/// Whatever preprocessor was installed before ours; we pass every exception on to it.
static PREVIOUS: OnceLock<Option<Preprocessor>> = OnceLock::new();

/// Installs the logger for the whole process (every thread). Calling it again does nothing.
pub fn install() {
    INSTALL.call_once(|| {
        let previous = unsafe { objc_setExceptionPreprocessor(Some(log_exception)) };
        let _ = PREVIOUS.set(previous);
    });
}

unsafe extern "C" fn log_exception(exception: Id) -> Id {
    // We are in the middle of an Objective-C throw: nothing may unwind out of here.
    let _ = panic::catch_unwind(AssertUnwindSafe(|| unsafe { print_exception(exception) }));

    match PREVIOUS.get().copied().flatten() {
        Some(previous) => unsafe { previous(exception) },
        None => exception,
    }
}

unsafe fn print_exception(exception: Id) {
    if exception.is_null() {
        return;
    }
    let mut out = String::new();

    let on_main: BOOL = unsafe { msg_send![class!(NSThread), isMainThread] };
    let thread = std::thread::current();
    let thread_name = thread.name().unwrap_or("<unnamed>");
    let where_ = format!(
        "thread '{}' ({})",
        thread_name,
        if on_main != NO { "the main thread" } else { "NOT the main thread" }
    );

    // Anything can be thrown, but only NSException answers `name` / `reason`. Sending those to
    // something else would throw again - from inside this preprocessor.
    let is_ns_exception: BOOL =
        unsafe { msg_send![exception, isKindOfClass: class!(NSException)] };
    if is_ns_exception != NO {
        let name: Id = unsafe { msg_send![exception, name] };
        let reason: Id = unsafe { msg_send![exception, reason] };
        out += &format!(
            "Objective-C exception thrown on {}: {}: {}\n",
            where_,
            unsafe { ns_string(name) },
            unsafe { ns_string(reason) }
        );
    } else {
        let description: Id = unsafe { msg_send![exception, description] };
        out += &format!(
            "Objective-C exception thrown on {}: {}\n",
            where_,
            unsafe { ns_string(description) }
        );
    }

    // The preprocessor runs inside the throw, so the current stack is the throw site.
    let symbols: Id = unsafe { msg_send![class!(NSThread), callStackSymbols] };
    if !symbols.is_null() {
        let count: usize = unsafe { msg_send![symbols, count] };
        for i in 0..count.min(MAX_FRAMES) {
            let symbol: Id = unsafe { msg_send![symbols, objectAtIndex: i] };
            out += &format!("    {}\n", unsafe { ns_string(symbol) });
        }
        if count > MAX_FRAMES {
            out += &format!("    ... {} more frame(s)\n", count - MAX_FRAMES);
        }
    }

    // One write, so lines from other threads can't end up in the middle of it.
    let _ = std::io::stderr().lock().write_all(out.as_bytes());
}

unsafe fn ns_string(string: Id) -> String {
    if string.is_null() {
        return "nil".to_owned();
    }
    let utf8: *const c_char = unsafe { msg_send![string, UTF8String] };
    if utf8.is_null() {
        return String::new();
    }
    unsafe { CStr::from_ptr(utf8) }.to_string_lossy().into_owned()
}
