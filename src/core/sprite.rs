use std::path::PathBuf;
use image::RgbaImage;

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
    
    pub fn build(&self, path: impl Into<PathBuf>) {
        self.image.save(path.into()).unwrap();
    }
}