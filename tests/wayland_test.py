"""Opens a native Wayland window and uses it: draws, reads pixels back, resizes, queries input.

Run under a Wayland compositor with no X server (DISPLAY unset), e.g. the sway
started by .github/workflows/wayland_test.yml. The engine is forced to
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


def screen_size():
    return int(p.screen_width()), int(p.screen_height())


# Tiling compositors (sway) resize the window right after it maps.
# Keep presenting frames until the size has been stable for a while.
last, stable = screen_size(), 0
for _ in range(120):
    p.clear_background(p.Color(0, 0, 1, 1))
    p.next_frame()
    now = screen_size()
    stable = stable + 1 if now == last else 0
    last = now
    if stable >= 10:
        break
SW, SH = last
print("settled window size:", SW, "x", SH, flush=True)


def pixel(img, x, y):
    """RGBA at (x, y) from the top-left. get_screen_data() rows come bottom-up."""
    sy = img.height - 1 - y
    i = (sy * img.width + x) * 4
    return tuple(img_bytes[i:i + 4])


def close(a, b, tol=8):
    return all(abs(x - y) <= tol for x, y in zip(a[:3], b[:3]))


def describe(img):
    """Where non-black pixels ended up - tells a size mismatch apart from an empty frame."""
    w, h = img.width, img.height
    xs, ys, count = [], [], 0
    for y in range(0, h, 4):
        row = (h - 1 - y) * w * 4
        for x in range(0, w, 4):
            if any(img_bytes[row + x * 4: row + x * 4 + 3]):
                count += 1
                xs.append(x)
                ys.append(y)
    total = ((w + 3) // 4) * ((h + 3) // 4)
    print(f"non-black samples: {count}/{total}", flush=True)
    if count:
        print(f"non-black bbox (top-left origin): x {min(xs)}..{max(xs)}, y {min(ys)}..{max(ys)}", flush=True)


def save_ppm(img, path):
    """Top-down RGB dump, viewable with most image viewers."""
    w, h = img.width, img.height
    with open(path, "wb") as f:
        f.write(f"P6 {w} {h} 255\n".encode())
        for y in range(h):
            row = (h - 1 - y) * w * 4
            f.write(b"".join(img_bytes[row + x * 4: row + x * 4 + 3] for x in range(w)))


# Draw a known picture and read it back from the GPU.
for frame in range(3):
    p.clear_background(p.Color(0, 0, 1, 1))
    p.draw_rectangle(0, 0, 40, 40, p.Color(1, 0, 0, 1))
    p.draw_circle(SW / 2, SH / 2, 30, p.Color(0, 1, 0, 1))
    if frame == 2:
        img = p.get_screen_data()
        img_bytes = bytes(img.bytes)
    p.next_frame()

print("framebuffer:", img.width, "x", img.height, flush=True)
describe(img)
shot = os.environ.get("WAYLAND_TEST_SCREENSHOT")
if shot:
    save_ppm(img, shot)
    print("screenshot saved to", shot, flush=True)
checks = {
    "top-left red rectangle": (pixel(img, 10, 10), (255, 0, 0)),
    "green circle centre": (pixel(img, SW // 2, SH // 2), (0, 255, 0)),
    "blue background": (pixel(img, SW - 10, SH - 10), (0, 0, 255)),
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
