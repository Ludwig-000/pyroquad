#!/usr/bin/env python3

from pyroquad import *
import random
import time
from pyinstrument import Profiler
import math
import time
from typing import Optional
from utils import *
from custom_assets import load_all_assets
tileSize = 32




class DEBUG():
    this_frame_draw_calls = 0


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
        temp_rec = Rectangle(pos, 0, Vec2.ONE,Color.INVISIBLE) # we create a tiny rectangle at the mouse position for convenience.

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
            dirt_path_var1 = []
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
            dirt_path_var1 = self.dirt_path(Vec2(60, 8), Vec2(16, 13), 5, 0.7)


            self.level += gf2 + gf1 + gp + dirt_path_var1

        if level == 2:
            gf2= []
            gf1= []
            gp= []
            dirt_path_var1 = []
            water = []
            bridge = []
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
            dirt_path_var1 = self.dirt_path(scale=Vec2(35, 8), offset=Vec2(0, 13), seed=4, density=0.6, min_transparency=0.5)
            dirt_path_var2 = self.dirt_path(scale=Vec2(35, 8), offset=Vec2(39, 13), seed=53, density=0.3, min_transparency=0.5)

            water = self.water_river(Vec2(5,40), Vec2(35,0))
            
            bridge = [
                Rectangle(Vec2(800,1250), 0, Vec2(1050,1600), Color.WHITE, textures.get("bridge"))
            ]
            self.level += gf2 + gf1 + gp + dirt_path_var1 + dirt_path_var2 +  water + bridge
        if level == 3:
            self.level = Background.grass_back(5,5,2,5553) + self.dirt_path(scale=Vec2(65, 8), offset=Vec2(0, 13), seed=33, density=0.4, min_transparency=0.5)

        if level == 4:
            self.level = Background.grass_back(5,1,1,13) 
            self.level += self.dirt_path(scale=Vec2(40, 8), offset=Vec2(0, 13), seed=4234, density=0.6, min_transparency=0.1) 
            self.level += self.dirt_path(scale=Vec2(40, 8), offset=Vec2(40, 17), seed=4234, density=0.6, min_transparency=0.1)
            self.level += self.water_river(Vec2(80,7), Vec2(0,7))
        if level == 5:
            self.level = Background.grass_back(5,1,1,11)
            self.level += self.water_river(Vec2(12,3), Vec2(50,4))
            self.level += self.water_river(Vec2(3,13), Vec2(60,4))
            self.level += self.water_river(Vec2(15,5), Vec2(13,33))
            self.level += self.water_river(Vec2(5,5), Vec2(33,33))

    @staticmethod
    def grass_back(g: float, f1: float, f2: float, seed: int) -> list[Rectangle]:
        gps, texs = ([], [], []), ["grass_plain", "grass_flower_2", "grass_flower_1"]
        rng = random.Random(seed)
        for w in range(int(2194 / tileSize) + 1):
            for h in range(int(1234 / tileSize) + 1):
                i = rng.choices([0, 1, 2], weights=[g, f2, f1] if (g + f1 + f2) else [1, 0, 0])[0]
                gps[i].append(Rectangle(Vec2(w * tileSize, h * tileSize), 0, Vec2.splat(tileSize), Color.WHITE, textures.get(texs[i])))
        return gps[0] + gps[1] + gps[2]
    

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
    

    
    
    @staticmethod
    def water_river(scale: Vec2, offset: Vec2) -> list[Rectangle]:
        """Generates a solid rectangular block of water tiles."""
        water_tiles = []
        global tileSize
        
        for h in range(0, int(scale.y)):
            for w in range(0, int(scale.x)):
                pos_x = (w + offset.x) * tileSize
                pos_y = (h + offset.y) * tileSize
                
                # Standard tile setup mapped directly to global tileSize grid
                tile = Rectangle(Vec2(pos_x, pos_y), 0, Vec2.splat(tileSize), Color.BLUEY_GREEN)
                water_tiles.append(tile)
                
        return water_tiles
    

    def draw(self):
        batch_draw_shapes(self.level)
        DEBUG.this_frame_draw_calls += self.level.__len__()



class Player():

    hitbox: Rectangle
    visual: Sprite
    sword_visual: Sprite
    playerSize: float
    speed: float = tileSize*15.0
    max_hp: float = 10.0 **300 # ranging from 1.0 to 0.0
    health: float = max_hp
    walking_animation_index = 0
    animation_frames: list[Texture2D] = []
    last_animation_switch  = time.time()


    attack_sprite: Sprite | None =  None
    attack_animation_index = 0
    attack_frames: list[Texture2D] = []
    last_attack_animation_switch  = time.time()

    hitbox_visual_offset = Vec2(0,tileSize/2)
    has_moved_once =  False

    def __init__(self) -> None:
        self.sword_visual = Sprite([])
        self.playerSize = tileSize*2
        starting_pos = Vec2(tileSize*20,tileSize*15)
        self.hitbox = Rectangle(starting_pos - self.hitbox_visual_offset, 0, Vec2(self.playerSize, 5), Color.INVISIBLE)
        vis = Rectangle(starting_pos, 0, Vec2.splat(self.playerSize), Color.WHITE)
        vis.texture = textures.get("character_3")

        self.visual = Sprite([vis])
        self.animation_frames = [textures.get("character_1"),textures.get("character_2"),textures.get("character_3")] # type: ignore
        self.attack_frames = [textures.get("slash_1"),textures.get("slash_2"), textures.get("slash_3"),textures.get("slash_4"),textures.get("slash_5"),textures.get("slash_6")] # type: ignore

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
        
        direction = no_nav_area.check_move(self.hitbox, direction, False)
        
        

        # updating the visuals.
        new_position = self.hitbox.position + direction
        has_moved = not (new_position == self.hitbox.position)
        self.hitbox.position = new_position
        self.visual.move_to( self.hitbox.position - self.hitbox_visual_offset )


        if has_moved and self.last_animation_switch < time.time()-0.1:
            self.last_animation_switch = time.time()
            if self.walking_animation_index == 2:
                self.walking_animation_index = 0
            else:
                self.walking_animation_index += 1
            self.visual.parts[0].texture = self.animation_frames[self.walking_animation_index]
        
        # update slash visual
        def update_slash_visual():

            lhs: Vec2 = self.hitbox.position
            rhs = camera.screen_to_world(get_mouse_position())
            direction = (rhs - lhs).normalize_or_zero() * 100
            
            rot = direction.to_angle()

            if MouseButton.Left in get_mouse_buttons_pressed(): # start a new attack
                if self.attack_sprite is None: # we are not currently attacking
                    AudioManager.push_sound(sounds.get("whoosh"), relative_volume=2) #type: ignore
                    self.attack_animation_index = 0
                    self.attack_sprite = Sprite([Rectangle( self.hitbox.position + direction, rot, Vec2.splat(150), Color.WHITE, self.attack_frames[self.attack_animation_index] )])
                    self.last_attack_animation_switch = time.time()
            
            if self.attack_sprite is not None: # check if we step the attack animation
                if self.last_attack_animation_switch < time.time() - 0.05:
                    self.last_attack_animation_switch = time.time()
                    if self.attack_animation_index == self.attack_frames.__len__() -1 : # we completed an attack animation
                        self.attack_sprite = None
                        return
                    else: # step animation
                        self.attack_animation_index +=1
                        self.attack_sprite = Sprite([Rectangle( self.hitbox.position + direction, rot, Vec2.splat(150), Color.WHITE, self.attack_frames[self.attack_animation_index] )])
        update_slash_visual()

        # draw directional arrow
        self.sword_visual = Sprite([])
        if KeyCode.Q in keys:
            lhs: Vec2 = self.hitbox.position
            rhs = camera.screen_to_world(get_mouse_position())
            direction = rhs - lhs


            rotation = direction.to_angle()
            self.sword_visual = Sprite([
                Rectangle(
                    self.hitbox.position + direction.normalize_or_zero()*100,
                    rotation, 
                    Vec2(self.playerSize * 1.7, self.playerSize * 1.4), 
                    Color.RED, 
                    None
                )
            ])

class Sprite():
    parts: list[Rectangle] = []
    def __init__(self, sprites: list[Rectangle]) -> None:
        self.parts = sprites
        self.sort_y = max(rec.max_y() for rec in self.parts) if self.parts else 0.0

    def draw_indiscriminate(self):
        for part in self.parts:
            part.draw()
    def move_to(self, loc: Vec2):
        for part in self.parts:
            part.position = loc
        self.sort_y = max(rec.max_y() for rec in self.parts) if self.parts else 0.0

