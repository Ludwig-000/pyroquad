/*
 * pyroquad - Emscripten platform shim for miniquad / quad-snd
 * ===========================================================
 *
 * WHY THIS FILE EXISTS
 * --------------------
 * macroquad/miniquad's web backend is written for `wasm32-unknown-unknown`:
 * it imports a pile of functions from the `env` module and expects miniquad's
 * own hand-written loader (`gl.js` / `mq_js_bundle.js`) to supply them by doing
 * `WebAssembly.instantiate` itself.
 *
 * Python-on-wasm, on the other hand, only exists as CPython compiled with
 * Emscripten (Pyodide), and a pyo3 extension module for it is an Emscripten
 * *side module* that Pyodide `dlopen()`s. Emscripten side modules do not get a
 * hand-written loader - every undefined import must be resolvable from the main
 * module's symbol table (its wasm exports + its JS library functions), or be
 * provided by the side module itself.
 *
 * It turns out the overlap is almost total. Of the 265 `env` function imports
 * of a release build of `_pyroquad.wasm`, Pyodide's main module already
 * resolves 242 of them:
 *
 *   - all 88 `gl*` calls, because miniquad's `webgl.rs` is a plain GLES2 C-ABI
 *     binding (`glClear`, `glDrawArrays`, ...) and Pyodide links `-lGL`,
 *   - all of libc, libm, the pthread stubs, the CPython C-API, and the
 *     wasm-EH personality routines.
 *
 * The 23 that are left are exactly miniquad's and quad-snd's own JS glue. This
 * file supplies them, as `EM_JS` functions compiled into the side module.
 * Emscripten's dynamic loader (which Pyodide uses unmodified) evaluates a side
 * module's `__em_js__*` exports at dlopen time with a *direct* `eval` inside the
 * runtime's own closure, so the bodies below can see `HEAPU8`, `wasmTable`,
 * `getWasmTableEntry`, `GL`, `UTF8ToString`, ... exactly like a JS library file
 * linked into the main module could.
 *
 * The behaviour is a direct port of:
 *   - miniquad 0.4.11  js/gl.js      (canvas, input, fs, cursor, fullscreen)
 *   - quad-snd  0.2.8  js/audio.js   (WebAudio)
 * with the WebGL-context parts rewritten against Emscripten's `GL` registry
 * (`GL.createContext` / `GL.makeContextCurrent`) so that the `gl*` functions
 * Pyodide already ships draw into *our* canvas.
 *
 * CALLING BACK INTO WASM
 * ----------------------
 * gl.js calls `wasm_exports.mouse_move(...)` etc. A side module's exports are
 * not reachable under `Module.` (CPython dlopens extension modules into a local
 * scope), so instead `pq_register_callbacks` hands JS the *addresses* of
 * miniquad's `#[no_mangle]` entry points; JS turns each into a callable with
 * `getWasmTableEntry`. Taking the address also guarantees the linker keeps them.
 *
 * Note on comments inside EM_JS bodies: the preprocessor strips comments before
 * stringification, and the stringified body ends up on a single line, so every
 * statement below terminates with an explicit `;` rather than relying on ASI.
 */

#ifdef __EMSCRIPTEN__

#include <emscripten.h>
#include <stddef.h>
#include <stdint.h>

/* ------------------------------------------------------------------------ */
/* miniquad's exported entry points (src/native/wasm.rs, src/fs.rs)          */
/* ------------------------------------------------------------------------ */

extern void frame(void);
extern void mouse_move(int x, int y);
extern void raw_mouse_move(int dx, int dy);
extern void mouse_down(int x, int y, int btn);
extern void mouse_up(int x, int y, int btn);
extern void mouse_wheel(int dx, int dy);
extern void key_down(unsigned int key, unsigned int modifiers, int repeat);
extern void key_press(unsigned int key);
extern void key_up(unsigned int key, unsigned int modifiers);
extern void resize(int width, int height);
extern void touch(unsigned int phase, unsigned int id, float x, float y);
extern void focus(int has_focus);
extern void on_clipboard_paste(unsigned char *msg, size_t len);
extern unsigned char *allocate_vec_u8(size_t len);
extern void file_loaded(unsigned int file_id);
extern void on_files_dropped_start(void);
extern void on_files_dropped_finish(void);
extern void on_file_dropped(unsigned char *path, size_t path_len,
                            unsigned char *bytes, size_t bytes_len);

/* Index layout of the callback table handed to JS. Keep in sync with the
 * `C` object built in pq_register_callbacks below. */
enum {
    PQ_CB_FRAME = 0,
    PQ_CB_MOUSE_MOVE,
    PQ_CB_RAW_MOUSE_MOVE,
    PQ_CB_MOUSE_DOWN,
    PQ_CB_MOUSE_UP,
    PQ_CB_MOUSE_WHEEL,
    PQ_CB_KEY_DOWN,
    PQ_CB_KEY_PRESS,
    PQ_CB_KEY_UP,
    PQ_CB_RESIZE,
    PQ_CB_TOUCH,
    PQ_CB_FOCUS,
    PQ_CB_ON_CLIPBOARD_PASTE,
    PQ_CB_ALLOCATE_VEC_U8,
    PQ_CB_FILE_LOADED,
    PQ_CB_FILES_DROPPED_START,
    PQ_CB_FILES_DROPPED_FINISH,
    PQ_CB_FILE_DROPPED,
    PQ_CB_COUNT
};

