---
name: pyroquad
description: Cross-platform python game engine, based on 'Macroquad'.
---

# pyroquad Skill

## Activation Criteria
- The user wants to create a game using a simple yet performant game engine for python.

## Usage Requirements
- **Platforms:** Windows, macOS, Linux
- **Python:** >= 3.9
- **Tooling:** Highly recommended to use **Pylance + Type Checking =(Standard)**. The engine is built with strict typing and avoids `Any`, allowing the IDE to provide full API discovery.

## General information
- **Architecture:** No subclassing; no async functions.
- **Initialization:** `activate_engine()` must be called to start the engine. Most functions require an active engine; blocking calls throw `BaseException` if called early.
- **Architecture:** No support for subclassing. No async functions.
- **Thread-Safety:** Fully thread-safe.
- **Utilities:** `next_frame()` draws, clears buffer, and cleans memory. Use `get_delta_time()`, `window_width()`, and `window_height()` for frame logic.

## Coordinate System & Colors

- **Origin:** (0,0) is Top-Left. X increases right, Y increases down.
- **Colors:** Use `Color` enum or `Color(r, g, b, a)` (0.0 to 1.0). All pre-defined colors are properties.
- **Textures:** 2D shapes support alpha; textures are tinted by the current color (`Color.WHITE` for no tint).

## Asset Loading
- **Root Path:** Assets are relative to the script execution directory. A custom asset path can be set via 'set_pc_assets_folder()'
- **Patterns:** For loading large numbers of Assets, there exists a povided generic loading screen function in 'examples.loading_screen'. Example usage: images: list[Image] = examples.loading_screen(func: Image, args_list: ["image1.png", "image2.png"], message: "Loading Images") with func taking each element of arg_list as an argument.

## Config
Configured via `Config(...)` passed to `activate_engine()`:
- **Fullscreen:** If `True`, `window_width/height` are overwritten by monitor max.
- **Swap Interval:** `None` (V-Sync), `0` (Max speed). Driver dependent; manual caps recommended if the user requests such.
- **Advanced Exit Flow:** `stop_python_when_closing_window` When set to False, combine with `prevent_quit()` -> prevents the window from closing, and `is_quit_requested()` to intercept the close signal and perform cleanup before the process terminates.

## Math
- The engine is equipped with `Vec2`, `Vec3`, `BVec2`, `BVec3`.
- All for vector types are immutable types, that are the key to concise and readable math. It is reccomended to use these in most usecases, where applicable.
- Immutability does not prevent re-assignment, since `vec += Vec2(1,1)` simply creates a new Vector.
- They implement everything from `+`, `+=` `-`, `-=` `*`, `*=`, `/`, `/=` aswell as all common vector operations. These types directly implement all of Rust: Glam Vector types.
- addition, subtraction, multiplication and division are also implemented for Vector + float.
    this: Vec2(1,0) + 1
    is equivalent to:  Vec2(1,0) + Vec2(1,1)
Common functions for Vec2 and Vec3 to use whenever applicable:
	.splat(self,value)
	.normalize(self)
	.normalize_or_zero(self)
	.length(self)
	.length_squared(self)
	.dot(self,rhs)
	.distance(self,rhs)
	.angle_between(self,rhs)
	.project_onto(self,rhs)
	.clamp_length(self,min, max)
	.move_towards(self,rhs, d)


## Quitting & Loops
### Standard Approach
```Python
while True
    ...
    next_frame()
```

### Manual Control
```Python
prevent_quit()
while not is_quit_requested():
    ...
    next_frame()
# run manual cleanup
```
Importantly, `is_quit_requested()` does nothing, unless `prevent_quit()` is also called, which prevents the window from being closed.

## Camera:
The set camera will be reset each frame.
the camera can be set by either creating a 2d or 3d camera, and calling `.set_camera()`
By default, you will be drawing to absolute pixels to the screen. that means, if the window is 600p by 800p, and a rectangle is drawn at 600p-900p, it will only be visible if the window is resized.
Cameras create a virtual screen, that is always projected onto the window, meaning resizing the window no longer reveals more screen, it simply stretches the bounds of the camera. the default behaviour can be re-established by calling `set_default_camera()`. This does not call, for the default 2D camera to be set, but for drawing to continue without a camera.


## Objects:
- pyroquad has types of powerful object types:
- 2D Objects:
    Rectangle, Circle
    - Rectangle especially is the recommended way of drawing almost ANY twoDObject.
    - Both Types support rotating, scaling on x,y axes, Transparency via Color, applying a Texture, collision and a `tick()` function.
    - Rectangle alone has special alternate setters and getters: `x1`,`y1`,`x2`,`y2` that allow for the corner of the rectangle to be moved. this is an alternative to moving it via position + scale. As a suplement, Rectangle also comes with an alternate constructor:
    def from_xy(x1, y1, x2, y2, color, texture) -> Rectangle:
- 3D Objects:
    Cube
    Cylinder
    Mesh
    Pill
    Sphere
    - All objects here are optimized for performant drawing and collision detection. They implement collision detection via rust's `rapier3D`.
    - ThreeD objects do not yet support transparency.
    - They support Physics, also implemented via `rapier3D` that can be accessed via obj.physics
    - rotation on all 3 Axes,
    - movement on all 3 Axes,
    - scaling on all 3 Axes,

    **IMPORTANT:**
    For Three-D object collision and physics, both will be stepped via `next_frame()`, with next frame taking as an argument None | int
    with None meaning neither collision nor physics,
    0 meaning collision but physics do not step,
    and a value of `next_frame( get_delta_time() )` being the reccommended value for realistic physics.
    TwoD objects do not yet implement physics.

    - Also, threeDObjects implement a depth buffer, meaning they generally should not be drawn manually, and the order of draws does not matter.
    By default, a ThreeD is queued to be drawn next frame, but to actually draw them the call: `draw_all_objects()` is requied.

    - To manage large ammounts of Objects, the running of an object's tick function is based on the lifetime of the object. AKA. an object's tick function will stop executing, once an object has been deleted,

    - And specifically for 3D objects, the same goes for their Physics and drawing. for this reason, it is recommended, in case of many objects, to keep them all in arrays, where they can be kept and deleted at will.

- For mixed ThreeD - and TwoD scenes, it is recommended to FIRST draw all ThreeD objects, THEN all TwoD objects, since TwoD objects do not have a depth buffer, while ThreeD objects do.

- Also, and this goes for TwoD objects aswell, the appropriate camera has to be set.
- The Default Camera is a 2D camera, covering the entire window.
- The camera will be reset after calling `next_frame()`
- A 3D camera has to be active for 3D object draw calls to be visible,
- A 2D camera has to be active for 2D object draw calls to be visible.


## Common Pitfalls
- Draw Order: Draw 3D objects before 2D objects.
- Cameras: 2D/3D cameras must be explicitly set for their respective objects to appear. Camera resets after next_frame().
- Lifetimes: `tick()` and physics stop when the object is deleted. Manage via arrays.

## Debugging & Resources
1. **Source Code/Repo:** [Github Repository](https://github.com/Ludwig-000/pyroquad) — This is the primary source for the most up-to-date documentation and logic.
2. **Examples:** Reference the `src/python/pyroquad/examples/...` directory in the source for "Golden Standard" implementations of core features.
3. **Stub Documentation:** Most function calls include usage information and parameter details directly in their stub types (accessible via hover in your IDE) or in the repository under `src/python/pyroquad/_pyroquad.pyi`
4. **Local README:** If offline, the `.whl` file contains an internal `README.md` (extractable via zip tools).

# Example code can be found in README.md