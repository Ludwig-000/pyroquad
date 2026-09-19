# pyroquad on WebAssembly — one module, stock Pyodide

> Status: **working.** `_pyroquad` builds for `wasm32-unknown-emscripten` and
> links as a single Emscripten side module with **zero** unresolved imports.
> Inside unmodified Pyodide 314.0.7 (CPython 3.14.2) in Chrome:
>
> * `import pyroquad` — works
> * [`test.py`](../test.py) — works
> * [`test2.py`](../test2.py), the full ~2000-line `ExampleGame2D` with ~90
>   downloaded textures/fonts/sounds, custom fonts, audio and mouse+keyboard
>   input — works, at **59.8 Python-driven frames per second** (measured, §6).
>
> One wasm module. No protocol, no worker, no `SharedArrayBuffer`, no changes to
> the game code, and no fork of macroquad, miniquad, quad-snd or Pyodide.
>
> The earlier two-wasm split is kept for reference in
> [WASM_REJECTED_TWO_MODULE_SPLIT.md](WASM_REJECTED_TWO_MODULE_SPLIT.md). Nothing
> from it is used here; this replaces it entirely.

---

## 1. The premise that turned out to be wrong

The blocker was stated as a target mismatch:

* CPython-on-wasm only exists as **`wasm32-unknown-emscripten`** (Pyodide), and a
  pyo3 extension module for it is an Emscripten **side module** that Pyodide
  `dlopen()`s.
* macroquad/miniquad's web backend is written for **`wasm32-unknown-unknown`**,
  and expects miniquad's own loader (`gl.js` / `mq_js_bundle.js`) to do
  `WebAssembly.instantiate` and hand it an import object full of hand-written JS.

That reads like two incompatible ecosystems. It isn't, and the reason is one
detail of how miniquad is written:

**miniquad's WebGL binding is not a bespoke JS protocol. It is a plain OpenGL ES
2.0 C ABI binding.** `src/native/wasm/webgl.rs` declares `glClear`,
`glDrawArrays`, `glTexImage2D`, ... with their standard names and standard
signatures. `gl.js` is just *one possible implementation* of that C ABI.

Emscripten ships another implementation of exactly that ABI: `library_webgl.js`,
linked with `-lGL`. And **Pyodide already links it** — its `MAIN_MODULE_LDFLAGS`
contain `-lGL -legl.js -lwebgl.js -lhtml5.js -lhtml5_webgl.js -sMAX_WEBGL_VERSION=2`,
with `-sMAIN_MODULE=1` and `-sEXPORT_ALL=1`, which means every one of those JS
functions sits in Pyodide's `wasmImports` table and is therefore resolvable by
any side module it loads.

So the two ecosystems do not need bridging. They need *linking*.

## 2. What that costs, measured

