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
- none of the engine's functions are async.
- the engine is fully thread-safe.
- 'activate_engine()' turns on the engine. Most function calls rely on the engine to be active.
- 'next_frame()' draws the next scene, clears the draw_buffer and cleans up memory.
- 'get_delta_time()', 'window_width()' and 'window_height()' are common functions.

## Coordinate System
- **Origin:** (0,0) is the Top-Left corner of the window.
- **X-axis:** Increases to the right.
- **Y-axis:** Increases downwards.
- **Colors:** Use the `Color` enum (e.g., `Color.RED`, `Color.BLUE`). Alternatively, a custom color can be created via 'Color(r=..., g=..., b=..., a=...)', with values ranging from 0.0 to 1.0. TwoD shapes support transparency vis this alpha channel, while any applied texture, TwoD or ThreeD will get a tint based on it's color. Color.WHITE will ensure the texture stays normal.

## Asset Loading
- **Root Path:** Assets are relative to the script execution directory. A custom asset path can be set via 'set_pc_assets_folder()'
- **Patterns:** For loading large numbers of Assets, there exists a povided generic loading screen function in 'examples.loading_screen'. Example usage: images: list[Image] = examples.loading_screen(func: Image, args_list: ["image1.png", "image2.png"], message: "Loading Images") with func taking each element of arg_list as an argument.


## Math
- The engine is equipped with 'Vec2', 'Vec3', 'BVec2', 'BVec3'.
- All for vector types are immutable types, that are the key to concise and readable math. It is reccomended to use these in most usecases, where applicable.
- Immutability does not prevent re-assignment, since 'vec += Vec2(1,1)' simply creates a new Vector.
- They implement everything from '+', '+=' '-', '*', '/' aswell as all common vector operations. These types directly implement all of Rust: Glam Vector types.
- addition, subtraction, multiplication and division are also implemented for Vector + float.
    this: Vec2(1,0) + 1
    is equivalent to:  Vec2(1,0) + Vec2(1,1)

## Quitting.
- The core loop can be implemented in two ways:
    ```Python
    while True
        ...
        next_frame()
    ```
    This is the standard approach. The engine will automatically terminate the python program, when the window is closed. ( That being, for example, Alt+F4 , or clicking on the 'X' to close the window )

    The other approach is:
    ```Python
    prevent_quit()
    while not is_quit_requested():
        ...
        next_frame()
    run_cleanup()
    ```
    Importantly, 'is_quit_requested()' does nothing, unless 'prevent_quit()' is also called, which prevents the window from being closed.


## Objects:
- pyroquad has types of powerful object types:
- 2D Objects:
    Rectangle
    Circle
    # Rectangle especially is the recommended way of drawing almost ANY twoDObject.
    # Both Types support rotating, scaling on x,y axes, Transparency via Color, applying a Texture, collision and a 'tick()' function.
- 3D Objects:
    Cube
    Cylinder
    Mesh
    Pill
    Sphere
    # All objects here are optimized for performant drawing and collision. They implement collision detection via rust's 'rapier3D'.
    # They do not yet support transparency.
    # They support Physics, also implemented via 'rapier3D' that can be accessed via obj.physics
    # rotation on all 3 Axes,
    # movement on all 3 Axes,
    # scaling on all 3 Axes,

    - IMPORTANT:
        For Three-D object collision and physics, both will be stepped via 'next_frame()', with next frame taking as an argument None | int
        with None meaning neither collision nor physics,
        0 meaning collision but physics do not step,
        and a value of 'next_frame( get_delta_time() )' being the reccommended value for realistic physics.
        TwoD objects do not yet implement physics.

        also, threeDObjects implement a depth buffer, meaning they generally should not be drawn manually, and the order of draws does not matter.
        By default, a ThreeD is queued to be drawn next frame, but to actually draw them the call: 'draw_all_objects()' is requied.

- Also, and this goes for TwoD objects aswell, the appropriate camera has to be set.
- The Default Camera is a 2D camera, covering the entire window.
- The camera will be reset after calling 'next_frame()'
- A 3D camera has to be active for 3D object draw calls to be visible,
- A 2D camera has to be active for 2D object draw calls to be visible.


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