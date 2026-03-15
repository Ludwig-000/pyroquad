from pyroquad import *
import time

def limit_fps(fps_max: int):
    target_dt = 1.0 / fps_max
    actual_dt = get_delta_time()
    if actual_dt < target_dt:
        time.sleep(target_dt - actual_dt)