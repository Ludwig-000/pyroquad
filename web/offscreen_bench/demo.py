# Correctness demos for the offscreen frame. Captures one frame as PNG and
# writes a JSON of whatever the demo measured. Run through run.html:
#   ?pkg=offscreen&script=demo.py&offscreen=1&demo=shapes&name=shapes_rt
import json
import os

import js

P = js.__PQB.to_py()
NAME = P.get("name", "demo")
DEMO = P.get("demo", "shapes")
CAPTURE_AT = int(P.get("capture_at", "30"))
if P.get("webgl2", "0") == "1":
    os.environ["PYROQUAD_WEB_WEBGL2"] = "1"
if P.get("offscreen", "0") == "1":
    os.environ["PYROQUAD_WEB_OFFSCREEN"] = "1"
    os.environ["PYROQUAD_WEB_OFFSCREEN_MSAA"] = P.get("msaa", "4")

from pyroquad import *  # noqa: E402

activate_engine(Config(high_dpi=True))
js.__pqbHook()

report = {"name": NAME, "demo": DEMO, "screen": [screen_width(), screen_height()], "dpi": screen_dpi_scale()}


def shapes(frame):
    clear_background(Color(0.12, 0.13, 0.16, 1.0))
    for i in range(12):
        draw_rectangle(20 + i * 45, 20, 35, 60, Color(i / 12, 0.4, 1 - i / 12, 1.0))
    draw_circle(120, 200, 60, Color.RED)
    draw_circle(220, 200, 40, Color(0.2, 0.9, 0.3, 0.6))
    draw_line(20, 300, 600, 340, 3.0, Color.YELLOW)
    draw_triangle(Vec2(400, 150), Vec2(560, 260), Vec2(380, 280), Color.ORANGE)
    draw_poly(680, 200, 7, 60, 15.0, Color.SKYBLUE)
    for i, size in enumerate((12, 20, 32, 48)):
        draw_text(f"pyroquad {size}px gyq", 20, 400 + i * 50, Color.WHITE, size)
    draw_rectangle(0, 0, 4, 4, Color.MAGENTA)  # top-left marker: y-flip check


cam_a = Camera2D.from_display_rect(0, 0, 400, 300)
cam_b = Camera2D.from_display_rect(0, 0, 1600, 1200)


def camfont(frame):
    clear_background(Color(0.1, 0.1, 0.12, 1.0))
    cam_a.set_camera()
    a = camera_font_scale(20)
    draw_text("cam A (zoomed in)", 10, 40, Color.WHITE, a[0], None, a[1], a[2])
    cam_b.set_camera()
    b = camera_font_scale(20)
    draw_text("cam B (zoomed out)", 10, 400, Color.YELLOW, b[0], None, b[1], b[2])
    set_default_camera()
    d = camera_font_scale(20)
    draw_text(f"A={a}  B={b}  default={d}", 10, screen_height() - 20, Color.GREEN, 16)
    report["camera_font_scale"] = {"A": list(a), "B": list(b), "default": list(d)}


cam3 = Camera3D(position=Vec3(6, 5, 8), target=Vec3(0, 0.5, 0))


def three_d(frame):
    clear_background(Color(0.08, 0.09, 0.12, 1.0))
    cam3.set_camera()
    draw_grid(20, 1.0, Color.RED, Color.GRAY)
    draw_cube(Vec3(0, 0.5, 0), Vec3(2, 1, 2), Color.SKYBLUE, None)
    draw_cube(Vec3(0.8, 1, 0.8), Vec3(1, 2, 1), Color.ORANGE, None)  # intersects: depth test
    draw_cube_wires(Vec3(-2, 0.5, 2), Vec3(1, 1, 1), Color.GREEN)
    draw_plane(Vec3(-3, 0.01, -3), Vec2(2, 2), Color.PURPLE, None)
    draw_line_3d(Vec3(0, 0, 0), Vec3(0, 4, 0), Color.YELLOW)
    set_default_camera()
    draw_text("3D + 2D overlay", 10, 30, Color.WHITE, 24)


sky = None


def skybox(frame):
    global sky
    if sky is None:
        sky = download_file("../../cool_sky.jpg").to_Texture2D()
    clear_background(Color.BLACK)
    cam3.set_camera()
    draw_skybox(sky)
    draw_cube(Vec3(0, 0.5, 0), Vec3(2, 1, 2), Color.SKYBLUE, None)
    draw_grid(10, 1.0, Color.RED, Color.GRAY)
    set_default_camera()
    draw_text("skybox", 10, 30, Color.WHITE, 24)


user_rt = None


def usert(frame):
    global user_rt
    if user_rt is None:
        user_rt = render_target(256, 256)
    clear_background(Color(0.1, 0.1, 0.1, 1.0))
    # A camera with its own target: none of this may reach the screen.
    Camera2D(zoom=Vec2(2 / 256, 2 / 256), target=Vec2(128, 128), render_target=user_rt).set_camera()
    clear_background(Color.RED)
    draw_rectangle(0, 0, 4000, 4000, Color.RED)
    # A screen camera with a viewport (lower-left origin, physical pixels).
    Camera2D(zoom=Vec2(2 / 200, -2 / 200), target=Vec2(100, 100), viewport=(40, 40, 300, 200)).set_camera()
    draw_rectangle(0, 0, 200, 200, Color.DARKBLUE)
    draw_circle(100, 100, 80, Color.GOLD)
    set_default_camera()
    draw_text("user RT must stay off-screen; gold circle in a viewport", 10, 30, Color.WHITE, 20)
    draw_rectangle(screen_width() - 60, 10, 50, 50, Color.GREEN)


def screendata(frame):
    shapes(frame)
    if frame == CAPTURE_AT:
        img = get_screen_data()
        w, h, b = img.width, img.height, bytes(img.bytes)

        def px(x, y):  # y from the top, image rows are bottom-up
            i = ((h - 1 - y) * w + x) * 4
            return list(b[i:i + 4])

        dpi = screen_dpi_scale()
        report["screen_data"] = {
            "size": [w, h],
            "nonzero_bytes": sum(1 for v in b[::97] if v),
            "red_circle_center": px(int(120 * dpi), int(200 * dpi)),
            "top_left_marker": px(1, 1),
        }


DEMOS = {"shapes": shapes, "camfont": camfont, "3d": three_d, "skybox": skybox,
         "usert": usert, "screendata": screendata}

for frame in range(CAPTURE_AT + 5):
    DEMOS[DEMO](frame)
    if frame == CAPTURE_AT:
        js.__pqbCapture = NAME + ".png"
    next_frame()

js.__pqbPost(NAME + ".json", json.dumps(report))
