"""
pyroquad vs pygame - 2D rendering benchmark (Windows).

Usage:
    pip install pyroquad pygame-ce        (classic `pygame` works too)
    python pyroquad_vs_pygame.py
    python pyroquad_vs_pygame.py --seconds 10 --repeat 3
    python pyroquad_vs_pygame.py --scenes rects,sprites --scale 2
    python pyroquad_vs_pygame.py --screenshots shots    (saves one frame per run, to check both draw the same thing)

How it is kept fair:
  * Every (library, scene) run is its own fresh Python process, so no state, caches or
    GC pressure leaks between runs. Runs alternate library order.
  * Same window size, same non-resizable window, vsync off on both
    (pygame: plain display surface, no vsync; pyroquad: swap_interval=0), high-DPI scaling off.
  * Scene data (positions, colours, sizes, strings, sprite pixels) is generated from one
    fixed seed and shared verbatim, so both libraries draw the same picture.
  * The per-frame Python work around the draw calls is the same code on both sides; only the
    draw call itself differs, and each library uses its own idiomatic call for it
    (e.g. pygame `Surface.blit` with a `convert_alpha()`ed surface, pyroquad `draw_texture`).
  * "_batched" scenes use each library's own bulk API (pygame `fblits`/`blits`,
    pyroquad `batch_draw_shapes`).
  * A frame is counted from the start of input/clear to the return of the present call
    (pygame `display.flip()`, pyroquad `next_frame()`, which blocks until the frame is drawn).
    pygame must pump events each frame to keep the window alive; pyroquad does this internally.
  * Warm-up time is discarded before measuring.

Notes on what differs by design:
  * pygame rasterizes on the CPU (SDL software surfaces); pyroquad renders on the GPU
    (OpenGL, via macroquad). That difference is the point of the comparison.
  * pyroquad's draw_circle is a 20-sided polygon; pygame.draw.circle is pixel-exact.
  * Text: pygame re-renders each changing string with `font.render` (the standard way for
    text that changes every frame); pyroquad caches glyphs and draws quads.
"""

from __future__ import annotations

import argparse
import json
import math
import os
import platform
import random
import statistics
import struct
import subprocess
import sys
import time
import zlib

LIBS = ("pygame", "pyroquad")

# name -> (default object count, description)
SCENES: dict[str, tuple[int, str]] = {
    "empty":           (0,     "clear + present only (fixed per-frame cost)"),
    "rects":           (5000,  "filled 24x24 rectangles"),
    "circles":         (2000,  "filled circles, r=10"),
    "lines":           (3000,  "lines, 2px thick"),
    "sprites":         (5000,  "32x32 alpha sprites, one call each"),
    "sprites_batched": (10000, "32x32 alpha sprites, library bulk-draw API"),
    "rotozoom":        (500,   "sprites rotated + scaled every frame"),
    "text":            (200,   "strings that change every frame, size 20"),
    "game":            (2000,  "bouncing sprites with Python physics + HUD rects + text"),
}

SEED = 1234
BG = (20, 20, 30)
SPRITE = 32
SHOT_FRAME = 37


# --------------------------------------------------------------------------------------------
# shared scene data (identical for both libraries)
# --------------------------------------------------------------------------------------------

def sprite_pixels() -> bytes:
    """32x32 RGBA: a soft-edged orange ball with an alpha falloff and an off-centre dark spot
    (so rotation is visible in screenshots)."""
    out = bytearray()
    c = (SPRITE - 1) / 2
    for y in range(SPRITE):
        for x in range(SPRITE):
            d = math.hypot(x - c, y - c) / (SPRITE / 2)
            a = max(0.0, min(1.0, (1.0 - d) * 3.0))
            if math.hypot(x - 22, y - 10) < 4:
                out += bytes((60, 20, 10, int(a * 255)))
            else:
                out += bytes((255, int(140 + 100 * (1 - d)) & 255, 40, int(a * 255)))
    return bytes(out)


