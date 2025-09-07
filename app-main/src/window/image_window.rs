use glium::Texture2d;
use winit::dpi::PhysicalSize;
use winit::error::NotSupportedError;
use winit::window::{CursorIcon, Window, WindowId};

use std::error::Error;
use std::path::PathBuf;

use crate::gfx::render::render_image;
use crate::settings::AppSettings;
use crate::util::vec2::Vec2;
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

        let inner_window = self.inner_window();
        inner_window.pre_present_notify();

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
        // TODO: Do focus indication animation
        self.inner_window().focus_window();
    }

    pub fn update_styles(&self, settings: &AppSettings) {
        self.inner_window().set_decorations(settings.show_title);
    }

    pub fn set_cursor_icon(&self, icon: CursorIcon) {
        self.inner_window().set_cursor(icon);
    }

    pub fn get_outer_pos(&self) -> Result<Vec2, NotSupportedError> {
        let outer_position = self.inner_window().outer_position()?;
        let outer_coordinates = Vec2::from(&outer_position);

        Ok(outer_coordinates)
    }

    pub fn set_outer_pos(&self, coordinates: &Vec2) {
        let outer_position = coordinates.to_physical_position();

        self.inner_window().set_outer_position(outer_position);
    }

    pub fn reset_size(&self) -> Result<(), Box<dyn Error>> {
        let inner_window = self.inner_window();
        let original_dim = self.image_data.meta.dimensions();

        let original_outer_pos = inner_window.outer_position()?;
        let original_outer_size = inner_window.outer_size();
        let original_inner_size = inner_window.inner_size();

        let decoration_size = Vec2::from(original_outer_size) - Vec2::from(original_inner_size);
        let center = Vec2::from(original_outer_pos) + Vec2::from(original_outer_size).half();

        {
            // Update inner size
            let original_dim = PhysicalSize::<u32>::from(original_dim);
            let _ = inner_window.request_inner_size(original_dim);

            // Keep the window center the same as the previous size
            let new_inner_size = Vec2::from(original_dim);
            let new_outer_size = new_inner_size + decoration_size;
            let new_outer_pos = center - Vec2::from(new_outer_size).half();

            inner_window.set_outer_position(new_outer_pos.to_physical_position());
        }

        self.apply_size();

        Ok(())
    }

    pub fn apply_size(&self) {
        let inner_window = self.inner_window();
        let inner_size = inner_window.inner_size();
        let inner_size = (inner_size.width, inner_size.height);

        // Sync OpenGL surface size
        self.window_handle.display.resize(inner_size);
    }
}

impl PartialEq for ImageWindow {
    fn eq(&self, other: &Self) -> bool {
        self.image_data == other.image_data
    }
}
