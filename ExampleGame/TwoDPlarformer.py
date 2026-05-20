#!/usr/bin/env python3

from pyroquad import *
import random
import time
from pyinstrument import Profiler
from utils import *


profiler = Profiler()
profiler.start()

activate_engine(Config("2D Game",fullscreen=True,swap_interval=0, sample_count=10))

camera = Camera2D(rotation=0,zoom=Vec2(0.0009115, 0.0009115*16/9),target=Vec2.ZERO,offset=Vec2(-1,1))

tileSize = 32


def load_all_assets() -> tuple[dict, dict, dict]:
    prefix = "https://raw.githubusercontent.com/Ludwig-000/Pyroquad_example_game_assets/main/TwoDGame/"

    asset_definitions = [
        # --- TEXTURES ---
        {"name": "W_Key", "url": "Inputs/Keyboard%20%26%20Mouse/Double/keyboard_w.png", "path": "keyboard_w.png", "type": "texture"},
        {"name": "A_Key", "url": "Inputs/Keyboard%20%26%20Mouse/Double/keyboard_a.png", "path": "keyboard_a.png", "type": "texture"},
        {"name": "S_Key", "url": "Inputs/Keyboard%20%26%20Mouse/Double/keyboard_s.png", "path": "keyboard_s.png", "type": "texture"},
        {"name": "D_Key", "url": "Inputs/Keyboard%20%26%20Mouse/Double/keyboard_d.png", "path": "keyboard_d.png", "type": "texture"},
        {"name": "ESC_Key", "url": "Inputs/Keyboard%20%26%20Mouse/Double/keyboard_escape.png", "path": "keyboard_escape.png", "type": "texture"},
        
        {"name": "grass_plain", "url": "TinyTown/Tiles/tile_0000.png", "path": "grass_plain.png", "type": "texture"},
        {"name": "grass_flower_1", "url": "TinyTown/Tiles/tile_0001.png", "path": "grass_flower_1.png", "type": "texture"},
        {"name": "grass_flower_2", "url": "TinyTown/Tiles/tile_0002.png", "path": "grass_flower_2.png", "type": "texture"},
        {"name": "tree_2", "url": "TinyTown/Tiles/tile_0004.png", "path": "tree_2.png", "type": "texture"},
        {"name": "tree_1", "url": "TinyTown/Tiles/tile_0016.png", "path": "tree_1.png", "type": "texture"},
        {"name": "yellow_tree_2", "url": "TinyTown/Tiles/tile_0003.png", "path": "yellow_tree_2.png", "type": "texture"},
        {"name": "yellow_tree_1", "url": "TinyTown/Tiles/tile_0015.png", "path": "yellow_tree_1.png", "type": "texture"},
        {"name": "bush", "url": "TinyTown/Tiles/tile_0005.png", "path": "bush.png", "type": "texture"},
        {"name": "dirt", "url": "TinyTown/Tiles/tile_0025.png", "path": "dirt.png", "type": "texture"},
        {"name": "door_frame", "url": "TinyTown/Tiles/tile_0074.png", "path": "door_frame.png", "type": "texture"},

        # house 
        {"name": "wall_center", "url": "TinyTown/Tiles/tile_0073.png", "path": "wall_center.png", "type": "texture"},
        {"name": "wall_right", "url": "TinyTown/Tiles/tile_0075.png", "path": "wall_right.png", "type": "texture"},
        {"name": "wall_left", "url": "TinyTown/Tiles/tile_0072.png", "path": "wall_left.png", "type": "texture"},
        {"name": "roof_1", "url": "TinyTown/Tiles/tile_0048.png", "path": "roof_1.png", "type": "texture"},
        {"name": "roof_2", "url": "TinyTown/Tiles/tile_0049.png", "path": "roof_2.png", "type": "texture"},
        {"name": "roof_3", "url": "TinyTown/Tiles/tile_0050.png", "path": "roof_3.png", "type": "texture"},
        {"name": "roof_4", "url": "TinyTown/Tiles/tile_0060.png", "path": "roof_4.png", "type": "texture"},
        {"name": "roof_5", "url": "TinyTown/Tiles/tile_0061.png", "path": "roof_5.png", "type": "texture"},
        {"name": "roof_6", "url": "TinyTown/Tiles/tile_0062.png", "path": "roof_6.png", "type": "texture"},
        {"name": "arched_roof", "url": "TinyTown/Tiles/tile_0063.png", "path": "arched_roof.png", "type": "texture"},
        {"name": "door", "url": "TinyTown/Tiles/tile_0085.png", "path": "door.png", "type": "texture"},
        
        # character
        {"name": "character_1", "url": "Characters/Tiles/tile_0355.png", "path": "tile_0355.png", "type": "texture"},
        {"name": "character_2", "url": "Characters/Tiles/tile_0356.png", "path": "tile_0356.png", "type": "texture"},
        {"name": "character_3", "url": "Characters/Tiles/tile_0357.png", "path": "tile_0357.png", "type": "texture"},

        # screens
        {"name": "skull_screen", "url": "screens/skull.png", "path": "skull.png", "type": "texture"},
        {"name": "tree_screen", "url": "screens/Tree_wallapaper.png", "path": "Tree_wallapaper.png", "type": "texture"},

        # UI
        {"name": "grey_button", "url": "UiAssets/01_Flat_Theme/Sprites/UI_Flat_Frame.png", "path": "UI_Flat_Frame.png", "type": "texture"},
        {"name": "grey_button_selected", "url": "UiAssets/01_Flat_Theme/Sprites/UI_Flat_Frame_selected.png", "path": "UI_Flat_Frame_selected.png", "type": "texture"},
        {"name": "full_hp_bar", "url": "UiAssets/01_Flat_Theme/Sprites/full_hp_bar.png", "path": "full_hp_bar.png", "type": "texture"},
        # --- FONTS ---
        {"name": "bitcount", "url": "Fonts/BitCount/BitcountPropDoubleInk-VariableFont_CRSV,ELSH,ELXP,SZP1,SZP2,XPN1,XPN2,YPN1,YPN2,slnt,wght.ttf", "path": "bitcount.ttf", "type": "font"},
        {"name": "arimo", "url": "Fonts/Arimo/Arimo-VariableFont_wght.ttf", "path": "arimo.ttf", "type": "font"},

        # --- SOUNDS ---
        {"name": "birds", "url": "sounds/birds.mp3", "path": "birds.mp3", "type": "sound"},
        {"name": "white_mist", "url": "sounds/white_mist.mp3", "path": "white_mist.mp3", "type": "sound"},
        {"name": "select_button", "url": "sounds/select_button.mp3", "path": "select_button.mp3", "type": "sound"},
        {"name": "page_turn", "url": "sounds/page_turn.mp3", "path": "page_turn.mp3", "type": "sound"},
    ]

    examples.loading_screen_future(
        lambda a: Loading.download_file_and_save_future(prefix + a["url"], a["path"]),
        asset_definitions,
        "Downloading All Assets"
    )

    paths = [a["path"] for a in asset_definitions]
    raw_files = examples.loading_screen(load_file, paths, "Reading Files")

    def convert_asset(item):
        raw_file, asset_type = item
        if asset_type == "texture":
            tex = raw_file.to_2DTexture()
            tex.set_filter(FilterMode.Nearest)
            return tex
        elif asset_type == "font":
            return raw_file.to_font()
        elif asset_type == "sound":
            return raw_file.to_Sound()
    conversion_inputs = list(zip(raw_files, [a["type"] for a in asset_definitions]))
    
    converted_assets = examples.loading_screen(
        convert_asset, 
        conversion_inputs, 
        "Loading Assets"
    )

    textures = {}
    fonts = {}
    sounds = {}

    for i, asset_def in enumerate(asset_definitions):
        name = asset_def["name"]
        asset_type = asset_def["type"]
        final_asset = converted_assets[i]

        if asset_type == "texture":
            textures[name] = final_asset
        elif asset_type == "font":
            fonts[name] = final_asset
        elif asset_type == "sound":
            sounds[name] = final_asset

    return textures, fonts, sounds

