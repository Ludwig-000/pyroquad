from pyroquad import *
import math



class PlayerCamera:

    movespeed = 0.2
    cam: Camera3D
    yaw: float = 0.0
    pitch: float = -20.0
    speed: float = 0.1
    rot_speed: float = 2.0
    pitch_speed: float= 2.0
    middleMousePos: Vec2

    def __init__(self, position: Vec3):
        r"""run once at the start of the program after engine initialization."""
        show_mouse(False)
        set_cursor_grab(True)
        self.cam =  Camera3D()
        self.middleMousePos =  get_mouse_position()
        
        self.cam.position = position
        self.cam.target = Vec3(0.0, 0.0, 0.0)
        self.cam.up = Vec3(0.0, 1.0, 0.0)
        self.cam.fovy = 45.0
        self.cam.z_far = 100000


    def update(self):
        r"""run at the start of every frame."""
        delta_time = get_delta_time()
        keys = get_keys_down()
        currentMousePos = get_mouse_position()
        
        mouseDiffX = currentMousePos.x - self.middleMousePos.x
        mouseDiffY = currentMousePos.y - self.middleMousePos.y
        self.yaw -= mouseDiffX * 0.03
        self.pitch -= mouseDiffY * 0.03
        self.middleMousePos = currentMousePos

        self.pitch = max(-89.0, min(89.0, self.pitch))

        # Forward vector
        forward = Vec3(
            math.sin(math.radians(self.yaw)) * math.cos(math.radians(self.pitch)),
            math.sin(math.radians(self.pitch)),
            math.cos(math.radians(self.yaw)) * math.cos(math.radians(self.pitch))
        ).normalize()
        
        
        # Calculate right and up vectors
        world_up = Vec3(0, 1, 0)
        right = forward.cross(world_up).normalize()
        
        FACTOR  = 100
        
        if KeyCode.W in keys:
            self.cam.position += forward * self.movespeed * delta_time * FACTOR
        if KeyCode.S in keys:
            self.cam.position -= forward * self.movespeed* delta_time*  FACTOR

        if KeyCode.A in keys:
            self.cam.position -= right * self.movespeed* delta_time* FACTOR
        if KeyCode.D in keys:
            self.cam.position += right * self.movespeed* delta_time* FACTOR
        
        zoom_factor  = 0.003
        if KeyCode.R in keys:
            self.movespeed*= (1.05** (zoom_factor/delta_time))
        if KeyCode.T in keys:
            self.movespeed*= (0.95** (zoom_factor/delta_time))
        if KeyCode.Escape in keys:
            show_mouse(True)
            set_cursor_grab(False)

        # Update camera target
        self.cam.target = self.cam.position + forward
        Camera3D.set_camera(self.cam)