/* ------------------------------------------------------------------------ */
/* One-time JS bootstrap: installs globalThis.__PQ                            */
/* ------------------------------------------------------------------------ */
/*
 * Every other EM_JS function below is a thin delegate to `__PQ`. They have to
 * share state (the canvas, the WebGL handle, the loaded-file table, the audio
 * graph) and separate EM_JS functions cannot share locals, so the state lives
 * on a global object - but one *created inside an EM_JS body*, which means its
 * methods still close over the Emscripten runtime scope.
 */
EM_JS(void, pq_js_register_callbacks, (unsigned int *cbs), {
    var P = globalThis.__PQ;
    if (!P) { P = globalThis.__PQ = {}; }

    var at = function (i) { return getWasmTableEntry(HEAPU32[(cbs >> 2) + i]); };

    P.cb = {
        frame:                at(0),
        mouse_move:           at(1),
        raw_mouse_move:       at(2),
        mouse_down:           at(3),
        mouse_up:             at(4),
        mouse_wheel:          at(5),
        key_down:             at(6),
        key_press:            at(7),
        key_up:               at(8),
        resize:               at(9),
        touch:                at(10),
        focus:                at(11),
        on_clipboard_paste:   at(12),
        allocate_vec_u8:      at(13),
        file_loaded:          at(14),
        files_dropped_start:  at(15),
        files_dropped_finish: at(16),
        file_dropped:         at(17)
    };

    if (P.ready) { return; }
    P.ready = true;

    P.high_dpi = false;
    P.blocking_event_loop = false;
    P.raf = null;
    P.clipboard = null;
    P.gl_handle = 0;
    P.files = {};
    P.file_id_next = 1;
    P.listeners_installed = false;

    P.canvas = function () {
        if (P._canvas) { return P._canvas; }
        var c = document.querySelector("#glcanvas");
        if (!c && typeof Module !== "undefined") { c = Module["canvas"]; }
        if (!c) {
            throw new Error(
                "pyroquad: no <canvas id='glcanvas'> found on the page. " +
                "Add one before calling activate_engine().");
        }
        P._canvas = c;
        return c;
    };

    P.dpi_scale = function () {
        return P.high_dpi ? (window.devicePixelRatio || 1.0) : 1.0;
    };

    /* gl.js `resize(canvas, on_resize)`: keep the backing store in sync with
     * the CSS box, and only notify miniquad when it actually changed. */
    P.resize_canvas = function (notify) {
        var canvas = P.canvas();
        var dpr = P.dpi_scale();
        var w = canvas.clientWidth * dpr;
        var h = canvas.clientHeight * dpr;
        if (canvas.width != w || canvas.height != h) {
            canvas.width = w;
            canvas.height = h;
            if (notify && P.cb) { P.cb.resize(Math.floor(w), Math.floor(h)); }
        }
    };

    P.mouse_rel = function (clientX, clientY) {
        var r = P.canvas().getBoundingClientRect();
        var s = P.dpi_scale();
        return { x: (clientX - r.left) * s, y: (clientY - r.top) * s };
    };

    P.into_sapp_mousebutton = function (btn) {
        if (btn == 0) { return 0; }
        if (btn == 1) { return 2; }
        if (btn == 2) { return 1; }
        return btn;
    };

    /* Verbatim from miniquad 0.4.11 js/gl.js `into_sapp_keycode`. */
    P.keymap = {
        "Space": 32, "Quote": 222, "Comma": 44, "Minus": 45, "Period": 46,
        "Slash": 189, "Digit0": 48, "Digit1": 49, "Digit2": 50, "Digit3": 51,
        "Digit4": 52, "Digit5": 53, "Digit6": 54, "Digit7": 55, "Digit8": 56,
        "Digit9": 57, "Semicolon": 59, "Equal": 61,
        "KeyA": 65, "KeyB": 66, "KeyC": 67, "KeyD": 68, "KeyE": 69, "KeyF": 70,
        "KeyG": 71, "KeyH": 72, "KeyI": 73, "KeyJ": 74, "KeyK": 75, "KeyL": 76,
        "KeyM": 77, "KeyN": 78, "KeyO": 79, "KeyP": 80, "KeyQ": 81, "KeyR": 82,
        "KeyS": 83, "KeyT": 84, "KeyU": 85, "KeyV": 86, "KeyW": 87, "KeyX": 88,
        "KeyY": 89, "KeyZ": 90,
        "BracketLeft": 91, "Backslash": 92, "BracketRight": 93, "Backquote": 96,
        "Escape": 256, "Enter": 257, "Tab": 258, "Backspace": 259,
        "Insert": 260, "Delete": 261, "ArrowRight": 262, "ArrowLeft": 263,
        "ArrowDown": 264, "ArrowUp": 265, "PageUp": 266, "PageDown": 267,
        "Home": 268, "End": 269, "CapsLock": 280, "ScrollLock": 281,
        "NumLock": 282, "PrintScreen": 283, "Pause": 284,
        "F1": 290, "F2": 291, "F3": 292, "F4": 293, "F5": 294, "F6": 295,
        "F7": 296, "F8": 297, "F9": 298, "F10": 299, "F11": 300, "F12": 301,
        "F13": 302, "F14": 303, "F15": 304, "F16": 305, "F17": 306, "F18": 307,
        "F19": 308, "F20": 309, "F21": 310, "F22": 311, "F23": 312, "F24": 313,
        "Numpad0": 320, "Numpad1": 321, "Numpad2": 322, "Numpad3": 323,
        "Numpad4": 324, "Numpad5": 325, "Numpad6": 326, "Numpad7": 327,
        "Numpad8": 328, "Numpad9": 329, "NumpadDecimal": 330,
        "NumpadDivide": 331, "NumpadMultiply": 332, "NumpadSubtract": 333,
        "NumpadAdd": 334, "NumpadEnter": 335, "NumpadEqual": 336,
        "ShiftLeft": 340, "ControlLeft": 341, "AltLeft": 342, "OSLeft": 343,
        "ShiftRight": 344, "ControlRight": 345, "AltRight": 346,
        "OSRight": 347, "ContextMenu": 348
    };

    P.keycode = function (code) {
        var k = P.keymap[code];
        return (k === undefined) ? -1 : k;
    };

    P.modifiers = function (e) {
        var m = 0;
        if (e.shiftKey) { m |= 1; }
        if (e.ctrlKey)  { m |= 2; }
        if (e.altKey)   { m |= 4; }
        if (e.metaKey)  { m |= 8; }
        return m;
    };

    /* Handed to Python as `pyodide.ffi.run_sync(js.__PQ.frame_yield())`, which
     * suspends the whole wasm stack through JSPI until the promise settles -
     * this is what lets a *synchronous* Python `next_frame()` give the browser
     * a chance to paint and to deliver input on the one thread we have. */
    P.frame_yield = function () {
        return new Promise(function (resolve) {
            requestAnimationFrame(function () { resolve(0); });
        });
    };

    /* Same, but only to the end of the current task - used while polling an
     * in-flight fetch, where waiting for a repaint would be pure latency. */
    P.task_yield = function () {
        return new Promise(function (resolve) { setTimeout(resolve, 0); });
    };

    P.animation = function () {
        P.cb.frame();
        if (!P.blocking_event_loop) {
            if (P.raf) { cancelAnimationFrame(P.raf); }
            P.raf = requestAnimationFrame(P.animation);
        }
    };

    P.schedule_update = function () {
        if (P.raf) { cancelAnimationFrame(P.raf); }
        P.raf = requestAnimationFrame(P.animation);
    };

    /* ---------------- WebAudio (port of quad-snd js/audio.js) ------------- */
    P.audio = {
        ctx: null,
        sounds: new Map(),
        playbacks: [],
        sound_key_next: 1,
        playback_key_next: 1
    };

    P.audio_stop = function (pb) {
        try {
            pb.source.removeEventListener("ended", pb.ended);
            pb.source.disconnect();
            pb.gain_node.disconnect();
            pb.sound_key = 0;
            pb.playback_key = 0;
        } catch (e) {
            console.error("pyroquad: error stopping sound", e);
        }
    };

    P.audio_recycle = function () {
        var a = P.audio;
        var pb = a.playbacks.find(function (p) { return p.sound_key === 0; });
        if (pb != null) {
            pb.source = a.ctx.createBufferSource();
        } else {
            pb = {
                sound_key: 0,
                playback_key: 0,
                source: a.ctx.createBufferSource(),
                gain_node: a.ctx.createGain(),
                ended: null
            };
            a.playbacks.push(pb);
        }
        return pb;
    };
});