def quit_program():
    examples.loading_screen(lambda a: a, range(0), "Bye Bye")

class Button():
    button: Rectangle
    button_label: str
    button_tex: Texture2D | None
    button_tex_hovered: Texture2D | None
    def __init__(self, button_pos: Vec2, button_scale: Vec2, button_color: Color = Color.GREY, button_tex: None | Texture2D = None, label: str = "Button") -> None:
        self.button  = Rectangle(button_pos, 0, button_scale, button_color,button_tex)
        if button_tex is None:
            self.button.texture = textures.get("grey_button")
            self.button_tex = textures.get("grey_button")
            self.button_tex_hovered = textures.get("grey_button_selected")
        self.button_label = label
        
    def draw(self):
        pos = camera.screen_to_world( get_mouse_position())
        temp_rec = Rectangle(pos, 0, Vec2.ONE,Color.INVISIBLE)
        if self.button_tex is None: # highlighting via color
            tmp_col = self.button.color
            if temp_rec.collides_with(self.button):
                self.button.color = Color(tmp_col.r+ 0.1, tmp_col.g + 0.1 , tmp_col.b+ 0.1, tmp_col.a)
            self.button.draw()
            self.button.color = tmp_col
        else: #highlighting via texture
            if temp_rec.collides_with(self.button):
                self.button.texture = self.button_tex_hovered
            else:
                self.button.texture = self.button_tex
            self.button.draw()

        draw_text( 
            self.button_label,  self.button.position.x - self.button.scale.x /2 + tileSize/2,  
            self.button.position.y + (self.button.scale.y * 0.2), 
            Color.ORANGE, font_size=int(self.button.scale.y * 0.6), font=None
        )
        draw_text( 
            self.button_label,  self.button.position.x - self.button.scale.x /2 -tileSize/15 + tileSize/2,  
            self.button.position.y + (self.button.scale.y * 0.2) +tileSize/15, 
            Color.RED, font_size=int(self.button.scale.y * 0.6),font=None
        )

    def check(self) -> bool:
        if MouseButton.Left in get_mouse_buttons_pressed():
            pos = camera.screen_to_world( get_mouse_position())
            temp_rec = Rectangle(pos, 0, Vec2.ONE,Color.INVISIBLE)
            if temp_rec.collides_with(self.button):
                AudioManager.push_sound(sounds.get("select_button")) #type: ignore
                return True
        return False

