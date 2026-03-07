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

t = Texture2D(Image.empty())
rec =  Rectangle(Vec2.ZERO,0, Vec2(screen_width(),screen_height()),Color.WHITE)
rec.texture = t
get_keys_down()
assert t == t
assert not t == None

for i in range(10):
    set_default_camera()
    rec.draw()
    next_frame()





file=  examples.loading_screen(download_file,["https://raw.githubusercontent.com/Ludwig-000/pyroquad/main/tests/sound_example.mp3"])[0]
f =  file.to_Sound()

f.play_sound_once()



threeDCAM = examples.PlayerCamera(Vec3.splat(2))
threeDCAM.yaw = -140
threeDCAM.pitch = -30

c = Cube(Vec3.ZERO,Vec3.splat(50),Vec3.ONE,Color.GREEN, None,ColliderOptions.NONE)

cyl = Cylinder(Vec3(2,0,0),Vec3.splat(20),Vec3.ONE,Color.YELLOW, None,ColliderOptions.DYNAMIC(0))


for i in range(100):
    clear_background(Color.AZURE)
    if i == 30:
        c.set_collider(ColliderOptions.DYNAMIC(gravity_scale=1))
    try:
        c.rot += get_delta_time()
    except:
        ...
    threeDCAM.update()
    draw_all_objects()
    
    next_frame(get_delta_time())

file = examples.loading_screen(download_file, ["https://raw.githubusercontent.com/Ludwig-000/pyroquad/main/tests/example_image.jpg",
                                               "https://raw.githubusercontent.com/Ludwig-000/pyroquad/main/tests/example_font.ttf"])
texture=  file[0].to_2DTexture()

if cyl.physics:
    cyl.physics.apply_impulse(Vec3(1,1,1))
else:
    assert False, "physics should be Some"
for i in range(30):
    threeDCAM.update()
    draw_skybox(texture,Color.WHITE)
    draw_all_objects()
    next_frame(get_delta_time())


assert not Color.WHITE == Color.WHEAT



set_default_camera()


recs: list[Rectangle] = []
for w in range(100):
    for h in range(100):
        recs.append(Rectangle(Vec2(w*5,h*5),0,Vec2.splat(4),Color.AMETHYST))


for i in range(30):
    colls= recs[0].collides_with_list(recs)
    assert colls.__len__() == 1
    for rec in recs:
        rec.draw()

    draw_text("Testing..", 300,300,Color.WHITE,font_scale=100, font=file[1].to_font())
    next_frame()