class NoNavArea:
    def __init__(self, level: int) -> None:
        # Pre-separated lists for fast native passing
        self.all_rects: list[Rectangle] = []
        self.projectile_rects: list[Rectangle] = []

        # Temporary list to build the level geometry easily
        raw_nav_area: list[tuple[Rectangle, bool]] = []

        if level == 0:
            raw_nav_area.extend([
                (Rectangle(Vec2(tileSize * 31, tileSize * 4.4), 0, Vec2(2194.0, 5), Color.INVISIBLE), True),
                (Rectangle(Vec2(tileSize * 31, tileSize * 33.5), 0, Vec2(2194.0, 5), Color.INVISIBLE), True),
                (Rectangle(Vec2(tileSize * 9.5, tileSize * 20), 0, Vec2(5, 1234.0), Color.INVISIBLE), True),
                (Rectangle(Vec2(tileSize * 58.4, tileSize * 20), 0, Vec2(5, 1234.0), Color.INVISIBLE), True),
            ])
        elif level == 1:
            raw_nav_area.extend([
                (Rectangle(Vec2(21 * tileSize, 13.4 * tileSize), 0, Vec2(tileSize * 14, tileSize * 3.2), Color.INVISIBLE), True)
            ])
        elif level == 2:
            raw_nav_area.extend([
                (Rectangle(Vec2(37, 0) * tileSize, 0, Vec2(5, 28.7) * tileSize, Color.INVISIBLE), False),
                (Rectangle(Vec2(37, 31.1) * tileSize, 0, Vec2(5, 28.7) * tileSize, Color.INVISIBLE), False),
            ])
        elif level == 4:
            raw_nav_area.extend([
                (Rectangle(Vec2(30, 10) * tileSize, 0, Vec2(100, 7) * tileSize, Color.INVISIBLE), False),
            ])
        elif level == 5:
            raw_nav_area.extend([
                (Rectangle(Vec2(55, 5) * tileSize, 0, Vec2(10, 3) * tileSize, Color.INVISIBLE), False),
                (Rectangle(Vec2(61, 10) * tileSize, 0, Vec2(3, 13) * tileSize, Color.INVISIBLE), False),

                (Rectangle(Vec2(20,35) * tileSize, 0, Vec2(15,5) * tileSize, Color.INVISIBLE), False),
                (Rectangle(Vec2(35,35) * tileSize, 0, Vec2(5,5) * tileSize, Color.INVISIBLE), False),
            ])
        # Distribute into the final flat lists
        for rect, collides_with_proj in raw_nav_area:
            self.all_rects.append(rect)
            if collides_with_proj:
                self.projectile_rects.append(rect)

    def debug_draw(self) -> None:
        debug_color = Color(0.365, 0.024, 0.914, 0.5)
        # Drawing from all_rects gives the same visual result
        for rect in self.all_rects:
            original_color = rect.color
            rect.color = debug_color
            rect.draw()
            rect.color = original_color

    def check_move(self, hitbox: Rectangle, attempted_move: Vec2, is_projectile: bool) -> Vec2:
        assert hitbox.rotation == 0, "for performance reasons, hitbox may not be rotated"

        move_x, move_y = attempted_move.x, attempted_move.y
        p_w, p_h = hitbox.scale.x / 2, hitbox.scale.y / 2
        p_x, p_y = hitbox.position.x, hitbox.position.y

        # Select the correct target list instantly (no boolean checks in loops)
        obstacles = (self.projectile_rects if is_projectile else self.all_rects) + [
            hb.hitbox for hb in destructible_manager.get(SceneManager.current_active_scene)
        ]

        # --- X-Axis Resolution ---
        if move_x != 0:
            # Broadphase: Create a rectangle covering the entire movement path
            sweep_x_pos = p_x + (move_x / 2.0)
            sweep_x_scale = hitbox.scale.x + abs(move_x)
            sweep_rect_x = Rectangle(Vec2(sweep_x_pos, p_y), 0, Vec2(sweep_x_scale, hitbox.scale.y), Color.INVISIBLE)
            
            # Fast native check: Only return shapes inside our movement path
            x_candidates = sweep_rect_x.collides_with_list(obstacles)

            for rect in x_candidates:
                n_w, n_h = rect.scale.x / 2, rect.scale.y / 2 #type: ignore
                n_x, n_y = rect.position.x, rect.position.y

                if abs(p_y - n_y) < (p_h + n_h):
                    if abs(p_x - n_x) < (p_w + n_w):  # Stuck inside collider
                        if (move_x > 0 and p_x < n_x) or (move_x < 0 and p_x > n_x):
                            move_x = 0
                    else:  # Approaching collider
                        if move_x > 0 and p_x < n_x:
                            gap = (n_x - n_w) - (p_x + p_w)
                            if gap >= 0 and move_x > gap:
                                move_x = gap
                        elif move_x < 0 and p_x > n_x:
                            gap = (n_x + n_w) - (p_x - p_w)
                            if gap <= 0 and move_x < gap:
                                move_x = gap

        # Screen Bounds X
        if move_x < 0:
            move_x = max(move_x, -(p_x - p_w)) if (p_x - p_w) >= 0.0 else 0.0
        elif move_x > 0:
            move_x = min(move_x, 2194.0 - (p_x + p_w)) if (p_x + p_w) <= 2194.0 else 0.0

        new_p_x = p_x + move_x

        # --- Y-Axis Resolution ---
        if move_y != 0:
            # Broadphase for Y, accounting for the newly resolved X position
            sweep_y_pos = p_y + (move_y / 2.0)
            sweep_y_scale = hitbox.scale.y + abs(move_y)
            sweep_rect_y = Rectangle(Vec2(new_p_x, sweep_y_pos), 0, Vec2(hitbox.scale.x, sweep_y_scale), Color.INVISIBLE)
            
            # Fast native check
            y_candidates = sweep_rect_y.collides_with_list(obstacles)

            for rect in y_candidates:
                n_w, n_h = rect.scale.x / 2, rect.scale.y / 2 #type: ignore
                n_x, n_y = rect.position.x, rect.position.y

                if abs(new_p_x - n_x) < (p_w + n_w):
                    if abs(p_y - n_y) < (p_h + n_h):
                        if (move_y > 0 and p_y < n_y) or (move_y < 0 and p_y > n_y):
                            move_y = 0
                    else:
                        if move_y > 0 and p_y < n_y:
                            gap = (n_y - n_h) - (p_y + p_h)
                            if gap >= 0 and move_y > gap:
                                move_y = gap
                        elif move_y < 0 and p_y > n_y:
                            gap = (n_y + n_h) - (p_y - p_h)
                            if gap <= 0 and move_y < gap:
                                move_y = gap

        if move_y < 0:
            move_y = max(move_y, -(p_y - p_h)) if (p_y - p_h) >= 0.0 else 0.0
        elif move_y > 0:
            move_y = min(move_y, 1234.0 - (p_y + p_h)) if (p_y + p_h) <= 1234.0 else 0.0

        return Vec2(move_x, move_y)

class DestructibleObject():
    hitbox: Rectangle
    visual: Sprite
    health: float
    def __init__(self, hitbox: Rectangle, visual: Sprite | None, health: float = 1.0) -> None:
        self.hitbox = hitbox
        if visual is None:
            self.visual = Sprite([
                hitbox
            ])
        else:
            self.visual = visual
        self.health = health

class DestructibleManager():
    destructibles_level_0: list[DestructibleObject]
    destructibles_level_1: list[DestructibleObject]
    destructibles_level_2: list[DestructibleObject]
    destructibles_level_3: list[DestructibleObject]
    destructibles_level_4: list[DestructibleObject]
    destructibles_level_5: list[DestructibleObject]
    def __init__(self) -> None:
        self.restock_level(0)
        self.restock_level(1)
        self.restock_level(2)
        self.restock_level(3)
        self.restock_level(4)
        self.restock_level(5)

    def get(self, level: int)-> list[DestructibleObject]:
        if level == 0:
            return self.destructibles_level_0
        elif level == 1:
            return self.destructibles_level_1
        elif level == 2:
            return self.destructibles_level_2
        elif level == 3:
            return self.destructibles_level_3
        elif level == 4:
            return self.destructibles_level_4
        elif level == 5:
            return self.destructibles_level_5
        else:
            return []
 
    def restock_level(self, level: int):
        crate_tex = textures.get("crate")
        if level == 0:
            self.destructibles_level_0 = []
        elif level == 1:
            self.destructibles_level_1 = []
        elif level == 2:
            self.destructibles_level_2 = [
                DestructibleObject(
                    Rectangle(Vec2(37*tileSize,15.6*tileSize), 0, Vec2.splat(75), Color.WHITE,crate_tex),
                    None,
                    1
                )
            ]
        elif level == 3:
            self.destructibles_level_3 = []
        elif level == 4:
            self.destructibles_level_4 = []
        elif level == 5:
            self.destructibles_level_5 = [
                DestructibleObject(
                    Rectangle(Vec2(37*tileSize,10*tileSize), 0, Vec2.splat(100), Color.WHITE,crate_tex), 
                None,4),
                DestructibleObject(
                    Rectangle(Vec2(37*tileSize,14*tileSize), 0, Vec2.splat(100), Color.WHITE,crate_tex), 
                None,4),
                DestructibleObject(
                    Rectangle(Vec2(41*tileSize,18*tileSize), 0, Vec2.splat(100), Color.WHITE,crate_tex), 
                None,4),

                DestructibleObject(
                    Rectangle(Vec2(15*tileSize,5*tileSize), 0, Vec2.splat(100), Color.WHITE,crate_tex), 
                None,4),
                DestructibleObject(
                    Rectangle(Vec2(19*tileSize,5*tileSize), 0, Vec2.splat(100), Color.WHITE,crate_tex), 
                None,4),
            ]
        else:
            raise BaseException("invalid level")


class BasicEnemy():
    disabled: bool
    hitbox: Rectangle
    visual: Sprite
    animation_frames = list[Texture2D]
    speed: float = tileSize*10.0
    active_scene: int #all enemies are only active in the scene they are spawned in.
    last_animation_switch  = time.time()
    walking_animation_index: int

    health: float = 1.0
    invulnerable_til: None | float = None

    def __init__(self, pos: Vec2, scene: int, speed: float | None = None) -> None:
        self.hitbox = Rectangle(pos, 0, Vec2.splat(100), Color.INVISIBLE)
        self.visual = Sprite([
            Rectangle(pos, 0, Vec2.splat(100),Color.GREEN)
        ])
        self.active_scene = scene

        self.animation_frames = [textures.get("character_1"),textures.get("character_2"),textures.get("character_3")] # type: ignore
        self.walking_animation_index = 0
        if speed:
            self.speed =speed
    def update(self, player: Player, no_nav: NoNavArea):
        if not SceneManager.current_active_scene == self.active_scene:
            return
        dt = get_delta_time()
        
        diff = player.hitbox.position - self.hitbox.position
        direction = diff.normalize_or_zero()
        
        move_step = direction * self.speed * dt
        
        move_vec_validated = no_nav.check_move(self.hitbox, move_step, False)
        
        self.hitbox.position += move_vec_validated
        self.visual.move_to(self.hitbox.position)

        if self.last_animation_switch < time.time()-0.1:
            self.last_animation_switch = time.time()
            if self.walking_animation_index == 2:
                self.walking_animation_index = 0
            else:
                self.walking_animation_index += 1
            self.visual.parts[0].texture = self.animation_frames[self.walking_animation_index] #type: ignore