class Background():
    level: list[Rectangle]
    
    def __init__(self, level= 1) -> None:
        self.level = []
        global tileSize
        if level == 0:
            self.level = [
                Rectangle(Vec2.splat(0),0, Vec2(2194.0*3, 1234.0*3), Color.CLAY_BROWN),  
            ]
            max_tiles_w = int(2194.0/tileSize)+1
            max_tiles_h  = int(1234.0/tileSize)+1
            for w in range(10, max_tiles_w-10):
                for h in range(5, max_tiles_h-5):
                    self.level.append(
                        Rectangle(Vec2(w*tileSize, h*tileSize),0, Vec2.splat(tileSize), Color.WHITE, textures.get("dirt"))
                    )
            
        if level == 1:
            gf2= []
            gf1= []
            gp= []
            dirt_path_var = []
            for w in range(0, (int(2194.0/tileSize)+1)):
                for h in range(0, (int(1234.0/tileSize)+1)):
                    seed_value = (w * 73856093) ^ (h * 19349663) ^ (level * 83492791)
                    rng = random.Random(seed_value)

                    tile  = Rectangle(Vec2(w*tileSize, h*tileSize),0,Vec2.splat(tileSize),Color.WHITE)
                    if rng.random() < 0.05:
                        tile.texture = textures.get("grass_flower_2")
                        gf2.append(tile)
                    elif rng.random() < 0.15:
                        tile.texture = textures.get("grass_flower_1")
                        gf1.append(tile)
                    else:
                        tile.texture = textures.get("grass_plain")
                        gp.append(tile)
            dirt_path_var = self.dirt_path(Vec2(60, 8), Vec2(16, 13), 5, 0.7)


            self.level += gf2 + gf1 + gp + dirt_path_var
        build_texture_atlas()

        if level == 2:
            gf2= []
            gf1= []
            gp= []
            dirt_path_var = []
            for w in range(0, (int(2194.0/tileSize)+1)):
                for h in range(0, (int(1234.0/tileSize)+1)):
                    seed_value = (w * 73856093) ^ (h * 19349663) ^ (level * 83492791)
                    rng = random.Random(seed_value)

                    tile  = Rectangle(Vec2(w*tileSize, h*tileSize),0,Vec2.splat(tileSize),Color.WHITE)
                    if rng.random() < 0.01:
                        tile.texture = textures.get("grass_flower_2")
                        gf2.append(tile)
                    elif rng.random() < 0.2:
                        tile.texture = textures.get("grass_flower_1")
                        gf1.append(tile)
                    else:
                        tile.texture = textures.get("grass_plain")
                        gp.append(tile)
            dirt_path_var = self.dirt_path(scale=Vec2(35, 8), offset=Vec2(0, 13), seed=4, density=0.6, min_transparency=0.5)


            self.level += gf2 + gf1 + gp + dirt_path_var
        build_texture_atlas()

    @staticmethod
    def dirt_path(scale: Vec2, offset: Vec2, seed: int, density: float = .5, min_transparency: float = 0.9, min_size: float = .7) -> list[Rectangle]:
        dirt_path = []
        global tileSize
        center_h = scale.y / 2.0
        max_distance = scale.y / 2.0 if scale.y > 0 else 1.0
        
        for h in range(0, int(scale.y)):
            for w in range(0, int(scale.x)):
                world_w = int(w + offset.x)
                world_h = int(h + offset.y)
                
                dirt_seed = (world_w * 73856093) ^ (world_h * 19349663) ^ (seed * 83492791) + 99999
                rng = random.Random(dirt_seed)
                
                distance_from_center = abs(h - center_h)
                normalized_dist = distance_from_center / max_distance
                
                spawn_chance = density - (normalized_dist * 0.70)
                
                if rng.random() < spawn_chance:
                    alpha = 1.0 - (normalized_dist * (1-min_transparency))
                    tile_color = Color(1.0, 1.0, 1.0, max(0.0, min(alpha, 1.0))) 
                    
                    pos_x = (w + offset.x) * tileSize
                    pos_y = (h + offset.y) * tileSize
                    size = Vec2.splat(rng.random() * tileSize * 1.5).clamp(Vec2.splat(min_size*tileSize), Vec2.MAX)
                    
                    tile = Rectangle(Vec2(pos_x, pos_y), 0, size, tile_color)
                    tile.texture = textures.get("dirt")
                    dirt_path.append(tile)
                    
        return dirt_path
    
    def draw(self):
        for rec in self.level:
            rec.draw()

class Sprite():
    parts: list[Rectangle] = []
    def __init__(self, sprites: list[Rectangle]) -> None:
        self.parts = sprites
    def draw_indiscriminate(self):
        for part in self.parts:
            part.draw()
    def move_to(self, loc: Vec2):
        for part in self.parts:
            part.position = loc