A release build of the crate for `wasm32-unknown-emscripten` imports 266
functions from `env` (before this work; 282 in the finished build, because the
shim's own JS functions count as imports too). Cross-referencing that import list against Pyodide
314.0.7's exports + JS library:

| | count |
|---|---|
| `env` function imports | 266 |
| **already resolved by stock Pyodide** | **242** |
| left over | 23 |

The 242 include all 88 `gl*` calls macroquad actually makes, all of libc/libm,
the pthread stubs, the whole CPython C-API surface pyo3 uses, and the wasm
exception-handling personality routines (Rust 1.97 uses wasm EH here, and
Pyodide's main module exports the `__cpp_exception` tag, so they match).

The 23 left over are exactly miniquad's and quad-snd's own JS glue:

```
setup_canvas_size  run_animation_loop  canvas_width  canvas_height  dpi_scale
init_webgl         now                 console_log   sapp_set_clipboard
sapp_set_cursor    sapp_set_cursor_grab  sapp_set_fullscreen  sapp_schedule_update
fs_load_file       fs_get_buffer_size    fs_take_buffer
audio_init  audio_add_buffer  audio_play_buffer  audio_source_is_loaded
audio_source_set_volume  audio_source_stop  audio_source_delete
```

That is the entire gap. `src/web/shim.c` fills it (plus the handful of siblings
that happened to be dead-code-eliminated in this build — `console_debug/info/warn/error`,
`sapp_is_fullscreen`, `sapp_set_window_size`, `sapp_is_elapsed_timer_supported`,
`audio_playback_stop`, `audio_playback_set_volume` — so that enabling more of the
Python API later does not reopen the hole).

## 3. How the shim works

`src/web/shim.c` is compiled by `emcc` from `build.rs` and its object file is
handed to the final link. It is a port of miniquad `js/gl.js` (minus all the
`gl*` wrappers, which Pyodide provides) and quad-snd `js/audio.js`.

Three mechanisms make it work inside a side module:

**`EM_JS`.** Emscripten's dynamic loader reads a side module's `__em_js__<name>`
exports and evaluates their bodies with a **direct `eval` inside the runtime's
own closure**. That is what makes this viable at all: the JS bodies in `shim.c`
can see `HEAPU8`, `HEAPU32`, `UTF8ToString`, `getWasmTableEntry`, `GL`, `Module`
— the same scope a JS library file linked into the main module would get.
Verified present in Pyodide's `pyodide.asm.mjs` (`addEmJs` → `eval(func)`).

**Shared state on `globalThis.__PQ`.** Separate `EM_JS` functions cannot share
locals, so `pq_js_register_callbacks` builds one object holding the canvas, the
WebGL handle, the keycode table, the pending-file table and the WebAudio graph.
Because that object is *constructed inside an `EM_JS` body*, its methods still
close over the Emscripten runtime scope.

**Function pointers instead of `wasm_exports`.** `gl.js` calls
`wasm_exports.mouse_move(...)`. A side module's exports are not reachable under
`Module.` (CPython `dlopen`s extension modules into a local scope), so
`pq_platform_init()` takes the *addresses* of miniquad's `#[no_mangle]` entry
points (`frame`, `mouse_move`, `key_down`, `resize`, `touch`, `focus`,
`allocate_vec_u8`, `file_loaded`, the drag-and-drop trio, ...) into a small
array, and JS turns each into a callable with `getWasmTableEntry`.

### The one non-obvious trap

Every symbol miniquad links against is a **real, defined C function** in the shim
that forwards to a `pq_js_*` `EM_JS` function — never the `EM_JS` function
directly.

That indirection is load-bearing. An `EM_JS` function is, at the wasm level, an
*import* that carries its JS body along in a data export. Calling one is fine.
But if anything takes its **address**, the PIC build emits a `GOT.func.<name>`
relocation, and the dynamic loader has no address to hand back for a function
that only exists in JS — `dlopen` fails with `undefined symbol 'console_debug'`.
This was hit for real; defining the imported names as ordinary C
functions gives all of them a genuine table address, so calls *and* address-taking
resolve locally.

### WebGL context creation

`init_webgl` does not use `emscripten_webgl_create_context` — that C entry point
pulls in `emscripten_webgl_init_context_attributes`, which Pyodide
dead-code-eliminated, and it would couple us to the
`EmscriptenWebGLContextAttributes` struct layout of whichever Emscripten version
Pyodide was built with (5.0.3; the local emsdk here is 6.0.8). Instead the shim
calls `GL.createContext(canvas, attrs)` + `GL.makeContextCurrent(handle)` from
JS, which is precisely what that C function does internally, with a plain JS
object for the attributes. The result is that the shim depends on Emscripten only
through **names**, never through struct layouts — the 6.0.8-built module loads
into a 5.0.3-built Pyodide without trouble.

## 4. Layout

```
build.rs                    compiles src/web/shim.c with emcc on the
                            emscripten target only; no-op everywhere else.
                            Also emits the `pq_std_fs` cfg (see §6.4)
src/web/shim.c              the 30-odd platform functions (see §3)
src/web/mod.rs              the Rust half: the single-threaded engine loop,
                            the JSPI yields, the query dispatch path (see §6)

web/build_web.py            stages one artifact for the page: the wheel from
                            dist/ if there is one, else the .so + the Python
                            half zipped into web/pyroquad_pkg.zip
web/check_imports.py        build gate: every import resolvable? (see §5)
web/serve.py                threaded static dev server for the repo root
web/index.html              loads Pyodide, installs the staged wheel through
                            micropip (or unpacks the zip), runs the import
                            smoke test - or ?script=test.py / ?script=test2.py
                            to run one of the test scripts
web/get_pyodide.py          fetches pyodide-core, plus micropip's wheel, which
                            pyodide-core lists but does not ship
web/pyodide/                stock pyodide-core 314.0.7, unmodified
```

Everything the browser needs from the rest of the crate is behind
`#[cfg(target_os = "emscripten")]`; the native Windows/macOS/Linux build takes
the same paths it always did, and `cargo check` for the host target passes
unchanged. The engine-side files that gained a browser path are
`src/engine/CoreLoop.rs` (split into `EngineState` + `dispatch`, §6.4),
`src/engine/PChannel.rs`, `src/engine/PThreading.rs`,
`src/engine/MouseInsideScreen.rs`, `src/py_abstractions/py_functions.rs`
(`activate_engine`, `next_frame`, `draw_text`) and
`src/py_abstractions/Loading/`.

## 5. Building and running

There are two ways to build this, and they differ only in packaging.

### The shipped artifact: a PEP 783 wheel

[PEP 783](https://peps.python.org/pep-0783/) was accepted in April 2026 and PyPI
now accepts `pyemscripten_*_wasm32` wheels, so the browser build is published
alongside the native ones and installs with `micropip.install("pyroquad")`. This
is what CI builds and what `pip`/`micropip` hand people, so it is the version
that matters.

Here `pyodide-build` owns the cross-build environment — it decides the Emscripten
version, the RUSTFLAGS and the platform tag, so the wheel matches the Pyodide
that will load it:

```bash
pip install "pyodide-build>=0.39" "maturin>=1.13.2"
pyodide xbuildenv install 314.0.7

CARGO_TARGET_WASM32_UNKNOWN_EMSCRIPTEN_RUSTFLAGS="$(pyodide config get rustflags)" \
MATURIN_PYEMSCRIPTEN_PLATFORM_VERSION="$(pyodide config get pyodide_abi_version)" \
PYO3_CROSS_PYTHON_VERSION=3.14 \
  maturin build --release --target wasm32-unknown-emscripten --out dist --features abi_314
```

Two details are load-bearing:

* **`CARGO_TARGET_<TARGET>_RUSTFLAGS`, not `RUSTFLAGS`.** This crate has a
  `build.rs`, which is compiled for the *host*. A bare `RUSTFLAGS` would apply
  the side-module flags to the build script too and break it.
* **`pyodide-build` does not run natively on Windows**, so the wheel is built on
  Linux, macOS or WSL. The path below is the Windows-friendly one.

`python web/build_web.py` then copies the newest `dist/*wasm32.whl` to
`web/pyroquad.whl`, and `web/index.html` installs *that* with micropip - the
same path a user takes, so micropip rejects a mis-tagged wheel locally rather
than in the wild. `web/get_pyodide.py` fetches micropip's own wheel for this:
`pyodide-core` lists micropip in `pyodide-lock.json` but does not ship the file,
and `loadPackage` resolves it against the page's `indexURL`.

### The short loop: a bare side module

For iterating on the Rust side, skipping the wheel is faster and works anywhere.
`RUSTFLAGS` here is the hand-written equivalent of what `pyodide config get
rustflags` returns above:

```bash
# 1. the side module
RUSTFLAGS="-C link-arg=-sSIDE_MODULE=2" PYO3_CROSS_PYTHON_VERSION=3.14 \
  cargo build --target wasm32-unknown-emscripten --release --features abi_314
```

```bash
# 2. stage it for the page (falls back to the zip when dist/ has no wheel)
python web/build_web.py
```

```bash
# 3. serve, then open one of
#      http://127.0.0.1:8000/web/index.html                 import smoke test
#      http://127.0.0.1:8000/web/index.html?script=test.py
#      http://127.0.0.1:8000/web/index.html?script=test2.py the example game
python web/serve.py
```

`emcc` is found by `build.rs` via `$EMCC`, then `$EMSDK`, then the project-local
`emsdk/` (the same one `.cargo/config.toml` points the linker at), then PATH.
No emsdk activation step is needed. This path uses whatever emsdk is on hand
rather than the one Pyodide was built with; §3 is why that skew is tolerable.

A plain static server is enough — this port uses **JSPI**, not
`SharedArrayBuffer`, so no COOP/COEP is required. (`serve.py` sends the headers
anyway; they are harmless.)

### Verifying a build

`web/check_imports.py` parses the built `.wasm`'s import section and
cross-references every name against `web/pyodide/pyodide.asm.wasm`'s exports and
`pyodide.asm.mjs`'s `wasmImports` object — the same table Emscripten's
`resolveGlobalSymbol` consults at dlopen time. It exits non-zero on anything
unresolved, so it works as a build gate — which is exactly how CI uses it, on
both workflows.

```bash
# the short loop's output (its default path)
python web/check_imports.py
```

```bash
# or the bytes that actually ship, unpacked from the wheel
python -m zipfile -e dist/*.whl wheel_unpacked/
python web/check_imports.py wheel_unpacked/pyroquad/_pyroquad*.so
```

```
env function imports : 282
  resolved by Pyodide: 246
  supplied by shim   : 36
  UNRESOLVED         : 0
GOT relocations      : 1063
  UNRESOLVED         : 0
```

## 6. The runtime: one thread, no engine thread

Linking was only half the problem. The other half is that natively pyroquad is a
**two-thread** engine, and Pyodide has no threads.

`activate_engine()` spawns an OS thread that owns the macroquad window and drains
[`COMMAND_QUEUE`](../src/engine/CoreLoop.rs); the Python thread pushes commands
onto it and blocks on a [`PChannel`](../src/engine/PChannel.rs) whenever it needs
an answer back. On Emscripten `std::thread::spawn` fails outright
(`Os { code: 138, kind: Unsupported }`), and even if it did not, CPython, the
engine and the browser's event loop would still be sharing one thread.

The replacement rests on two mechanisms.

### 6.1 Frames are pulled, not pushed

`pq_set_frame_driver(1)` stops miniquad's `requestAnimationFrame` pump. From then
on the only thing that produces a frame is `pq_frame_now()`, called from Python's
`next_frame()`, which runs one miniquad frame **on the Python stack**:

```
next_frame()          Python
  └─ pq_frame_now()   → miniquad frame()
        begin_frame()                       macroquad clears + resets the draw list
        poll(engine_loop)                   drains COMMAND_QUEUE: every draw this
                                            frame, then Command::NextFrame
                                              → frame-info + physics + reply
                                              → yield  (this ends the frame)
        end_frame()                         flush to WebGL
  └─ receiver.recv()  the reply is already there
  └─ frame_yield()    JSPI: suspend, let the browser paint and deliver input,
                      resume
```

Because the engine only ever runs inside that call, it can never advance while
Python is halfway through a frame — which is the invariant the second OS thread
provided natively.

`Command::NextFrame` is handled in [`web::engine_loop`](../src/web/mod.rs) rather
than in the shared `dispatch`, for one reason: the end-of-frame bookkeeping has
to happen *before* the yield, because the yield is what returns control to
Python, and Python must find its reply already waiting. (Natively the order is
the other way round, since there the engine thread keeps running afterwards.)

### 6.2 Blocking becomes stack switching

`frame_yield()` is
`pyodide.ffi.run_sync(js.__PQ.frame_yield())` — a **JSPI** suspension. The whole
wasm stack, CPython frames included, is parked while the browser gets its turn,
then resumed exactly where it was. Synchronous Python (`while True: ...
next_frame()`) therefore works unmodified, and the page stays responsive.

JSPI needs no special build of anything: it wraps imports with
`WebAssembly.Suspending` and exports with `WebAssembly.promising` at
instantiation time, and Pyodide already does that (`pyodide.ffi.run_sync`,
reached from `runPythonAsync`). It is a **Chrome 137+** feature; the page reports
`JSPI available` on load, and pyroquad surfaces the failure rather than papering
over it if it is missing.

### 6.3 Why commands are split in two

macroquad brackets every frame with `begin_frame()` / `end_frame()`, and
`begin_frame()` **resets the draw list**. So anything that draws has to run
inside `pq_frame_now()`. But a command that owes Python an answer cannot wait for
a frame — Python is blocked on it *now*, so that frame would never come. Running
a frame to service it instead would throw away everything drawn so far this
logical frame.

[`Command::is_frame_local`](../src/engine/CoreLoop.rs) settles it, exhaustively,
for all 92 variants:

| | goes where | count |
|---|---|---|
| draws, camera state, `ClearBackground`, `NextFrame` | queued, run inside the frame | 35 |
| queries, resource creation, object/physics mutation, audio, file loads | run immediately at push time | 57 |

`CommandQueue::push` routes on it, so every existing `COMMAND_QUEUE.push(...)`
call site — there are hundreds — is unchanged. Ordering stays correct for
everything Python can observe, because an immediate command runs strictly before
any command Python queues after it.

Two consequences worth knowing:

* **`DrawText` is the only command that both draws and answers.** In the browser
  its two halves are separated: the measurement is taken immediately (it needs no
  frame) and only the drawing is queued. The numbers are identical, not
  approximate — macroquad derives the width from the same glyph advances as
  `measure_text`, scaled by `font_scale * font_scale_aspect`, while height and
  `offset_y` depend on `font_scale` alone and rotation affects none of them, so
  applying the aspect afterwards is exact.
* **Blocking queries cost nothing.** `measure_text`, texture creation, camera
  maths and so on resolve on the caller's stack with no round trip at all. This
  is the thing the rejected two-module port could not do: there every blocking
  call cost an engine tick, which is why its game logic ran at single-digit fps
  while its counter reported 60.

### 6.4 The supporting pieces

`CoreLoop.rs` was refactored so both drivers can share the command handling:
`EngineState` (the object storage + camera memory that used to be locals of
`proccess_commands_loop`) and `async fn dispatch(command, &mut EngineState)`
holding the original `match`. The native loop is now three lines around
`dispatch`; the browser loop lives in `src/web/mod.rs`. Behaviour on native is
unchanged.

Beyond that, four things needed browser paths:

* **Downloads.** `download_file_future` uses `miniquad::fs::load_file`, i.e. the
  `fetch` in `shim.c`, and hands the future straight back — so the example game's
  ~90 asset downloads genuinely overlap and the loading screen animates while
  they run. The blocking `download_file` is the same fetch, waited on by
  suspending through JSPI rather than by blocking the thread (blocking would stop
  the very event loop that has to deliver the response).
* **The filesystem.** Emscripten has a real, writable `std::fs` (MEMFS), which is
  easy to miss because it is also `target_arch = "wasm32"`. `build.rs` emits a
  `pq_std_fs` cfg for targets that have one, and `Loading.rs` keys off that
  instead of repeating a three-clause predicate per function. The effect is that
  `write_to_file` → `load_file` → `to_Texture2D()/to_font()/to_Sound()` — the
  exact pipeline `custom_assets.py` uses — works on the web with no changes to
  the game.
* **Threads.** `PThreading::thread_pool` / `limited_thread` and
  `ThreadedLoading::threaded_map` run their work inline on Emscripten.
* **`mouse_inside_window`.** The old wasm path called `web_sys`, but
  `wasm-bindgen` is implemented only for `wasm32-unknown-unknown` and panics at
  runtime under Emscripten. It now uses macroquad's own mouse position and screen
  size, which drops the `web-sys` dependency from this path entirely.

## 7. What was verified in the browser

Chrome, `http://127.0.0.1:8011/web/index.html`, stock `pyodide-core-314.0.7`.

**Import** (`?` with no script):

```
pyodide 314.0.7  (CPython 3.14.2)
JSPI available: true
python 3.14.2 / platform emscripten / exports 125
Color.RED <builtins.Color object>  |  Vec2(3,4).length() 5.0   →  import OK
```

That alone means the 7.4 MB side module `dlopen`s cleanly: all 282 `env` imports
bind, all 1063 `GOT` relocations resolve, every `__em_js__` body evaluates, and
pyo3's module init runs.

**`?script=test.py`** — a red 100×100 rectangle for 120 frames, then
`finished test.py`.

**`?script=test2.py`** — `ExampleGame_.launch_game()`: the download/read/decode
loading screens (with live percentages and real font rendering), then the main
menu with its background texture and custom fonts, then — after clicking *Play* —
the game world, with WASD movement and a camera that follows. Audio, textures,
fonts, mouse and keyboard all work. No console errors.

**Frame rate**, measured by counting actual Python-driven `next_frame()` calls
while the game was running:

```
python-driven frames: 180 in 3009 ms  =>  59.8 fps
```

That is game-logic frames, not a render-tick counter — one `next_frame()` per
displayed frame, pinned to the display's refresh rate.

## 8. Known gaps

* **Chrome only, for now.** JSPI is what makes synchronous Python yield to the
  browser; Firefox and Safari have not shipped it. There is no workaround that
  keeps the API synchronous other than Asyncify, which would mean rebuilding
  Pyodide and paying a large size/speed cost.
* **`set_fullscreen(True)` at startup is ignored**, as it is for any web page:
  browsers only grant fullscreen from a user gesture. The rejected promise is
  swallowed rather than logged.
* **Physics, meshes and custom shaders are untested on the web.** They compile
  and are wired through the same `dispatch`, and `rapier3d` is pure Rust, but the
  example game does not exercise them. `rapier3d`'s `parallel` feature pulls in
  rayon and should be turned off for wasm before relying on it.
* **`multiprocessing`** (the "multiple windows" example) cannot work in a browser.
* The engine-mediated `load_file` path for `wasm32-unknown-unknown` is still
  there but is now unused by this port, which goes through `std::fs`.

## 9. Toolchain used

* rustc/cargo 1.97.1 stable, target `wasm32-unknown-emscripten` installed.
* Project-local `emsdk/` at Emscripten 6.0.8 (nothing global was touched, and no
  emsdk activation is required — `build.rs` locates `emcc` itself).
* Pyodide 314.0.7 (CPython 3.14.2), the stock `pyodide-core` tarball, unmodified.
  Pyodide itself was built with Emscripten 5.0.3; the version skew is fine
  because, per §3, the shim depends on Emscripten only through symbol names.

That is what §7 was verified on. **CI does not reproduce the skew**: both
`.github/workflows/verify.yml` (every push) and `.github/workflows/ci.yml` (the
release) install the emsdk that `pyodide config get emscripten_version` names, so
the published wheel is built with the same Emscripten as the Pyodide that loads
it. The version pinned in `web/get_pyodide.py` is the single source of truth for
which Pyodide that is — both workflows read `DEFAULT_VERSION` back out of it
rather than hardcoding a version of their own.