class ProjectileEnemy():
    disabled: bool
    hitbox: Rectangle
    visual: Sprite
    animation_frames = list[Texture2D]
    
    base_speed: float = tileSize * 5.0
    base_target_distance: float = tileSize * 15.0
    
    active_scene: int 
    last_animation_switch: float
    walking_animation_index: int

    health: float = 1.0
    invulnerable_til: None | float = None
    
    speed: float
    target_distance: float
    orbit_direction: float 
    last_orbit_switch: float
    orbit_switch_interval: float
    drift_seed: float  # Unique offset so their breathing patterns aren't synced

    last_fire_time: float
    fire_cooldown: float
    projectiles_ref: list[EnemyProjectile]

    def __init__(self, pos: Vec2, scene: int, projectiles_ref: list[EnemyProjectile]) -> None:
        self.hitbox = Rectangle(pos, 0, Vec2.splat(100), Color.INVISIBLE)
        self.active_scene = scene

        self.animation_frames = [textures.get("character_1"), textures.get("character_2"), textures.get("character_3")] # type: ignore
        self.visual = Sprite([
            Rectangle(pos, 0, Vec2.splat(100), Color.YELLOW, self.animation_frames[0])
        ])
        self.walking_animation_index = 0
        
        self.speed = self.base_speed * random.uniform(0.9, 1.1)
        self.target_distance = self.base_target_distance + random.uniform(-tileSize * 1.0, tileSize * 1.0)
        
        self.orbit_direction = random.choice([-1.0, 1.0])
        self.last_orbit_switch = time.time()
        self.orbit_switch_interval = random.uniform(1.5, 3.5)
        self.drift_seed = random.uniform(0.0, 100.0)
        self.last_animation_switch = time.time() - random.uniform(0.0, 0.1)
        
        # --- NEW: Firing Logic Setup ---
        self.projectiles_ref = projectiles_ref
        self.last_fire_time = time.time()
        self.fire_cooldown = random.uniform(2.0, 4.0)

    def update(self, player: Player, no_nav: NoNavArea):
        if not SceneManager.current_active_scene == self.active_scene:
            return
        dt = get_delta_time()
        
        # --- EXISTING: Movement code ---
        diff = player.hitbox.position - self.hitbox.position
        current_distance = diff.length() 
        direction = diff.normalize_or_zero()
        
        tangent = Vec2(-direction.y, direction.x)
        radius_drift = math.sin(time.time() * 1.5 + self.drift_seed) * (tileSize * 0.5)
        flexible_target_distance = self.target_distance + radius_drift
        
        if time.time() - self.last_orbit_switch > self.orbit_switch_interval:
            self.orbit_direction *= -1
            self.last_orbit_switch = time.time()
            self.orbit_switch_interval = random.uniform(1.5, 4.0)
            
        buffer = 12.0
        if abs(current_distance - flexible_target_distance) <= buffer:
            radial_factor = 0.0   
        elif current_distance > flexible_target_distance:
            radial_factor = 1.0   
        else:
            radial_factor = -1.2  
            
        move_dir = (direction * radial_factor) + (tangent * self.orbit_direction)
        move_step = move_dir.normalize_or_zero() * self.speed * dt
        
        move_vec_validated = no_nav.check_move(self.hitbox, move_step, False)
        self.hitbox.position += move_vec_validated
        self.visual.move_to(self.hitbox.position)
        
        if self.last_animation_switch < time.time() - 0.1:
            self.last_animation_switch = time.time()
            if self.walking_animation_index == 2:
                self.walking_animation_index = 0
            else:
                self.walking_animation_index += 1
            self.visual.parts[0].texture = self.animation_frames[self.walking_animation_index] #type: ignore

        # --- NEW: Firing execution ---
        if time.time() - self.last_fire_time > self.fire_cooldown:
            self.last_fire_time = time.time()
            self.fire_cooldown = random.uniform(2.0, 4.0) # randomize next shot
            
            proj_hitbox = Rectangle(self.hitbox.position, 0, Vec2.splat(tileSize * 0.6), Color.INVISIBLE)
            projectile = EnemyProjectile(
                homing=False, 
                hitbox=proj_hitbox, 
                player_or_direction=player,
                active_scene=self.active_scene
            )
            self.projectiles_ref.append(projectile)

class EnemyProjectile():
    proj_speed: float = tileSize * 15.0
    max_lifetime: float # in seconds
    spawn_time: float

    homing: bool
    velocity: Vec2
    hitbox: Rectangle
    visual: Sprite 
    animation_frames: list[Texture2D]

    current_animation_index: int
    last_animation_switch: float
    active_scene: int
    is_deflected: bool

    def __init__(self, homing: bool, hitbox: Rectangle, player_or_direction: Player | Vec2, active_scene: int) -> None:
        self.max_lifetime = 5.0
        self.spawn_time = time.time()
        self.homing = homing
        self.hitbox = hitbox
        self.active_scene = active_scene
        self.is_deflected = False 
        
        # Default velocity aimed directly at the player
        if isinstance(player_or_direction, Player):
            self.velocity = (player_or_direction.hitbox.position - hitbox.position).normalize_or_zero() * self.proj_speed
        else:
            self.velocity = player_or_direction.normalize_or_zero() * self.proj_speed
        
        self.last_animation_switch = time.time()
        self.current_animation_index = 0

        self.animation_frames = [  # type: ignore
            textures.get(f"fireball_{i}") for i in range(1, 31)
        ]
        
        vis_rect = Rectangle(self.hitbox.position, self.velocity.to_angle(), self.hitbox.scale * 5, Color.WHITE, self.animation_frames[0])
        self.visual = Sprite([vis_rect])

    def deflect(self, new_direction: Vec2):
        self.is_deflected = True
        self.homing = False
        self.velocity = new_direction.normalize_or_zero() * tileSize * 40.0 

        self.animation_frames = [  # type: ignore
            textures.get(f"fireball_fast_blue_{i}") for i in range(1, 31)
        ]
        self.visual.parts[0].scale = self.hitbox.scale * 7
        self.spawn_time = time.time() 

    def update(self, player: Player, no_nav: NoNavArea) -> bool:
        """Returns True if the projectile should be destroyed."""
        if not SceneManager.current_active_scene == self.active_scene:
            return False

        dt = get_delta_time()
        
        # Optional homing interpolation towards the player (only if not deflected)
        if self.homing and not self.is_deflected:
            target_dir = (player.hitbox.position - self.hitbox.position).normalize_or_zero()
            self.velocity = (self.velocity * 0.95 + target_dir * (self.velocity.length() * 0.05))
            
        move_step = self.velocity * dt
        
        # Move the projectile and check bounds/walls
        move_vec_validated = no_nav.check_move(self.hitbox, move_step, True)
        
        # If the validated vector is smaller than the intended move step, we hit a wall
        if move_vec_validated.length() < move_step.length() * 0.99:
            return True 

        # Apply movement
        self.hitbox.position += move_vec_validated
        self.update_visual()

        # Check age
        if time.time() - self.spawn_time > self.max_lifetime:
            return True
            
        return False
    
    def update_visual(self):

        # Update position and rotation
        self.visual.parts[0].position = self.hitbox.position
        self.visual.parts[0].rotation = self.velocity.to_angle()
        
        # Step the animation
        now = time.time()
        if now - self.last_animation_switch > 0.015:
            self.last_animation_switch = now
            self.current_animation_index = (self.current_animation_index + 1) % self.animation_frames.__len__()
            self.visual.parts[0].texture = self.animation_frames[self.current_animation_index]



