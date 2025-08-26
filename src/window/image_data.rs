use std::path::PathBuf;

#[derive(Default, Copy, Clone)]
pub struct ImageMeta {
    width: u32,
    height: u32,
}

impl ImageMeta {
    pub fn new(width: u32, height: u32) -> Self {
        ImageMeta { width, height }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

#[derive(Clone)]
pub struct ImageData {
    pub path: PathBuf,
    pub meta: ImageMeta,
}

impl ImageData {
    pub fn new(image_path: PathBuf, image_meta: ImageMeta) -> Self {
        ImageData {
            path: image_path,
            meta: image_meta,
        }
    }

    pub fn is_path(&self, image_path: &PathBuf) -> bool {
        &self.path == image_path
    }
}

impl PartialEq for ImageData {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}
