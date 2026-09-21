# <img src="https://raw.githubusercontent.com/Ludwig-000/pyquad/refs/heads/main/docs/PyroquadLogo.png" width="40" align="left" style="margin-right: 15px;"> Pyroquad Game Engine
<br clear="left"/>



This is a Python game engine based on [macroquad](https://github.com/not-fl3/macroquad).


* Install via `pip install pyroquad`
  (in the browser: `micropip.install("pyroquad")`)
* Requires Python >= 3.9
* Supported Platforms: windows, linux, mac, and WASM via Piodide.

---

>## Featuring 2D:  
#### [view the demo here](https://ludwig-000.github.io/Pyroquad_example_game_assets/)
![2D Screenshot](https://raw.githubusercontent.com/Ludwig-000/pyquad/refs/heads/main/docs/2d_screenshot.png)

>## and 3D:
![3D Screenshot](https://raw.githubusercontent.com/Ludwig-000/pyquad/refs/heads/main/docs/3d_screenshot.png)





>## How to build:
>    1) Prerequesites:
>       - Python >= 3.9 ( python >= 3.14 is recommended)
>       - Rust Compiler >= 1.85.0 (Required for the Rust 2024 Edition)
>       - **System libraries if using Linux** (required for X11, OpenGL, and audio bindings):
>         
>         *Ubuntu / Debian:*
>         `sudo apt-get install -y libasound2-dev libx11-dev libxi-dev libgl1-mesa-dev libxcursor-dev libxinerama-dev libxrandr-dev`
>         
>         *Fedora:*
>         `sudo dnf install alsa-lib-devel libX11-devel libXi-devel mesa-libGL-devel libXcursor-devel libXinerama-devel libXrandr-devel`
>         
>         *Arch Linux:*
>         `sudo pacman -S alsa-lib libx11 libxi mesa libxcursor libxinerama libxrandr`
>
>
>    2) Create and activate a Python virtual environment:
>
>        - macOS / Linux:
>
>          `python3 -m venv .venv`
>
>          `source .venv/bin/activate`
>
>       - Windows:
>
>         `python -m venv .venv`
>
>         `.venv\Scripts\activate`
>
>    3) Install the [maturin build tool](https://github.com/PyO3/maturin) using `pip install maturin`
>
>    5) Compilation:
>
>       run  `maturin build --release --features abi_39 --generate-stubs    ` for python >= 3.9
>
>       run  `maturin build --release --features abi_310 --generate-stubs    ` for python >= 3.10
>
>       run  `maturin build --release --features abi_314 --generate-stubs    ` for python >= 3.14
>
>       *(Note: Older ABIs offer better backward compatibility, but newer versions yield performance improvements)*
>
>    6) Installation:
>       - The generated package can be found at: `/target/wheels/`
>       - Install the package: `pip install \path\to\your\file.whl --force-reinstall`