class EnemySpawner:

    class Wave:
        def __init__(self, enemies: list, max_enemies_remaining: int = 0, time_limit: float | None = None):
            self.enemies = enemies
            self.max_enemies_remaining = max_enemies_remaining
            self.time_limit = time_limit

    class Barrage:
        def __init__(self, active_phases: list[int], spawn_interval: float = 0.5, predictable: bool = True):
            self.active_phases = active_phases
            self.spawn_interval = spawn_interval
            self.timer = 0.0
            self.predictable: bool = predictable
            self.last_spawned_y: float = 0.0
            self.descending: bool = True

    class LevelConditions:
        def __init__(self, level: int, battle_spawns: list[EnemySpawner.Wave], barrages: list[EnemySpawner.Barrage] = []) -> None:
            self.level_number = level
            self.player_already_visited = False
            self.battle_phase = 0
            self.current_phase_already_spawned = False
            self.battle_spawns = battle_spawns
            self.barrages = barrages or []  # New: list of barrages for this level
            self.phase_start_time = 0.0 
            
            self.is_warning_phase = False
            self.warning_start_time = 0.0
            self.warning_duration = 1.0
            self.warning_markers: list[Rectangle] = []

    def __init__(self) -> None:
        global enemy_projectiles
        self.levels: dict[int, EnemySpawner.LevelConditions] = {
            3: self.LevelConditions(3, [
                self.Wave([BasicEnemy(Vec2(60*tileSize, 15*tileSize), 3, 100.0)]),
                self.Wave(
                    [BasicEnemy(Vec2(70*tileSize, 0), 3, 150.0), BasicEnemy(Vec2(70*tileSize, 40*tileSize), 3, 150.0), 
                     BasicEnemy(Vec2(0, 0), 3, 150.0), BasicEnemy(Vec2(0, 40*tileSize), 3, 150.0)],
                    max_enemies_remaining=1
                ),
                self.Wave(
                    [BasicEnemy(Vec2(70*tileSize, 0), 3, 200.0), BasicEnemy(Vec2(70*tileSize, 40*tileSize), 3, 200.0), 
                     BasicEnemy(Vec2(0, 0), 3, 200.0), BasicEnemy(Vec2(0, 40*tileSize), 3, 200.0),
                     BasicEnemy(Vec2(10*tileSize, 0), 3, 200.0), BasicEnemy(Vec2(20*tileSize, 0), 3, 200.0),
                     BasicEnemy(Vec2(30*tileSize, 0), 3, 200.0), BasicEnemy(Vec2(40*tileSize, 0), 3, 200.0),
                     BasicEnemy(Vec2(50*tileSize, 0), 3, 200.0)],
                ),
                self.Wave([BasicEnemy(Vec2(0*tileSize, 0*tileSize), 3, 400),BasicEnemy(Vec2(10*tileSize, 0*tileSize), 3, 400),BasicEnemy(Vec2(20*tileSize, 0*tileSize), 3, 400),BasicEnemy(Vec2(30*tileSize, 0*tileSize), 3, 400),BasicEnemy(Vec2(40*tileSize, 0*tileSize), 3, 400),BasicEnemy(Vec2(50*tileSize, 0*tileSize), 3, 400),BasicEnemy(Vec2(60*tileSize, 0*tileSize), 3, 400),], 5 ),
                self.Wave([BasicEnemy(Vec2(0*tileSize, 50*tileSize), 3, 400),BasicEnemy(Vec2(10*tileSize, 50*tileSize), 3, 400),BasicEnemy(Vec2(20*tileSize, 50*tileSize), 3, 400),BasicEnemy(Vec2(30*tileSize, 50*tileSize), 3, 400),BasicEnemy(Vec2(40*tileSize, 50*tileSize), 3, 400),BasicEnemy(Vec2(50*tileSize, 50*tileSize), 3, 400),BasicEnemy(Vec2(60*tileSize, 50*tileSize), 3, 400)])

            ]),

            4: self.LevelConditions(4, [
                self.Wave([ProjectileEnemy(Vec2(40*tileSize, 0), 4, enemy_projectiles)]),
                self.Wave([ProjectileEnemy(Vec2(20*tileSize, 0), 4, enemy_projectiles), ProjectileEnemy(Vec2(50*tileSize, 0), 4, enemy_projectiles),
                           BasicEnemy(Vec2(0, 40*tileSize), 4, 200.0), BasicEnemy(Vec2(0, 35*tileSize), 4, 200.0), 
                           BasicEnemy(Vec2(70*tileSize, 40*tileSize), 4, 200.0), BasicEnemy(Vec2(70*tileSize, 35*tileSize), 4, 200.0)]),
            ]),

            5: self.LevelConditions(5, [
                self.Wave([ProjectileEnemy(Vec2(60*tileSize, 0), 5, enemy_projectiles), BasicEnemy(Vec2(55*tileSize, 0), 5, 500), ProjectileEnemy(Vec2(65*tileSize, 10*tileSize), 5, enemy_projectiles)], max_enemies_remaining=1, time_limit= 1),
                self.Wave([ProjectileEnemy(Vec2(60*tileSize, 0), 5, enemy_projectiles), ProjectileEnemy(Vec2(55*tileSize, 0), 5, enemy_projectiles), ProjectileEnemy(Vec2(65*tileSize, 10*tileSize), 5, enemy_projectiles),
                           BasicEnemy(Vec2(0*tileSize, 40*tileSize), 5, 350), BasicEnemy(Vec2(10*tileSize, 40*tileSize), 5, 350), BasicEnemy(Vec2(20*tileSize, 40*tileSize), 5, 350), BasicEnemy(Vec2(30*tileSize, 40*tileSize), 5, 350), BasicEnemy(Vec2(40*tileSize, 40*tileSize), 5, 350), BasicEnemy(Vec2(50*tileSize, 40*tileSize), 5, 350), BasicEnemy(Vec2(60*tileSize, 40*tileSize), 5, 350),
                           ProjectileEnemy(Vec2(60*tileSize, 40*tileSize), 5, enemy_projectiles), ProjectileEnemy(Vec2(60*tileSize, 30*tileSize), 5, enemy_projectiles), ProjectileEnemy(Vec2(60*tileSize, 20*tileSize), 5, enemy_projectiles)], max_enemies_remaining=8),
                self.Wave([BasicEnemy(Vec2(0*tileSize, 40*tileSize), 5, 350), BasicEnemy(Vec2(10*tileSize, 40*tileSize), 5, 350), BasicEnemy(Vec2(20*tileSize, 40*tileSize), 5, 350),
                           BasicEnemy(Vec2(30*tileSize, 40*tileSize), 5, 350), BasicEnemy(Vec2(40*tileSize, 40*tileSize), 5, 350), BasicEnemy(Vec2(50*tileSize, 40*tileSize), 5, 350), BasicEnemy(Vec2(50*tileSize, 40*tileSize), 5, 350)], 5),
                self.Wave([ProjectileEnemy(Vec2(60*tileSize, 0), 5, enemy_projectiles),ProjectileEnemy(Vec2(0*tileSize, 0*tileSize), 5, enemy_projectiles),ProjectileEnemy(Vec2(10*tileSize, 0*tileSize), 5, enemy_projectiles),ProjectileEnemy(Vec2(20*tileSize, 0*tileSize), 5, enemy_projectiles),ProjectileEnemy(Vec2(30*tileSize, 0*tileSize), 5, enemy_projectiles),ProjectileEnemy(Vec2(40*tileSize, 0*tileSize), 5, enemy_projectiles),ProjectileEnemy(Vec2(50*tileSize, 0*tileSize), 5, enemy_projectiles),ProjectileEnemy(Vec2(60*tileSize, 0*tileSize), 5, enemy_projectiles),ProjectileEnemy(Vec2(70*tileSize, 0*tileSize), 5, enemy_projectiles),ProjectileEnemy(Vec2(70*tileSize, 10*tileSize), 5, enemy_projectiles),ProjectileEnemy(Vec2(70*tileSize, 20*tileSize), 5, enemy_projectiles),ProjectileEnemy(Vec2(70*tileSize, 30*tileSize), 5, enemy_projectiles),ProjectileEnemy(Vec2(70*tileSize, 40*tileSize), 5, enemy_projectiles)], max_enemies_remaining=5),
                self.Wave([BasicEnemy(Vec2(0*tileSize,40*tileSize), 5, 300),BasicEnemy(Vec2(5*tileSize,40*tileSize), 5, 300),BasicEnemy(Vec2(10*tileSize,40*tileSize), 5, 300),BasicEnemy(Vec2(15*tileSize,40*tileSize), 5, 300),BasicEnemy(Vec2(20*tileSize,40*tileSize), 5, 300),BasicEnemy(Vec2(25*tileSize,40*tileSize), 5, 300),BasicEnemy(Vec2(30*tileSize,40*tileSize), 5, 300),BasicEnemy(Vec2(35*tileSize,40*tileSize), 5, 300),BasicEnemy(Vec2(40*tileSize,40*tileSize), 5, 300),BasicEnemy(Vec2(45*tileSize,40*tileSize), 5, 300),BasicEnemy(Vec2(40*tileSize,50*tileSize), 5, 300),BasicEnemy(Vec2(55*tileSize,40*tileSize), 5, 300),BasicEnemy(Vec2(40*tileSize,60*tileSize), 5, 300),BasicEnemy(Vec2(65*tileSize,40*tileSize), 5, 300),BasicEnemy(Vec2(70*tileSize,40*tileSize), 5, 300),BasicEnemy(Vec2(75*tileSize,40*tileSize), 5, 300),BasicEnemy(Vec2(80*tileSize,40*tileSize), 5, 300),], max_enemies_remaining=20),
                self.Wave([BasicEnemy(Vec2(0*tileSize,40*tileSize), 5, 300),BasicEnemy(Vec2(5*tileSize,40*tileSize), 5, 300),BasicEnemy(Vec2(10*tileSize,40*tileSize), 5, 300),BasicEnemy(Vec2(15*tileSize,40*tileSize), 5, 300),BasicEnemy(Vec2(20*tileSize,40*tileSize), 5, 300),BasicEnemy(Vec2(25*tileSize,40*tileSize), 5, 300),BasicEnemy(Vec2(30*tileSize,40*tileSize), 5, 300),BasicEnemy(Vec2(35*tileSize,40*tileSize), 5, 300),BasicEnemy(Vec2(40*tileSize,40*tileSize), 5, 300),BasicEnemy(Vec2(45*tileSize,40*tileSize), 5, 300),BasicEnemy(Vec2(40*tileSize,50*tileSize), 5, 300),BasicEnemy(Vec2(55*tileSize,40*tileSize), 5, 300),BasicEnemy(Vec2(40*tileSize,60*tileSize), 5, 300),BasicEnemy(Vec2(65*tileSize,40*tileSize), 5, 300),BasicEnemy(Vec2(70*tileSize,40*tileSize), 5, 300),BasicEnemy(Vec2(75*tileSize,40*tileSize), 5, 300),BasicEnemy(Vec2(80*tileSize,40*tileSize), 5, 300),], max_enemies_remaining=50),
                self.Wave([BasicEnemy(Vec2(0*tileSize, 0*tileSize), 5, 350),BasicEnemy(Vec2(0*tileSize, 5*tileSize), 5, 350),BasicEnemy(Vec2(0*tileSize, 10*tileSize), 5, 350),BasicEnemy(Vec2(0*tileSize, 15*tileSize), 5, 350),BasicEnemy(Vec2(0*tileSize, 20*tileSize), 5, 350),BasicEnemy(Vec2(0*tileSize, 25*tileSize), 5, 350),BasicEnemy(Vec2(0*tileSize, 30*tileSize), 5, 350),BasicEnemy(Vec2(0*tileSize, 35*tileSize), 5, 350),BasicEnemy(Vec2(0*tileSize, 40*tileSize), 5, 350),BasicEnemy(Vec2(0*tileSize, 45*tileSize), 5, 350),], max_enemies_remaining=5),
                
                self.Wave([ProjectileEnemy(Vec2(60*tileSize, 0), 5, enemy_projectiles), ProjectileEnemy(Vec2(55*tileSize, 0), 5, enemy_projectiles), ProjectileEnemy(Vec2(65*tileSize, 10*tileSize), 5, enemy_projectiles),
                           BasicEnemy(Vec2(0*tileSize, 40*tileSize), 5, 350), BasicEnemy(Vec2(10*tileSize, 40*tileSize), 5, 350), BasicEnemy(Vec2(20*tileSize, 40*tileSize), 5, 350), BasicEnemy(Vec2(30*tileSize, 40*tileSize), 5, 350), BasicEnemy(Vec2(40*tileSize, 40*tileSize), 5, 350), BasicEnemy(Vec2(50*tileSize, 40*tileSize), 5, 350), BasicEnemy(Vec2(60*tileSize, 40*tileSize), 5, 350),
                
                           ProjectileEnemy(Vec2(60*tileSize, 40*tileSize), 5, enemy_projectiles), ProjectileEnemy(Vec2(60*tileSize, 30*tileSize), 5, enemy_projectiles), ProjectileEnemy(Vec2(60*tileSize, 20*tileSize), 5, enemy_projectiles)], 50),
                self.Wave([ProjectileEnemy(Vec2(0*tileSize, 0*tileSize), 5, enemy_projectiles),ProjectileEnemy(Vec2(10*tileSize, 0*tileSize), 5, enemy_projectiles),ProjectileEnemy(Vec2(20*tileSize, 0*tileSize), 5, enemy_projectiles),ProjectileEnemy(Vec2(30*tileSize, 0*tileSize), 5, enemy_projectiles),ProjectileEnemy(Vec2(40*tileSize, 0*tileSize), 5, enemy_projectiles),ProjectileEnemy(Vec2(50*tileSize, 0*tileSize), 5, enemy_projectiles),ProjectileEnemy(Vec2(60*tileSize, 0*tileSize), 5, enemy_projectiles),ProjectileEnemy(Vec2(70*tileSize, 0*tileSize), 5, enemy_projectiles)], 50),
                self.Wave([ProjectileEnemy(Vec2(0*tileSize, 40*tileSize), 5, enemy_projectiles),ProjectileEnemy(Vec2(10*tileSize, 40*tileSize), 5, enemy_projectiles),ProjectileEnemy(Vec2(20*tileSize, 40*tileSize), 5, enemy_projectiles),ProjectileEnemy(Vec2(30*tileSize, 40*tileSize), 5, enemy_projectiles),ProjectileEnemy(Vec2(40*tileSize, 40*tileSize), 5, enemy_projectiles),ProjectileEnemy(Vec2(50*tileSize, 40*tileSize), 5, enemy_projectiles),ProjectileEnemy(Vec2(60*tileSize, 40*tileSize), 5, enemy_projectiles),ProjectileEnemy(Vec2(70*tileSize, 40*tileSize), 5, enemy_projectiles)], 10),
                self.Wave([BasicEnemy(Vec2(0*tileSize, 0*tileSize), 5, 400),BasicEnemy(Vec2(0*tileSize, 10*tileSize), 5, 400),BasicEnemy(Vec2(0*tileSize, 20*tileSize), 5, 400),BasicEnemy(Vec2(0*tileSize, 30*tileSize), 5, 400),BasicEnemy(Vec2(0*tileSize, 40*tileSize), 5, 400),]),
                self.Wave([])
            ],
            barrages=[
                self.Barrage(active_phases=[1,2,3], spawn_interval=0.05, predictable= True),
                self.Barrage(active_phases=[7,8,9,10], spawn_interval=0.1, predictable= False)
            ]
            ),
        }
        
    def tick(self, enemies: list, delta_time: float):
        current_scene = SceneManager.current_active_scene
        level = self.levels.get(current_scene)
        
        if not level:
            return
            
        self._process_barrages(level, delta_time, enemy_projectiles)

        if level.current_phase_already_spawned and not level.is_warning_phase:
            self._check_wave_advance(level, enemies)

        if not level.current_phase_already_spawned and not level.is_warning_phase:
            self._start_warning_phase(level)

        if level.is_warning_phase:
            self._update_warning_phase(level, enemies)


    def _process_barrages(self, level: EnemySpawner.LevelConditions, delta_time: float, projectiles: list):
        SCREEN_WIDTH = 2160
        SCREEN_HEIGHT = 1231
        CURSOR_SPEED = 30.0

        for barrage in level.barrages:
            if level.battle_phase in barrage.active_phases:
                barrage.timer += delta_time
                if barrage.timer >= barrage.spawn_interval:
                    barrage.timer = 0.0
                    
                    start_y: float = 0.0
                    if not barrage.predictable:
                        start_y = random.uniform(0, SCREEN_HEIGHT)
                    else:
                        if barrage.last_spawned_y >= SCREEN_HEIGHT:
                            start_y = SCREEN_HEIGHT - 1.0
                            barrage.last_spawned_y = start_y
                            barrage.descending = False
                        elif barrage.last_spawned_y <= 0:
                            start_y = 1.0
                            barrage.last_spawned_y = start_y
                            barrage.descending = True
                        else:
                            if barrage.descending:
                                start_y = barrage.last_spawned_y + CURSOR_SPEED
                            else:
                                start_y = barrage.last_spawned_y - CURSOR_SPEED
                            barrage.last_spawned_y = start_y


                
                    hitbox = Rectangle(
                        position=Vec2(SCREEN_WIDTH, start_y), 
                        rotation=0.0, 
                        scale=Vec2.splat(16),
                        color=Color(1, 0, 0, 1)
                    )
                    
                    proj = EnemyProjectile(
                        homing=False, 
                        hitbox=hitbox, 
                        player_or_direction=Vec2(-1, 0), 
                        active_scene=level.level_number
                    )
                    projectiles.append(proj)

    def _check_wave_advance(self, level: EnemySpawner.LevelConditions, enemies: list[BasicEnemy | ProjectileEnemy]):
        if level.battle_phase >= len(level.battle_spawns):
            return  # Level is fully complete

        current_wave = level.battle_spawns[level.battle_phase]
        threshold_met = len(enemies) <= current_wave.max_enemies_remaining
        time_met = False
        
        if current_wave.time_limit is not None:
            time_passed = time.time() - level.phase_start_time
            if time_passed >= current_wave.time_limit:
                time_met = True
                
        if threshold_met or time_met:
            if level.battle_phase < len(level.battle_spawns) - 1:
                level.battle_phase += 1
                level.current_phase_already_spawned = False
            elif len(enemies) == 0:
                pass # Trigger level end/win condition here later

    def _start_warning_phase(self, level: EnemySpawner.LevelConditions):
        if level.battle_phase >= len(level.battle_spawns):
            return

        level.is_warning_phase = True
        level.warning_start_time = time.time()
        current_wave = level.battle_spawns[level.battle_phase]
        
        SCREEN_WIDTH = 2160
        SCREEN_HEIGHT = 1231
        PADDING = 10.0 
        
        for enemy in current_wave.enemies:
            marker_pos = enemy.hitbox.position
            marker_pos = marker_pos.clamp(Vec2.splat(PADDING), Vec2(SCREEN_WIDTH - PADDING, SCREEN_HEIGHT - PADDING))
            
            marker = Rectangle(
                position=marker_pos, 
                rotation=0.0, 
                scale=Vec2.splat(32*3), 
                color=Color(1, 0, 0, 1) 
            )
            
            def spin_marker(obj: Rectangle):
                obj.rotation += 10.0 * get_delta_time()
                if obj.color.a >= 1.0:
                    obj.color = Color(1,0,0,0.4)
                else:
                    obj.color = Color(1,0,0,obj.color.a + (3 * get_delta_time()))

            marker.tick(spin_marker)
            level.warning_markers.append(marker)

    def _update_warning_phase(self, level: EnemySpawner.LevelConditions, enemies: list):
        time_warning_active = time.time() - level.warning_start_time
        
        if time_warning_active >= level.warning_duration:
            # Clean up markers
            for marker in level.warning_markers:
                marker.remove_tick() 
            level.warning_markers.clear()
            
            # Spawn wave
            if level.battle_phase < len(level.battle_spawns):
                current_wave = level.battle_spawns[level.battle_phase]
                enemies.extend(current_wave.enemies)
            
            # Reset phase states
            level.is_warning_phase = False
            level.current_phase_already_spawned = True
            level.phase_start_time = time.time()

    def draw_warnings(self) -> None:
        current_scene = SceneManager.current_active_scene
        level = self.levels.get(current_scene)
        
        if level and level.is_warning_phase:
            for marker in level.warning_markers:
                marker.draw()



