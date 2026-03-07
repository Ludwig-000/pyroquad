use pyo3::prelude::*;
 
//use pyo3::type_gen::generate_type;
//use pyo3::type_gen::generate_type_as_function;
use macroquad::prelude as mq;

use pyo3_stub_gen::derive::* ;

use crate::{engine::{PArc::PArc, PChannel::PChannel}, py_abstractions::{Loading::FileData::FileData, Textures_and_Images::FilterMode}};
use crate::engine::CoreLoop::COMMAND_QUEUE;
use crate::engine::CoreLoop::Command;
#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct Font {
    font: PArc<mq::Font>,
}


#[gen_stub_pymethods]
#[pymethods]
impl Font {

    /// Loads a font from file.
    #[new]
    pub fn new(path: String) -> PyResult<Font> {
        let (sender, receiver) = PChannel::sync_channel(1);

        COMMAND_QUEUE.push(  Command::LoadTTFFOnt { path, sender });
        Ok(receiver.recv()??.into())
    }

    /// List of ascii characters, may be helpful in combination with "populate_font_cache"
    #[staticmethod]
    pub fn ascii_character_list() -> Vec<char> {
        (0..255).filter_map(::std::char::from_u32).collect()
    }

    /// List of latin characters
    #[staticmethod]
    pub fn latin_character_list() -> Vec<char> {
        "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890!@#$%^&*(){}[].,:"
            .chars()
            .collect()
    }

    #[staticmethod]
    pub fn load_ttf_font_from_bytes(data: FileData)-> PyResult<Font>{
        let (sender, receiver) = PChannel::sync_channel(1);

        COMMAND_QUEUE.push(  Command::LoadTTFFontFromBytes { bytes: data.bytes, sender });
        
        Ok(receiver.recv()??.into())

    }

    pub fn populate_font_cache(&self, characters: Vec<char>, size: u16) {
        COMMAND_QUEUE.push(  Command::PopulateFontCache { font: self.clone().into(), characters, size }   );
    }

    /// Sets the FilterMode of this font's texture atlas.
    ///
    pub fn set_filter(&mut self, filter_mode: FilterMode) {
        COMMAND_QUEUE.push(  Command::SetFontFilter { font: self.clone().into(), filter: filter_mode.into() }   );
    }

}


impl From<mq::Font> for Font{
    fn from(value: mq::Font) -> Self {
        Font { font: PArc::new(value) }
    }
}
impl From<Font> for mq::Font{
    fn from(value: Font) -> Self {
        (*value.font).clone()
    }
}


/// World space dimensions of the text, measured by 'measure_text' function
#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug, Default, Clone, Copy)]
pub struct TextDimensions {
    /// Distance from very left to very right of the rasterized text
    #[pyo3(get,set)]
    pub width: f32,
    /// Distance from the bottom to the top of the text.
    #[pyo3(get,set)]
    pub height: f32,
    /// Height offset from the baseline of the text.
    /// "draw_text(.., X, Y, ..)" will be rendered in a "Rect::new(X, Y - dimensions.offset_y, dimensions.width, dimensions.height)"
    /// For reference check "text_measures" example.
    #[pyo3(get,set)]
    pub offset_y: f32,
}

#[gen_stub_pymethods]
#[pymethods]
impl TextDimensions {
    #[new]
    pub fn new(width: f32, height: f32, offset_y: f32) -> Self {
       TextDimensions { width, height, offset_y }
    }
}

impl From<mq::TextDimensions> for TextDimensions{
    fn from(value: mq::TextDimensions) -> Self {
        TextDimensions { width: value.width, height: value.height, offset_y: value.offset_y }
    }
}
impl From<TextDimensions> for mq::TextDimensions{
    fn from(value: TextDimensions) -> Self {
        mq::TextDimensions{ width: value.width, height: value.height, offset_y: value.offset_y  }
    }
}
