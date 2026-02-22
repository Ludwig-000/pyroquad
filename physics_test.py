from pyroquad import *
import random
import math
import time
activate_engine(Config("Physics test",2000,2000,True, None,10,True,True))

cam  =examples.PlayerCamera(Vec3.splat(500))

def cube_gen()-> list[Cube]:
    lis = []
    num_cubes = 2000
    radius = 150
    
    for i in range(num_cubes):
        angle = (i / num_cubes) * 2 * math.pi
        x = math.cos(angle) * radius
        z = math.sin(angle) * radius
        pos = Vec3(x, 0, z)
        
        charitable_color = Color(random.random(), random.random(), random.random())
        
        lis.append(
            Cube(
                pos, 
                Vec3.ZERO, 
                Vec3.splat(10), 
                charitable_color, 
                None,
                ColliderOptions.DYNAMIC(0,0.1,0.1,1)
            )
        )

    def cube_gravity(cube: Cube):
        t = time.time()
        
        center_radius = 50
        center = Vec3(math.cos(t) * center_radius, 0, math.sin(t) * center_radius)
        jitter = random.random()
        if cube.physics:
            cube.physics.apply_impulse(
                ((center - cube.pos)*2) + (jitter*10)
            )
            random_shock  = random.random()
            if random_shock < 0.01:
                factor  = 5000
                cube.physics.apply_impulse(Vec3(random.random()*factor,random.random()*factor,random.random()*factor,))
    for cube in lis:
        cube.tick(cube_gravity)

    return lis

cubes  =cube_gen()

set_cursor_grab(True)
show_mouse(False)




while True:
    if KeyCode.Escape in get_keys_pressed():
        break
    cam.update()
    draw_grid(100,10,Color.YELLOW,Color.CYAN)
    draw_cube(Vec3.ZERO,Vec3.ONE,Color.PURE_BLUE)
    draw_all_objects()
    set_default_camera()
    draw_text(f"{get_fps()}",20,20,40,Color.GREEN)
    next_frame(0)