/* ------------------------------------------------------------------------ */
/* miniquad: console                                                         */
/* ------------------------------------------------------------------------ */

EM_JS(void, pq_js_console_debug, (const char *msg), { console.debug(UTF8ToString(msg)); });
EM_JS(void, pq_js_console_log,   (const char *msg), { console.log(UTF8ToString(msg)); });
EM_JS(void, pq_js_console_info,  (const char *msg), { console.info(UTF8ToString(msg)); });
EM_JS(void, pq_js_console_warn,  (const char *msg), { console.warn(UTF8ToString(msg)); });
EM_JS(void, pq_js_console_error, (const char *msg), { console.error(UTF8ToString(msg)); });

/* ------------------------------------------------------------------------ */
/* miniquad: canvas / WebGL context                                          */
/* ------------------------------------------------------------------------ */
/*
 * Unlike gl.js we do not keep our own `gl` object: the `gl*` functions come
 * from Pyodide's `-lGL` JS library, which dispatches to whatever context is
 * currently registered as `GLctx`. `GL.createContext` + `GL.makeContextCurrent`
 * is exactly what `emscripten_webgl_create_context` does internally; going
 * through the JS registry directly keeps us independent of the
 * `EmscriptenWebGLContextAttributes` struct layout (and therefore of which
 * Emscripten version Pyodide happened to be built with).
 */
