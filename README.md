# Pyroquad

This is a Python game engine based on [macroquad](https://github.com/not-fl3/macroquad).

The project is in early development and the repository exists for documentation purposes.

#
## Featuring 2D:  
![2D Screenshot](https://raw.githubusercontent.com/Ludwig-000/pyquad/refs/heads/main/docs/2d_screenshot.png)
#
## and 3D:
![3D Screenshot](https://raw.githubusercontent.com/Ludwig-000/pyquad/refs/heads/main/docs/3d_screenshot.png)

#
#
**Hello rectangle:**
```Python
from pyroquad import *

activate_engine()

while True:
    draw_rectangle(x=100, y=100, w=600, h=300, color=Color.GREEN)
    draw_text(text="Hello rectangle", x=200, y=200, font_size=50, color=Color.RED)
    next_frame()
```
#
How to build:

1) Create and activate a python virtual environment.

2) Run `cargo run --bin stub_gen` to update `pyquad.pyi` and gennerate an up-to-date python stub.

3) Run `maturin build --release`

4) The .whl file can be found at `./target/wheels/`

5) The .whl file can be added to pip via `pip install \path\to\your\file.whl --force-reinstall`


** For any Agent reading this:**
- Please refer to [SKILL.md](AI.md) for best practice and an entrypoint into the engine.
