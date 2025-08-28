use glium::backend::glutin;
use glium::buffer::BufferCreationError;
use glium::glutin::surface::WindowSurface;
use glium::texture::RawImage2d;
use glium::{Display, Texture2d};
use image::{DynamicImage, GenericImageView, ImageReader};
use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode::{Backspace, Delete, Tab};
use winit::window::CursorIcon::{Default, Grabbing};
use winit::window::{CursorIcon, WindowId};

use std::collections::VecDeque;
use std::error::Error;
use std::path::PathBuf;

use crate::settings::AppSettings;
use crate::ui::AppUi;
use crate::util::coordinates::Coordinates;
use crate::util::window::{initial_window_attributes, window_title};
use crate::window::event::drag_context::DragContext;
use crate::window::image_data::{ImageData, ImageMeta};
use crate::window::image_window::ImageWindow;
use crate::window::window_handle::WindowHandle;

#[derive(Default)]
pub struct App {
    // Application level
    settings: AppSettings,

    // Window+image managers
    image_windows: Vec<ImageWindow>,
    images_queue: VecDeque<(DynamicImage, ImageData)>,

    // Event contexts
    absolute_cursor_coords: Coordinates,
    drag_context: Option<DragContext>,
}

impl App {
    pub fn from_settings(settings: AppSettings) -> Self {
        let mut app = App::default();

        app.settings = settings;

        app
    }

    pub fn queue_image_open(&mut self, image_path: PathBuf) -> Result<(), Box<dyn Error>> {
        {
            let is_path_in_queued = self
                .images_queue
                .iter()
                .any(|(_, image_data)| image_data.is_path(&image_path));

            if is_path_in_queued {
                return Err(Box::from("Image already queued to render"));
            }
        }

        {
            let is_path_in_windows = self
                .image_windows
                .iter()
                .any(|image_window| image_window.is_path(&image_path));

            if is_path_in_windows {
                return Err(Box::from("Image already open in image window"));
            }
        }

        let image = ImageReader::open(&image_path)?;
        let decoded_image = image.decode()?;
        let (width, height) = decoded_image.dimensions();

        let image_meta = ImageMeta::new(width, height);
        let image_data = ImageData::new(image_path, image_meta);

        self.images_queue.push_back((decoded_image, image_data));

        Ok(())
    }

    fn create_window(
        &mut self,
        event_loop: &ActiveEventLoop,
        image: DynamicImage,
        image_data: ImageData,
    ) -> Result<(), Box<dyn Error>> {
        let title = window_title(&image_data);
        let size = PhysicalSize::from(image_data.meta);
        let attributes = initial_window_attributes(title.as_str(), size);

        let (window, display) = glutin::SimpleWindowBuilder::new()
            .set_window_builder(attributes)
            .build(event_loop);

        let texture = self.create_texture(&display, &image)?;
        let window_handle = WindowHandle::new(window, display);
        let image_window = ImageWindow::new(window_handle, image_data, texture);

        self.image_windows.push(image_window);

        Ok(())
    }

    fn create_texture(
        &self,
        display: &Display<WindowSurface>,
        image: &DynamicImage,
    ) -> Result<Texture2d, BufferCreationError> {
        let image_dim = image.dimensions();
        let image_rgba8 = image.clone().into_rgba8();
        let image_raw = image_rgba8.into_raw();

        let image = RawImage2d::from_raw_rgba_reversed(&image_raw, image_dim);
        let texture = Texture2d::new(display, image).unwrap();

        Ok(texture)
    }

    /// Get a window
    fn get_window(&self, window_id: &WindowId) -> Option<&ImageWindow> {
        self.image_windows
            .iter()
            .find(|&image_window| image_window.is_id(window_id))
    }

    /// Close a window
    fn close_window(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
    ) -> Result<(), String> {
        if self.image_windows.is_empty() {
            return Err("No image windows to close".to_string());
        }

        // Quit application if the last image is closed
        if self.image_windows.len() == 1 {
            event_loop.exit();
        }

        // TODO: Refactor this => `self.remove_image_window(&self, window_id: WindowId)`
        let Some(close_window_index) = self
            .image_windows
            .iter()
            .position(|image_window| image_window.is_id(&window_id))
        else {
            return Err("Window with the provided ID does not exist".to_string());
        };

        self.image_windows.remove(close_window_index);

        self.reset_focus();

        Ok(())
    }

    /// Render all image windows
    fn render_all(&self) -> Result<(), Box<dyn Error>> {
        for window in &self.image_windows {
            window.render()?;
        }

        Ok(())
    }

    /// Apply all settings that may have been updated
    fn apply_settings(&self) {
        for image_window in &self.image_windows {
            // TODO: update window settings
            image_window.update_styles(&self.settings);
        }
    }

    /// Resets focus to the top window
    fn reset_focus(&self) {
        let Some(image_window) = self.image_windows.first() else {
            return;
        };

        image_window.focus();
    }