EM_JS(void, pq_js_init_webgl, (int version), {
    var P = globalThis.__PQ;
    var canvas = P.canvas();

    var attrs = {
        majorVersion: (version == 1) ? 1 : 2,
        minorVersion: 0,
        alpha: true,
        depth: true,
        stencil: true,
        antialias: true,
        premultipliedAlpha: true,
        preserveDrawingBuffer: false,
        powerPreference: "high-performance",
        failIfMajorPerformanceCaveat: false,
        enableExtensionsByDefault: true,
        explicitSwapControl: false,
        renderViaOffscreenBackBuffer: false,
        proxyContextToMainThread: 0
    };

    var handle = GL.createContext(canvas, attrs);
    if (!handle) {
        console.error("pyroquad: unable to create a WebGL context");
        return;
    }
    P.gl_handle = handle;
    GL.makeContextCurrent(handle);
});

EM_JS(void, pq_js_setup_canvas_size, (int high_dpi), {
    var P = globalThis.__PQ;
    P.high_dpi = !!high_dpi;
    P.resize_canvas(false);
});

EM_JS(int, pq_js_canvas_width, (void), {
    return Math.floor(globalThis.__PQ.canvas().width);
});

EM_JS(int, pq_js_canvas_height, (void), {
    return Math.floor(globalThis.__PQ.canvas().height);
});

EM_JS(float, pq_js_dpi_scale, (void), {
    return globalThis.__PQ.dpi_scale();
});

EM_JS(double, pq_js_now, (void), { return Date.now() / 1000.0; });

EM_JS(int, pq_js_sapp_is_elapsed_timer_supported, (void), { return 1; });

/* ------------------------------------------------------------------------ */
/* miniquad: the event loop                                                  */
/* ------------------------------------------------------------------------ */

EM_JS(void, pq_js_run_animation_loop, (int blocking), {
    var P = globalThis.__PQ;
    var canvas = P.canvas();
    P.blocking_event_loop = !!blocking;

    if (!P.listeners_installed) {
        P.listeners_installed = true;

        if (canvas.tabIndex < 0) { canvas.tabIndex = 0; }
        canvas.focus();

        canvas.addEventListener("mousemove", function (e) {
            var p = P.mouse_rel(e.clientX, e.clientY);
            P.cb.mouse_move(Math.floor(p.x), Math.floor(p.y));
            if (e.movementX != 0 || e.movementY != 0) {
                P.cb.raw_mouse_move(Math.floor(e.movementX), Math.floor(e.movementY));
            }
        });
        canvas.addEventListener("mousedown", function (e) {
            var p = P.mouse_rel(e.clientX, e.clientY);
            P.cb.mouse_down(Math.floor(p.x), Math.floor(p.y), P.into_sapp_mousebutton(e.button));
        });
        canvas.addEventListener("mouseup", function (e) {
            var p = P.mouse_rel(e.clientX, e.clientY);
            P.cb.mouse_up(Math.floor(p.x), Math.floor(p.y), P.into_sapp_mousebutton(e.button));
        });
        canvas.addEventListener("wheel", function (e) {
            e.preventDefault();
            P.cb.mouse_wheel(-e.deltaX, -e.deltaY);
        }, { passive: false });

        canvas.addEventListener("keydown", function (e) {
            var k = P.keycode(e.code);
            if (k < 0) { return; }
            /* space, arrows, F1-F10, backspace, tab, quote, slash: swallow so
             * the page does not scroll / navigate back / open Quick Find. */
            if (k == 32 || (k >= 262 && k <= 265) || (k >= 290 && k <= 299) ||
                k == 259 || k == 258 || k == 39 || k == 47) {
                e.preventDefault();
            }
            P.cb.key_down(k, P.modifiers(e), e.repeat ? 1 : 0);
            if (k == 32 || k == 39 || k == 47) { P.cb.key_press(k); }
            /* preventDefault above also kills `keypress`, so synthesise the
             * character events miniquad still needs for text input. */
            if (e.key && e.key.length == 1 && !e.ctrlKey && !e.metaKey) {
                if (k != 32 && k != 39 && k != 47) {
                    P.cb.key_press(e.key.codePointAt(0));
                }
            }
        });
        canvas.addEventListener("keyup", function (e) {
            var k = P.keycode(e.code);
            if (k < 0) { return; }
            P.cb.key_up(k, P.modifiers(e));
        });

        var touch_handler = function (phase) {
            return function (e) {
                e.preventDefault();
                for (var i = 0; i < e.changedTouches.length; i++) {
                    var t = e.changedTouches[i];
                    var p = P.mouse_rel(t.clientX, t.clientY);
                    P.cb.touch(phase, t.identifier, p.x, p.y);
                }
            };
        };
        canvas.addEventListener("touchstart",  touch_handler(10), { passive: false });
        canvas.addEventListener("touchmove",   touch_handler(11), { passive: false });
        canvas.addEventListener("touchend",    touch_handler(12), { passive: false });
        canvas.addEventListener("touchcancel", touch_handler(13), { passive: false });

        window.addEventListener("resize", function () { P.resize_canvas(true); });

        window.addEventListener("copy", function (e) {
            if (P.clipboard != null) {
                e.clipboardData.setData("text/plain", P.clipboard);
                e.preventDefault();
            }
        });
        window.addEventListener("cut", function (e) {
            if (P.clipboard != null) {
                e.clipboardData.setData("text/plain", P.clipboard);
                e.preventDefault();
            }
        });
        window.addEventListener("paste", function (e) {
            e.stopPropagation();
            e.preventDefault();
            var cd = e.clipboardData || window.clipboardData;
            var text = cd ? cd.getData("Text") : null;
            if (text) {
                var bytes = new TextEncoder().encode(text);
                var ptr = P.cb.allocate_vec_u8(bytes.length);
                HEAPU8.set(bytes, ptr);
                P.cb.on_clipboard_paste(ptr, bytes.length);
            }
        });

        window.addEventListener("dragover", function (e) { e.preventDefault(); });
        window.addEventListener("drop", async function (e) {
            e.preventDefault();
            P.cb.files_dropped_start();
            for (var f of e.dataTransfer.files) {
                var nameBytes = new TextEncoder().encode(f.name);
                var namePtr = P.cb.allocate_vec_u8(nameBytes.length);
                HEAPU8.set(nameBytes, namePtr);
                var buf = new Uint8Array(await f.arrayBuffer());
                var bufPtr = P.cb.allocate_vec_u8(buf.length);
                HEAPU8.set(buf, bufPtr);
                P.cb.file_dropped(namePtr, nameBytes.length, bufPtr, buf.length);
            }
            P.cb.files_dropped_finish();
        });

        var lastFocus = document.hasFocus();
        var checkFocus = function () {
            var has = document.hasFocus() && document.visibilityState == "visible";
            if (lastFocus != has) {
                P.cb.focus(has ? 1 : 0);
                lastFocus = has;
            }
        };
        document.addEventListener("visibilitychange", checkFocus);
        window.addEventListener("focus", checkFocus);
        window.addEventListener("blur", checkFocus);
    }

    /* miniquad expects the rAF pump to be running from here on. Whether frames
     * are actually produced by rAF or pulled synchronously by Python's
     * next_frame() is decided by pq_set_frame_driver(). */
    if (!P.manual_frames) { P.schedule_update(); }
});

