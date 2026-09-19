from pyroquad import *


activate_engine()

while True:
    draw_rectangle(100,100,100,100, Color.RED)
    next_frame()
    examples.limit_fps(60)