    /// Get the current focused window
    fn get_focused_window(&self) -> Option<&ImageWindow> {
        self.image_windows
            .iter()
            .find(|&image_window| image_window.inner_window().has_focus())
    }
}

impl AppUi for App {
    fn get_cursor_icon(&self) -> CursorIcon {
        // TODO: Grab if cursor is on the window but not dragging
        if self.drag_context.is_some() {
            Grabbing
        } else {
            Default
        }
    }

    fn ui_poll_close(
        &mut self,
        event: &KeyEvent,
        event_loop: &ActiveEventLoop,
        window_id: &WindowId,
    ) {
        // TODO: Replace with event.logical_key
        let key = event.physical_key;

        if key == Delete || key == Backspace {
            let window_id = window_id.clone();
            let _ = self.close_window(event_loop, window_id);
        }
    }

    fn ui_poll_toggle_decorations(&mut self, event: &KeyEvent, settings: &mut AppSettings) {
        // TODO: Replace with event.logical_key
        let key = event.physical_key;

        if key == Tab {
            settings.toggle_show_title();
        }
    }

    fn ui_window_focus(&self, window_id: &WindowId, state: &ElementState) {
        if !state.is_pressed() {
            return;
        }

        let Some(image_window) = self.get_window(window_id) else {
            return;
        };

        image_window.focus();
    }

    fn ui_update_cursor_context(&mut self, position: &PhysicalPosition<f64>) {
        let Some(focus_window) = self.get_focused_window() else {
            return;
        };

        let Ok(window_outer_coords) = focus_window.get_outer_coordinates() else {
            return;
        };

        let relative_cursor_coords = Coordinates::from(position);
        let absolute_cursor_coords = window_outer_coords + relative_cursor_coords;

        self.absolute_cursor_coords = absolute_cursor_coords;
    }

    fn ui_update_drag_context(&mut self, window_id: &WindowId, state: &ElementState) {
        let Some(focused_window) = self.get_focused_window() else {
            return;
        };

        let Ok(target_coords) = focused_window.get_outer_coordinates() else {
            return;
        };

        self.drag_context = match state.is_pressed() {
            true => {
                focused_window.set_cursor_icon(Grabbing);

                let displacement = self.absolute_cursor_coords - target_coords;
                let drag_context = DragContext::new(window_id.clone(), displacement);

                Some(drag_context)
            }

            false => {
                focused_window.set_cursor_icon(Default);

                None
            }
        };
    }

    fn sync_drag(&self, drag_context: &DragContext) -> Result<(), String> {
        let Some(focus_window) = self.get_focused_window() else {
            return Err("Focus does not exist on any window".to_string());
        };

        // Check that the current window is the drag target
        if !focus_window.is_id(&drag_context.target_window_id) {
            return Err("Focused window does not match target window ID".to_string());
        }

        // position_target' = position_cursor' + displacement
        let target_coords = self.absolute_cursor_coords - drag_context.displacement;

        focus_window.set_outer_coordinates(&target_coords);

        Ok(())
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // TODO: Make async or run in separate thread...?
        while !self.images_queue.is_empty() {
            let (image, image_data) = self.images_queue.pop_front().unwrap();
            let _ = self.create_window(event_loop, image, image_data);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                let _ = self.close_window(event_loop, window_id);
            }

            WindowEvent::RedrawRequested => {
                let _ = self.apply_settings();
                let _ = self.render_all();

                for image_window in &self.image_windows {
                    image_window.request_redraw();
                }
            }

            WindowEvent::Resized(_size) => {
                let Some(focus_window) = self.get_focused_window() else {
                    return;
                };

                let _ = focus_window.render();

                focus_window.request_redraw();
            }

            WindowEvent::CursorMoved { position, .. } => {
                self.ui_update_cursor_context(&position);

                let Some(focus_window) = self.get_focused_window() else {
                    return;
                };

                let cursor_icon = self.get_cursor_icon();
                focus_window.set_cursor_icon(cursor_icon);

                let Some(drag_context) = &self.drag_context else {
                    return;
                };

                let Err(msg) = self.sync_drag(drag_context) else {
                    return;
                };

                eprintln!("Sync drag error: {}", msg);
            }

            WindowEvent::MouseInput { state, .. } => {
                self.ui_window_focus(&window_id, &state);
                self.ui_update_drag_context(&window_id, &state);
            }

            WindowEvent::KeyboardInput {
                event,
                is_synthetic: false,
                ..
            } => {
                if !event.state.is_pressed() {
                    return;
                }

                self.ui_poll_close(&event, &event_loop, &window_id);

                {
                    // Poll settings changes
                    let mut updated_settings = self.settings.clone();

                    self.ui_poll_toggle_decorations(&event, &mut updated_settings);

                    self.settings = updated_settings;
                }
            }

            _ => (),
        }
    }
}
