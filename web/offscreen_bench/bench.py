# Frame-time benchmark for the offscreen frame. Run through run.html:
#   ?pkg=offscreen&script=bench.py&offscreen=1&scenario=2d&frames=1000&name=...
# Vsync is off (swap_interval=0) so the loop is not pinned to the display, and
# by default gl.finish() closes every iteration so GPU work is inside the timing.
import json
import os
import time

import js

P = js.__PQB.to_py()
NAME = P.get("name", "bench")
SCENARIO = P.get("scenario", "2d")
FRAMES = int(P.get("frames", "1000"))
WARMUP = int(P.get("warmup", "60"))
FINISH = P.get("finish", "1") == "1"
# Chrome's gl.finish() does not wait for the GPU process; a 1-pixel readPixels
# does, so `sync=readpixels` is what puts GPU time inside the measurement.
SYNC = P.get("sync", "finish")
if P.get("webgl2", "0") == "1":
    os.environ["PYROQUAD_WEB_WEBGL2"] = "1"
if P.get("offscreen", "0") == "1":
    os.environ["PYROQUAD_WEB_OFFSCREEN"] = "1"
    os.environ["PYROQUAD_WEB_OFFSCREEN_MSAA"] = P.get("msaa", "4")

from pyroquad import *  # noqa: E402

activate_engine(Config(high_dpi=True, swap_interval=0))
canvas = js.document.getElementById("glcanvas")
gl = canvas.getContext("webgl2") or canvas.getContext("webgl")  # whichever the engine created
pixel = js.Uint8Array.new(4)


def scene_2d(i):
    clear_background(Color(0.1, 0.1, 0.12, 1.0))
    w, h = 900, 560
    for k in range(600):
        x = (k * 37 + i * 3) % w
        y = (k * 53) % h
        draw_rectangle(x, y, 18, 12, Color((k % 7) / 7, 0.5, (k % 5) / 5, 1.0))
    for k in range(300):
        draw_circle((k * 61 + i) % w, (k * 29) % h, 7, Color(0.9, (k % 9) / 9, 0.3, 0.8))
    for k in range(100):
        draw_line((k * 11) % w, 0, (k * 17 + i) % w, h, 1.5, Color.YELLOW)
    for k in range(20):
        draw_text(f"text {k} frame {i}", 10 + (k % 4) * 220, 20 + (k // 4) * 24, Color.WHITE, 20)


cam3 = Camera3D(position=Vec3(12, 9, 14), target=Vec3(0, 0, 0))


def scene_3d(i):
    clear_background(Color(0.05, 0.06, 0.08, 1.0))
    cam3.set_camera()
    draw_grid(30, 1.0, Color.RED, Color.GRAY)
    for k in range(200):
        x, z = (k % 20) - 10, (k // 20) - 5
        draw_cube(Vec3(x, ((k + i) % 13) * 0.1, z), Vec3(0.6, 0.6, 0.6), Color((k % 7) / 7, 0.6, 0.9, 1.0), None)
    for k in range(50):
        draw_cube_wires(Vec3((k % 10) - 5, 2, (k // 10) - 2), Vec3(0.8, 0.8, 0.8), Color.GREEN)
    set_default_camera()
    draw_text(f"3d frame {i}", 10, 30, Color.WHITE, 24)


cam_a = Camera2D.from_display_rect(0, 0, 900, 560)
cam_b = Camera2D.from_display_rect(-100, -50, 1000, 700)


def scene_cams(i):
    clear_background(Color(0.1, 0.1, 0.12, 1.0))
    for k in range(50):
        (cam_a if k % 2 else cam_b).set_camera()
        for j in range(10):
            draw_rectangle((k * 17 + j * 9 + i) % 880, (k * 11) % 540, 16, 10, Color((k % 5) / 5, 0.7, 0.4, 1.0))
        set_default_camera()
        draw_circle((k * 19) % 900, (k * 23 + i) % 560, 5, Color.WHITE)


SCENES = {"2d": scene_2d, "3d": scene_3d, "cams": scene_cams}
scene = SCENES[SCENARIO]

total, draw_t, frame_t, finish_t = [], [], [], []
for i in range(WARMUP + FRAMES):
    t0 = time.perf_counter()
    scene(i)
    t1 = time.perf_counter()
    next_frame()
    t2 = time.perf_counter()
    if SYNC == "readpixels":
        gl.readPixels(0, 0, 1, 1, gl.RGBA, gl.UNSIGNED_BYTE, pixel)
    elif FINISH:
        gl.finish()
    t3 = time.perf_counter()
    if i >= WARMUP:
        total.append(t3 - t0)
        draw_t.append(t1 - t0)
        frame_t.append(t2 - t1)
        finish_t.append(t3 - t2)


def stats(xs):
    s = sorted(xs)
    pick = lambda q: s[min(len(s) - 1, int(q * len(s)))] * 1000  # noqa: E731
    return {"median_ms": round(pick(0.5), 3), "p95_ms": round(pick(0.95), 3),
            "mean_ms": round(sum(s) / len(s) * 1000, 3)}


js.__pqbPost(NAME + ".json", json.dumps({
    "name": NAME, "params": P, "screen": [screen_width(), screen_height()], "dpi": screen_dpi_scale(),
    "frames": FRAMES, "total": stats(total), "draw_phase": stats(draw_t),
    "next_frame": stats(frame_t), "finish": stats(finish_t),
}))