EM_JS(void, pq_js_sapp_schedule_update, (void), {
    var P = globalThis.__PQ;
    if (!P.manual_frames) { P.schedule_update(); }
});

/* ------------------------------------------------------------------------ */
/* miniquad: window / cursor / clipboard                                     */
/* ------------------------------------------------------------------------ */

EM_JS(void, pq_js_sapp_set_clipboard, (const char *ptr, size_t len), {
    globalThis.__PQ.clipboard = UTF8ToString(ptr, len);
});

EM_JS(void, pq_js_sapp_set_cursor_grab, (int grab), {
    var canvas = globalThis.__PQ.canvas();
    /* Both of these reject unless they come from a user gesture, and an
     * unhandled rejection in a game loop is just console noise. */
    try {
        var p = grab ? canvas.requestPointerLock() : document.exitPointerLock();
        if (p && p.catch) { p.catch(function () {}); }
    } catch (e) {}
});

EM_JS(void, pq_js_sapp_set_cursor, (const unsigned char *ptr, size_t len), {
    globalThis.__PQ.canvas().style.cursor = UTF8ToString(ptr, len);
});

EM_JS(int, pq_js_sapp_is_fullscreen, (void), {
    var el = document.fullscreenElement;
    return (el != null && el === globalThis.__PQ.canvas()) ? 1 : 0;
});

EM_JS(void, pq_js_sapp_set_fullscreen, (int fullscreen), {
    var canvas = globalThis.__PQ.canvas();
    /* Browsers only grant fullscreen from a user gesture; a game asking for it
     * at startup gets a rejected promise, which is not worth reporting. */
    try {
        var p = fullscreen ? canvas.requestFullscreen() : document.exitFullscreen();
        if (p && p.catch) { p.catch(function () {}); }
    } catch (e) {}
});

EM_JS(void, pq_js_sapp_set_window_size, (unsigned int w, unsigned int h), {
    var P = globalThis.__PQ;
    var canvas = P.canvas();
    canvas.width = w;
    canvas.height = h;
    P.resize_canvas(true);
});

/* ------------------------------------------------------------------------ */
/* miniquad: async file loading                                              */
/* ------------------------------------------------------------------------ */
/*
 * Same contract as gl.js: `fs_load_file` returns an id immediately, the bytes
 * land in `__PQ.files[id]` later and `file_loaded(id)` is invoked. A failed
 * load stores `null`, which `fs_get_buffer_size` reports as -1 - miniquad turns
 * that into a load error.
 */
