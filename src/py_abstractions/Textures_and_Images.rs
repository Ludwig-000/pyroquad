
use pyo3::prelude::*;
 
use macroquad::prelude as mq;
use std::sync::Arc;

use crate::engine::PChannel::PChannel;
use crate::engine::CoreLoop::COMMAND_QUEUE;
use crate::engine::CoreLoop::Command;
use crate::engine::PChannel::PSender;
use crate::py_abstractions::structs::GLAM::Vec2::Vec2;
use crate::py_assert;
use pyo3::exceptions::PyValueError;
use crate::py_abstractions::Color::*;
use crate::engine::PArc::PArc;


use image::ImageReader as ImageReader;
use std::io::Cursor;

/// Image, data stored in CPU memory
#[pyclass(from_py_object)]
#[derive(Debug, Clone, PartialEq, Eq,)]
pub struct Image {
    #[pyo3(get, set)]
    pub bytes: Vec<u8>, 

    #[pyo3(get, set)]
    pub width: u16, 

    #[pyo3(get, set)]
    pub height: u16,
}

#[pymethods]
impl Image {

    /// Creates an image from a given file path.
    /// supported image formats are: ".png", ".jpeg", ".webp"
    #[new]
    pub fn new(path: &str) -> PyResult<Image> {
        let data = crate::py_abstractions::Loading::Loading::load_file(path)?;

        let cursor = Cursor::new(data.bytes);
        let reader = ImageReader::new(cursor)
            .with_guessed_format()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to guess format: {e}")))?;

        let image = reader
            .decode()
            .map_err(|e|  PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Decode error: {e}")))?;

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
    pub fn update(&mut self, colors: Vec<Color>) -> PyResult<()>{
        py_assert!(self.width as usize * self.height as usize == colors.len());

        for i in 0..colors.len() {
            self.bytes[i * 4] = (colors[i].r * 255.) as u8;
            self.bytes[i * 4 + 1] = (colors[i].g * 255.) as u8;
            self.bytes[i * 4 + 2] = (colors[i].b * 255.) as u8;
            self.bytes[i * 4 + 3] = (colors[i].a * 255.) as u8;
        }
        Ok(())
    }

    /// Returns this image's data as a slice of 4-byte arrays.
    pub fn get_image_data(&self) ->  Vec<[u8; 4]> {
        use std::slice;

        let r =unsafe {
            slice::from_raw_parts(
                self.bytes.as_ptr() as *const [u8; 4],
                self.width as usize * self.height as usize,
            )
        };
        r.to_vec()
    }

    /// Modifies a pixel's color in this image.
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) -> PyResult<()>{
        py_assert!(x < self.width as u32 );
        py_assert!(y < self.height as u32);

        let index = (y * self.width as u32 + x) as usize * 4;

        let r = (color.r * 255.0) as u8;
        let g = (color.g * 255.0) as u8;
        let b = (color.b * 255.0) as u8;
        let a = (color.a * 255.0) as u8;

        self.bytes[index] = r;
        self.bytes[index + 1] = g;
        self.bytes[index + 2] = b;
        self.bytes[index + 3] = a;
        Ok(())
    }

    /// Returns a pixel [Color] from this image.
    pub fn get_pixel(&self, x: u32, y: u32) -> PyResult<Color> {
        py_assert!(x < self.width as u32);
        py_assert!(y < self.height as u32);

        let index = (y * self.width as u32+ x) as usize * 4;
        
        let r = self.bytes[index];
        let g = self.bytes[index + 1];
        let b = self.bytes[index + 2];
        let a = self.bytes[index + 3];
        
        Ok(Color {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        })
    }

    /// Creates an image from a given file.
    /// 
    /// supported image formats are: ".png", ".jpeg", ".webp"
    #[staticmethod]
    pub fn from_bytes(bytes: Vec<u8>, width: u16, height: u16, ) -> PyResult<Self> {
        if bytes.len() != (width as usize) * (height as usize) * 4 {
            return Err(PyErr::new::<PyValueError, _>(
                "Invalid image data size: expected width * height * 4 bytes",
            ));
        }
        
        Ok(Self { bytes, width, height })
    }

    /// Flip the image horizontally (mirror left-right)
    pub fn flip_horizontal(&mut self) -> PyResult<()>{
        let w = self.width as usize;
        let h = self.height as usize;
        py_assert!((w * h * 4) == self.bytes.len());

        for y in 0..h {
            for x in 0..w / 2 {
                let left = (y * w + x) * 4;
                let right = (y * w + (w - 1 - x)) * 4;
                self.bytes.swap(left, right);
                self.bytes.swap(left + 1, right + 1);
                self.bytes.swap(left + 2, right + 2);
                self.bytes.swap(left + 3, right + 3);
            }
        }
        Ok(())
    }

    /// Flip the image vertically (mirror top-bottom)
    pub fn flip_vertical(&mut self) -> PyResult<()>{
        let w = self.width as usize;
        let h = self.height as usize;
        py_assert!((w * h * 4) == self.bytes.len());
        
        for y in 0..h / 2 {
            let top_row = y * w * 4;
            let bottom_row = (h - 1 - y) * w * 4;
            for x in 0..w * 4 {
                self.bytes.swap(top_row + x, bottom_row + x);
            }
        }
        Ok(())
    }

    pub fn to_texture(&self)-> PyResult<Texture2D>{
        Texture2D::new(self.clone())
    }

    /// Creates a slice from a an image.
    /// The slice is top_left (x1, y1), bottom_right (x2, y2). (absolute coordinates in pixels)
    /// This is very useful when working with tile maps
    pub fn slice(&self, x1: u32, y1: u32, x2: u32, y2: u32) -> PyResult<Image>{
        py_assert!( x1 < x2 && y1 < y2, "top_left must be smaller than bottom_right");
        py_assert!( x2 <= self.width as u32 && y2 <= self.height as u32, "slice may not exceed image width or height");

        let bpp = 4;
        let bytes: Vec<u8> = (y1..y2)
            .flat_map(|y| {
                let start = ((y * self.width as u32 + x1) * bpp) as usize;
                let end = ((y * self.width as u32 + x2) * bpp) as usize;
                &self.bytes[start..end]
            })
            .copied()
            .collect();
        
        Ok(Image{
            width: (x2 - x1) as u16,
            bytes: bytes,
            height: (y2 - y1) as u16,
        })
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


impl From<mq::Image> for Image {
    fn from(t: mq::Image) -> Self {
        Image { bytes: t.bytes, width: t.width, height: t.height }
    }
}


impl From<Image> for mq::Image {
    fn from(t: Image) -> Self {
        mq::Image { bytes: t.bytes, width: t.width, height: t.height }
    }
}




/// Texture, data stored in GPU memory
#[pyclass(from_py_object)]
#[derive(Clone, Debug, PartialEq)]
pub struct Texture2D {
   pub texture: PArc<mq::Texture2D>,
}

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
        
        let (sender, receiver) = PChannel::channel();
        
        let command = 
            Command::TexImEnum(EngineTexImgEnum::TextureImageToTexture { im: imagePointer.clone(), sender});

        COMMAND_QUEUE.push(command);

        let mq_texture = receiver.recv()?;
        let ourTexture=  Texture2D{
            texture: PArc::new(mq_texture),
        };
        Ok(ourTexture)
        
    }


    #[staticmethod]
    pub fn empty()-> PyResult<Texture2D>{
        let (sy,rx) = PChannel::channel();
        let c = EngineTexImgEnum::TextureEmpty(sy);
        COMMAND_QUEUE.push(Command::TexImEnum(c));
        let res =rx.recv()?;
        Ok(res.into())
    }

    /// Creates a Texture2D from a slice of bytes in an R,G,B,A sequence,
    /// with the given width and height.
    ///
    /// # Example
    ///
    /// ```
    /// bytes = [255, 0, 0, 192, 0, 255, 0, 192, 0, 0, 255, 192, 255, 255, 255, 192]
    /// texture = Texture2D.from_rgba8(2, 2, bytes)
    /// ```
    #[staticmethod]
    pub fn from_rgba8(width: u16, height: u16, bytes: Vec<u8>)-> PyResult<Texture2D>{
        let (sy,rx) = PChannel::channel();
        let c = EngineTexImgEnum::TextureFromRGBA { width, height, bytes, sender: sy };
        COMMAND_QUEUE.push(Command::TexImEnum(c));
        let res =rx.recv()?;
        Ok(res.into())
    }

    /// Uploads [Image] data to this texture.
    pub fn update(&self, image: Image) {
        let c = EngineTexImgEnum::TextureUpdate {  tex: (*self.texture).clone(), im: image.into() };
        COMMAND_QUEUE.push(Command::TexImEnum(c));
    }

    // Updates the texture from an array of bytes.
    pub fn update_from_bytes(&self, width: u32, height: u32, bytes: Vec<u8>) {
        let c = EngineTexImgEnum::TextureUpdateFromBytes { width, height, bytes, tex: (*self.texture).clone() };
        COMMAND_QUEUE.push(Command::TexImEnum(c));
    }

    /// Uploads [Image] data to part of this texture.
    pub fn update_part(
        &self,
        image: Image,
        x_offset: i32,
        y_offset: i32,
        width: i32,
        height: i32,
    ) {
        let c = EngineTexImgEnum::TextureUpdatePart { 
            image: image.into(), x_offset, y_offset, width, height, tex: (*self.texture).clone() };
        COMMAND_QUEUE.push(Command::TexImEnum(c));
    }

    /// Returns the width of this texture.
    pub fn width(&self) -> PyResult<f32> {
        let (sy,rx) = PChannel::channel();
        let c = EngineTexImgEnum::TextureWidth { tex: (*self.texture).clone(), sender: sy };
        COMMAND_QUEUE.push(Command::TexImEnum(c));
        Ok(rx.recv()?)
    }

    /// Returns the height of this texture.
    pub fn height(&self) -> PyResult<f32> {
        let (sy,rx) = PChannel::channel();
        let c = EngineTexImgEnum::TextureHeight{ tex: (*self.texture).clone(), sender: sy };
        COMMAND_QUEUE.push(Command::TexImEnum(c));
        Ok(rx.recv()?)
    }
    /// Returns the size of this texture
    pub fn size(&self) -> PyResult<Vec2> {
        let (sy,rx) = PChannel::channel();
        let c = EngineTexImgEnum::TextureSize{ tex: (*self.texture).clone(), sender: sy };
        COMMAND_QUEUE.push(Command::TexImEnum(c));
        Ok(rx.recv()?.into())
    }

    pub fn set_filter(&mut self, filter_mode: FilterMode){
        let filter = match filter_mode{
            FilterMode::Linear => mq::FilterMode::Linear,
            FilterMode::Nearest => mq::FilterMode::Nearest,
        };
        let c = EngineTexImgEnum::TextureSetFilter { filter, tex: (*self.texture).clone() };
        COMMAND_QUEUE.push(Command::TexImEnum(c));
    }


    /// Updates this texture from the screen.
    pub fn grab_screen(&self) {
        let c = EngineTexImgEnum::TextureGrabScreen((*self.texture).clone());
        COMMAND_QUEUE.push(Command::TexImEnum(c));
    }

    /// Returns an [Image] from the pixel data in this texture.
    ///
    /// This operation can be expensive.
    pub fn get_texture_data(&self) -> PyResult<Image> {
        let (sy,rx) = PChannel::channel();
        let c = EngineTexImgEnum::TextureGetTexData{ tex: (*self.texture).clone(), sender: sy };
        COMMAND_QUEUE.push(Command::TexImEnum(c));
        let res =rx.recv()?;
        Ok(res.into())
    }

}


#[pyclass(from_py_object)]
#[derive(Clone, Copy)]
pub enum FilterMode{
    Nearest,
    Linear
}
impl From<mq::FilterMode> for FilterMode{
    fn from(value: mq::FilterMode) -> Self {
        match value {
            mq::FilterMode::Linear => FilterMode::Linear,
            mq::FilterMode::Nearest => FilterMode::Nearest,
        }
    }
}
impl From<FilterMode> for mq::FilterMode{
    fn from(value: FilterMode) -> Self {
        match value{
            FilterMode::Linear => mq::FilterMode::Linear,
            FilterMode::Nearest => mq::FilterMode::Nearest,
        }
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




pub enum EngineTexImgEnum{
    TextureImageToTexture{
        im: Arc<mq::Image>,
        sender: PSender<mq::Texture2D>
    },
    TextureEmpty(PSender<mq::Texture2D>),
    TextureFromRGBA{
        width: u16, height: u16, bytes: Vec<u8>,
        sender: PSender<mq::Texture2D>
    },
    TextureUpdate{
        tex: mq::Texture2D,
        im: mq::Image
    },
    TextureUpdateFromBytes{
        width: u32, height: u32, bytes: Vec<u8>,
        tex: mq::Texture2D,
    },
    TextureUpdatePart{
        image: mq::Image,
        x_offset: i32,
        y_offset: i32,
        width: i32,
        height: i32,
        tex: mq::Texture2D
    },
    TextureWidth{
        tex: mq::Texture2D,
        sender: PSender<f32>,
    },
    TextureHeight{
        tex: mq::Texture2D,
        sender: PSender<f32>,
    },
    TextureSize{
        tex: mq::Texture2D,
        sender: PSender<mq::Vec2>,
    },
    TextureSetFilter{
        filter: mq::FilterMode,
        tex: mq::Texture2D,
    },
    TextureGrabScreen(mq::Texture2D),
    TextureGetTexData{
        tex: mq::Texture2D,
        sender: PSender<mq::Image>
    }

}
impl EngineTexImgEnum{
    pub fn execute(self){
        match self{ 
            Self::TextureEmpty(sender)=> {
                let _ = sender.send(mq::Texture2D::empty());
            },
            Self::TextureImageToTexture { im, sender }=>{
                let _  = sender.send( mq::Texture2D::from_image(&im) );
            }
            Self::TextureFromRGBA { width, height, bytes, sender }=>{
                let _ = sender.send( mq::Texture2D::from_rgba8(width, height, &bytes));
            }
            Self::TextureUpdate { tex, im }=>{
                tex.update(&im);
            }
            Self::TextureUpdateFromBytes { width, height, bytes, tex }=>{
                tex.update_from_bytes(width, height, &bytes);
            }
            Self::TextureUpdatePart { image, x_offset, y_offset, width, height, tex }=>{
                tex.update_part(&image, x_offset, y_offset, width, height);
            }
            
            Self::TextureWidth { tex, sender } => {
                let _ = sender.send(tex.width());
            }
            Self::TextureHeight { tex, sender } => {
                let _ = sender.send(tex.height());
            }
            Self::TextureSize { tex, sender } => {
                let _ = sender.send(tex.size());
            }
            Self::TextureSetFilter { filter, tex } => {
                tex.set_filter(filter);
            }
            Self::TextureGrabScreen(tex) => {
                tex.grab_screen();
            }
            Self::TextureGetTexData { tex, sender } => {
                let _ = sender.send(tex.get_texture_data());
            }
        }
    }
}




/// TODO: doesnt seem to work atm, find out why.
/// Build an atlas out of all currently loaded texture
/// Later on all draw_texture calls with texture available in the atlas will use
/// the one from the atlas
/// NOTE: the GPU memory and texture itself in Texture2D will still be allocated
/// and Texture->Image conversions will work with Texture2D content, not the atlas
#[pyfunction]
pub fn build_texture_atlas(){
    COMMAND_QUEUE.push( Command::BuildTextureAtlas );
}