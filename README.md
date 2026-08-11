# <img src="https://raw.githubusercontent.com/Ludwig-000/pyquad/refs/heads/main/docs/PyroquadLogo.png" width="40" align="left" style="margin-right: 15px;"> Pyroquad Game Engine
<br clear="left"/>



This is a Python game engine based on [macroquad](https://github.com/not-fl3/macroquad).


* Install via `pip install pyroquad`
* Requires Python >= 3.9
* Supported Platforms: windows, linux, mac, (wasm eventually)

---

>## Featuring 2D:  
![2D Screenshot](https://raw.githubusercontent.com/Ludwig-000/pyquad/refs/heads/main/docs/2d_screenshot.png)

>## and 3D:
![3D Screenshot](https://raw.githubusercontent.com/Ludwig-000/pyquad/refs/heads/main/docs/3d_screenshot.png)





>## How to build:
>    1) Prerequesites:
>       - Python >= 3.9 ( python >= 3.14 is recommended)
>       - Rust Compiler >= 1.85.0 (Required for the Rust 2024 Edition)
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
>    4) (**optional**) Run `cargo run --bin stub_gen --no-default-features --features use-real` to gennerate up-to-date python stubs in [init.pyi](src/python/pyroquad/_pyroquad/__init__.pyi). 
>
>       Stub generation specifically requires python >= 3.10
>
>    5) To compile:
>
>       run  `maturin build --release --features abi_39` for python >= 3.9
>
>       run  `maturin build --release --features abi_310` for python >= 3.10
>
>       run  `maturin build --release --features abi_314` for python >= 3.14
>
>       *(Note: Older ABIs offer better backward compatibility, but newer versions yield performance improvements)*
>
>    6) The generated package can be found at: `/target/wheels/`
>
>    7) Install the package: `pip install \path\to\your\file.whl --force-reinstall`



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