>## How to build for the browser (WebAssembly):
>
>The web build is a single WebAssembly module that runs inside
>[Pyodide](https://pyodide.org) (CPython compiled with Emscripten). Nothing is
>forked or patched - macroquad, miniquad, quad-snd and Pyodide are all stock.
>[docs/WASM.md](docs/WASM.md) explains how and why it works.
>
>Since [PEP 783](https://peps.python.org/pep-0783/) the browser build is
>published to PyPI like any other wheel, so **most people never need to build it
>at all** - see *Running in the browser* below. The rest of this section is for
>building it yourself.
>
>    1) Prerequesites (in addition to the ones above):
>       - The `wasm32-unknown-emscripten` Rust target:
>
>         `rustup target add wasm32-unknown-emscripten`
>
>       - `pyodide-build` and a recent `maturin`:
>
>         `pip install "pyodide-build>=0.39" "maturin>=1.13.2"`
>
>         `pyodide-build` owns the cross-build environment: it decides the
>         Emscripten version, the linker flags and the wheel's platform tag, so
>         that what you build matches the Pyodide that will load it. `maturin`
>         1.13.2 is the first release that emits the PEP 783 tag.
>
>         **`pyodide-build` does not run natively on Windows.** Build the wheel
>         on Linux, macOS or WSL. For local iteration on Windows there is a
>         wheel-less path - see *Developing on the module itself* below.
>
>       - The [Emscripten SDK](https://emscripten.org/docs/getting_started/downloads.html),
>         at the version Pyodide was built with (step 3 prints it). The build
>         script looks for `emcc` under `$EMCC`, then `$EMSDK`, then the
>         project-local `emsdk/`, then `PATH`, so **no `emsdk_env` activation
>         step is needed**:
>
>         `git clone https://github.com/emscripten-core/emsdk.git`
>
>         `./emsdk/emsdk install <version>`
>
>         `./emsdk/emsdk activate <version>`
>
>         *(on Windows, use `emsdk\emsdk.bat` in place of `./emsdk/emsdk`)*
>
>       - A browser with **JSPI** (JavaScript Promise Integration): Chrome 137+.
>         This is what lets synchronous Python (`while True: ... next_frame()`)
>         hand the page back to the browser between frames. Firefox and Safari
>         have not shipped it yet.
>
>    2) Fetch the Pyodide runtime into `web/pyodide/` (once):
>
>       `python web/get_pyodide.py`
>
>       This is the stock `pyodide-core` release, unpacked and otherwise
>       untouched - the web build deliberately runs on an unmodified Pyodide.
>       It is what the local test page loads, and what `check_imports.py`
>       resolves symbols against.
>
>       *(Pyodide releases are versioned after the CPython they ship: the pinned
>       314.0.7 is CPython 3.14.2, which is what `--features abi_314` targets.
>       `python web/get_pyodide.py <version>` takes a different one.)*
>
>    3) Install the matching cross-build environment:
>
>       `pyodide xbuildenv install 314.0.7`
>
>       Then read back what it wants - these are the values the build uses, and
>       `emscripten_version` is the emsdk version to install in step 1:
>
>       `pyodide config get emscripten_version`
>
>       `pyodide config get rustflags`
>
>       `pyodide config get pyodide_abi_version`
>
>    4) Build the wheel (macOS / Linux / WSL):
>
>       ```
>       CARGO_TARGET_WASM32_UNKNOWN_EMSCRIPTEN_RUSTFLAGS="$(pyodide config get rustflags)" \
>       MATURIN_PYEMSCRIPTEN_PLATFORM_VERSION="$(pyodide config get pyodide_abi_version)" \
>       PYO3_CROSS_PYTHON_VERSION=3.14 \
>       maturin build --release --target wasm32-unknown-emscripten --out dist --features abi_314
>       ```
>
>       This writes `dist/pyroquad-<version>-cp314-abi3-pyemscripten_<year>_<n>_wasm32.whl`.
>
>       `pyodide config get rustflags` is what turns the crate into an Emscripten
>       *side module* - the same thing Pyodide builds every other extension
>       module as. It is passed as `CARGO_TARGET_<TARGET>_RUSTFLAGS` rather than
>       as a bare `RUSTFLAGS` on purpose: this crate has a `build.rs`, which is
>       compiled for the **host**, and bare `RUSTFLAGS` would apply the
>       side-module flags to it as well and break it.
>
>    5) Check the build (optional, but it catches the one failure mode that is
>       otherwise only visible as a cryptic `dlopen` error in the browser):
>
>       ```
>       python -m zipfile -e dist/*.whl wheel_unpacked/
>       python web/check_imports.py wheel_unpacked/pyroquad/_pyroquad*.so
>       ```
>
>       It cross-references every symbol the module imports against what Pyodide
>       actually provides, and exits non-zero if anything is unresolved.
>
>    6) Stage the wheel for the test page, then run it:
>
>       `python web/build_web.py`
>
>       `python web/serve.py`
>
>       then open
>       - `http://127.0.0.1:8000/web/index.html?script=tests/test_rec.py`
>
>       `build_web.py` copies the newest `dist/*wasm32.whl` to
>       `web/pyroquad.whl`, and the page installs it with micropip - the same
>       path `micropip.install("pyroquad")` takes for a real user, so a
>       mis-tagged wheel is rejected here rather than in the wild.
>
>
>## Running in the browser:
>
>Install it inside [Pyodide](https://pyodide.org) with
>[micropip](https://micropip.pyodide.org) - the `pyemscripten_*_wasm32` wheel
>comes straight from PyPI, exactly like the desktop ones:
>
>```html
><script type="module">
>  import { loadPyodide } from "./pyodide/pyodide.mjs";
>
>  const pyodide = await loadPyodide();
>  await pyodide.loadPackage("micropip");
>  const micropip = pyodide.pyimport("micropip");
>  await micropip.install("pyroquad");
>
>  await pyodide.runPythonAsync(`
>      from pyroquad import *
>
>      activate_engine()
>
>      while True:
>          draw_rectangle(x=100, y=100, w=600, h=300, color=Color.GREEN)
>          next_frame()
>          examples.limit_fps(60)
>  `);
></script>
>```
>
>The page needs a `<canvas>` for the engine to draw into, and Chrome 137+ for
>JSPI. `web/index.html` is a complete working example of both.
>
>A plain static server is enough - this port uses JSPI rather than
>`SharedArrayBuffer`, so no COOP/COEP headers are required.
>
>
>## Developing on the module itself:
>
>For iterating on the Rust side there is a shorter loop that skips the wheel
>(and works on Windows, where `pyodide-build` does not run). It builds the side
>module directly and hands it to the test page as a zip:
>
>    - macOS / Linux:
>
>      `RUSTFLAGS="-C link-arg=-sSIDE_MODULE=2" PYO3_CROSS_PYTHON_VERSION=3.14 cargo build --target wasm32-unknown-emscripten --release --features abi_314`
>
>    - Windows (PowerShell):
>
>      `$env:RUSTFLAGS="-C link-arg=-sSIDE_MODULE=2"; $env:PYO3_CROSS_PYTHON_VERSION="3.14"`
>
>      `cargo build --target wasm32-unknown-emscripten --release --features abi_314`
>
>then `python web/build_web.py`. With no wheel in `dist/` it falls back to
>zipping the module together with the pure-Python half of the package into
>`web/pyroquad_pkg.zip`, which `web/index.html` unpacks directly instead of
>going through micropip. Re-run it after every `cargo build`.
>
>Only ever one of the two is staged - `build_web.py` deletes the other - so the
>page cannot quietly load a stale artifact of the kind you are not testing.
>
>*(Remember to clear `RUSTFLAGS` again before building natively in the same
>shell - it applies to every target. This path uses whatever emsdk you have
>rather than the one Pyodide was built with; that skew is usually fine, for the
>reason given in [docs/WASM.md](docs/WASM.md) §3, but the wheel build in step 4
>is the one that matches the shipped artifact.)*
>



## For any Agents reading this:

Please refer to [AI.md](AI.md) for best practice and an entrypoint into the engine.



## Example Code: 
### Hello rectangle
```Python
from pyroquad import *

activate_engine()

while True:
    draw_rectangle(x=100, y=100, w=600, h=300, color=Color.GREEN)
    draw_text(text="Hello rectangle", x=200, y=200, font_size=50, color=Color.RED)
    next_frame()
    examples.limit_fps(60)
```

### Textures
```Python
from pyroquad import *

activate_engine()

texture =  Loading.download_file(
    "https://raw.githubusercontent.com/Ludwig-000/pyroquad/main/docs/PyroquadLogo.png"
).to_Texture2D()

while True:
    draw_texture(texture)
    next_frame()
    examples.limit_fps(60)
```

### Cube
```Python
from pyroquad import *

activate_engine()

cube = Cube(
    position=Vec3.ZERO,
    rotation=Vec3.ZERO,
    scale=Vec3.ONE,
    color=Color.RED)

cam = Camera3D(position=Vec3.splat(2), target=Vec3.ZERO)

while True:
    cam.set_camera()
    cube.rot += get_delta_time()
    draw_all_objects()
    next_frame()
    examples.limit_fps(60)
```

### Skybox
```Python
from pyroquad import *

activate_engine()


skybox_tex = examples.loading_screen_future(
    lambda a: download_file_future(a),
    ["https://raw.githubusercontent.com/Ludwig-000/pyroquad/main/tests/HDR_blue_nebulae_2.png"],
    show_rotating_square=True
)[0].to_Texture2D()

player = examples.PlayerCamera(position=Vec3.ONE)

while True:
    if KeyCode.Escape in get_keys_pressed():
        break

    player.update()
    draw_skybox(skybox_tex)
    draw_grid(
        slices=1_000,
        spacing=1.0,
        axes_color=Color.YELLOW,
        other_color=Color.GREEN)

    next_frame()
    examples.limit_fps(60)
```

### Multiple windows
```Python
import multiprocessing
from pyroquad import *

def task(message, color_name):
    activate_engine()
    prevent_quit()
    color = getattr(Color, color_name)
    
    while not is_quit_requested():
        clear_background(color)
        draw_text(message, 200, 200, Color.GREEN, 60)
        next_frame()
        examples.limit_fps(60)

if __name__ == "__main__":
    seq_data = [("multiple", "YELLOW"), ("windows", "BRICK"), ("using", "ORANGE")]
    for msg, color in seq_data:
        p = multiprocessing.Process(target=task, args=(msg, color))
        p.start()
        p.join()

    procs = [multiprocessing.Process(target=task, args=("multiprocessing", "BLUE")) for _ in range(5)]
    for p in procs: p.start()
    for p in procs: p.join()
```

### Rectangle deletes itself (heartbreaking)
```Python
from pyroquad import *

activate_engine()

re = Rectangle(Vec2.splat(200), 0, Vec2.splat(100), Color.WHITE)

timer  = 120
def t(rec: Rectangle):
    global timer, re
    timer-= 1
    if timer == 0:
        del(re)
    rec.draw()

re.tick(t)

while True:
    next_frame()
    examples.limit_fps(60)
```
