//! macOS only: falls back to the software OpenGL renderer when there is no hardware-accelerated one.
//!
//! miniquad always asks for an accelerated pixel format (`NSOpenGLPFAAccelerated`). Macs without
//! GPU access - virtual machines, such as GitHub's macOS runners - only have Apple's software
//! renderer, so `-[NSOpenGLPixelFormat initWithAttributes:]` returns nil there and miniquad's
//! window setup panics. We replace that method: when the original fails, it is tried once more
//! with the same attributes minus `NSOpenGLPFAAccelerated`. Requests that succeed are untouched.

// objc 0.2's msg_send!/class! test a `feature = "cargo-clippy"` cfg that this crate doesn't declare
#![allow(unexpected_cfgs)]

use std::io::Write;
use std::mem;
use std::sync::{Once, OnceLock};

use objc::runtime::{Imp, Method, Object, Sel, class_getInstanceMethod, method_getImplementation, method_setImplementation};
use objc::{class, msg_send, sel, sel_impl};

type Id = *mut Object;
type InitWithAttributes = unsafe extern "C" fn(Id, Sel, *const u32) -> Id;

// NSOpenGLPixelFormatAttribute values, see AppKit's NSOpenGL.h
const ACCELERATED: u32 = 73;
/// Attributes followed by a value. All others stand alone.
const WITH_VALUE: [u32; 12] = [
    7,   // AuxBuffers
    8,   // ColorSize
    11,  // AlphaSize
    12,  // DepthSize
    13,  // StencilSize
    14,  // AccumSize
    55,  // SampleBuffers
    56,  // Samples
    70,  // RendererID
    84,  // ScreenMask
    99,  // OpenGLProfile
    128, // VirtualScreenCount
];

static INSTALL: Once = Once::new();
static ORIGINAL: OnceLock<InitWithAttributes> = OnceLock::new();

/// Installs the fallback for the whole process. Calling it again does nothing.
pub fn install() {
    INSTALL.call_once(|| unsafe {
        let method = class_getInstanceMethod(class!(NSOpenGLPixelFormat), sel!(initWithAttributes:));
        if method.is_null() {
            return;
        }
        let original = mem::transmute::<Imp, InitWithAttributes>(method_getImplementation(method));
        let _ = ORIGINAL.set(original);
        let ours: InitWithAttributes = init_with_attributes;
        method_setImplementation(method as *mut Method, mem::transmute::<InitWithAttributes, Imp>(ours));
    });
}

unsafe extern "C" fn init_with_attributes(this: Id, cmd: Sel, attributes: *const u32) -> Id {
    // always set: `install` stores it before replacing the method
    let Some(&original) = ORIGINAL.get() else {
        return std::ptr::null_mut();
    };
    let format = unsafe { original(this, cmd, attributes) };
    if !format.is_null() || attributes.is_null() {
        return format;
    }
    let Some(software) = (unsafe { without_accelerated(attributes) }) else {
        return format;
    };

    let _ = writeln!(
        std::io::stderr(),
        "pyroquad: no hardware-accelerated OpenGL renderer available, using the software renderer."
    );
    // A failed init releases its object, so the retry needs a new one.
    let fresh: Id = unsafe { msg_send![class!(NSOpenGLPixelFormat), alloc] };
    unsafe { original(fresh, cmd, software.as_ptr()) }
}

/// The 0-terminated attribute list without `NSOpenGLPFAAccelerated`, or None if it isn't in there.
unsafe fn without_accelerated(attributes: *const u32) -> Option<Vec<u32>> {
    let mut out = Vec::new();
    let mut found = false;
    let mut i = 0;
    loop {
        let attribute = unsafe { *attributes.add(i) };
        i += 1;
        if attribute == 0 {
            break;
        }
        if attribute == ACCELERATED {
            found = true;
            continue;
        }
        out.push(attribute);
        if WITH_VALUE.contains(&attribute) {
            out.push(unsafe { *attributes.add(i) });
            i += 1;
        }
    }
    out.push(0);
    found.then_some(out)
}
