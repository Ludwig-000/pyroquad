
from typing import Callable, TypeVar, Iterable
from pyroquad import *

T = TypeVar("T")
R = TypeVar("R")

from collections.abc import Sized

def loading_screen(
    func: Callable[[T], R],
    args_list: Iterable[T],
    message: str = "Loading: "
) -> list[R]:
    """
    Example:
    ```
    >>> image_paths= ["first.png", "second.png", "third.png", "fourth.png", "fith.png"]
    >>> images = loading_screen( Image, image_paths )
    ```
    """
    results: list[R] = []
    
    ds = screen_width() / 2200

    draw_text(message,902*ds, 502*ds, Color.ORANGE, font_size= int(70*ds))
    draw_text(message,900*ds, 500*ds, Color.WHITE, font_size= int(70*ds))
    next_frame(None)

    if isinstance(args_list, Sized):
        total = len(args_list)
        for i, arg in enumerate(args_list, 1):
            percent = int((i / total) * 100)

            draw_text(message,902*ds, 502*ds, Color.ORANGE, font_size= int(70*ds))
            draw_text(message,900*ds, 500*ds, Color.WHITE, font_size= int(70*ds))
            draw_text(f"{percent}%",900*ds, 600*ds, Color.WHITE, font_size= int(70*ds))
            next_frame(None)
            results.append(func(arg))
    else:
        for i, arg in enumerate(args_list, 1):
            draw_text(message,902*ds, 502*ds, Color.ORANGE, font_size= int(70*ds))
            draw_text(message,900*ds, 500*ds, Color.WHITE, font_size= int(70*ds))
            draw_text(f"{i}",900*ds, 600*ds, Color.WHITE, font_size= int(70*ds))
            next_frame(None)
            results.append(func(arg))

    return results