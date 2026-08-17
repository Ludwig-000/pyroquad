/// VERY expermimental stuff. will likely break idk what i'm doing.
/// ( i did use AI to make this >.< )
/// really need it though.
/// 


/// this function is guaranteed to be run on the macroquad thread.
#[cfg(target_os = "windows")]
pub unsafe fn get_mouse_state_info() -> (f32, f32, bool) {
    use windows_sys::Win32::Foundation::{POINT, RECT};
    use windows_sys::Win32::Graphics::Gdi::ScreenToClient;

    let hwnd = unsafe {
        let active = windows_sys::Win32::UI::Input::KeyboardAndMouse::GetActiveWindow();
        if active.is_null() {
            windows_sys::Win32::UI::WindowsAndMessaging::GetForegroundWindow()
        } else {
            active
        }
    } as *mut std::ffi::c_void;

    use windows_sys::Win32::UI::WindowsAndMessaging::{
        GetClientRect, GetCursorPos,
    };

    unsafe {
        let mut pt: POINT = std::mem::zeroed();
        if GetCursorPos(&mut pt) == 0 {
            return (0.0, 0.0, false);
        }

        let hwnd_typed = hwnd as _;
        if ScreenToClient(hwnd_typed, &mut pt) == 0 {
            return (0.0, 0.0, false);
        }

        let mut rect: RECT = std::mem::zeroed();
        if GetClientRect(hwnd_typed, &mut rect) == 0 {
            return (0.0, 0.0, false);
        }

        let is_inside = pt.x >= 0 && pt.y >= 0 && pt.x < rect.right && pt.y < rect.bottom;
        (pt.x as f32, pt.y as f32, is_inside)
    }
}



#[cfg(target_os = "macos")]
pub unsafe fn get_mouse_state_info() -> (f32, f32, bool) {
    use cocoa::appkit::{NSApp, NSEvent};
    use cocoa::base::{id, nil};
    use cocoa::foundation::{NSPoint, NSRect};
    use objc::{class, msg_send, sel, sel_impl};

    // Get the global application instance and its main window
    let app: id = NSApp();
    if app == nil { return (0.0, 0.0, false); }
    
    let window: id = msg_send![app, mainWindow];
    if window == nil { return (0.0, 0.0, false); }

    // Get global mouse location (bottom-left origin)
    let mouse_loc: NSPoint = msg_send![class!(NSEvent), mouseLocation];
    
    // Get window bounds and convert to client/content bounds
    let window_rect: NSRect = msg_send![window, frame];
    let content_rect: NSRect = msg_send![window, contentRectForFrameRect: window_rect];
    
    // Calculate relative coordinates (flip Y for top-left origin)
    let rel_x = mouse_loc.x - content_rect.origin.x;
    let rel_y = content_rect.size.height - (mouse_loc.y - content_rect.origin.y);

    let is_inside = rel_x >= 0.0 
        && rel_y >= 0.0 
        && rel_x < content_rect.size.width 
        && rel_y < content_rect.size.height;
        
    (rel_x as f32, rel_y as f32, is_inside)
}



#[cfg(target_os = "linux")]
pub unsafe fn get_mouse_state_info() -> (f32, f32, bool) {
    use std::ptr;
    use x11::xlib;

    // 1. Create a silent error handler. 
    // By default, X11 errors exit the program. This forces them to be ignored.
    unsafe extern "C" fn silent_error_handler(
        _display: *mut xlib::Display,
        _error_event: *mut xlib::XErrorEvent,
    ) -> libc::c_int {
        0 // Return 0 to safely ignore the error
    }

    // Connect to X server. Will return null on pure Wayland.
    let display = xlib::XOpenDisplay(ptr::null());
    if display.is_null() {
        return (0.0, 0.0, false); 
    }

    // 2. Install the silent error handler, saving the original one
    let old_handler = xlib::XSetErrorHandler(Some(silent_error_handler));

    // Find the currently focused window
    let mut focus_window: xlib::Window = 0;
    let mut revert_to: libc::c_int = 0;
    xlib::XGetInputFocus(display, &mut focus_window, &mut revert_to);

    // X11 PointerRoot is typically 1 (0x1). This is the exact resource ID that crashed your app.
    let pointer_root = 1 as xlib::Window;

    // Check for invalid, root, or pointer-root windows
    if focus_window == 0 || focus_window == pointer_root || focus_window == xlib::XDefaultRootWindow(display) {
        xlib::XSetErrorHandler(old_handler); // Restore before closing
        xlib::XCloseDisplay(display);
        return (0.0, 0.0, false);
    }

    let mut root_return: xlib::Window = 0;
    let mut child_return: xlib::Window = 0;
    let mut root_x: libc::c_int = 0;
    let mut root_y: libc::c_int = 0;
    let mut win_x: libc::c_int = 0;
    let mut win_y: libc::c_int = 0;
    let mut mask_return: libc::c_uint = 0;

    // Query pointer relative to the focused window
    let result = xlib::XQueryPointer(
        display,
        focus_window,
        &mut root_return,
        &mut child_return,
        &mut root_x,
        &mut root_y,
        &mut win_x,
        &mut win_y,
        &mut mask_return,
    );

    // Get window dimensions to check bounds
    let mut attrs: xlib::XWindowAttributes = std::mem::zeroed();
    let attr_result = xlib::XGetWindowAttributes(display, focus_window, &mut attrs);

    // 3. Restore the original error handler and close the display
    xlib::XSetErrorHandler(old_handler);
    xlib::XCloseDisplay(display);

    // If either query failed (e.g., the window closed mid-query), fail gracefully
    if result == 0 || attr_result == 0 {
        return (0.0, 0.0, false);
    }

    let is_inside = win_x >= 0 && win_y >= 0 && win_x < attrs.width && win_y < attrs.height;
    
    (win_x as f32, win_y as f32, is_inside)
}


#[cfg(target_arch = "wasm32")]
pub unsafe fn get_mouse_state_info() -> (f32, f32, bool) {
    // Note: Because you cannot synchronously poll the mouse on the web without 
    // an active mouse event, this relies on what Macroquad is already doing, 
    // or requires a globally registered JS 'mousemove' listener storing state in a static Mutex.
    //
    // For a pure drop-in, we use Macroquad's internal state combined with the canvas bounds.
    
    let (mq_x, mq_y) = macroquad::input::mouse_position();
    
    let window = match web_sys::window() {
        Some(w) => w,
        None => return (mq_x, mq_y, false),
    };
    
    let document = match window.document() {
        Some(d) => d,
        None => return (mq_x, mq_y, false),
    };
    
    // Macroquad uses a canvas element with id "glcanvas" by default
    let canvas = match document.get_element_by_id("glcanvas") {
        Some(c) => c,
        None => return (mq_x, mq_y, false),
    };

    let rect = canvas.get_bounding_client_rect();
    
    // In a WASM context, Macroquad stops updating mouse_position() when outside the canvas.
    // If you need tracking OUTSIDE the canvas, you cannot do it here synchronously. 
    // You must write a JS snippet that attaches to `window.onmousemove` and writes to WASM memory.
    
    let is_inside = mq_x >= 0.0 
        && mq_y >= 0.0 
        && mq_x < rect.width() as f32 
        && mq_y < rect.height() as f32;

    (mq_x, mq_y, is_inside)
}