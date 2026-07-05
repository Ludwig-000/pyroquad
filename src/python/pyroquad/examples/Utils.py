import time

def limit_fps(fps_max: int):
    """
    Limits the frame rate to a maximum target.

    Example:
        >>> while True:
        ...     # ... handle inputs and game updates ...
        ...     clear_background(Color.WHITE)
        ...     # ... draw your items ...
        ...     next_frame()
        ...     limit_fps(60)  # Call right after next_frame()
    """
    if not hasattr(limit_fps, "target_time"):
        limit_fps.target_time = time.perf_counter()  # type: ignore
        return

    target_dt = 1.0 / fps_max
    limit_fps.target_time += target_dt  # type: ignore

    while True:
        now = time.perf_counter()
        remaining = limit_fps.target_time - now  # type: ignore
        
        if remaining <= 0.0015: 
            break
        time.sleep(remaining - 0.0015)

    while time.perf_counter() < limit_fps.target_time:  # type: ignore
        time.sleep(0)