EM_JS(unsigned int, pq_js_fs_load_file, (const char *ptr, unsigned int len), {
    var P = globalThis.__PQ;
    var url = UTF8ToString(ptr, len);
    var id = P.file_id_next++;
    fetch(url).then(function (r) {
        if (!r.ok) { throw new Error("HTTP " + r.status); }
        return r.arrayBuffer();
    }).then(function (buf) {
        P.files[id] = new Uint8Array(buf);
        P.cb.file_loaded(id);
    }).catch(function (e) {
        console.warn("pyroquad: failed to load", url, e);
        P.files[id] = null;
        P.cb.file_loaded(id);
    });
    return id;
});

EM_JS(int, pq_js_fs_get_buffer_size, (unsigned int file_id), {
    var f = globalThis.__PQ.files[file_id];
    return (f == null) ? -1 : f.length;
});

EM_JS(void, pq_js_fs_take_buffer, (unsigned int file_id, unsigned char *ptr, unsigned int max_size), {
    var P = globalThis.__PQ;
    var f = P.files[file_id];
    if (f == null) { return; }
    HEAPU8.set(f.subarray(0, Math.min(f.length, max_size)), ptr);
    delete P.files[file_id];
});

/* ------------------------------------------------------------------------ */
/* quad-snd: WebAudio                                                        */
/* ------------------------------------------------------------------------ */

EM_JS(void, pq_js_audio_init, (void), {
    var a = globalThis.__PQ.audio;
    if (a.ctx != null) { return; }
    var Ctx = window.AudioContext || window.webkitAudioContext;
    a.ctx = new Ctx();
    /* Browsers start an AudioContext suspended until a user gesture. */
    var unlock = function () {
        a.ctx.resume();
        document.removeEventListener("touchstart", unlock);
        document.removeEventListener("touchend", unlock);
        document.removeEventListener("mousedown", unlock);
        document.removeEventListener("keydown", unlock);
    };
    document.addEventListener("touchstart", unlock);
    document.addEventListener("touchend", unlock);
    document.addEventListener("mousedown", unlock);
    document.addEventListener("keydown", unlock);
});

EM_JS(unsigned int, pq_js_audio_add_buffer, (const unsigned char *content, unsigned int content_len), {
    var a = globalThis.__PQ.audio;
    var bytes = HEAPU8.slice(content, content + content_len);
    var key = a.sound_key_next++;
    a.ctx.decodeAudioData(bytes.buffer, function (buffer) {
        a.sounds.set(key, buffer);
    }, function (e) {
        console.error("pyroquad: failed to decode audio buffer", e);
    });
    return key;
});

EM_JS(int, pq_js_audio_source_is_loaded, (unsigned int sound_key), {
    var a = globalThis.__PQ.audio;
    return (a.sounds.has(sound_key) && a.sounds.get(sound_key) != undefined) ? 1 : 0;
});

EM_JS(unsigned int, pq_js_audio_play_buffer, (unsigned int sound_key, float volume, int repeat), {
    var P = globalThis.__PQ;
    var a = P.audio;
    var playback_key = a.playback_key_next++;
    var pb = P.audio_recycle();

    pb.sound_key = sound_key;
    pb.playback_key = playback_key;
    pb.source.connect(pb.gain_node);
    pb.gain_node.connect(a.ctx.destination);
    pb.gain_node.gain.value = volume;
    pb.source.loop = !!repeat;
    pb.ended = function () { P.audio_stop(pb); };
    pb.source.addEventListener("ended", pb.ended);

    try {
        pb.source.buffer = a.sounds.get(sound_key);
        pb.source.start(0);
    } catch (e) {
        console.error("pyroquad: error starting sound", e);
    }
    return playback_key;
});

EM_JS(void, pq_js_audio_source_set_volume, (unsigned int sound_key, float volume), {
    globalThis.__PQ.audio.playbacks.forEach(function (pb) {
        if (pb.sound_key === sound_key) { pb.gain_node.gain.value = volume; }
    });
});

EM_JS(void, pq_js_audio_source_stop, (unsigned int sound_key), {
    var P = globalThis.__PQ;
    P.audio.playbacks.forEach(function (pb) {
        if (pb.sound_key === sound_key) { P.audio_stop(pb); }
    });
});

EM_JS(void, pq_js_audio_source_delete, (unsigned int sound_key), {
    var P = globalThis.__PQ;
    P.audio.playbacks.forEach(function (pb) {
        if (pb.sound_key === sound_key) { P.audio_stop(pb); }
    });
    P.audio.sounds.delete(sound_key);
});

EM_JS(void, pq_js_audio_playback_stop, (unsigned int playback_key), {
    var P = globalThis.__PQ;
    var pb = P.audio.playbacks.find(function (p) { return p.playback_key === playback_key; });
    if (pb != null) { P.audio_stop(pb); }
});

EM_JS(void, pq_js_audio_playback_set_volume, (unsigned int playback_key, float volume), {
    var pb = globalThis.__PQ.audio.playbacks.find(function (p) {
        return p.playback_key === playback_key;
    });
    if (pb != null) { pb.gain_node.gain.value = volume; }
});

