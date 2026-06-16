from pyroquad import *

def load_all_assets() -> tuple[dict, dict, dict]:
    prefix = "https://raw.githubusercontent.com/Ludwig-000/Pyroquad_example_game_assets/main/TwoDGame/"

    asset_definitions = [
        # --- TEXTURES ---
        # misc
        {"name": "W_Key", "url": "Inputs/Keyboard%20%26%20Mouse/Double/keyboard_w.png", "path": "keyboard_w.png", "type": "texture"},
        {"name": "A_Key", "url": "Inputs/Keyboard%20%26%20Mouse/Double/keyboard_a.png", "path": "keyboard_a.png", "type": "texture"},
        {"name": "S_Key", "url": "Inputs/Keyboard%20%26%20Mouse/Double/keyboard_s.png", "path": "keyboard_s.png", "type": "texture"},
        {"name": "D_Key", "url": "Inputs/Keyboard%20%26%20Mouse/Double/keyboard_d.png", "path": "keyboard_d.png", "type": "texture"},
        {"name": "ESC_Key", "url": "Inputs/Keyboard%20%26%20Mouse/Double/keyboard_escape.png", "path": "keyboard_escape.png", "type": "texture"},
        
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
        
        # attacks
        {"name": "slash_1", "url": "Slashes/Sword Slashes/White Slash Wide/File1.png", "path": "slash_1.png", "type": "texture"},
        {"name": "slash_2", "url": "Slashes/Sword Slashes/White Slash Wide/File2.png", "path": "slash_2.png", "type": "texture"},
        {"name": "slash_3", "url": "Slashes/Sword Slashes/White Slash Wide/File3.png", "path": "slash_3.png", "type": "texture"},
        {"name": "slash_4", "url": "Slashes/Sword Slashes/White Slash Wide/File4.png", "path": "slash_4.png", "type": "texture"},
        {"name": "slash_5", "url": "Slashes/Sword Slashes/White Slash Wide/File5.png", "path": "slash_5.png", "type": "texture"},
        {"name": "slash_6", "url": "Slashes/Sword Slashes/White Slash Wide/File6.png", "path": "slash_6.png", "type": "texture"},

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
            tex = raw_file.to_Texture2D()
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

    build_texture_atlas()
    return textures, fonts, sounds