class NoNavArea():
    no_nav_area: list[Rectangle]

    def __init__(self, level) -> None:
        self.no_nav_area = []
        if level == 0:
            self.no_nav_area = [
                Rectangle(Vec2(tileSize*31, tileSize*4.4),0,Vec2(2194.0,5),Color.INVISIBLE),
                Rectangle(Vec2(tileSize*31, tileSize*33.5),0,Vec2(2194.0,5),Color.INVISIBLE),
                Rectangle(Vec2(tileSize*9.5, tileSize*20),0,Vec2(5,1234.0),Color.INVISIBLE),
                Rectangle(Vec2(tileSize*58.4, tileSize*20),0,Vec2(5,1234.0),Color.INVISIBLE),
            ]
        if level == 1:
            self.no_nav_area.append(  Rectangle(Vec2(21*tileSize,13.4*tileSize),0,Vec2(tileSize*14,tileSize*3.2),Color.INVISIBLE)   )

    def debug_draw(self):
        for nav in self.no_nav_area:
            nav.color = Color.BLUE_VIOLET
            nav.draw()
            nav.color = Color.INVISIBLE
    
    
    def check_move(self, hitbox: Rectangle, attempted_move: Vec2) -> Vec2:
        assert hitbox.rotation == 0, "for performance reasons, hitbox may not be rotated"
        final_move_x = attempted_move.x
        final_move_y = attempted_move.y
        p_w = hitbox.scale.x / 2
        p_h = hitbox.scale.y / 2
        p_x = hitbox.position.x
        p_y = hitbox.position.y

        # X-axes
        if final_move_x != 0:
            for nav in self.no_nav_area:
                n_w = nav.scale.x / 2
                n_h = nav.scale.y / 2
                n_x = nav.position.x
                n_y = nav.position.y
                if abs(p_y - n_y) < (p_h + n_h):
                    # we are stuck inside the collider
                    if abs(p_x - n_x) < (p_w + n_w):
                        if final_move_x > 0 and p_x < n_x:
                            final_move_x = 0
                        elif final_move_x < 0 and p_x > n_x:
                            final_move_x = 0
                    # we are not stuck inside the collider
                    else:
                        if final_move_x > 0 and p_x < n_x:
                            gap = (n_x - n_w) - (p_x + p_w)
                            if gap >= 0 and final_move_x > gap:
                                final_move_x = gap
                        elif final_move_x < 0 and p_x > n_x:
                            gap = (n_x + n_w) - (p_x - p_w)
                            if gap <= 0 and final_move_x < gap:
                                final_move_x = gap

        new_p_x = p_x + final_move_x

        # Y-axes
        if final_move_y != 0:
            for nav in self.no_nav_area:
                n_w = nav.scale.x / 2
                n_h = nav.scale.y / 2
                n_x = nav.position.x
                n_y = nav.position.y

                if abs(new_p_x - n_x) < (p_w + n_w):
                    
                    if abs(p_y - n_y) < (p_h + n_h):
                        if final_move_y > 0 and p_y < n_y:
                            final_move_y = 0
                        elif final_move_y < 0 and p_y > n_y:
                            final_move_y = 0
                    else:
                        if final_move_y > 0 and p_y < n_y:
                            gap = (n_y - n_h) - (p_y + p_h)
                            if gap >= 0 and final_move_y > gap:
                                final_move_y = gap
                        elif final_move_y < 0 and p_y > n_y:
                            gap = (n_y + n_h) - (p_y - p_h)
                            if gap <= 0 and final_move_y < gap:
                                final_move_y = gap

        return Vec2(final_move_x, final_move_y)

class Player():
    hitbox: Rectangle
    visual: Sprite
    sword_visual: Sprite = Sprite([])
    playerSize: float
    speed: float = tileSize*15.0
    walking_animation_index = 0
    animation_frames: list[Texture2D] = []
    last_animation_switch  = time.time()
    hitbox_visual_offset = Vec2(0,tileSize/2)
    has_moved_once =  False
    def __init__(self) -> None:
        self.playerSize = tileSize*2
        starting_pos = Vec2(tileSize*20,tileSize*15)
        self.hitbox = Rectangle(starting_pos - self.hitbox_visual_offset, 0, Vec2(self.playerSize, 5), Color.INVISIBLE)
        vis = Rectangle(starting_pos, 0, Vec2.splat(self.playerSize), Color.WHITE)
        vis.texture = textures.get("character_3")
        self.visual = Sprite([vis])
        self.animation_frames = [textures.get("character_1"),textures.get("character_2"),textures.get("character_3")] # type: ignore

    def update(self, no_nav: NoNavArea):
        dt = get_delta_time()
        keys = get_keys_down()

        dir = Vec2.ZERO
        if KeyCode.W in keys: dir -= Vec2(0,1)
        if KeyCode.S in keys: dir += Vec2(0,1)
        if KeyCode.A in keys: dir -= Vec2(1,0)
        if KeyCode.D in keys: dir += Vec2(1,0)

        if not dir == Vec2.ZERO:
            Player.has_moved_once = True
        direction = dir.normalize_or_zero() *self.speed * dt
        
        direction = no_nav_area.check_move(self.hitbox,direction)
        
        # Bounds Check
        position = self.hitbox.position + direction
        w = self.hitbox.scale.x/2
        h = self.hitbox.scale.y/2
        limit_x = 2194.0 - w
        limit_y = 1234.0 - h
        new_pos = position.clamp(Vec2(w, h), Vec2(limit_x, limit_y))
        has_moved = not (new_pos == self.hitbox.position)
        self.hitbox.position = position.clamp(Vec2(w, h), Vec2(limit_x, limit_y))
        self.visual.move_to( self.hitbox.position - self.hitbox_visual_offset )

        # updating the visuals.
        self.hitbox.draw()
        if has_moved and self.last_animation_switch < time.time()-0.1:
            self.last_animation_switch = time.time()
            if self.walking_animation_index == 2:
                self.walking_animation_index = 0
            else:
                self.walking_animation_index += 1
            self.visual.parts[0].texture = self.animation_frames[self.walking_animation_index]
        
        # attacks
        self.sword_visual = Sprite([])
        if KeyCode.Space in keys:
            self.sword_visual  = Sprite([Rectangle(self.hitbox.position+ Vec2(self.playerSize - 10, -10), 0, Vec2(self.playerSize*1.7,self.playerSize*1.4), Color.RED, None)])

class Enemy():
    disabled: bool
    hitbox: Rectangle
    visual: Sprite
    animation_frames = list[Texture2D]
    speed: float = tileSize*10.0
    active_scene: int #all enemies are only active in the scene they are spawned in.
    last_animation_switch  = time.time()
    walking_animation_index: int

    def __init__(self, pos: Vec2, scene: int) -> None:
        self.hitbox = Rectangle(pos, 0, Vec2.splat(100), Color.INVISIBLE)
        self.visual = Sprite([
            Rectangle(pos, 0, Vec2.splat(100),Color.GREEN)
        ])
        self.active_scene = scene

        self.animation_frames = [textures.get("character_1"),textures.get("character_2"),textures.get("character_3")] # type: ignore
        self.walking_animation_index = 0
    def update(self, player: Player, no_nav: NoNavArea):
        if not SceneManager.current_active_scene == self.active_scene:
            return
        dt = get_delta_time()
        
        diff = player.hitbox.position - self.hitbox.position
        direction = diff.normalize_or_zero()
        
        move_step = direction * self.speed * dt
        
        move_vec_validated = no_nav.check_move(self.hitbox, move_step)
        
        self.hitbox.position += move_vec_validated
        self.visual.move_to(self.hitbox.position)

        if self.last_animation_switch < time.time()-0.1:
            self.last_animation_switch = time.time()
            if self.walking_animation_index == 2:
                self.walking_animation_index = 0
            else:
                self.walking_animation_index += 1
            self.visual.parts[0].texture = self.animation_frames[self.walking_animation_index]

