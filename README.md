# <img src="https://raw.githubusercontent.com/Ludwig-000/pyquad/refs/heads/main/docs/PyroquadLogo.png" width="40" align="left" style="margin-right: 15px;"> Pyroquad Game Engine
<br clear="left"/>



This is a Python game engine based on [macroquad](https://github.com/not-fl3/macroquad).


---

>## Featuring 2D:  
![2D Screenshot](https://raw.githubusercontent.com/Ludwig-000/pyquad/refs/heads/main/docs/2d_screenshot.png)

>## and 3D:
![3D Screenshot](https://raw.githubusercontent.com/Ludwig-000/pyquad/refs/heads/main/docs/3d_screenshot.png)




Install via `pip install pyroquad`

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
>    4) (**optional**) Run `cargo run --bin stub_gen --no-default-features --features use-real` to update [_pyroquad.pyi](src/python/pyroquad/_pyroquad.pyi) and gennerate up-to-date python stubs. 
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

activate_engine(Config(window_width=889, window_height=500))

texture =  Loading.download_file(
    "https://raw.githubusercontent.com/Ludwig-000/pyroquad/main/docs/PyroquadLogo.png"
).to_Texture2D()

rec = Rectangle(
    position=Vec2.splat(500),
    rotation=0,
    scale=Vec2.splat(1000),
    color=Color.WHITE,
    texture=texture[0])

while True:
    rec.draw()
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

player = examples.PlayerCamera(position=Vec3.ONE)

skybox_tex = examples.loading_screen(
    lambda a: download_file(a).to_Texture2D(),
    ["https://raw.githubusercontent.com/Ludwig-000/pyroquad/main/tests/HDR_blue_nebulae_2.png"]
)[0]

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