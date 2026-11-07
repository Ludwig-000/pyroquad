from pyroquad import *
def launch():
    import random


    activate_engine(Config("marbles", window_width=800*2, window_height=600*2, swap_interval=0))


    hdr_raw: Image = examples.loading_screen_future(
        lambda a: download_file_future(a),
        ["https://github.com/Ludwig-000/pyroquad/blob/main/tests/sundowner_overlook_hdr.png?raw=true"],
        "Grabbing HDR",
        show_rotating_square=True
    )[0].to_Image()
    hdr_raw.flip_vertical()
    hdr = hdr_raw.to_texture()



    cam = examples.PlayerCamera(position=Vec3.splat(3), target=Vec3.ZERO)
    WALL = 0.2
    OUTER = 1.0 + 2 * WALL
    I = 1.0 / 2  # inner half extent
    O = OUTER / 2   # outer half extent

    wall_layout = [
        (-O, -O, -O,  O, -I,  O),  # bottom
        (-O,  I, -O,  O,  O,  O),  # top
        (-O, -O, -O, -I,  O,  O),  # left
        ( I, -O, -O,  O,  O,  O),  # right
        (-O, -O, -O,  O,  O, -I),  # back
        (-O, -O,  I,  O,  O,  O),  # front
    ]
    walls = [Cube.from_xyz(*corners, collider_type=Collider.FIXED())
            for corners in wall_layout]



    for wall in walls:
        wall.set_draw_each_frame(False)

    SPHERE_SIZE = 0.07
    SPHERE_COUNT = 729
    SPACING = 0.11
    grid = [(i, j, k) for i in range(9) for j in range(9) for k in range(9)]
    spheres = []
    for cell in grid[:SPHERE_COUNT]:
        p = Vec3(*((n - 4) * SPACING + random.uniform(-0.004, 0.004) for n in cell))
        spheres.append(Sphere(p, scale=Vec3.splat(SPHERE_SIZE), color=Color.BLUE,
                            collider_type=Collider.DYNAMIC()))

    angle = 0.0
    while True:
        if KeyCode.Escape in get_keys_pressed():
            break

        cam.update()

        draw_skybox(hdr)
        angle += get_delta_time()
        for wall in walls:
            wall.rotate_around(Vec3.ZERO, Vec3(get_delta_time(), 0,0))

        draw_cube_wires(rotation=Vec3(angle, 0,0))
        draw_grid(100, 1, Color.BLUE_VIOLET, Color.YELLOW_GREEN, Vec3(0, -1, 0))
        draw_all_objects()
        set_default_camera()
        draw_text(f"{get_fps()} fps", 50, 50, Color.BLACK, 30)
        draw_multiline_text("WASD to move\nMouse to look around\nESC to quit", 30, 90, 30, Color.RED)
        next_frame(get_delta_time())


if __name__ == "__main__":
    launch()