class KeyHints():
    key_size: Vec2 = Vec2.splat(0.06*2194.0)
    w: Rectangle
    a: Rectangle
    s: Rectangle
    d: Rectangle
    esc: Rectangle
    __should_display_key_hints= True
    def __init__(self) -> None:
        sw = 2194.0
        sh = 1234.0
        self.d = Rectangle(Vec2(sw*0.6,sh*0.8),0, self.key_size, Color.WHITE)
        self.d.texture = textures.get("D_Key")
        self.s = Rectangle(Vec2(sw*0.5,sh*0.8),0, self.key_size, Color.WHITE)
        self.s.texture = textures.get("S_Key")
        self.a = Rectangle(Vec2(sw*0.4,sh*0.8),0, self.key_size, Color.WHITE)
        self.a.texture = textures.get("A_Key")
        self.w = Rectangle(Vec2(sw*0.5,sh*0.65),0, self.key_size, Color.WHITE)
        self.w.texture = textures.get("W_Key")
        self.esc = Rectangle(Vec2(sw*0.9,sh*0.1),0, self.key_size, Color.WHITE)
        self.esc.texture = textures.get("ESC_Key")

    def draw(self):
        if KeyHints.__should_display_key_hints and not (Player.has_moved_once == True):
            self.w.draw()
            self.a.draw()
            self.s.draw()
            self.d.draw()
            self.esc.draw()

class Tree(Sprite):
    def __init__(self, size: Vec2, pos: Vec2, type: int = 1) -> None:
        if type == 1:
            sprites = [
                Rectangle(pos,0,size,Color.WHITE, textures.get("tree_2")),
                Rectangle(pos+ Vec2(0, size.y),0,size,Color.WHITE, textures.get("tree_1"))
            ]
            super().__init__(sprites)
        elif type == 2:
            sprites = [
                Rectangle(pos,0,size,Color.WHITE, textures.get("yellow_tree_2")),
                Rectangle(pos+ Vec2(0, size.y),0,size,Color.WHITE, textures.get("yellow_tree_1"))
            ]
            super().__init__(sprites)
        else:
            Exception()

class House(Sprite):
    def __init__(self, size: Vec2, pos: Vec2, type: int = 1) -> None:
        self.pos = pos
        self.size = size
        if type == 1:
            sprites = [
                
                Rectangle(pos+ Vec2(0, size.y*2),0,size,Color.WHITE, textures.get("wall_left")),
                Rectangle(pos + Vec2(size.x, size.y*2),0,size,Color.WHITE, textures.get("wall_center")),
                Rectangle(pos + Vec2(size.x*2, size.y*2),0,size,Color.WHITE, textures.get("door")),
                Rectangle(pos + Vec2(size.x*3, size.y*2),0,size,Color.WHITE, textures.get("wall_right")),

                Rectangle(pos + Vec2(0, size.y)                ,0,size,Color.WHITE, textures.get("roof_4")),
                Rectangle(pos + Vec2(size.x, size.y),0,size,Color.WHITE, textures.get("roof_5")),
                Rectangle(pos + Vec2(size.x*2, size.y),0,size,Color.WHITE, textures.get("roof_5")),
                Rectangle(pos + Vec2(size.x*3, size.y),0,size,Color.WHITE, textures.get("arched_roof")),

                Rectangle(pos,0,size,Color.WHITE, textures.get("roof_1")),
                Rectangle(pos + Vec2(size.x, 0),0,size,Color.WHITE, textures.get("roof_2")),
                Rectangle(pos + Vec2(size.x*2, 0),0,size,Color.WHITE, textures.get("roof_2")),
                Rectangle(pos + Vec2(size.x*3, 0),0,size,Color.WHITE, textures.get("roof_3")),
            ]
            super().__init__(sprites)
        else:
            Exception()

