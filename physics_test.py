from pyroquad import *
import random
import math
import time
activate_engine(Config("Physics test",2000,2000,True, None,10,True,True))

cam  = examples.PlayerCamera(position=Vec3.splat(500))

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

cubes  = cube_gen()

set_cursor_grab(True)
show_mouse(False)


hdr1 = Image("tests/HDR_blue_nebulae_2.png")
hdr1.flip_vertical()

hdr = hdr1.to_texture()

hdr2 = hdr1.to_texture()

tex = load_file("cool_sky.jpg").to_Texture2D()
asteroid = Mesh.from_file_data(FileData("./little_forest/Asteroid.glb"), texture=tex, collider_type=ColliderOptions.NONE)


while True:


    if KeyCode.Escape in get_keys_pressed():
        break
    
    cam.update()
    draw_skybox(hdr, Color.WHITE)
    
    draw_grid(100,10,Color.YELLOW,Color.CYAN)
    draw_cube(Vec3.ZERO,Vec3.ONE,Color.PURE_BLUE, None)
    #draw_all_objects()
    
    for cube in cubes:
        asteroid.scale = cube.scale
        asteroid.pos = cube.pos
        asteroid.rot = cube.rot
        asteroid.manually_draw_now()

    
    set_default_camera()
    draw_text(f"{get_fps()}",240,40,Color.GREEN)
    
    draw_text(f"YIPPIE",600.0, 300.0, Color.VOMIT_YELLOW,)
    draw_text(f"{int(round(5))} FPS (10s Avg)", 10.0, 30.0, Color.GREEN, 40)
    next_frame(get_delta_time())