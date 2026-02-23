use macroquad::audio as au;
use pyo3::{pyclass,pymethods};
use crate::engine::PChannel::PChannel;
use pyo3_stub_gen::derive::*;
use crate::engine::CoreLoop::COMMAND_QUEUE;
use crate::engine::CoreLoop::Command;
use pyo3::prelude::*;
use crate::engine::PArc::PArc;

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone, Debug)]
pub struct Sound {
   audio: PArc<au::Sound>,
}



#[gen_stub_pymethods]
#[pymethods]
impl Sound {

    /// Load audio file.
    ///
    /// Attempts to automatically detect the format of the source of data.
    /// 
    /// supported filetypes: ".wav", ".mp3"
    #[staticmethod]
    pub fn load_sound(path: String)-> PyResult<Sound>{

        let (tx, rx) = PChannel::sync_channel(1);

        COMMAND_QUEUE.push( Command::LoadSound { path, sender: tx } );

        let sound  = rx.recv()?;

        match sound{
            Ok(s) => { Ok( Sound{audio: s}   )  },
            Err(e) => {
                Err(e.into())
            }
        }

    }

    /// Load audio data.
    ///
    /// Attempts to automatically detect the format of the source of data.
    #[staticmethod]
    pub fn load_sound_from_bytes(data: Vec<u8>)-> PyResult<Sound> {

        let (sender, receiver) = PChannel::sync_channel(1);
        
        COMMAND_QUEUE.push( Command::LoadSoundFromBytes { data, sender} );

        let sound =  receiver.recv()?;
        match sound{
            Ok(s) => { Ok( Sound{audio: s} )  },
            Err(e) => {
                Err(e.into())
            }
        }
 
    }

    pub fn play_sound(&self, params: PlaySoundParams){ 
        COMMAND_QUEUE.push( Command::PlaySound { sound: au::Sound::from(self) , params: params.into() }  );
    }

    pub fn play_sound_once(&self){ 
        COMMAND_QUEUE.push( Command::PlaySoundOnce { sound: au::Sound::from(self) } );
    }

    pub fn set_sound_volume(&self, volume: f32){ 
        
        COMMAND_QUEUE.push( Command::SetSoundVolume { sound: au::Sound::from(self) , volume} );
    }

    pub fn stop_sound(&self){ 
        COMMAND_QUEUE.push( Command::StopSound { sound: au::Sound::from(self) } );
    }

}



impl From<au::Sound> for Sound{
    fn from(s: au::Sound) -> Self {
        Sound{ audio: PArc::new(s) }
    }
}


impl From<Sound> for au::Sound{
    fn from(s: Sound) -> Self {
        (*s.audio).clone()
    }
}
impl From<&Sound> for au::Sound {
    fn from(s: &Sound) -> Self {
        (*s.audio).clone()
    }
}


#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone, Copy)]
pub struct PlaySoundParams {

    #[pyo3(get, set)]
    pub looped: bool,

    #[pyo3(get, set)]
    pub volume: f32,
}

#[gen_stub_pymethods]
#[pymethods]
impl PlaySoundParams {

    #[new]
    #[pyo3(signature = ( looped = false, volume = 1.))]
    pub fn new(looped: bool, volume: f32) -> Self {
        Self { looped, volume }
    }

}


impl From<au::PlaySoundParams> for PlaySoundParams{
    fn from(r: au::PlaySoundParams) -> Self {
        PlaySoundParams { looped: r.looped, volume: r.volume }
    }
}



impl From<PlaySoundParams> for au::PlaySoundParams{
    fn from(r: PlaySoundParams) -> Self {
        au::PlaySoundParams { looped: r.looped, volume: r.volume }
    }
}
