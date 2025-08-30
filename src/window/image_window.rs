use glium::Texture2d;
use winit::error::NotSupportedError;
use winit::window::{CursorIcon, Window, WindowId};

use crate::gfx::render::render_image;
use crate::settings::AppSettings;
use crate::util::coordinates::Coordinates;
use crate::window::image_data::ImageData;
use crate::window::window_handle::WindowHandle;
use std::error::Error;
use std::path::PathBuf;
use winit::dpi::PhysicalSize;

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

    pub fn is_id(&self, window_id: &WindowId) -> bool {
        self.inner_window().id() == *window_id
    }

    pub fn is_path(&self, image_path: &PathBuf) -> bool {
        self.image_data.is_path(image_path)
    }

    pub fn inner_window(&self) -> &Window {
        &self.window_handle.window
    }

    pub fn render(&self) -> Result<(), Box<dyn Error>> {
        render_image(&self.window_handle.display, &self.texture)?;

        Ok(())
    }

    pub fn request_redraw(&self) {
        let inner_window = self.inner_window();

        match inner_window.is_visible() {
            Some(false) => {
                inner_window.set_visible(true);
                self.focus();
            }
            _ => (),
        }

        inner_window.request_redraw();
    }

    pub fn focus(&self) {
        self.inner_window().focus_window();
    }

    pub fn update_styles(&self, settings: &AppSettings) {
        self.inner_window().set_decorations(settings.show_title);
    }

    pub fn set_cursor_icon(&self, icon: CursorIcon) {
        self.inner_window().set_cursor(icon);
    }

    pub fn get_outer_coordinates(&self) -> Result<Coordinates, NotSupportedError> {
        let outer_position = self.inner_window().outer_position()?;
        let outer_coordinates = Coordinates::from(&outer_position);

        Ok(outer_coordinates)
    }

    pub fn set_outer_coordinates(&self, coordinates: &Coordinates) {
        let outer_position = coordinates.to_physical();

        self.inner_window().set_outer_position(outer_position);
    }

    // TODO: Keep window center the same
    pub fn reset_size(&self) {
        let inner_window = self.inner_window();
        let original_dim = self.image_data.meta.dimensions();

        let _ = inner_window.request_inner_size(PhysicalSize::<u32>::from(original_dim));
    }
}

impl PartialEq for ImageWindow {
    fn eq(&self, other: &Self) -> bool {
        self.image_data == other.image_data
    }
}