class MiddleLayer():
    level: int
    sp: list[Sprite]
    def __init__(self, level) -> None:
        self.sp =  []
        self.level = level
        if level == 0:
            door= Rectangle(Vec2(47*tileSize, 31*tileSize),0, Vec2.splat(tileSize*5), Color.WHITESMOKE, textures.get("door_frame"))
            self.sp.append(
                Sprite([door])
            )
        if level == 1:
            def trees():
                ts = tileSize
                seed_value = 42 
                rng = random.Random(seed_value)
                min_x, max_x = 20.0, 65.0
                min_y, max_y = 22.0, 36.0

                for i in range(18):
                    tree_type = 2 if i < 5 else 1
                    
                    pos_x = rng.uniform(min_x, max_x)
                    pos_y = rng.uniform(min_y, max_y)
                    
                    size_mult = rng.uniform(4.0, 6.5)
                    
                    if tree_type == 2:
                        self.sp.append(Tree(Vec2.splat(ts * size_mult), Vec2(ts * pos_x, ts * pos_y), 2))
                    else:
                        self.sp.append(Tree(Vec2.splat(ts * size_mult), Vec2(ts * pos_x, ts * pos_y)))
                for _ in range(4):
                    pos_x = rng.uniform(min_x, max_x)
                    pos_y = rng.uniform(min_y, max_y)
                    bush_size = rng.uniform(35, 55)
                    
                    bush_rect = Rectangle(Vec2(ts * pos_x, ts * pos_y), 0, Vec2.splat(bush_size), Color.WHITE)
                    bush_rect.texture = textures.get("bush")
                self.sp.append(Sprite([bush_rect])) # type: ignore
            trees()
            self.sp.append(
                House(Vec2.splat(tileSize*3.5), Vec2(tileSize*15.8,tileSize*6.25),1)
            )

        if level == 2:
            
            self.trees()
            self.trees(min_x=0,min_y=18,max_x=80,max_y=40,seed=333,count=60,min_size=3,max_size=7)

        sp = sorted(
            self.sp,
            key=lambda sp: max(rec.max_y() for rec in sp.parts), 
        )

    def trees(self, min_x: float=0.0, min_y: float=-2.0, max_x: float=80.0, max_y: float=5.0, seed: int=477 , count: int=35, min_size: float = 3.0, max_size: float = 5.0):
        ts = tileSize
        seed_value = seed
        rng = random.Random(seed_value)

        for i in range(count):
            tree_type = 2 if i < 5 else 1
            
            pos_x = rng.uniform(min_x, max_x)
            pos_y = rng.uniform(min_y, max_y)
            
            size_mult = rng.uniform(min_size, max_size)
            
            if tree_type == 2:
                self.sp.append(Tree(Vec2.splat(ts * size_mult), Vec2(ts * pos_x, ts * pos_y), 2))
            else:
                self.sp.append(Tree(Vec2.splat(ts * size_mult), Vec2(ts * pos_x, ts * pos_y)))
        for _ in range(4):
            pos_x = rng.uniform(min_x, max_x)
            pos_y = rng.uniform(min_y, max_y)
            bush_size = rng.uniform(35, 55)
            
            bush_rect = Rectangle(Vec2(ts * pos_x, ts * pos_y), 0, Vec2.splat(bush_size), Color.WHITE)
            bush_rect.texture = textures.get("bush")
            self.sp.append(Sprite([bush_rect])) # type: ignore

    def draw(self, other_sprites: list[Sprite], player: Player):
        tmp_all_sprites = [] + self.sp + other_sprites + [player.visual] + [player.sword_visual]

        sp_sorted = sorted(
            tmp_all_sprites,
            key=lambda sp: max(rec.max_y() for rec in sp.parts) if sp.parts else 0.0, 
        )
        for sp in sp_sorted:
            sp.draw_indiscriminate()

class LevelTriggers():
    triggers: list[Trigger]
    current_level: int
    def __init__(self, level: int) -> None:
        self.load_new_triggers_for_area(level)
    def check(self, player: Player):
        global no_nav_area
        global middle_layer
        for trigger in self.triggers:
            if player.hitbox.collides_with(trigger.hitbox):
                if trigger.transition_to is not None:
                    self.triggers = []
                    self.current_level = trigger.transition_to
                    SceneManager.switch_scene(trigger.transition_to)
                    self.load_new_triggers_for_area(trigger.transition_to)
                if trigger.audio is not None:
                    trigger.audio.play_sound_once()
                if trigger.player_pos is not None:
                    player.hitbox.position = trigger.player_pos
    def debug_draw(self):
        for trigger in self.triggers:
            trigger.hitbox.color = Color.ORANGE
            trigger.hitbox.draw()
            trigger.hitbox.color = Color.INVISIBLE
    def load_new_triggers_for_area(self, area: int):
        self.triggers =  []
        if area == 0:
            r = Rectangle(Vec2(tileSize*47, tileSize*33),0,Vec2(tileSize*4,tileSize*1), Color.INVISIBLE)
            self.triggers = [
                Trigger(r,1, player_pos=Vec2(tileSize*22,tileSize*17))
            ]
        if area == 1:
            house = Rectangle(Vec2(tileSize*22.5,tileSize*15),0,Vec2(tileSize*3,tileSize*0.5), Color.INVISIBLE)
            forest= Rectangle(Vec2(tileSize*69,tileSize*16),0,Vec2(tileSize*3,tileSize*10), Color.INVISIBLE)
            self.triggers = [
                Trigger(house,transition_to=0, player_pos=Vec2(tileSize*47,tileSize*30)),
                Trigger(forest,transition_to=2, player_pos=Vec2(tileSize*5,tileSize*16))
            ]
        if area == 2:
            house= Rectangle(Vec2(0, tileSize*16),0,Vec2(tileSize*3,tileSize*10), Color.INVISIBLE)
            self.triggers = [
                Trigger(house,transition_to=1, player_pos=Vec2(tileSize*66,tileSize*16))
            ]

class Trigger():
    hitbox: Rectangle
    audio: None | Sound
    transition_to: None | int
    player_pos: None | Vec2
    def __init__(self, hitbox: Rectangle, transition_to: None | int = None, 
                 audio: None | Sound = None, player_pos: None | Vec2 = None) -> None:
        self.hitbox = hitbox
        self.audio = audio
        self.transition_to = transition_to
        self.player_pos = player_pos

class Hud():
    health_bar_frame: Rectangle
    hp_width: float
    hp_height: float
    def __init__(self) -> None:
        h_ratio = 17 / 145
        width_total = 100
        self.hp_width = width_total
        self.hp_height = width_total * h_ratio
        
        x = 1000
        y = 1000
        self.health_bar_frame = Rectangle(Vec2(x, y), 0, Vec2(self.hp_width*1.04, self.hp_height*1.30), Color.WHITE, textures.get("full_hp_bar"))
    
    def _draw_hp_bar(self, percent: float, position: Vec2):
        self.health_bar_frame.position = position + Vec2(0, 50)
        self.health_bar_frame.draw()
    
        half_width = self.hp_width / 2
        half_height = self.hp_height / 2
        
        bar_right_edge = self.health_bar_frame.position.x + half_width
        bar_top_edge = self.health_bar_frame.position.y - half_height
        
        empty_width = self.hp_width * (1.0 - percent)
        
        black_x = bar_right_edge - empty_width
        
        if empty_width > 0:
            draw_rectangle(
                black_x,
                bar_top_edge,
                empty_width,
                self.hp_height,
                Color.BLACK
            )

    def draw(self, player: Player):
        self._draw_hp_bar(0.3, player.hitbox.position)

