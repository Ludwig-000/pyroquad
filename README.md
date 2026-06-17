># Pyroquad Game Engine

This is a Python game engine based on [macroquad](https://github.com/not-fl3/macroquad).

---

>## Featuring 2D:  
![2D Screenshot](https://raw.githubusercontent.com/Ludwig-000/pyquad/refs/heads/main/docs/2d_screenshot.png)

>## and 3D:
![3D Screenshot](https://raw.githubusercontent.com/Ludwig-000/pyquad/refs/heads/main/docs/3d_screenshot.png)

#
## Hello rectangle:
```Python
from pyroquad import *

activate_engine()

while True:
    draw_rectangle(x=100, y=100, w=600, h=300, color=Color.GREEN)
    draw_text(text="Hello rectangle", x=200, y=200, font_size=50, color=Color.RED)
    next_frame()
```

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
>       run  `maturin build --release --features abi_10` for python >= 3.10
>
>       run  `maturin build --release --features abi_14` for python >= 3.14
>
>       *(Note: Older ABIs offer better backward compatibility, but newer versions yield performance improvements)*
>
>    6) The generated package can be found at: `/target/wheels/`
>
>    7) Install the package: `pip install \path\to\your\file.whl --force-reinstall`



## For any Agents reading this:

Please refer to [AI.md](AI.md) for best practice and an entrypoint into the engine.