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

    ds = screen_width() / 2200

    def draw(percent_text: str):
        draw_text(message, 902*ds, 502*ds, Color.ORANGE, font_size=int(70*ds))
        draw_text(message, 900*ds, 500*ds, Color.WHITE, font_size=int(70*ds))
        draw_text(percent_text, 900*ds, 600*ds, Color.WHITE, font_size=int(70*ds))
        next_frame(None)

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