class SceneManager():
    current_active_scene: int = 0
    @staticmethod
    def switch_scene(level: int):
        global background
        global key_hints
        global no_nav_area
        global middle_layer
        global level_triggers
        SceneManager.current_active_scene = level
        background = Background(level)
        no_nav_area = NoNavArea(level)
        middle_layer = MiddleLayer(level)
        level_triggers = LevelTriggers(level)

class Menue():
    main_background: list[Rectangle | Button]
    pause_background: list[Rectangle | Button]
    death_background: list[Rectangle | Button]
    
    static_fullscreen_toggle = True
    def __init__(self) -> None:
        
        self.main_background  = [
            Rectangle(position=Vec2(2194.0/2, 1234.0/2),rotation=0, scale=Vec2(2194.0, 1234.0), color=Color.WHITE, texture=textures.get("tree_screen")),
            Rectangle(position=Vec2(2194.0/2, tileSize*15),rotation=0, scale=Vec2(tileSize*30,tileSize*5), color=Color(1,1,1,0.5)),
            Button(Vec2(2194.0/2+ tileSize*4, tileSize*25), Vec2(tileSize*6,tileSize*3), button_color=Color(0.7,0.7,0.7,1),label="Play"),
            Button(Vec2(2194.0/2- tileSize*4, tileSize*25), Vec2(tileSize*6,tileSize*3), button_color=Color(0.7,0.7,0.7,1),label="Quit"),
            Button(Vec2(2194.0/2- tileSize*15, tileSize*25), Vec2(tileSize*10,tileSize*2), button_color=Color(0.7,0.7,0.7,1),label="Toggle Fullscreen"),
            Button(Vec2(2194.0/2- tileSize*15, tileSize*28), Vec2(tileSize*10,tileSize*2), button_color=Color(0.7,0.7,0.7,1),label="Toggle Mute"),
            ]
        self.pause_background  = [
            Rectangle(Vec2(2194.0/2, 1234.0/2),0, Vec2(2194.0, 1234.0), Color.WHITE, textures.get("tree_screen")),
            Rectangle(Vec2(2194.0/2, tileSize*15),0, Vec2(tileSize*30,tileSize*5), Color(1,1,1,0.5)),
            Button(Vec2(2194.0/2+ tileSize*4, tileSize*25), Vec2(tileSize*7,tileSize*3), button_color=Color(0.7,0.7,0.7,1),label="Continue"),
            Button(Vec2(2194.0/2- tileSize*4, tileSize*25), Vec2(tileSize*6,tileSize*3), button_color=Color(0.7,0.7,0.7,1),label="Quit"),
            Button(Vec2(2194.0/2- tileSize*15, tileSize*25), Vec2(tileSize*10,tileSize*2), button_color=Color(0.7,0.7,0.7,1),label="Toggle Fullscreen"),
            Button(Vec2(2194.0/2- tileSize*15, tileSize*28), Vec2(tileSize*10,tileSize*2), button_color=Color(0.7,0.7,0.7,1),label="Toggle Mute Audio"),
            ]
        self.death_background  = [
            Rectangle(Vec2(1300, 1234.0/2),0, Vec2(2194.0, 1234.0), Color.WHITE, textures.get("skull_screen")),
            Rectangle(Vec2(600, tileSize*15),0, Vec2(tileSize*30,tileSize*5), Color(1,1,1,0.5)),
            Button(Vec2(300, tileSize*25), Vec2(tileSize*10,tileSize*2), button_color=Color(0.7,0.7,0.7,1),label="Perservere"),
            Button(Vec2(300, tileSize*28), Vec2(tileSize*10,tileSize*2), button_color=Color(0.7,0.7,0.7,1),label="Give up"),
            Button(Vec2(300, tileSize*31), Vec2(tileSize*10,tileSize*2), button_color=Color(0.7,0.7,0.7,1),label="Toggle Fullscreen"),
            Button(Vec2(300, tileSize*34), Vec2(tileSize*10,tileSize*2), button_color=Color(0.7,0.7,0.7,1),label="Toggle Mute Audio"),
            ]

    def start(self, screen: int) -> bool:
        global camera
        global audio_manager
        if screen == 0:
            AudioManager.set_background_sound(sounds.get("birds"))
            clear_background(Color.GREY)
            next_frame()
            while True:
                camera.set_camera(camera)
                for item in self.main_background:
                    if isinstance(item, Button):
                        if item.check():
                            if item.button_label == "Play":
                                return False
                            elif item.button_label == "Quit":
                                return True
                            elif item.button_label == "Toggle Fullscreen":
                                Menue.static_fullscreen_toggle = not Menue.static_fullscreen_toggle
                                set_fullscreen(Menue.static_fullscreen_toggle)
                            elif item.button_label == "Toggle Mute":
                                AudioManager.toggle_mute()
                            else:
                                RuntimeError("Unknown label")
                clear_background(Color.WHITE)
                for item in self.main_background:
                    item.draw()

                draw_text("Game or something",
                          (2194.0/2)-tileSize*15 +2 , 
                          1234.0/2 - tileSize*3 +2, Color.ORANGE_RED,
                          tileSize*2, None, font_scale=2)
                draw_text("Game or something",
                          (2194.0/2)-tileSize*15, 
                          1234.0/2 - tileSize*3, Color.BLOOD_RED,
                          tileSize*2, None, font_scale=2)
                next_frame()
                examples.limit_fps(60)
                
        elif screen == 1:
            AudioManager.push_sound( sounds.get("page_turn"), 2 ) #type: ignore
            clear_background(Color.GREY_PURPLE)
            next_frame()
            while True:
                if KeyCode.Escape in get_keys_pressed():
                    next_frame()
                    return False
                if is_quit_requested():
                    next_frame()
                    return True
                for item in self.pause_background:
                    if isinstance(item, Button):
                        if item.check():
                            if item.button_label == "Continue":
                                return False
                            elif item.button_label == "Quit":
                                return True
                            elif item.button_label == "Toggle Fullscreen":
                                Menue.static_fullscreen_toggle = not Menue.static_fullscreen_toggle
                                set_fullscreen(Menue.static_fullscreen_toggle)
                            elif item.button_label == "Toggle Mute Audio":
                                AudioManager.toggle_mute()
                            else:
                                RuntimeError("Unknown label")
                clear_background(Color.GREY_PURPLE)
                for item in self.pause_background:
                    item.draw()
                draw_multiline_text("Game Paused.\nPress 'Escape' to return.",
                          (2194.0/2)-tileSize*15, 1234.0/2 - tileSize*5,tileSize*2,None, Color.BLUE_VIOLET,
                          )
                next_frame()
                examples.limit_fps(60)
        elif screen == 2:
            AudioManager.set_background_sound(sounds.get("white_mist"), 0.3)
            bg_col = Color(0.03,0.03,0.03,1)
            clear_background(bg_col)
            next_frame()
            while True:
                if KeyCode.Escape in get_keys_pressed():
                    next_frame()
                    return False
                if is_quit_requested():
                    next_frame()
                    return True
                for item in self.death_background:
                    if isinstance(item, Button):
                        if item.check():
                            if item.button_label == "Perservere":
                                return False
                            elif item.button_label == "Give up":
                                return True
                            elif item.button_label == "Toggle Fullscreen":
                                Menue.static_fullscreen_toggle = not Menue.static_fullscreen_toggle
                                set_fullscreen(Menue.static_fullscreen_toggle)
                            elif item.button_label == "Toggle Mute Audio":
                                AudioManager.toggle_mute()
                            else:
                                RuntimeError("Unknown label")
                clear_background(bg_col)
                for item in self.death_background:
                    item.draw()
                draw_multiline_text("You Have Died.\nWhat will you do?",
                          (150), 1234.0/2 - tileSize*5,tileSize*2,None, Color.BLUE_VIOLET,
                          )
                next_frame()
                examples.limit_fps(60)
        else :
            RuntimeError("invalid screen value")
            return False