class KeyHints():
    key_size: Vec2 = Vec2.splat(0.06*2194.0)
    w: Rectangle
    a: Rectangle
    s: Rectangle
    d: Rectangle
    esc: Rectangle

    attack_hint: Rectangle
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

        self.attack_hint = Rectangle(Vec2(sw*0.54,sh*0.3),0, self.key_size, Color.WHITE, textures.get("LMB"))

    def draw(self):
        global player
        
        if KeyHints.__should_display_key_hints and (Player.has_moved_once == False):
            self.w.draw()
            self.a.draw()
            self.s.draw()
            self.d.draw()
            self.esc.draw()
        if SceneManager.current_active_scene == 2 and player.hitbox.position.x < 1300 and player.hitbox.position.x > 800:
            self.attack_hint.draw()

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

                Rectangle(pos + Vec2(0, size.y),0,size,Color.WHITE, textures.get("roof_4")),
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
            self.trees(min_x= 0, min_y= -2,max_x= 34,max_y= 5,seed= 473,count= 20,min_size= 3, max_size= 5)
            self.trees(min_x= 40, min_y= -2,max_x= 70,max_y= 5,seed= 43427,count= 20,min_size= 3, max_size= 5)
            self.trees(min_x=0,min_y=20,max_x=34,max_y=40,seed=5354,count=30,min_size=3,max_size=7)
            self.trees(min_x=40,min_y=20,max_x=70,max_y=40,seed=535,count=30,min_size=3,max_size=7)

        if level == 3:
            self.trees(min_x= 0, min_y= -2,max_x= 70,max_y= 5,seed= 43423,count= 40,min_size= 3, max_size= 4)
            self.trees(min_x=0,min_y=25,max_x=70,max_y=40,seed=3242,count=35,min_size=3,max_size=5)
        if level == 4:
            self.trees(min_x=0,min_y=25,max_x=70,max_y=40,seed=34242,count=35,min_size=3,max_size=5)
        if level == 5:
            self.trees(min_x=0,min_y=25,max_x=70,max_y=40,seed=2231,count=10,min_size=3,max_size=5)
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
        tmp_all_sprites = [] + self.sp + other_sprites + [player.visual] + [player.sword_visual] + [player.attack_sprite]


        sp_sorted: list[Sprite | None] = sorted(
            tmp_all_sprites,
            key=lambda sp: sp.sort_y if sp is not None else 0.0,
        )


        tmp_array: list[Rectangle] = [
            part 
            for sprite in sp_sorted if sprite is not None
            for part in sprite.parts
        ]
        batch_draw_shapes(tmp_array)
        DEBUG.this_frame_draw_calls += tmp_array.__len__()



