---
name: pyroquad
description: Cross-platform python game engine, based on 'Macroquad'.
---

# pyroquad Skill

## Activation Criteria
- The user wants to create a game using a simple yet performant game engine for python.

## Usage Requirements
- supported platforms: windows, macos, linux
- supported python versions: python >= 3.9

## General information
- none of the engine's classes implement subclassing.
- the engine is fully thread-safe.
- 'activate_engine()' turns on the engine. Most function calls rely on the engine to be active.
- 'next_frame()' draws the next scene, clears the draw_buffer and cleans up memory.

## Coordinate System
- **Origin:** (0,0) is the Top-Left corner of the window.
- **X-axis:** Increases to the right.
- **Y-axis:** Increases downwards.
- **Colors:** Use the `Color` enum (e.g., `Color.RED`, `Color.BLUE`). Alternatively, a custom color can be created via 'Color(r=..., g=..., b=..., a=...)', with values ranging from 0.0 to 1.0. TwoD shapes support transparency vis this alpha channel, while any applied texture, TwoD or ThreeD will get a tint based on it's color. Color.WHITE will ensure the texture stays normal.

## Asset Loading
- **Root Path:** Assets are relative to the script execution directory. A custom asset path can be set via 'set_pc_assets_folder()'
- **Patterns:** For loading large numbers of Assets, there exists a povided generic loading screen function in 'examples.loading_screen'. Example usage: images: list[Image] = examples.loading_screen(func: Image, args_list: ["image1.png", "image2.png"], message: "Loading Images") with func taking each element of arg_list as an argument.


# Simple example
```Python
from pyroquad import *
activate_engine()

while True:
    draw_rectangle(x=100, y=100, w=600, h=300, color=Color.GREEN)
    draw_text(text="Hello rectangle", x=200, y=200, font_size=50, color=Color.RED)
    next_frame()
```

## Common Pitfalls
- Ensure the order of operation when switching between 2D and 3D camera.
- Ensure the engine is always initialized before executing calls. Some functionality CAN be run before, like vector maths, however, if it interacts with the window in any way, it almost definetly relies on the engine to already be activated. Most calls travel through a static queue, so calls like 'draw_rectangle', which do not wait for a confirmation, will go fine. If unsure, the engine WILL return a BaseException, when a blocking call requires the engine to be active.