def rand_color(rng: random.Random) -> tuple[int, int, int]:
    return (rng.randint(40, 255), rng.randint(40, 255), rng.randint(40, 255))


def scene_data(scene: str, n: int, w: int, h: int) -> dict:
    rng = random.Random(SEED)
    pad = 64  # leaves room for the horizontal wobble
    pts = [(rng.uniform(pad, w - pad - 32), rng.uniform(0, h - 32)) for _ in range(n)]
    d: dict = {"pts": pts}
    if scene in ("rects", "circles", "lines", "game"):
        d["colors"] = [rand_color(rng) for _ in range(n)]
    if scene == "lines":
        d["ends"] = [(x + rng.uniform(-60, 60), y + rng.uniform(-60, 60)) for x, y in pts]
    if scene == "rotozoom":
        d["scales"] = [rng.uniform(0.75, 2.5) for _ in range(n)]
        d["spins"] = [rng.uniform(-3.0, 3.0) for _ in range(n)]  # radians / 100 frames
    if scene == "text":
        cols = 8
        d["pts"] = [(20 + (k % cols) * (w - 40) / cols, 10 + (k // cols) * ((h - 30) / ((n + cols - 1) // cols)))
                    for k in range(n)]
        d["colors"] = [rand_color(rng) for _ in range(n)]
    if scene == "game":
        d["vel"] = [(rng.uniform(-4, 4), rng.uniform(-4, 4)) for _ in range(n)]
        d["hud"] = [(10 + (k % 25) * 50, h - 40 + (k // 25) * 8) for k in range(100)]
    return d


def wobble(i: int) -> float:
    """Moves everything a little each frame so nothing can be cached between frames."""
    return float((i * 2) % 64 - 32)


def step_game(pos: list, vel: list, w: int, h: int) -> None:
    """Plain-Python game logic. Same code for both libraries."""
    maxx = w - SPRITE
    maxy = h - SPRITE
    for k in range(len(pos)):
        x, y = pos[k]
        vx, vy = vel[k]
        x += vx
        y += vy
        if x < 0 or x > maxx:
            vx = -vx
            x = min(max(x, 0), maxx)
        if y < 0 or y > maxy:
            vy = -vy
            y = min(max(y, 0), maxy)
        pos[k] = (x, y)
        vel[k] = (vx, vy)


# --------------------------------------------------------------------------------------------
# pygame worker
# --------------------------------------------------------------------------------------------

def setup_pygame(scene: str, n: int, w: int, h: int):
    os.environ.setdefault("PYGAME_HIDE_SUPPORT_PROMPT", "1")
    import pygame

    pygame.init()
    screen = pygame.display.set_mode((w, h), vsync=0)
    pygame.display.set_caption(f"pygame - {scene}")
    sprite = pygame.image.frombuffer(sprite_pixels(), (SPRITE, SPRITE), "RGBA").convert_alpha()
    font = pygame.font.Font(None, 20)
    d = scene_data(scene, n, w, h)
    pts = d["pts"]
    blit = screen.blit

    if scene == "empty":
        def draw(i):
            pass

    elif scene == "rects":
        # Surface.fill(color, rect) is pygame's fastest solid-rect path (a bit faster than draw.rect).
        rect = screen.fill
        items = list(zip(pts, d["colors"]))
        def draw(i):
            o = wobble(i)
            for (x, y), c in items:
                rect(c, (x + o, y, 24, 24))

    elif scene == "circles":
        circle = pygame.draw.circle
        items = list(zip(pts, d["colors"]))
        def draw(i):
            o = wobble(i)
            for (x, y), c in items:
                circle(screen, c, (x + o, y), 10)

    elif scene == "lines":
        line = pygame.draw.line
        items = list(zip(pts, d["ends"], d["colors"]))
        def draw(i):
            o = wobble(i)
            for (x1, y1), (x2, y2), c in items:
                line(screen, c, (x1 + o, y1), (x2 + o, y2), 2)

    elif scene == "sprites":
        def draw(i):
            o = wobble(i)
            for x, y in pts:
                blit(sprite, (x + o, y))

    elif scene == "sprites_batched":
        seq = [(sprite, (x, y)) for x, y in pts]
        if hasattr(screen, "fblits"):  # pygame-ce
            fblits = screen.fblits
            def draw(i):
                fblits(seq)
        else:  # classic pygame
            blits = screen.blits
            def draw(i):
                blits(seq, doreturn=False)

    elif scene == "rotozoom":
        rotozoom = pygame.transform.rotozoom
        items = list(zip(pts, d["scales"], d["spins"]))
        def draw(i):
            t = i / 100.0
            for (x, y), s, spin in items:
                img = rotozoom(sprite, math.degrees(-spin * t), s)
                blit(img, img.get_rect(center=(x, y)))

    elif scene == "text":
        render = font.render
        items = list(zip(pts, d["colors"]))
        def draw(i):
            for k, ((x, y), c) in enumerate(items):
                blit(render(f"score {i + k}", True, c), (x, y))

    elif scene == "game":
        rect = screen.fill
        pos = list(pts)
        vel = list(d["vel"])
        hud = list(zip(d["hud"], d["colors"]))
        render = font.render
        def draw(i):
            step_game(pos, vel, w, h)
            for p in pos:
                blit(sprite, p)
            for (x, y), c in hud:
                rect(c, (x, y, 40, 6))
            for k in range(10):
                blit(render(f"entities {len(pos)}  frame {i}  line {k}", True, (255, 255, 255)), (10, 10 + k * 22))
    else:
        raise ValueError(scene)

    pump = pygame.event.pump
    fill = screen.fill
    flip = pygame.display.flip

    def frame(i):
        pump()
        fill(BG)
        draw(i)
        flip()

    def capture(i, path):
        pump()
        fill(BG)
        draw(i)
        write_png(path, w, h, pygame.image.tobytes(screen, "RGBA"))
        flip()

    info = {"lib_version": f"pygame{'-ce' if getattr(pygame, 'IS_CE', False) else ''} {pygame.version.ver}",
            "backend": f"SDL {'.'.join(map(str, pygame.get_sdl_version()))}, software surface",
            "framebuffer": list(screen.get_size())}
    return frame, capture, info


# --------------------------------------------------------------------------------------------
# pyroquad worker
# --------------------------------------------------------------------------------------------

def setup_pyroquad(scene: str, n: int, w: int, h: int):
    import pyroquad as pq

    pq.activate_engine(pq.Config(
        window_title=f"pyroquad - {scene}",
        window_width=w,
        window_height=h,
        swap_interval=0,
        window_resizable=False,
        high_dpi=False,
    ))

    def col(c):
        return pq.Color(c[0] / 255, c[1] / 255, c[2] / 255, 1.0)

    white = pq.Color(1.0, 1.0, 1.0, 1.0)
    bg = col(BG)
    tex = pq.Texture2D.from_rgba8(SPRITE, SPRITE, list(sprite_pixels()))
    d = scene_data(scene, n, w, h)
    pts = d["pts"]
    draw_texture = pq.draw_texture

    if scene == "empty":
        def draw(i):
            pass

    elif scene == "rects":
        rect = pq.draw_rectangle
        items = [(p, col(c)) for p, c in zip(pts, d["colors"])]
        def draw(i):
            o = wobble(i)
            for (x, y), c in items:
                rect(x + o, y, 24, 24, c)

    elif scene == "circles":
        circle = pq.draw_circle
        items = [(p, col(c)) for p, c in zip(pts, d["colors"])]
        def draw(i):
            o = wobble(i)
            for (x, y), c in items:
                circle(x + o, y, 10, c)

    elif scene == "lines":
        line = pq.draw_line
        items = [(p, e, col(c)) for p, e, c in zip(pts, d["ends"], d["colors"])]
        def draw(i):
            o = wobble(i)
            for (x1, y1), (x2, y2), c in items:
                line(x1 + o, y1, x2 + o, y2, 2, c)

    elif scene == "sprites":
        def draw(i):
            o = wobble(i)
            for x, y in pts:
                draw_texture(tex, x + o, y, white)

    elif scene == "sprites_batched":
        # Rectangle position is its centre, scale is its size in pixels.
        half = SPRITE / 2
        shapes = [pq.Rectangle(position=pq.Vec2(x + half, y + half), scale=pq.Vec2(SPRITE, SPRITE),
                               color=white, texture=tex) for x, y in pts]
        batch = pq.batch_draw_shapes
        def draw(i):
            batch(shapes)

    elif scene == "rotozoom":
        items = [(pq.Rectangle(position=pq.Vec2(x, y), scale=pq.Vec2(SPRITE * s, SPRITE * s),
                               color=white, texture=tex), spin)
                 for (x, y), s, spin in zip(pts, d["scales"], d["spins"])]
        def draw(i):
            t = i / 100.0
            for r, spin in items:
                r.rotation = spin * t
                r.draw()

    elif scene == "text":
        text = pq.draw_text
        items = [(p, col(c)) for p, c in zip(pts, d["colors"])]
        def draw(i):
            for k, ((x, y), c) in enumerate(items):
                # pyroquad's y is the baseline, pygame's is the top; +14 lines them up.
                text(f"score {i + k}", x, y + 14, c, 20)

    elif scene == "game":
        rect = pq.draw_rectangle
        text = pq.draw_text
        pos = list(pts)
        vel = list(d["vel"])
        hud = [(p, col(c)) for p, c in zip(d["hud"], d["colors"])]
        def draw(i):
            step_game(pos, vel, w, h)
            for x, y in pos:
                draw_texture(tex, x, y, white)
            for (x, y), c in hud:
                rect(x, y, 40, 6, c)
            for k in range(10):
                text(f"entities {len(pos)}  frame {i}  line {k}", 10, 24 + k * 22, white, 20)
    else:
        raise ValueError(scene)

    clear = pq.clear_background
    next_frame = pq.next_frame

    def frame(i):
        clear(bg)
        draw(i)
        next_frame()

    def capture(i, path):
        clear(bg)
        draw(i)
        img = pq.get_screen_data()
        iw, ih, raw = img.width, img.height, img.bytes
        stride = iw * 4
        flipped = b"".join(raw[r * stride:(r + 1) * stride] for r in range(ih - 1, -1, -1))  # GL is bottom-up
        write_png(path, iw, ih, flipped)
        next_frame()

    try:
        from importlib.metadata import version
        ver = f"pyroquad {version('pyroquad')}"
    except Exception:
        ver = "pyroquad (unknown version)"
    info = {"lib_version": ver, "backend": "macroquad, OpenGL",
            "framebuffer": [int(pq.screen_width()), int(pq.screen_height())]}
    return frame, capture, info


# --------------------------------------------------------------------------------------------
# measurement
# --------------------------------------------------------------------------------------------

def write_png(path: str, w: int, h: int, rgba: bytes) -> None:
    def chunk(tag: bytes, data: bytes) -> bytes:
        return struct.pack(">I", len(data)) + tag + data + struct.pack(">I", zlib.crc32(tag + data) & 0xFFFFFFFF)
    # Saved as opaque RGB: a GL framebuffer's alpha channel is not meaningful once blending ran.
    rgb = bytearray(w * h * 3)
    for ch in range(3):
        rgb[ch::3] = rgba[ch::4]
    stride = w * 3
    raw = b"".join(b"\x00" + rgb[y * stride:(y + 1) * stride] for y in range(h))
    with open(path, "wb") as f:
        f.write(b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", w, h, 8, 2, 0, 0, 0))
                + chunk(b"IDAT", zlib.compress(raw, 6)) + chunk(b"IEND", b""))


def worker(lib: str, scene: str, n: int, w: int, h: int, warmup: float, seconds: float, shot: str | None) -> None:
    setup = setup_pygame if lib == "pygame" else setup_pyroquad
    frame, capture, info = setup(scene, n, w, h)

    if shot:
        # Captured before warm-up at a fixed frame index, so both libraries' images show the same state.
        capture(SHOT_FRAME, shot)

    i = 0
    t_end = time.perf_counter() + warmup
    while time.perf_counter() < t_end:
        frame(i)
        i += 1

    times = []
    append = times.append
    cpu0 = time.process_time()
    start = prev = time.perf_counter()
    end = start + seconds
    while prev < end:
        frame(i)
        i += 1
        now = time.perf_counter()
        append(now - prev)
        prev = now
    elapsed = prev - start
    cpu = time.process_time() - cpu0

    times.sort()
    res = {
        "lib": lib, "scene": scene, "n": n, **info,
        "frames": len(times),
        "fps": len(times) / elapsed,
        "median_ms": statistics.median(times) * 1000,
        "p99_ms": times[min(len(times) - 1, int(len(times) * 0.99))] * 1000,
        "cpu_pct": 100 * cpu / elapsed,  # all threads of the process; 100% = one full core
    }
    print("RESULT " + json.dumps(res), flush=True)
    sys.stdout.flush()
    os._exit(0)  # don't wait on engine threads / SDL teardown


def run_one(lib, scene, n, args, shot) -> dict | None:
    cmd = [sys.executable, "-u", os.path.abspath(__file__), "--worker", lib, scene, str(n),
           "--size", args.size, "--warmup", str(args.warmup), "--seconds", str(args.seconds)]
    if shot:
        cmd += ["--shot", shot]
    env = dict(os.environ, PYGAME_HIDE_SUPPORT_PROMPT="1")
    try:
        p = subprocess.run(cmd, capture_output=True, text=True, env=env,
                           timeout=args.warmup + args.seconds + 120)
    except subprocess.TimeoutExpired:
        print(f"    {lib}/{scene}: timed out")
        return None
    for line in p.stdout.splitlines():
        if line.startswith("RESULT "):
            return json.loads(line[7:])
    print(f"    {lib}/{scene}: failed (exit {p.returncode})\n{p.stdout[-2000:]}{p.stderr[-2000:]}")
    return None


def system_info() -> list[str]:
    lines = [f"OS:     {platform.platform()}",
             f"Python: {platform.python_version()} ({platform.python_implementation()})",
             f"CPU:    {platform.processor() or 'unknown'} ({os.cpu_count()} logical cores)"]
    if sys.platform == "win32":
        try:
            gpu = subprocess.run(["powershell", "-NoProfile", "-Command",
                                  "(Get-CimInstance Win32_VideoController).Name"],
                                 capture_output=True, text=True, timeout=20).stdout.strip()
            lines.append(f"GPU:    {', '.join(gpu.splitlines()) or 'unknown'}")
        except Exception:
            pass
    return lines


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--scenes", default=",".join(SCENES), help=f"comma list, from: {', '.join(SCENES)}")
    ap.add_argument("--scale", type=float, default=1.0, help="multiply every scene's object count")
    ap.add_argument("--size", default="1280x720", help="window size WxH")
    ap.add_argument("--warmup", type=float, default=1.5, help="seconds discarded before measuring")
    ap.add_argument("--seconds", type=float, default=5.0, help="seconds measured per run")
    ap.add_argument("--repeat", type=int, default=1, help="runs per library per scene; the median is reported")
    ap.add_argument("--screenshots", metavar="DIR", help="save one frame per scene and library as PNG here")
    ap.add_argument("--json", metavar="FILE", help="also write all raw results to this file")
    ap.add_argument("--worker", nargs=3, metavar=("LIB", "SCENE", "N"), help=argparse.SUPPRESS)
    ap.add_argument("--shot", help=argparse.SUPPRESS)
    args = ap.parse_args()
    w, h = (int(v) for v in args.size.lower().split("x"))

    if args.worker:
        lib, scene, n = args.worker
        worker(lib, scene, int(n), w, h, args.warmup, args.seconds, args.shot)
        return

    scenes = [s.strip() for s in args.scenes.split(",") if s.strip()]
    for s in scenes:
        if s not in SCENES:
            ap.error(f"unknown scene {s!r}")
    if args.screenshots:
        os.makedirs(args.screenshots, exist_ok=True)

    print("pyroquad vs pygame benchmark")
    for line in system_info():
        print("  " + line)
    print(f"  window {w}x{h}, vsync off, warm-up {args.warmup}s, measure {args.seconds}s x {args.repeat} run(s)\n")

    results: dict[str, dict[str, list[dict]]] = {}
    versions: dict[str, dict] = {}
    for scene in scenes:
        n = int(SCENES[scene][0] * args.scale)
        results[scene] = {lib: [] for lib in LIBS}
        for r in range(args.repeat):
            order = LIBS if r % 2 == 0 else LIBS[::-1]
            for lib in order:
                shot = (os.path.join(args.screenshots, f"{scene}_{lib}.png")
                        if args.screenshots and r == 0 else None)
                print(f"  running {scene:<16} {lib:<9} run {r + 1}/{args.repeat} ...", end="", flush=True)
                res = run_one(lib, scene, n, args, shot)
                if res:
                    results[scene][lib].append(res)
                    versions[lib] = res
                    print(f" {res['fps']:8.1f} fps")
                else:
                    print(" failed")

    def med(rs, key):
        return statistics.median(r[key] for r in rs) if rs else float("nan")

    print()
    for lib in LIBS:
        if lib in versions:
            v = versions[lib]
            print(f"  {v['lib_version']:<22} {v['backend']}, framebuffer {v['framebuffer'][0]}x{v['framebuffer'][1]}")
    print()
    hdr = (f"{'scene':<16}{'count':>7} | {'pygame fps':>11}{'ms med':>8}{'ms p99':>8}{'cpu%':>6} | "
           f"{'pyroquad fps':>13}{'ms med':>8}{'ms p99':>8}{'cpu%':>6} | {'speedup':>8}")
    print(hdr)
    print("-" * len(hdr))
    speedups = []
    for scene in scenes:
        pg, pq = results[scene]["pygame"], results[scene]["pyroquad"]
        n = int(SCENES[scene][0] * args.scale)
        pg_fps, pq_fps = med(pg, "fps"), med(pq, "fps")
        sp = pq_fps / pg_fps if pg and pq else float("nan")
        if pg and pq:
            speedups.append(sp)
        print(f"{scene:<16}{n:>7} | {pg_fps:>11.1f}{med(pg, 'median_ms'):>8.2f}{med(pg, 'p99_ms'):>8.2f}"
              f"{med(pg, 'cpu_pct'):>6.0f} | {pq_fps:>13.1f}{med(pq, 'median_ms'):>8.2f}{med(pq, 'p99_ms'):>8.2f}"
              f"{med(pq, 'cpu_pct'):>6.0f} | {sp:>7.2f}x")
    print("-" * len(hdr))
    if speedups:
        geo = math.exp(sum(math.log(s) for s in speedups) / len(speedups))
        print(f"geometric mean speedup (pyroquad fps / pygame fps): {geo:.2f}x over {len(speedups)} scenes")
    print("cpu% counts every thread of the process (100% = one core); speedup > 1 means pyroquad is faster.")

    if args.json:
        with open(args.json, "w") as f:
            json.dump({"args": vars(args), "system": system_info(), "results": results}, f, indent=2)
        print(f"raw results written to {args.json}")


if __name__ == "__main__":
    main()
