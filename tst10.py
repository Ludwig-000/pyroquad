
from pyroquad import *



activate_engine()

prefix = "https://raw.githubusercontent.com/Ludwig-000/Pyroquad_example_game_assets/main/TwoDGame/"

asset_definitions = [
    # --- TEXTURES ---
    # misc
    {"name": "W_Key", "url": "Inputs/Keyboard%20%26%20Mouse/Double/keyboard_w.png", "path": "keyboard_w.png", "type": "texture"},
    {"name": "A_Key", "url": "Inputs/Keyboard%20%26%20Mouse/Double/keyboard_a.png", "path": "keyboard_a.png", "type": "texture"},
    {"name": "S_Key", "url": "Inputs/Keyboard%20%26%20Mouse/Double/keyboard_s.png", "path": "keyboard_s.png", "type": "texture"},
    {"name": "D_Key", "url": "Inputs/Keyboard%20%26%20Mouse/Double/keyboard_d.png", "path": "keyboard_d.png", "type": "texture"},
    {"name": "ESC_Key", "url": "Inputs/Keyboard%20%26%20Mouse/Double/keyboard_escape.png", "path": "keyboard_escape.png", "type": "texture"},
    {"name": "LMB", "url": "Inputs/Keyboard%20%26%20Mouse/Double/mouse_left.png", "path": "mouse_left.png", "type": "texture"},
    
    {"name": "Cursor", "url": "Cursor/StoneCursorWenrexa/PNG/01.png", "path": "Cursor.png", "type": "texture"},
    {"name": "MenueCursor", "url": "Cursor/StoneCursorWenrexa/PNG/12.png", "path": "MenueCursor.png", "type": "texture"},

    # enviroment
    {"name": "grass_plain", "url": "TinyTown/Tiles/tile_0000.png", "path": "grass_plain.png", "type": "texture"},
    {"name": "grass_flower_1", "url": "TinyTown/Tiles/tile_0001.png", "path": "grass_flower_1.png", "type": "texture"},
    {"name": "grass_flower_2", "url": "TinyTown/Tiles/tile_0002.png", "path": "grass_flower_2.png", "type": "texture"},
    {"name": "tree_2", "url": "TinyTown/Tiles/tile_0004.png", "path": "tree_2.png", "type": "texture"},
    {"name": "tree_1", "url": "TinyTown/Tiles/tile_0016.png", "path": "tree_1.png", "type": "texture"},
    {"name": "yellow_tree_2", "url": "TinyTown/Tiles/tile_0003.png", "path": "yellow_tree_2.png", "type": "texture"},
    {"name": "yellow_tree_1", "url": "TinyTown/Tiles/tile_0015.png", "path": "yellow_tree_1.png", "type": "texture"},
    {"name": "bush", "url": "TinyTown/Tiles/tile_0005.png", "path": "bush.png", "type": "texture"},
    {"name": "dirt", "url": "TinyTown/Tiles/tile_0025.png", "path": "dirt.png", "type": "texture"},
    
    {"name": "teleporter1", "url": "Teleporter/Teleporter1.png", "path": "teleporter1.png", "type": "texture"},
    {"name": "teleporter2", "url": "Teleporter/Teleporter2.png", "path": "teleporter2.png", "type": "texture"},
    {"name": "teleporter3", "url": "Teleporter/Teleporter3.png", "path": "teleporter3.png", "type": "texture"},
    {"name": "teleporter4", "url": "Teleporter/Teleporter4.png", "path": "teleporter4.png", "type": "texture"},
    {"name": "teleporter5", "url": "Teleporter/Teleporter5.png", "path": "teleporter5.png", "type": "texture"},
    {"name": "teleporter6", "url": "Teleporter/Teleporter6.png", "path": "teleporter6.png", "type": "texture"},
    {"name": "teleporter7", "url": "Teleporter/Teleporter7.png", "path": "teleporter7.png", "type": "texture"},

    # attacks
    {"name": "slash_1", "url": "Slashes/Sword Slashes/White Slash Wide/File1.png", "path": "slash_1.png", "type": "texture"},
    {"name": "slash_2", "url": "Slashes/Sword Slashes/White Slash Wide/File2.png", "path": "slash_2.png", "type": "texture"},
    {"name": "slash_3", "url": "Slashes/Sword Slashes/White Slash Wide/File3.png", "path": "slash_3.png", "type": "texture"},
    {"name": "slash_4", "url": "Slashes/Sword Slashes/White Slash Wide/File4.png", "path": "slash_4.png", "type": "texture"},
    {"name": "slash_5", "url": "Slashes/Sword Slashes/White Slash Wide/File5.png", "path": "slash_5.png", "type": "texture"},
    {"name": "slash_6", "url": "Slashes/Sword Slashes/White Slash Wide/File6.png", "path": "slash_6.png", "type": "texture"},

    {"name": "fireball_1", "url": "fireballs/fire_ball_side_medium/imgs/img_0.png", "path": "fireball_1.png", "type": "texture"},
    {"name": "fireball_2", "url": "fireballs/fire_ball_side_medium/imgs/img_1.png", "path": "fireball_2.png", "type": "texture"},
    {"name": "fireball_3", "url": "fireballs/fire_ball_side_medium/imgs/img_2.png", "path": "fireball_3.png", "type": "texture"},
    {"name": "fireball_4", "url": "fireballs/fire_ball_side_medium/imgs/img_3.png", "path": "fireball_4.png", "type": "texture"},
    {"name": "fireball_5", "url": "fireballs/fire_ball_side_medium/imgs/img_4.png", "path": "fireball_5.png", "type": "texture"},
    {"name": "fireball_6", "url": "fireballs/fire_ball_side_medium/imgs/img_5.png", "path": "fireball_6.png", "type": "texture"},
    {"name": "fireball_7", "url": "fireballs/fire_ball_side_medium/imgs/img_6.png", "path": "fireball_7.png", "type": "texture"},
    {"name": "fireball_8", "url": "fireballs/fire_ball_side_medium/imgs/img_7.png", "path": "fireball_8.png", "type": "texture"},
    {"name": "fireball_9", "url": "fireballs/fire_ball_side_medium/imgs/img_8.png", "path": "fireball_9.png", "type": "texture"},
    {"name": "fireball_10", "url": "fireballs/fire_ball_side_medium/imgs/img_9.png", "path": "fireball_10.png", "type": "texture"},
    {"name": "fireball_11", "url": "fireballs/fire_ball_side_medium/imgs/img_10.png", "path": "fireball_11.png", "type": "texture"},
    {"name": "fireball_12", "url": "fireballs/fire_ball_side_medium/imgs/img_11.png", "path": "fireball_12.png", "type": "texture"},
    {"name": "fireball_13", "url": "fireballs/fire_ball_side_medium/imgs/img_12.png", "path": "fireball_13.png", "type": "texture"},
    {"name": "fireball_14", "url": "fireballs/fire_ball_side_medium/imgs/img_13.png", "path": "fireball_14.png", "type": "texture"},
    {"name": "fireball_15", "url": "fireballs/fire_ball_side_medium/imgs/img_14.png", "path": "fireball_15.png", "type": "texture"},
    {"name": "fireball_16", "url": "fireballs/fire_ball_side_medium/imgs/img_15.png", "path": "fireball_16.png", "type": "texture"},
    {"name": "fireball_17", "url": "fireballs/fire_ball_side_medium/imgs/img_16.png", "path": "fireball_17.png", "type": "texture"},
    {"name": "fireball_18", "url": "fireballs/fire_ball_side_medium/imgs/img_17.png", "path": "fireball_18.png", "type": "texture"},
    {"name": "fireball_19", "url": "fireballs/fire_ball_side_medium/imgs/img_18.png", "path": "fireball_19.png", "type": "texture"},
    {"name": "fireball_20", "url": "fireballs/fire_ball_side_medium/imgs/img_19.png", "path": "fireball_20.png", "type": "texture"},
    {"name": "fireball_21", "url": "fireballs/fire_ball_side_medium/imgs/img_20.png", "path": "fireball_21.png", "type": "texture"},
    {"name": "fireball_22", "url": "fireballs/fire_ball_side_medium/imgs/img_21.png", "path": "fireball_22.png", "type": "texture"},
    {"name": "fireball_23", "url": "fireballs/fire_ball_side_medium/imgs/img_22.png", "path": "fireball_23.png", "type": "texture"},
    {"name": "fireball_24", "url": "fireballs/fire_ball_side_medium/imgs/img_23.png", "path": "fireball_24.png", "type": "texture"},
    {"name": "fireball_25", "url": "fireballs/fire_ball_side_medium/imgs/img_24.png", "path": "fireball_25.png", "type": "texture"},
    {"name": "fireball_26", "url": "fireballs/fire_ball_side_medium/imgs/img_25.png", "path": "fireball_26.png", "type": "texture"},
    {"name": "fireball_27", "url": "fireballs/fire_ball_side_medium/imgs/img_26.png", "path": "fireball_27.png", "type": "texture"},
    {"name": "fireball_28", "url": "fireballs/fire_ball_side_medium/imgs/img_27.png", "path": "fireball_28.png", "type": "texture"},
    {"name": "fireball_29", "url": "fireballs/fire_ball_side_medium/imgs/img_28.png", "path": "fireball_29.png", "type": "texture"},
    {"name": "fireball_30", "url": "fireballs/fire_ball_side_medium/imgs/img_29.png", "path": "fireball_30.png", "type": "texture"},

    {"name": "fireball_fast_blue_1", "url": "fireballs/fireball_high_speed_side_small_blue/img_0.png", "path": "fireball_fast_blue_1.png", "type": "texture"},
    {"name": "fireball_fast_blue_2", "url": "fireballs/fireball_high_speed_side_small_blue/img_1.png", "path": "fireball_fast_blue_2.png", "type": "texture"},
    {"name": "fireball_fast_blue_3", "url": "fireballs/fireball_high_speed_side_small_blue/img_2.png", "path": "fireball_fast_blue_3.png", "type": "texture"},
    {"name": "fireball_fast_blue_4", "url": "fireballs/fireball_high_speed_side_small_blue/img_3.png", "path": "fireball_fast_blue_4.png", "type": "texture"},
    {"name": "fireball_fast_blue_5", "url": "fireballs/fireball_high_speed_side_small_blue/img_4.png", "path": "fireball_fast_blue_5.png", "type": "texture"},
    {"name": "fireball_fast_blue_6", "url": "fireballs/fireball_high_speed_side_small_blue/img_5.png", "path": "fireball_fast_blue_6.png", "type": "texture"},
    {"name": "fireball_fast_blue_7", "url": "fireballs/fireball_high_speed_side_small_blue/img_6.png", "path": "fireball_fast_blue_7.png", "type": "texture"},
    {"name": "fireball_fast_blue_8", "url": "fireballs/fireball_high_speed_side_small_blue/img_7.png", "path": "fireball_fast_blue_8.png", "type": "texture"},
    {"name": "fireball_fast_blue_9", "url": "fireballs/fireball_high_speed_side_small_blue/img_8.png", "path": "fireball_fast_blue_9.png", "type": "texture"},
    {"name": "fireball_fast_blue_10", "url": "fireballs/fireball_high_speed_side_small_blue/img_9.png", "path": "fireball_fast_blue_10.png", "type": "texture"},
    {"name": "fireball_fast_blue_11", "url": "fireballs/fireball_high_speed_side_small_blue/img_10.png", "path": "fireball_fast_blue_11.png", "type": "texture"},
    {"name": "fireball_fast_blue_12", "url": "fireballs/fireball_high_speed_side_small_blue/img_11.png", "path": "fireball_fast_blue_12.png", "type": "texture"},
    {"name": "fireball_fast_blue_13", "url": "fireballs/fireball_high_speed_side_small_blue/img_12.png", "path": "fireball_fast_blue_13.png", "type": "texture"},
    {"name": "fireball_fast_blue_14", "url": "fireballs/fireball_high_speed_side_small_blue/img_13.png", "path": "fireball_fast_blue_14.png", "type": "texture"},
    {"name": "fireball_fast_blue_15", "url": "fireballs/fireball_high_speed_side_small_blue/img_14.png", "path": "fireball_fast_blue_15.png", "type": "texture"},
    {"name": "fireball_fast_blue_16", "url": "fireballs/fireball_high_speed_side_small_blue/img_15.png", "path": "fireball_fast_blue_16.png", "type": "texture"},
    {"name": "fireball_fast_blue_17", "url": "fireballs/fireball_high_speed_side_small_blue/img_16.png", "path": "fireball_fast_blue_17.png", "type": "texture"},
    {"name": "fireball_fast_blue_18", "url": "fireballs/fireball_high_speed_side_small_blue/img_17.png", "path": "fireball_fast_blue_18.png", "type": "texture"},
    {"name": "fireball_fast_blue_19", "url": "fireballs/fireball_high_speed_side_small_blue/img_18.png", "path": "fireball_fast_blue_19.png", "type": "texture"},
    {"name": "fireball_fast_blue_20", "url": "fireballs/fireball_high_speed_side_small_blue/img_19.png", "path": "fireball_fast_blue_20.png", "type": "texture"},
    {"name": "fireball_fast_blue_21", "url": "fireballs/fireball_high_speed_side_small_blue/img_20.png", "path": "fireball_fast_blue_21.png", "type": "texture"},
    {"name": "fireball_fast_blue_22", "url": "fireballs/fireball_high_speed_side_small_blue/img_21.png", "path": "fireball_fast_blue_22.png", "type": "texture"},
    {"name": "fireball_fast_blue_23", "url": "fireballs/fireball_high_speed_side_small_blue/img_22.png", "path": "fireball_fast_blue_23.png", "type": "texture"},
    {"name": "fireball_fast_blue_24", "url": "fireballs/fireball_high_speed_side_small_blue/img_23.png", "path": "fireball_fast_blue_24.png", "type": "texture"},
    {"name": "fireball_fast_blue_25", "url": "fireballs/fireball_high_speed_side_small_blue/img_24.png", "path": "fireball_fast_blue_25.png", "type": "texture"},
    {"name": "fireball_fast_blue_26", "url": "fireballs/fireball_high_speed_side_small_blue/img_25.png", "path": "fireball_fast_blue_26.png", "type": "texture"},
    {"name": "fireball_fast_blue_27", "url": "fireballs/fireball_high_speed_side_small_blue/img_26.png", "path": "fireball_fast_blue_27.png", "type": "texture"},
    {"name": "fireball_fast_blue_28", "url": "fireballs/fireball_high_speed_side_small_blue/img_27.png", "path": "fireball_fast_blue_28.png", "type": "texture"},
    {"name": "fireball_fast_blue_29", "url": "fireballs/fireball_high_speed_side_small_blue/img_28.png", "path": "fireball_fast_blue_29.png", "type": "texture"},
    {"name": "fireball_fast_blue_30", "url": "fireballs/fireball_high_speed_side_small_blue/img_29.png", "path": "fireball_fast_blue_30.png", "type": "texture"},
    {"name": "fireball_fast_blue_31", "url": "fireballs/fireball_high_speed_side_small_blue/img_30.png", "path": "fireball_fast_blue_31.png", "type": "texture"},
    {"name": "fireball_fast_blue_32", "url": "fireballs/fireball_high_speed_side_small_blue/img_31.png", "path": "fireball_fast_blue_32.png", "type": "texture"},
    {"name": "fireball_fast_blue_33", "url": "fireballs/fireball_high_speed_side_small_blue/img_32.png", "path": "fireball_fast_blue_33.png", "type": "texture"},
    {"name": "fireball_fast_blue_34", "url": "fireballs/fireball_high_speed_side_small_blue/img_33.png", "path": "fireball_fast_blue_34.png", "type": "texture"},
    {"name": "fireball_fast_blue_35", "url": "fireballs/fireball_high_speed_side_small_blue/img_34.png", "path": "fireball_fast_blue_35.png", "type": "texture"},
    {"name": "fireball_fast_blue_36", "url": "fireballs/fireball_high_speed_side_small_blue/img_35.png", "path": "fireball_fast_blue_36.png", "type": "texture"},
    {"name": "fireball_fast_blue_37", "url": "fireballs/fireball_high_speed_side_small_blue/img_36.png", "path": "fireball_fast_blue_37.png", "type": "texture"},
    {"name": "fireball_fast_blue_38", "url": "fireballs/fireball_high_speed_side_small_blue/img_37.png", "path": "fireball_fast_blue_38.png", "type": "texture"},
    {"name": "fireball_fast_blue_39", "url": "fireballs/fireball_high_speed_side_small_blue/img_38.png", "path": "fireball_fast_blue_39.png", "type": "texture"},

    #bridge
    {"name": "bridge", "url": "bridge/PNG_n_Tiled/bridge_cropped.png", "path": "bridge_cropped.png", "type": "texture"},

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
    {"name": "door_frame", "url": "TinyTown/Tiles/tile_0074.png", "path": "door_frame.png", "type": "texture"},

    {"name": "crate", "url": "Crate/Industrial_Sprites/sCrate.png", "path": "crate.png", "type": "texture"},
    
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
    {"name": "dungeon_font", "url": "Fonts/DungeonFont.ttf", "path": "dungeon_font.ttf", "type": "font"},

    # --- SOUNDS ---
    # music
    {"name": "white_mist", "url": "sounds/white_mist.mp3", "path": "white_mist.mp3", "type": "sound"},
    {"name": "birds", "url": "sounds/birds.mp3", "path": "birds.mp3", "type": "sound"},
    
    # sound effects
    {"name": "select_button", "url": "sounds/select_button.mp3", "path": "select_button.mp3", "type": "sound"},
    {"name": "page_turn", "url": "sounds/page_turn.mp3", "path": "page_turn.mp3", "type": "sound"},
    {"name": "whoosh", "url": "SoundEffects/Other/whoosh_1.wav", "path": "whoosh_1.wav", "type": "sound"},
    {"name": "door_open", "url": "SoundEffects/Environment/door_open.wav", "path": "door_open.wav", "type": "sound"},
    {"name": "leaf", "url": "SoundEffects/leaf.mp3", "path": "leaf.mp3", "type": "sound"},
    
]


set_pc_assets_folder("assets/TwoDGame")
import time
start = time.time()
examples.loading_screen_future(
    lambda a: Loading.download_file_future(prefix + a["url"]),
    asset_definitions,
    "Downloading Assets"
)

end = time.time()

print(f"Download took {end-start}seconds")



