
use pyo3::prelude::*;
use pyo3_stub_gen::derive::* ;
use crate::py_abstractions::{Loading::Loading::load_file, structs::{ ThreeDObjects::{ColliderOptions::ColliderOptions, Mesh::Mesh}}};
use crate::py_abstractions::Audio::Sound;
use crate::py_abstractions::Textures_and_Images::{Image,Texture2D};
use crate::py_abstractions::Textures_and_Images;
use crate::py_abstractions::structs::GLAM::Vec3::Vec3;


/// A wrapper around raw filedata that has not yet been parsed.
/// 
/// To load FileData, there are functions like:
/// ```
/// >>>load_file(...)
/// ...
/// >>>download_file(...)
/// ...
/// ```
/// or 'Loading'.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct FileData{

    /// These are the raw bytes of a file.
    /// They likely do not need to be edited manually.
    #[pyo3(get,set)]
    pub bytes: Vec<u8>,
}

#[gen_stub_pymethods]
#[pymethods]
impl FileData{

    /// Loads FileData from file.
    #[new]
    pub fn new(file_path: &str)-> PyResult<Self>{
        load_file(file_path)
    }
    
    /// Loads file data from raw bytes.
    /// This is mostly useful to be compatible with other libraries,
    /// which may return F.E. an audio file as Bytes.
    /// 
    /// For more practical ways to load a file, look into 'Loading'.
    #[staticmethod]
    pub fn from_raw_bytes(bytes: Vec<u8>)-> Self{
        FileData { bytes}
    }

    /// Attempts to parse the file data as an Image.
    pub fn to_Image(&self)-> PyResult<Image>{
        Textures_and_Images::image_from_bytes(&self.bytes)
    }

    /// Attempts to parse the file data as a texture.
    /// FileData -> Image -> Texture2D
    /// 
    pub fn to_2DTexture(&self)-> PyResult<Texture2D>{
        let image = Textures_and_Images::image_from_bytes(&self.bytes)?;
        Texture2D::new(image)
    }

    /// Attempts to parse the file data as a Sound.
    pub fn to_Sound(&self)-> PyResult<Sound>{
        Sound::load_sound_from_bytes(self.bytes.clone())
    }

    /// Attempts to parse the file as a mesh.
    /// Immediately returns a fully fledged Mesh object that has collision, is queued to be drawn,
    /// and is positioned at 0,0,0
    #[pyo3(signature = (texture=None,collider_type = ColliderOptions::NONE()))]
    pub fn to_mesh_data(&self, py: Python<'_>, texture: Option<Texture2D>,collider_type: ColliderOptions)-> PyResult<Py<Mesh>>{
        Mesh::from_file_data(py, self.clone(),texture,collider_type)
    }
    
}
