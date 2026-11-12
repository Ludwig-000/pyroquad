"""Opens a native Wayland window and uses it: draws, reads pixels back, resizes, queries input.

Run under a Wayland compositor with no X server (DISPLAY unset), e.g. the headless
Weston started by .github/workflows/wayland_test.yml. The engine is forced to
LinuxBackend.WaylandOnly, so if Wayland doesn't work, it fails instead of falling back to X11.
"""
import os
import sys

import pyroquad as p

if sys.platform != "linux":
    sys.exit("wayland_test.py only runs on Linux")
if not os.environ.get("WAYLAND_DISPLAY"):
    sys.exit("WAYLAND_DISPLAY is not set - start a Wayland compositor first")
if os.environ.get("DISPLAY"):
    sys.exit("DISPLAY is set - unset it so nothing can fall back to X11")

W, H = 320, 240

p.activate_engine(p.Config(
    window_title="pyroquad wayland test",
    window_width=W,
    window_height=H,
    window_resizable=True,
    high_dpi=False,
    swap_interval=0,
    linux=p.LinuxConfig(
        backend=p.LinuxBackend.WaylandOnly,
        wayland_decorations=p.WaylandDecorations.ServerWithMiniquadFallback,
        wm_class="pyroquad-wayland-test",
    ),
))
print("window opened:", p.screen_width(), "x", p.screen_height(), flush=True)

for _ in range(5):
    p.next_frame()


def pixel(img, x, y):
    """RGBA at (x, y) from the top-left. get_screen_data() rows come bottom-up."""
    sy = img.height - 1 - y
    i = (sy * img.width + x) * 4
    return tuple(bytes(img.bytes)[i:i + 4])


def close(a, b, tol=8):
    return all(abs(x - y) <= tol for x, y in zip(a[:3], b[:3]))


# Draw a known picture and read it back from the GPU.
for frame in range(3):
    p.clear_background(p.Color(0, 0, 1, 1))
    p.draw_rectangle(0, 0, 40, 40, p.Color(1, 0, 0, 1))
    p.draw_circle(W / 2, H / 2, 30, p.Color(0, 1, 0, 1))
    if frame == 2:
        img = p.get_screen_data()
    p.next_frame()

print("framebuffer:", img.width, "x", img.height, flush=True)
checks = {
    "top-left red rectangle": (pixel(img, 10, 10), (255, 0, 0)),
    "green circle centre": (pixel(img, W // 2, H // 2), (0, 255, 0)),
    "blue background": (pixel(img, W - 10, H - 10), (0, 0, 255)),
}
failed = False
for name, (got, want) in checks.items():
    ok = close(got, want)
    failed |= not ok
    print(f"{'ok  ' if ok else 'FAIL'} {name}: got {got[:3]}, want {want}", flush=True)

# Input queries must work without an X server.
print("mouse_inside_window:", p.mouse_inside_window(), flush=True)
print("mouse buttons down:", p.get_mouse_buttons_down(), flush=True)

# Ask the compositor for a new size and keep rendering.
p.request_new_screen_size(400, 300)
for _ in range(10):
    p.clear_background(p.Color(0.2, 0.2, 0.2, 1))
    p.next_frame()
print("after resize request:", p.screen_width(), "x", p.screen_height(), flush=True)

if failed:
    print("wayland test FAILED: rendered pixels don't match", flush=True)
    os._exit(1)
print("wayland test passed", flush=True)
os._exit(0)