/* ------------------------------------------------------------------------ */
/* Entry points used by the Rust side                                        */
/* ------------------------------------------------------------------------ */

/*
 * Chooses who produces frames. `manual = 1` stops the rAF pump so that Python's
 * next_frame() is the only thing that ever calls miniquad's `frame()`; that is
 * the mode pyroquad runs in, because the engine and the interpreter share one
 * thread here and a free-running rAF would advance the engine while Python is
 * mid-frame. `manual = 0` restores stock miniquad behaviour (useful for
 * debugging the shim without Python in the picture).
 */
EM_JS(void, pq_js_set_frame_driver, (int manual), {
    var P = globalThis.__PQ;
    P.manual_frames = !!manual;
    if (manual && P.raf) {
        cancelAnimationFrame(P.raf);
        P.raf = null;
    }
});

/* Run exactly one miniquad frame, synchronously, on the caller's stack. */
EM_JS(void, pq_js_frame_now, (void), {
    globalThis.__PQ.cb.frame();
});

/* True once the page has a canvas and the runtime scope looks sane. Lets the
 * Rust side produce a readable Python exception instead of a JS TypeError. */
EM_JS(int, pq_js_probe, (void), {
    try {
        if (typeof GL === "undefined") { return 0; }
        if (!document.querySelector("#glcanvas")) { return 0; }
        return 1;
    } catch (e) {
        return 0;
    }
});

/* ------------------------------------------------------------------------ */
/* The symbols miniquad and quad-snd actually import                         */
/* ------------------------------------------------------------------------ */
/*
 * Everything above is named `pq_js_*` and every symbol the rest of the crate
 * links against is a real, *defined* C function here that forwards to it.
 *
 * That indirection is not cosmetic. An `EM_JS` function is, at the wasm level,
 * an *import* that happens to carry its own JS body along in a `__em_js__`
 * data export. That is fine as long as it is only ever called - but if anything
 * takes its address, the PIC build emits a `GOT.func.<name>` entry, and the
 * dynamic loader has no address to give for a function that only exists in JS:
 * dlopen fails with "undefined symbol". Defining the imported names as ordinary
 * C functions gives every one of them a real address in the module's table, so
 * calls and address-taking both resolve locally.
 */

EMSCRIPTEN_KEEPALIVE void console_debug(const char *msg) { pq_js_console_debug(msg); }
EMSCRIPTEN_KEEPALIVE void console_log(const char *msg)   { pq_js_console_log(msg); }
EMSCRIPTEN_KEEPALIVE void console_info(const char *msg)  { pq_js_console_info(msg); }
EMSCRIPTEN_KEEPALIVE void console_warn(const char *msg)  { pq_js_console_warn(msg); }
EMSCRIPTEN_KEEPALIVE void console_error(const char *msg) { pq_js_console_error(msg); }

EMSCRIPTEN_KEEPALIVE void init_webgl(int version) { pq_js_init_webgl(version); }
EMSCRIPTEN_KEEPALIVE void setup_canvas_size(int high_dpi) { pq_js_setup_canvas_size(high_dpi); }
EMSCRIPTEN_KEEPALIVE int canvas_width(void) { return pq_js_canvas_width(); }
EMSCRIPTEN_KEEPALIVE int canvas_height(void) { return pq_js_canvas_height(); }
EMSCRIPTEN_KEEPALIVE float dpi_scale(void) { return pq_js_dpi_scale(); }
EMSCRIPTEN_KEEPALIVE double now(void) { return pq_js_now(); }
EMSCRIPTEN_KEEPALIVE int sapp_is_elapsed_timer_supported(void) {
    return pq_js_sapp_is_elapsed_timer_supported();
}

EMSCRIPTEN_KEEPALIVE void run_animation_loop(int blocking) { pq_js_run_animation_loop(blocking); }
EMSCRIPTEN_KEEPALIVE void sapp_schedule_update(void) { pq_js_sapp_schedule_update(); }

EMSCRIPTEN_KEEPALIVE void sapp_set_clipboard(const char *ptr, size_t len) {
    pq_js_sapp_set_clipboard(ptr, len);
}
EMSCRIPTEN_KEEPALIVE void sapp_set_cursor_grab(int grab) { pq_js_sapp_set_cursor_grab(grab); }
EMSCRIPTEN_KEEPALIVE void sapp_set_cursor(const unsigned char *ptr, size_t len) {
    pq_js_sapp_set_cursor(ptr, len);
}
EMSCRIPTEN_KEEPALIVE int sapp_is_fullscreen(void) { return pq_js_sapp_is_fullscreen(); }
EMSCRIPTEN_KEEPALIVE void sapp_set_fullscreen(int fullscreen) { pq_js_sapp_set_fullscreen(fullscreen); }
EMSCRIPTEN_KEEPALIVE void sapp_set_window_size(unsigned int w, unsigned int h) {
    pq_js_sapp_set_window_size(w, h);
}

