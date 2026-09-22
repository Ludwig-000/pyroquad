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



>## How to run in the browser:
>
>place this code inside a `.html` file and open in the browser.
>
>```html
><style>#glcanvas { width: 800px !important; height: 600px !important; display: block; } <style>
><canvas id="glcanvas" width="800" height="600"></canvas>
><pre id="out"></pre>
>
><script type="module">
>import { loadPyodide } from "https://cdn.jsdelivr.net/pyodide/v314.0.3/full/pyodide.mjs";
>
>const out = document.getElementById("out");
>const print = t => out.textContent += t + "\n";
>
>const pyodide = await loadPyodide({ stdout: print, stderr: print });
>await pyodide.loadPackage("micropip");
>const micropip = pyodide.pyimport("micropip");
>await micropip.install("pyroquad");
>
>await pyodide.runPythonAsync(`
>from pyroquad import *
>
>activate_engine()
>
>while True:
>    draw_rectangle(x=100, y=100, w=600, h=300, color=Color.GREEN)
>    draw_text(text="Hello rectangle", x=200, y=200, font_size=50, color=Color.RED)
>    next_frame()
>    examples.limit_fps(60)
>`).catch(e => print(e));
></script>
>```


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
>In addition to the prerequisites above:
>
>- the `wasm32-unknown-emscripten` target: `rustup target add wasm32-unknown-emscripten`
>- `pip install "pyodide-build>=0.39" "maturin>=1.13.2"` — `pyodide-build` does not run
>  natively on Windows, so build the wheel on Linux, macOS or WSL
>- the [Emscripten SDK](https://emscripten.org/docs/getting_started/downloads.html), at the
>  version `pyodide config get emscripten_version` reports. No `emsdk_env` activation needed:
>  `emcc` is looked up via `$EMCC`, `$EMSDK`, the project-local `emsdk/`, then `PATH`.
>- a browser with JSPI (JavaScript Promise Integration) — Chrome 137+. That is what lets
>  synchronous Python (`while True: ... next_frame()`) hand the page back between frames.
>
>```bash
>pyodide xbuildenv install 314.0.7   # the Pyodide that --features abi_314 targets
>python web/get_pyodide.py           # fetch that runtime into web/pyodide/ (once)
>
>CARGO_TARGET_WASM32_UNKNOWN_EMSCRIPTEN_RUSTFLAGS="$(pyodide config get rustflags)" \
>MATURIN_PYEMSCRIPTEN_PLATFORM_VERSION="$(pyodide config get pyodide_abi_version)" \
>PYO3_CROSS_PYTHON_VERSION=3.14 \
>  maturin build --release --target wasm32-unknown-emscripten --out dist --features abi_314
>
>python web/build_web.py   # stage dist/*wasm32.whl for the test page
>python web/serve.py       # http://127.0.0.1:8000/web/index.html?script=tests/test_rec.py
>```
>
>The flags go in `CARGO_TARGET_<TARGET>_RUSTFLAGS`, never a bare `RUSTFLAGS`: this crate has
>a `build.rs`, which is compiled for the host.
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
