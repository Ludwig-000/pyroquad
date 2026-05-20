from pyroquad import *
import random
import time



class AudioManager():
    __background_sound: None | Sound = None
    __background_sound_relative_volue: float = 1.0
    __sounds: list[tuple[Sound, float]] = []
    __mute_toggle = False
    __global_volume= 0.05

    @staticmethod
    def push_sound(sound: Sound, relative_volume: float = 1.0):
        if AudioManager.__mute_toggle == False:
            sound.play_sound(PlaySoundParams(False, relative_volume* AudioManager.__global_volume))
        else:
            sound.play_sound(PlaySoundParams(False, 0))
        AudioManager.__sounds.append( (sound, relative_volume) )
    @staticmethod
    def set_background_sound(sound: None | Sound, relative_volume: float = 1.0):
        if AudioManager.__background_sound is not None:
            AudioManager.__background_sound.stop_sound()
            AudioManager.__background_sound = None
        if sound is not None:
            AudioManager.__background_sound = sound
            AudioManager.__background_sound_relative_volue = relative_volume
            AudioManager.__background_sound.play_sound(PlaySoundParams(True, AudioManager.__global_volume * relative_volume))
            if AudioManager.__mute_toggle:
                AudioManager.__background_sound.set_sound_volume(0)
    @staticmethod
    def toggle_mute():
        AudioManager.__mute_toggle = not AudioManager.__mute_toggle
        if not AudioManager.__background_sound == None:
            if AudioManager.__mute_toggle:
                AudioManager.__background_sound.set_sound_volume(0)
            else:
                AudioManager.__background_sound.set_sound_volume(AudioManager.__global_volume * AudioManager.__background_sound_relative_volue)

        for (sound, relative_volue) in AudioManager.__sounds:
            if AudioManager.__mute_toggle == True:
                sound.set_sound_volume(0)
            else:
                sound.set_sound_volume(AudioManager.__global_volume * relative_volue)







                