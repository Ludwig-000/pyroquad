from pyroquad import *




activate_engine()

cube1  = Cube(position=Vec3.splat(100))
cube2  = Cube(position=Vec3.splat(200))

assert cube1.__str__() == "Cube at (100.00, 100.00, 100.00)"


assert cube1 == cube1
assert not (cube1 == cube2)
assert not (cube1 == 5)
assert not (cube1 == None)

set = {cube1: "Hello first mesh", cube2: "Hello 2nd mesh"}

assert set.get(cube1) == "Hello first mesh"
assert set.get(cube2) == "Hello 2nd mesh"


c= [Cube(collider_type=ColliderOptions.DYNAMIC())]
phys = c[0].physics
assert not phys == None
c.pop()

try:
    phys.add_force(Vec3.ZERO)
    assert False, "should error"
except:
    ...



draw_rectangle(100,100,100,100,Color.RED)
draw_arc(100,100,10,5,0,100,5,Color.GREEN)
draw_circle(100,100,100,Color.WHITE)


next_frame(None)

cam  = examples.PlayerCamera(Vec3.ZERO)

for i in range(10):
    clear_background(Color.ACID_GREEN)
    cam.update()
    draw_all_objects()

    next_frame()

t = Texture2D(Image("docs/3d_screenshot.png"))
rec =  Rectangle(Vec2.ZERO,0, Vec2(screen_width(),screen_height()),Color.WHITE)
rec.texture = t
get_keys_down()
assert t == t
assert not t == None

for i in range(10):
    set_default_camera()
    rec.draw()
    next_frame()