class Trigger():
    hitbox: Rectangle
    audio: None | tuple[Sound, float]
    transition_to: None | int
    player_pos: None | Vec2
    active: bool
    name: str
    def __init__(self, hitbox: Rectangle, transition_to: None | int = None, 
                 audio: None | tuple[Sound, float] = None, player_pos: None | Vec2 = None, name: str = "Trigger") -> None:
        self.hitbox = hitbox
        self.audio = audio
        self.transition_to = transition_to
        self.player_pos = player_pos
        self.active = True
        self.name = name


class LevelTriggers():
    triggers: list[Trigger]
    current_level: int
    def __init__(self, level: int) -> None:
        self.load_new_triggers_for_area(level)

    def check(self, player: Player):
        global no_nav_area
        global middle_layer

        self.conditional_activate_triggers()

        for trigger in self.triggers:
            if not trigger.active:
                continue
            if player.hitbox.collides_with(trigger.hitbox):
                if trigger.transition_to is not None:
                    self.triggers = []
                    self.current_level = trigger.transition_to
                    SceneManager.switch_scene(trigger.transition_to, False)
                    self.load_new_triggers_for_area(trigger.transition_to)
                if trigger.audio is not None:
                    AudioManager.push_sound(trigger.audio[0], trigger.audio[1])
                if trigger.player_pos is not None:
                    player.hitbox.position = trigger.player_pos
    def debug_draw(self):
        for trigger in self.triggers:
            if trigger.active:
                trigger.hitbox.color = Color.ORANGE
            else:
                trigger.hitbox.color = Color.RED_BROWN
            trigger.hitbox.draw()
            trigger.hitbox.color = Color.INVISIBLE
    def load_new_triggers_for_area(self, area: int):
        self.current_level = area
        self.triggers =  []
        if area == 0:
            r = Rectangle(Vec2(tileSize*47, tileSize*33),0,Vec2(tileSize*4,tileSize*1), Color.INVISIBLE)
            self.triggers = [
                Trigger(r,1, player_pos=Vec2(tileSize*22,tileSize*17), audio= (sounds.get("door_open"), 1.0)) #type: ignore
            ]
        elif area == 1:
            house = Rectangle(Vec2(tileSize*22.5,tileSize*15),0,Vec2(tileSize*3,tileSize*0.5), Color.INVISIBLE)
            forest= Rectangle(Vec2(tileSize*69,tileSize*16),0,Vec2(tileSize*3,tileSize*10), Color.INVISIBLE)
            self.triggers = [
                Trigger(house,transition_to=0, player_pos=Vec2(tileSize*47,tileSize*30), audio= (sounds.get("door_open"), 1.0)), #type: ignore
                Trigger(forest,transition_to=2, player_pos=Vec2(tileSize*5,tileSize*16))
            ]
        elif area == 2:
            woods_trigger= Rectangle(Vec2(0, tileSize*16),0,Vec2(tileSize*3,tileSize*10), Color.INVISIBLE)
            woods_trigger2= Rectangle(Vec2(tileSize*68, tileSize*16),0,Vec2(tileSize*3,tileSize*10), Color.INVISIBLE)
            self.triggers = [
                Trigger(woods_trigger,transition_to=1, player_pos=Vec2(tileSize*65,tileSize*16)),
                Trigger(woods_trigger2,transition_to=3, player_pos=Vec2(tileSize*1,tileSize*16), name= "woods3")
            ]
        elif area == 3:
            woods_trigger2= Rectangle(Vec2(tileSize*68, tileSize*16),0,Vec2(tileSize*3,tileSize*10), Color.INVISIBLE)
            self.triggers = [
                Trigger(woods_trigger2,transition_to=4, player_pos=Vec2(tileSize*1,tileSize*20), name= "woods4")
            ]
        elif area == 4:
            woods_trigger2= Rectangle(Vec2(tileSize*68, tileSize*23),0,Vec2(tileSize*3,tileSize*10), Color.INVISIBLE)
            self.triggers = [
                Trigger(woods_trigger2,transition_to=5, player_pos=Vec2(tileSize*1,tileSize*20), name= "woods5")
            ]
    
    def set_trigger(self, name: str, active: bool):
        for trigger in self.triggers:
            if trigger.name == name:
                trigger.active = active
                return
        raise BaseException("Trigger not found")

    def conditional_activate_triggers(self):
        if self.current_level == 2:
            if destructible_manager.destructibles_level_2.__len__() == 0:
                self.set_trigger("woods3", True)
            else:
                self.set_trigger("woods3", False)
        elif self.current_level == 3:
            if enemies.__len__() == 0:
                self.set_trigger("woods4", True)
            else:
                self.set_trigger("woods4", False)
        elif self.current_level == 4:
            if enemies.__len__() == 0:
                self.set_trigger("woods5", True)
            else:
                self.set_trigger("woods5", False)
        



class Hud():
    health_bar_frame: Rectangle
    hp_width: float
    hp_height: float
    mouse_visual: Rectangle

    def __init__(self) -> None:
        h_ratio = 17 / 145
        width_total = 100
        self.hp_width = width_total
        self.hp_height = width_total * h_ratio
        
        x = 1000
        y = 1000
        self.health_bar_frame = Rectangle(Vec2(x, y), 0, Vec2(self.hp_width*1.04, self.hp_height*1.30), Color.WHITE, textures.get("full_hp_bar"))
        self.mouse_visual = Rectangle(Vec2(0,0), 0, Vec2.splat(30), Color.WHITE, textures.get("Cursor"))
    
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
        global camera
        self._draw_hp_bar(player.health/player.max_hp, player.hitbox.position)

        # draw the mouse overlay
        if mouse_inside_window():
            show_mouse(False)
            offset = Vec2(15 * (screen_width()/ 2194.0) ,15 * (screen_height()/ 1234.0))
            self.mouse_visual.position = Camera2D.screen_to_world(camera, get_mouse_position() + offset)
            self.mouse_visual.draw()
        else:
            show_mouse(True)

class SceneManager():
    current_active_scene: int = 0
    @staticmethod
    def switch_scene(level: int, by_death: bool):
        global background
        global key_hints
        global no_nav_area
        global middle_layer
        global level_triggers
        global enemy_spawner
        global enemy_projectiles
        global enemies
        global destructible_manager
        global player
        SceneManager.current_active_scene = level

        background = Background(level)
        no_nav_area = NoNavArea(level)
        middle_layer = MiddleLayer(level)
        level_triggers = LevelTriggers(level)
        enemy_spawner = EnemySpawner()
        destructible_manager.restock_level(level)
        enemy_projectiles.clear()
        enemies.clear()
        if by_death:
            player.hitbox.position = Vec2(70.0, 500.0)
            player.health = player.max_hp


