use pyo3::prelude::*;
 
use macroquad::prelude as mq;
use std::sync::Arc;

use crate::engine::PChannel::PChannel;
use crate::engine::CoreLoop::COMMAND_QUEUE;
use crate::engine::CoreLoop::Command;
use pyo3::exceptions::PyValueError;use pyo3_stub_gen::derive::*;
use crate::py_abstractions::Color::*;
use crate::engine::PArc::PArc;


use image::ImageReader as ImageReader;
use std::io::Cursor;

/// Image, data stored in CPU memory
#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq,)]
pub struct Image {
    #[pyo3(get, set)]
    pub bytes: Vec<u8>, 

    #[pyo3(get, set)]
    pub width: u16, 

    #[pyo3(get, set)]
    pub height: u16,
}

#[gen_stub_pymethods]
#[pymethods]
impl Image {

    #[new]
    pub fn new(path: String) -> PyResult<Image> {
        // Load file bytes via your Python-callable function
        let data = crate::py_abstractions::Loading::Loading::load_file(&path)?;

        // Wrap bytes for image::Reader
        let cursor = Cursor::new(data.bytes);
        let reader = ImageReader::new(cursor)
            .with_guessed_format()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to guess format: {e}")))?;

        // Decode image
        let image = reader
            .decode()
            .map_err(|e|  PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Decode error: {e}")))?;

        // Convert to RGBA8 and get raw bytes
        let rgba = image.to_rgba8();
        let (width, height) = rgba.dimensions();

        Ok(Image {
            bytes: rgba.into_raw(),
            width: width as u16,
            height: height as u16,
        })
    }
    
    #[staticmethod]
    pub fn empty() -> Image {
        Image {
            width: 0,
            height: 0,
            bytes: vec![],
        }
    }

    /// Creates an Image filled with the provided [Color].
    #[staticmethod]
    pub fn gen_image_color(width: u16, height: u16, color: Color) -> Image {
        let mut bytes = vec![0; width as usize * height as usize * 4];
        for i in 0..width as usize * height as usize {
            bytes[i * 4] = (color.r * 255.) as u8;
            bytes[i * 4 + 1] = (color.g * 255.) as u8;
            bytes[i * 4 + 2] = (color.b * 255.) as u8;
            bytes[i * 4 + 3] = (color.a * 255.) as u8;
        }
        Image {
            width,
            height,
            bytes,
        }
    }

    /// Updates this image from a slice of [Color]s.
    pub fn update(&mut self, colors: Vec<Color>) {
        assert!(self.width as usize * self.height as usize == colors.len());

        for i in 0..colors.len() {
            self.bytes[i * 4] = (colors[i].r * 255.) as u8;
            self.bytes[i * 4 + 1] = (colors[i].g * 255.) as u8;
            self.bytes[i * 4 + 2] = (colors[i].b * 255.) as u8;
            self.bytes[i * 4 + 3] = (colors[i].a * 255.) as u8;
        }
    }

    /// Returns this image's data as a slice of 4-byte arrays.
    pub fn get_image_data(&self) -> Vec<[u8; 4]> {
        assert!(self.width as usize * self.height as usize * 4 == self.bytes.len());

        // SAFETY: We're converting a slice of u8 to a slice of [u8; 4].
        // This is safe because we have asserted that the total number of bytes
        // is a multiple of 4, and the memory layout of u8 is the same as [u8; 4]
        // for alignment purposes.
        let image_slice = unsafe {
            std::slice::from_raw_parts(
                self.bytes.as_ptr() as *const [u8; 4],
                self.width as usize * self.height as usize,
            )
        };

        image_slice.to_vec()
    }

    /// Modifies a pixel's color in this image.
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        // Assert that the x and y coordinates are within the image boundaries.
        assert!(x < self.width as u32);
        assert!(y < self.height as u32);

        // Calculate the starting byte index for the given pixel.
        // Each pixel takes up 4 bytes (r, g, b, a).
        let index = (y * self.width as u32 + x) as usize * 4;

        // Convert the float color values (assuming a 0.0 to 1.0 range) to u8 (0 to 255).
        // The cast truncates the fractional part, so we multiply by 255.0 first.
        let r = (color.r * 255.0) as u8;
        let g = (color.g * 255.0) as u8;
        let b = (color.b * 255.0) as u8;
        let a = (color.a * 255.0) as u8;

        // Set the individual byte values in the underlying 'bytes' vector.
        self.bytes[index] = r;
        self.bytes[index + 1] = g;
        self.bytes[index + 2] = b;
        self.bytes[index + 3] = a;
    }

    /// Returns a pixel [Color] from this image.
    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        assert!(x < self.width as u32);
        assert!(y < self.height as u32);

        let index = (y * self.width as u32+ x) as usize * 4;
        
        let r = self.bytes[index];
        let g = self.bytes[index + 1];
        let b = self.bytes[index + 2];
        let a = self.bytes[index + 3];
        
        Color {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    /// Creates an image from a given file.
    /// 
    /// supported image formats are: ".png", ".jpeg"
    #[staticmethod]
    pub fn from_bytes(bytes: Vec<u8>, width: u16, height: u16, ) -> PyResult<Self> {
        if bytes.len() != (width as usize) * (height as usize) * 4 {
            return Err(PyErr::new::<PyValueError, _>(
                "Invalid image data size: expected width * height * 4 bytes",
            ));
        }
        
        Ok(Self { bytes, width, height })
    }

    
    
}
pub fn image_from_bytes(bytes: &Vec<u8>)-> PyResult<Image>{
    let cursor = Cursor::new(bytes);
        let reader = ImageReader::new(cursor)
            .with_guessed_format()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to guess format: {e}")))?;

        // Decode image
        let image = reader
            .decode()
            .map_err(|e|  PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Decode error: {e}")))?;

        // Convert to RGBA8 and get raw bytes
        let rgba = image.to_rgba8();
        let (width, height) = rgba.dimensions();

        Ok(Image {
            bytes: rgba.into_raw(),
            width: width as u16,
            height: height as u16,
        })
}

/// Texture, data stored in GPU memory
#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone, Debug, PartialEq)]
pub struct Texture2D {
   pub texture: PArc<mq::Texture2D>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Texture2D {
   
    #[new]
    pub fn new(image: Image) -> PyResult<Texture2D> {
        
        let inner_im = mq::Image {
            bytes: image.bytes,
            width: image.width,
            height: image.height,
        };
        let imagePointer = Arc::new(inner_im);
        
        let (sender, receiver) = PChannel::sync_channel(1);
        
        let command = Command::ImgToTexture {
            image: imagePointer.clone(),
            sender,
        };

        COMMAND_QUEUE.push(command);

        let mq_texture = receiver.recv()?;
        let ourTexture=  Texture2D{
            texture: mq_texture,
        };
        Ok(ourTexture)
        
    }

}



use std::ops::Deref;

impl Deref for Texture2D {
    type Target = mq::Texture2D;

    fn deref(&self) -> &Self::Target {
        &self.texture
    }
}



impl From<mq::Texture2D> for Texture2D {
    fn from(t: mq::Texture2D) -> Self {
        Texture2D { texture: PArc::new(  t  ) }
    }
}


impl From<Texture2D> for mq::Texture2D {
    fn from(t: Texture2D) -> Self {
        (*t.texture).clone()
    }
}
