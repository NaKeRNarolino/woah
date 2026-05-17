use std::path::PathBuf;
use image::{Rgba, RgbaImage};
use image::buffer::ConvertBuffer;

/// A struct for describing textures.
#[derive(Clone, Debug)]
pub struct Sprite {
    image: RgbaImage,
}

impl Sprite {
    pub fn read(path: impl Into<PathBuf>) -> Self {
        Self {
            image: image::open(path.into()).unwrap().to_rgba8()
        }
    }
    
    pub fn accept(&mut self, cl: fn(u32, u32, Rgba<u8>) -> Rgba<u8>) {
        for (x, y, color) in self.image.enumerate_pixels_mut() {
            *color = cl(x, y, *color);
        }
    }
    
    pub fn build(&self, path: impl Into<PathBuf>) {
        self.image.save(path.into()).unwrap();
    }
}