EMSCRIPTEN_KEEPALIVE unsigned int fs_load_file(const char *ptr, unsigned int len) {
    return pq_js_fs_load_file(ptr, len);
}
EMSCRIPTEN_KEEPALIVE int fs_get_buffer_size(unsigned int file_id) {
    return pq_js_fs_get_buffer_size(file_id);
}
EMSCRIPTEN_KEEPALIVE void fs_take_buffer(unsigned int file_id, unsigned char *ptr,
                                         unsigned int max_size) {
    pq_js_fs_take_buffer(file_id, ptr, max_size);
}

EMSCRIPTEN_KEEPALIVE void audio_init(void) { pq_js_audio_init(); }
EMSCRIPTEN_KEEPALIVE unsigned int audio_add_buffer(const unsigned char *content,
                                                   unsigned int content_len) {
    return pq_js_audio_add_buffer(content, content_len);
}
EMSCRIPTEN_KEEPALIVE int audio_source_is_loaded(unsigned int sound_key) {
    return pq_js_audio_source_is_loaded(sound_key);
}
EMSCRIPTEN_KEEPALIVE unsigned int audio_play_buffer(unsigned int sound_key, float volume,
                                                    int repeat) {
    return pq_js_audio_play_buffer(sound_key, volume, repeat);
}
EMSCRIPTEN_KEEPALIVE void audio_source_set_volume(unsigned int sound_key, float volume) {
    pq_js_audio_source_set_volume(sound_key, volume);
}
EMSCRIPTEN_KEEPALIVE void audio_source_stop(unsigned int sound_key) {
    pq_js_audio_source_stop(sound_key);
}
EMSCRIPTEN_KEEPALIVE void audio_source_delete(unsigned int sound_key) {
    pq_js_audio_source_delete(sound_key);
}
EMSCRIPTEN_KEEPALIVE void audio_playback_stop(unsigned int playback_key) {
    pq_js_audio_playback_stop(playback_key);
}
EMSCRIPTEN_KEEPALIVE void audio_playback_set_volume(unsigned int playback_key, float volume) {
    pq_js_audio_playback_set_volume(playback_key, volume);
}

/* Called from Rust (see src/web/mod.rs). */
EMSCRIPTEN_KEEPALIVE void pq_set_frame_driver(int manual) { pq_js_set_frame_driver(manual); }
EMSCRIPTEN_KEEPALIVE void pq_frame_now(void) { pq_js_frame_now(); }
EMSCRIPTEN_KEEPALIVE int pq_probe(void) { return pq_js_probe(); }

/*
 * Called once from Rust before miniquad starts. Publishes the addresses of
 * miniquad's exported entry points to JS and builds `globalThis.__PQ`.
 */
EMSCRIPTEN_KEEPALIVE void pq_platform_init(void) {
    static unsigned int cbs[PQ_CB_COUNT];

    cbs[PQ_CB_FRAME]                = (unsigned int)(uintptr_t)&frame;
    cbs[PQ_CB_MOUSE_MOVE]           = (unsigned int)(uintptr_t)&mouse_move;
    cbs[PQ_CB_RAW_MOUSE_MOVE]       = (unsigned int)(uintptr_t)&raw_mouse_move;
    cbs[PQ_CB_MOUSE_DOWN]           = (unsigned int)(uintptr_t)&mouse_down;
    cbs[PQ_CB_MOUSE_UP]             = (unsigned int)(uintptr_t)&mouse_up;
    cbs[PQ_CB_MOUSE_WHEEL]          = (unsigned int)(uintptr_t)&mouse_wheel;
    cbs[PQ_CB_KEY_DOWN]             = (unsigned int)(uintptr_t)&key_down;
    cbs[PQ_CB_KEY_PRESS]            = (unsigned int)(uintptr_t)&key_press;
    cbs[PQ_CB_KEY_UP]               = (unsigned int)(uintptr_t)&key_up;
    cbs[PQ_CB_RESIZE]               = (unsigned int)(uintptr_t)&resize;
    cbs[PQ_CB_TOUCH]                = (unsigned int)(uintptr_t)&touch;
    cbs[PQ_CB_FOCUS]                = (unsigned int)(uintptr_t)&focus;
    cbs[PQ_CB_ON_CLIPBOARD_PASTE]   = (unsigned int)(uintptr_t)&on_clipboard_paste;
    cbs[PQ_CB_ALLOCATE_VEC_U8]      = (unsigned int)(uintptr_t)&allocate_vec_u8;
    cbs[PQ_CB_FILE_LOADED]          = (unsigned int)(uintptr_t)&file_loaded;
    cbs[PQ_CB_FILES_DROPPED_START]  = (unsigned int)(uintptr_t)&on_files_dropped_start;
    cbs[PQ_CB_FILES_DROPPED_FINISH] = (unsigned int)(uintptr_t)&on_files_dropped_finish;
    cbs[PQ_CB_FILE_DROPPED]         = (unsigned int)(uintptr_t)&on_file_dropped;

    pq_js_register_callbacks(cbs);
}

#endif /* __EMSCRIPTEN__ */