set_pc_assets_folder("assets/TwoDGame")
dt = get_delta_time()
textures, fonts, sounds = load_all_assets()

examples.loading_screen(lambda a: a, [],"Initializing Scene")

menue = Menue()
player = Player()
hud = Hud()
background  = Background(2)
key_hints = KeyHints()
no_nav_area = NoNavArea(2)
middle_layer = MiddleLayer(2)
level_triggers = LevelTriggers(2)
fps = get_fps()
last_fps_update = time.time()


should_quit = menue.start(0)

enemies: list[Enemy] = []

prevent_quit()
while True:
    
    dt= get_delta_time()
    if is_quit_requested() or should_quit:
        quit_program()
        print("Bye")
        break
    
    if KeyCode.Escape in get_keys_pressed():
        val  = menue.start(1)
        if val:
            quit_program()
            print("Bye")
            break
    if KeyCode.L in get_keys_pressed():
        val  = menue.start(2)
        if val:
            quit_program()
            print("Bye")
            break
    if KeyCode.O in get_keys_pressed():
        enemies.append(
            Enemy(Vec2.splat(500), SceneManager.current_active_scene)
        )
    if KeyCode.K in get_keys_pressed():
        enemies.clear()
    if KeyCode.O in get_keys_pressed() and KeyCode.LeftShift in get_keys_down():
        for i in range(10):
            enemies.append(
            Enemy(Vec2.splat(500), SceneManager.current_active_scene)
            )

    # logic 
    player.update(no_nav_area)
    level_triggers.check(player)
    for enemy in enemies:
        enemy.update(player, no_nav_area)
    # drawing

    camera.set_camera(camera)
    background.draw()
    middle_layer.draw([enemy.visual for enemy in enemies if enemy.active_scene == SceneManager.current_active_scene], player)
    hud.draw(player)
    no_nav_area.debug_draw()
    
    key_hints.draw()
    level_triggers.debug_draw()

    if last_fps_update < time.time()-1:
        last_fps_update = time.time()
        fps  = get_fps()
    
    draw_text(f"{fps} fps",tileSize*2,tileSize*2,Color.WHITE,font_size=int(tileSize*1.3),font= fonts.get("bitcount_font"))
    draw_text(f"{((player.hitbox.position.x/tileSize),(player.hitbox.position.y / tileSize))} player pos",tileSize*2,tileSize*4,Color.WHITE,font_size=int(tileSize*0.8),font=fonts.get("bitcount_font"))
    draw_text(f"{enemies.__len__()} enemies",tileSize*2,200,Color.WHITE,font_size=int(tileSize*1.3),font= fonts.get("bitcount_font"))

    next_frame(None) #since this is a purely 2D game, we do not require 3d physics.



profiler.stop()
profiler.print()

