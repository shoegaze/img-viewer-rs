use glium::Texture2d;
use winit::window::Window;

use std::error::Error;
use std::path::PathBuf;

use crate::gfx::render::render_image;
use crate::window::image_data::ImageData;
use crate::window::window_handle::WindowHandle;

pub struct ImageWindow {
    window_handle: WindowHandle,
    image_data: ImageData,
    texture: Texture2d,
}

impl ImageWindow {
    pub fn new(window_handle: WindowHandle, image_data: ImageData, texture: Texture2d) -> Self {
        ImageWindow {
            window_handle,
            image_data,
            texture,
        }
    }

    pub fn is_path(&self, image_path: &PathBuf) -> bool {
        self.image_data.is_path(image_path)
    }

    pub fn render(&self) -> Result<(), Box<dyn Error>> {
        render_image(&self.window_handle.display, &self.texture)?;

        Ok(())
    }

    pub fn image_data(&self) -> &ImageData {
        &self.image_data
    }

    pub fn inner_window(&self) -> &Window {
        &self.window_handle.window
    }
}

impl PartialEq for ImageWindow {
    fn eq(&self, other: &Self) -> bool {
        self.image_data == other.image_data
    }
}