class DamageSystem(): # combined class handling all Damage related events.
    __last_damage_tick = time.time()
    __player_invulnerability_between_attacks = .5
    __enemy_invulnerability_between_attacks = .1
    enemy_dmg: float =  .5
    player_dmg: float = .51
    projectile_damage_player_: float = .5
    projectile_damage_enemy: float = 1.
    player_damage_to_destructible: float = .2

    @staticmethod
    def tick(enemies: list[BasicEnemy | ProjectileEnemy], projectiles: list[EnemyProjectile], player: Player, destructible_manager: DestructibleManager):
        DamageSystem.__player_damage_enemy(enemies, player)
        DamageSystem.__enemy_damage_player(enemies, player)
        DamageSystem.__player_damage_projectile(projectiles, player)
        DamageSystem.__projectile_damage_player(projectiles, player)
        DamageSystem.__projectile_damage_enemy(projectiles, enemies)
        DamageSystem.__player_damage_destructible(destructible_manager, player)
    
    @staticmethod
    def __enemy_damage_player(enemies: list[BasicEnemy | ProjectileEnemy], player: Player):
        colission = player.hitbox.collides_with_list([enemy.hitbox for enemy in enemies if enemy.active_scene == SceneManager.current_active_scene])
        if not colission == []:
            if DamageSystem.__last_damage_tick < (time.time() - DamageSystem.__player_invulnerability_between_attacks):
                
                player.health -= DamageSystem.enemy_dmg
                if player.health < 0.0:
                    player.health = 0.0
                DamageSystem.__last_damage_tick = time.time()

    @staticmethod
    def __player_damage_enemy(enemies: list[BasicEnemy | ProjectileEnemy], player: Player):
        if player.attack_sprite and player.attack_animation_index == 0:
            colissions = [enemy for enemy in enemies if enemy.active_scene == SceneManager.current_active_scene and enemy.hitbox.collides_with(player.attack_sprite.parts[0])]
            now = time.time()
            for col in colissions:

                if col.invulnerable_til is None or col.invulnerable_til < now:
                    col.health -= DamageSystem.player_dmg
                    TwoDPhysics.add_shove(
                        col,
                        ((col.hitbox.position - player.hitbox.position).normalize_or_zero() * 50) + col.hitbox.position
                    )
                    col.invulnerable_til = now + DamageSystem.__enemy_invulnerability_between_attacks
                    if col.health <= 0:
                        enemies.remove(col)
    @staticmethod
    def __projectile_damage_player(projectiles: list[EnemyProjectile], player: Player):
        now = time.time()
        projectiles_to_remove = []
        for proj in projectiles:
            if proj.active_scene == SceneManager.current_active_scene:
                if player.hitbox.collides_with(proj.hitbox):
                    
                    if DamageSystem.__last_damage_tick < (now - DamageSystem.__player_invulnerability_between_attacks):
                        player.health -= DamageSystem.projectile_damage_player_
                        if player.health < 0.0:
                            player.health = 0.0
                        DamageSystem.__last_damage_tick = now
                        
                    projectiles_to_remove.append(proj)
                    
        for proj in projectiles_to_remove:
            if proj in projectiles:
                projectiles.remove(proj)

    @staticmethod
    def __player_damage_projectile(projectiles: list[EnemyProjectile], player: Player):
        if player.attack_sprite and player.attack_animation_index == 0:
            for proj in projectiles:
                if proj.active_scene == SceneManager.current_active_scene and not proj.is_deflected:
                    if proj.hitbox.collides_with(player.attack_sprite.parts[0]):
                        rot = player.attack_sprite.parts[0].rotation
                        
                        new_dir = Vec2(math.cos(rot), math.sin(rot))
                        proj.deflect(new_dir)

    @staticmethod
    def __projectile_damage_enemy(projectiles: list[EnemyProjectile], enemies: list[BasicEnemy | ProjectileEnemy]):
        projectiles_to_remove = []
        now = time.time()
        for proj in projectiles:
            if proj.active_scene == SceneManager.current_active_scene and proj.is_deflected:
                for enemy in enemies:
                    if enemy.active_scene == SceneManager.current_active_scene and proj.hitbox.collides_with(enemy.hitbox):
                        if enemy.invulnerable_til is None or enemy.invulnerable_til < now:
                            enemy.health -= DamageSystem.projectile_damage_enemy
                            TwoDPhysics.add_shove(
                                enemy, 
                                ((enemy.hitbox.position - proj.hitbox.position).normalize_or_zero() * 50) + enemy.hitbox.position
                            )
                            enemy.invulnerable_til = now + DamageSystem.__enemy_invulnerability_between_attacks
                            if enemy.health <= 0:
                                enemies.remove(enemy)
                        
                        if proj not in projectiles_to_remove:
                            projectiles_to_remove.append(proj)
                        break
                        
        for proj in projectiles_to_remove:
            if proj in projectiles:
                projectiles.remove(proj)

    @staticmethod
    def __player_damage_destructible(destructible_manager: DestructibleManager, player: Player):
        if not (player.attack_sprite and player.attack_animation_index == 0):
            return

        current_level = SceneManager.current_active_scene
        level_objects = destructible_manager.get(current_level)
        weapon_hitbox = player.attack_sprite.parts[0]

        for obj in level_objects[:]:
            if obj.hitbox.collides_with(weapon_hitbox):
                obj.health -= DamageSystem.player_damage_to_destructible
                
                if obj.health <= 0.0:
                    level_objects.remove(obj)


class TwoDPhysics():
    __shove_speed_scale = 15.0

    class Shove_Event(): 
        def __init__(self, entity: Player | BasicEnemy | ProjectileEnemy, shove_location: Vec2, speed_scale: float) -> None:
            self.entity = entity
            self.shove_location = shove_location
            
            # Keep track of where we started and where we are going
            self.start_position = Vec2(entity.hitbox.position.x, entity.hitbox.position.y)
            to_target = shove_location - self.start_position
            
            self.total_distance = to_target.length()
            self.direction = to_target.normalize_or_zero()
            
            # Linear interpolation progress tracker (0.0 to 1.0)
            self.progress = 0.0
            self.speed_scale = speed_scale

        def update(self, speed_scale: float, delta_time: float, no_nav_area: NoNavArea) -> bool:
            """
            Updates the entity's position using time-based progression, 
            guaranteeing they stop precisely at the target distance.
            """
            if self.progress >= 1.0 or self.total_distance <= 0:
                return True 
            
            self.progress += delta_time * self.speed_scale
            if self.progress > 1.0:
                self.progress = 1.0
                
            eased_progress = 1.0 - (1.0 - self.progress) ** 2
            intended_target_dist = self.total_distance * eased_progress
            
            current_relative_pos = self.entity.hitbox.position - self.start_position
            current_dist_moved = current_relative_pos.length()
            
            move_step = intended_target_dist - current_dist_moved
            if move_step <= 0:
                return self.progress >= 1.0

            intended_displacement = self.direction * move_step
            actual_displacement = no_nav_area.check_move(self.entity.hitbox, intended_displacement, False)
            
            self.entity.hitbox.position += actual_displacement
            
            if actual_displacement.length() == 0 and move_step > 0:
                return True

            return self.progress >= 1.0

    events: list[Shove_Event] = []

    @staticmethod
    def tick(delta_time: float, no_nav_area: NoNavArea):
        TwoDPhysics.events = [
            event for event in TwoDPhysics.events 
            if not event.update(TwoDPhysics.__shove_speed_scale, delta_time, no_nav_area)
        ]

    @staticmethod
    def add_shove(entity: Player | BasicEnemy | ProjectileEnemy, target_location: Vec2):
        event = TwoDPhysics.Shove_Event(entity, target_location, TwoDPhysics.__shove_speed_scale)
        TwoDPhysics.events.append(event)


