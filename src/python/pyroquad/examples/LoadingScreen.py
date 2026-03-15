from typing import Callable, TypeVar, Iterable
from pyroquad import *
from collections.abc import Sized
import time

T = TypeVar("T")
R = TypeVar("R")

def loading_screen(
    func: Callable[[T], R],
    args_list: Iterable[T],
    message: str = "Loading: "
) -> list[R]:

    results: list[R] = []

    
    zoom = 0.0009115
    camera = Camera2D(rotation=0,zoom=Vec2(zoom, zoom *16/9),target=Vec2.ZERO,offset=Vec2(-1,1))

    factor = 2194.0 / 2200

    def draw(percent_text: str):
        draw_text(message, 902*factor, 502*factor, Color.ORANGE, font_size=int(70*factor))
        draw_text(message, 900*factor, 500*factor, Color.WHITE, font_size=int(70*factor))
        draw_text(percent_text, 900*factor, 600*factor, Color.WHITE, font_size=int(70*factor))
        next_frame(None)

    Camera2D.set_camera(camera)
    draw("0%")

    min_interval = 0.05  # seconds between frames (~10 fps max for loading UI)
    last_draw = time.perf_counter()

    if isinstance(args_list, Sized):
        total = len(args_list)
        last_percent = -1

        for i, arg in enumerate(args_list, 1):
            results.append(func(arg))

            percent = int((i / total) * 100)
            now = time.perf_counter()

            if percent != last_percent and (now - last_draw) >= min_interval:
                Camera2D.set_camera(camera)
                draw(f"{percent}%")
                last_draw = now
                last_percent = percent
    else:
        last_shown = 0

        for i, arg in enumerate(args_list, 1):
            results.append(func(arg))

            now = time.perf_counter()
            if (now - last_draw) >= min_interval:
                draw(str(i))
                last_draw = now
                last_shown = i

    return results