class Menue():
    main_background: list[Rectangle | Button]
    pause_background: list[Rectangle | Button]
    death_background: list[Rectangle | Button]

    static_fullscreen_toggle = True
    mouse_cursor: Rectangle
    def __init__(self) -> None:
        
        self.mouse_cursor = Rectangle(Vec2(0,0), 0, Vec2.splat(20), Color.WHITE, textures.get("MenueCursor"))

        moving_rectangles_center = Vec2(800.0, 500.0)

        moving_rectangles: list[Rectangle | Button] = [
            Rectangle(Vec2(1420.0, 500.0), 0, Vec2(120, 180), color=Color(0.12, 0.82, 0.18, 0.65)),
            Rectangle(Vec2(1335.0, 712.0), 0, Vec2(85, 140),  color=Color(0.06, 0.71, 0.24, 0.82)),
            Rectangle(Vec2(1082.0, 834.0), 0, Vec2(160, 90),  color=Color(0.20, 0.94, 0.08, 0.54)),
            Rectangle(Vec2(935.0,  922.0), 0, Vec2(110, 110), color=Color(0.10, 0.78, 0.29, 0.88)),
            Rectangle(Vec2(780.0,  790.0), 0, Vec2(190, 75),  color=Color(0.16, 1.00, 0.12, 0.73)),
            Rectangle(Vec2(800.0,  1080.0),0, Vec2(60, 130),  color=Color(0.04, 0.67, 0.20, 0.58)),
            Rectangle(Vec2(560.0,  815.0), 0, Vec2(145, 105), color=Color(0.14, 0.88, 0.25, 0.79)),
            Rectangle(Vec2(410.0,  882.0), 0, Vec2(80, 160),  color=Color(0.08, 0.76, 0.16, 0.61)),
            Rectangle(Vec2(325.0,  638.0), 0, Vec2(175, 85),  color=Color(0.22, 0.90, 0.31, 0.85)),
            Rectangle(Vec2(180.0,  500.0), 0, Vec2(95, 190),  color=Color(0.12, 0.69, 0.10, 0.52)),
            Rectangle(Vec2(365.0,  285.0), 0, Vec2(130, 70),  color=Color(0.18, 0.84, 0.22, 0.76)),
            Rectangle(Vec2(440.0,  190.0), 0, Vec2(115, 125), color=Color(0.07, 0.73, 0.14, 0.68)),
            Rectangle(Vec2(680.0,  80.0),  0, Vec2(200, 60),  color=Color(0.15, 0.96, 0.27, 0.89)),
            Rectangle(Vec2(710.0,  245.0), 0, Vec2(75, 150),  color=Color(0.09, 0.80, 0.06, 0.55)),
            Rectangle(Vec2(800.0,  -80.0), 0, Vec2(155, 115), color=Color(0.24, 0.92, 0.18, 0.81)),
            Rectangle(Vec2(980.0,  135.0), 0, Vec2(100, 100), color=Color(0.05, 0.75, 0.27, 0.63)),
            Rectangle(Vec2(1140.0, 170.0), 0, Vec2(165, 80),  color=Color(0.11, 0.86, 0.20, 0.77)),
            Rectangle(Vec2(1270.0, 220.0), 0, Vec2(90, 170),  color=Color(0.20, 0.65, 0.15, 0.50)),
            Rectangle(Vec2(1330.0, 310.0), 0, Vec2(135, 125), color=Color(0.06, 0.82, 0.32, 0.84)),
            Rectangle(Vec2(1190.0, 480.0), 0, Vec2(180, 95),  color=Color(0.16, 0.98, 0.24, 0.71)),
        ]
        def rec_movement(rec: Rectangle):
            angle = 0.0005
            v = rec.position - moving_rectangles_center
            cos_a = math.cos(angle)
            sin_a = math.sin(angle)
            rotated_v = Vec2(
                v.x * cos_a - v.y * sin_a,
                v.x * sin_a + v.y * cos_a
            )
            rec.position =  moving_rectangles_center + rotated_v
        for rectangle in moving_rectangles: rectangle.tick(rec_movement) # type: ignore


        self.main_background = (
            [
                Rectangle(position=Vec2(2194.0/2, 1234.0/2), rotation=0, scale=Vec2(2194.0, 1234.0), color=Color.WHITE, texture=textures.get("tree_screen"))
            ] 
            + moving_rectangles 
            + [
                Rectangle(position=Vec2(2194.0/2, tileSize*15), rotation=0, scale=Vec2(tileSize*30, tileSize*5), color=Color(1,1,1,0.5)),
                Button(Vec2(2194.0/2 + tileSize*4, tileSize*25), Vec2(tileSize*6, tileSize*3), button_color=Color(0.7, 0.7, 0.7, 1), label="Play"),
                Button(Vec2(2194.0/2 - tileSize*4, tileSize*25), Vec2(tileSize*6, tileSize*3), button_color=Color(0.7, 0.7, 0.7, 1), label="Quit"),
                Button(Vec2(2194.0/2 - tileSize*15, tileSize*25), Vec2(tileSize*10, tileSize*2), button_color=Color(0.7, 0.7, 0.7, 1), label="Toggle Fullscreen"),
                Button(Vec2(2194.0/2 - tileSize*15, tileSize*28), Vec2(tileSize*10, tileSize*2), button_color=Color(0.7, 0.7, 0.7, 1), label="Toggle Mute"),
            ]
        )
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
            Button(Vec2(700, tileSize*34), Vec2(tileSize*13,tileSize*2), button_color=Color(0.7,0.7,0.7,1),label="Toggle Difficulty: Hard"),
            ]





    def start(self, screen: int) -> bool:
        show_mouse(False)
        global camera
        global audio_manager
        ffp =  False # first frame passed.
        if screen == 0:
            AudioManager.set_background_sound(sounds.get("birds"))
            while True:
                camera.set_camera()
                for item in self.main_background:
                    if isinstance(item, Button):
                        if item.check():
                            if item.button_label == "Play":
                                clear_input_queue()
                                return False
                            elif item.button_label == "Quit":
                                clear_input_queue()
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
                

                draw_text("Haven't thought of a name yet  >_<",
                          (2194.0/2)-tileSize*15 +2 , 
                          1234.0/2 - tileSize*3 +2, Color.ORANGE_RED,
                          tileSize*20, fonts.get("dungeon_font"), font_scale=.1, font_scale_aspect=1)
                draw_text("Haven't thought of a name yet  >_<",
                          (2194.0/2)-tileSize*15, 
                          1234.0/2 - tileSize*3, Color.BLOOD_RED,
                          tileSize*20, fonts.get("dungeon_font"), font_scale=.1)
                self.draw_mouse()
                next_frame()
                examples.limit_fps(60)
        
        elif screen == 1:
            AudioManager.push_sound( sounds.get("page_turn"), 2 ) #type: ignore

            while True:
                if KeyCode.Escape in get_keys_pressed() and ffp:
                    next_frame()
                    return False
                if is_quit_requested():
                    next_frame()
                    return True
                for item in self.pause_background:
                    if isinstance(item, Button):
                        if item.check():
                            if item.button_label == "Continue":
                                clear_input_queue()
                                return False
                            elif item.button_label == "Quit":
                                clear_input_queue()
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
                          (2194.0/2)-tileSize*15, 1234.0/2 - tileSize*5,tileSize*20,Color.BLUE_VIOLET, fonts.get("dungeon_font"),0, None, .1
                          )
                self.draw_mouse()
                next_frame()
                ffp = True
                examples.limit_fps(60)
        elif screen == 2:
            AudioManager.set_background_sound(sounds.get("white_mist"), 0.3)
            bg_col = Color(0.03,0.03,0.03,1)
            while True:
                if KeyCode.Escape in get_keys_pressed() and ffp:
                    next_frame()
                    AudioManager.set_background_sound(sounds.get("birds"))
                    return False
                if is_quit_requested():
                    next_frame()
                    AudioManager.set_background_sound(sounds.get("birds"))
                    return True
                for item in self.death_background:
                    if isinstance(item, Button):
                        if item.check():
                            if item.button_label == "Perservere":
                                clear_input_queue()
                                AudioManager.set_background_sound(sounds.get("birds"))
                                return False
                            elif item.button_label == "Give up":
                                AudioManager.set_background_sound(sounds.get("birds"))
                                return True
                            elif item.button_label == "Toggle Fullscreen":
                                Menue.static_fullscreen_toggle = not Menue.static_fullscreen_toggle
                                set_fullscreen(Menue.static_fullscreen_toggle)
                            elif item.button_label == "Toggle Mute Audio":
                                AudioManager.toggle_mute()
                            
                            elif item.button_label == "Toggle Difficulty: Hard":
                                item.button_label = "Toggle Difficulty: Challenging"
                                player.max_hp = 2.0
                                item.button.x2 += 120
                            elif item.button_label == "Toggle Difficulty: Challenging":
                                item.button_label = "Toggle Difficulty: Easy"
                                player.max_hp = 10.0
                                item.button.x2 -= 120
                            elif item.button_label == "Toggle Difficulty: Easy":
                                item.button_label = "Toggle Difficulty: Hard"
                                player.max_hp = 1.0
                            

                clear_background(bg_col)
                for item in self.death_background:
                    item.draw()
                draw_multiline_text("You Have Died.\nWhat will you do?",
                          (150), 1234.0/2 - tileSize*5,tileSize*20,Color.BLUE_VIOLET, fonts.get("dungeon_font"), 0, None, .1
                          )
                self.draw_mouse()
                next_frame()
                ffp = True
                examples.limit_fps(60)
        else :
            RuntimeError("invalid screen value")
            return False
        
    def draw_mouse(self):

        offset = Vec2(8 * (screen_width()/ 2194.0) ,10 * (screen_height()/ 1234.0))
        global camera
        self.mouse_cursor.position = Camera2D.screen_to_world(camera, get_mouse_position() + offset)
        

        if mouse_inside_window():
            show_mouse(False)
            self.mouse_cursor.draw()
        else:
            show_mouse(True)



# profiler = Profiler(interval=0.1)
# profiler.start()

activate_engine(Config("2D Game",fullscreen=True,swap_interval=0, sample_count=10))
camera = Camera2D(rotation=0,zoom=Vec2(0.0009115, 0.0009115*16/9),target=Vec2.ZERO,offset=Vec2(-1,1))

dt = get_delta_time()
textures, fonts, sounds = load_all_assets()

examples.loading_screen(lambda a: a, [],"Initializing Scene")



enemies: list[BasicEnemy | ProjectileEnemy] = []
enemy_projectiles: list[EnemyProjectile] = []


menue = Menue()
player = Player()
hud = Hud()
destructible_manager = DestructibleManager()
background  = Background(0)
key_hints = KeyHints()
enemy_spawner = EnemySpawner()
no_nav_area = NoNavArea(0)
middle_layer = MiddleLayer(0)
level_triggers = LevelTriggers(0)

fps = get_fps()
last_fps_update = time.time()




should_quit: bool = menue.start(0)



prevent_quit()
while True:

    DEBUG.this_frame_draw_calls = 0

    dt= get_delta_time()
    if is_quit_requested() or should_quit:
        quit_program()
        print("Bye")
        break
    
    if KeyCode.Escape in get_keys_pressed():
        val = menue.start(1)
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
            BasicEnemy(Vec2.splat(500), SceneManager.current_active_scene)
        )
        enemies.append(
            ProjectileEnemy(Vec2.splat(500), SceneManager.current_active_scene, enemy_projectiles)
        )
    if KeyCode.K in get_keys_pressed():
        enemies.clear()
    if KeyCode.O in get_keys_pressed() and KeyCode.LeftShift in get_keys_down():
        for i in range(10):
            enemies.append(
            BasicEnemy(Vec2.splat(500), SceneManager.current_active_scene)
            )

    if KeyCode.Key0 in get_keys_pressed():
        SceneManager.switch_scene(0, True)
    if KeyCode.Key1 in get_keys_pressed():
        SceneManager.switch_scene(1, True)
    if KeyCode.Key2 in get_keys_pressed():
        SceneManager.switch_scene(2, True)
    if KeyCode.Key3 in get_keys_pressed():
        SceneManager.switch_scene(3, True)
    if KeyCode.Key4 in get_keys_pressed():
        SceneManager.switch_scene(4, True)
    if KeyCode.Key5 in get_keys_pressed():
        SceneManager.switch_scene(5, True)


    # logic
    if player.health <= 0.0:
        val = menue.start(2)
        if val:
            quit_program()
            print("Bye")
            break
        else:
            SceneManager.switch_scene(SceneManager.current_active_scene, True)

    player.update(no_nav_area)
    level_triggers.check(player)
    for enemy in enemies:
        enemy.update(player, no_nav_area)

    active_projectiles = []
    for proj in enemy_projectiles:
        should_destroy = proj.update(player, no_nav_area)
        if not should_destroy:
            active_projectiles.append(proj)
    enemy_projectiles[:] = active_projectiles


    DamageSystem.tick(enemies,enemy_projectiles, player, destructible_manager)

    TwoDPhysics.tick(get_delta_time(), no_nav_area)
    
    enemy_spawner.tick(enemies, get_delta_time())
    
    # drawing
    camera.set_camera()
    background.draw()
    middle_layer.draw(
        [enemy.visual for enemy in enemies if enemy.active_scene == SceneManager.current_active_scene] +
        [proj.visual for proj in enemy_projectiles if proj.active_scene == SceneManager.current_active_scene] +
        [obj.visual for obj in destructible_manager.get(SceneManager.current_active_scene)],
        player
    )
    enemy_spawner.draw_warnings()
    hud.draw(player)
    
    
    key_hints.draw()

    no_nav_area.debug_draw()
    level_triggers.debug_draw()

    if last_fps_update < time.time()-1:
        last_fps_update = time.time()
        fps  = get_fps()
    
    draw_text(f"{fps} fps",tileSize*2,tileSize*2,Color.WHITE,font_size=int(tileSize*1.3),font= fonts.get("bitcount_font"))
    draw_text(f"{DEBUG.this_frame_draw_calls} draw calls",tileSize*2,tileSize*3,Color.WHITE,font_size=int(tileSize*0.8),font= fonts.get("bitcount_font"))
    draw_text(f"player pos {player.hitbox.position.x:.1f}, {player.hitbox.position.y:.1f}", tileSize*2, tileSize*4, Color.WHITE, font_size=int(tileSize*0.8), font=fonts.get("bitcount_font"))
    draw_text(f"{enemies.__len__()} entities",tileSize*2,tileSize*5,Color.WHITE,font_size=int(tileSize*0.8),font= fonts.get("bitcount_font"))

    next_frame(None) #since this is a purely 2D game, we do not require 3d physics.
    #examples.limit_fps(300)

# profiler.stop()
# profiler.print()