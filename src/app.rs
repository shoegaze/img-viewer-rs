use glium::backend::glutin;
use glium::buffer::BufferCreationError;
use glium::glutin::surface::WindowSurface;
use glium::texture::RawImage2d;
use glium::{Display, Texture2d};
use image::{DynamicImage, GenericImageView, ImageReader};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode::Tab;
use winit::window::{Window, WindowAttributes, WindowId};

use std::collections::VecDeque;
use std::env;
use std::error::Error;
use std::path::PathBuf;

use crate::config::AppConfig;
use crate::window::image_data::{ImageData, ImageMeta};
use crate::window::image_window::ImageWindow;
use crate::window::window_handle::WindowHandle;

#[derive(Default)]
pub struct App {
    config: AppConfig,
    image_windows: Vec<ImageWindow>,
    queued: VecDeque<(DynamicImage, ImageData)>,
}

impl App {
    pub fn from_config(config: AppConfig) -> Self {
        let mut app = App::default();

        app.config = config;

        app
    }

    pub fn queue_open_image(&mut self, image_path: PathBuf) -> Result<(), Box<dyn Error>> {
        {
            let is_path_in_queued = self
                .queued
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

        let image_meta = ImageMeta::new(decoded_image.width(), decoded_image.height());
        let image_data = ImageData::new(image_path, image_meta);

        self.queued.push_back((decoded_image, image_data));

        Ok(())
    }

    fn render_all(&self) -> Result<(), Box<dyn Error>> {
        for window in &self.image_windows {
            window.render()?;
        }

        Ok(())
    }

    fn create_window(
        &mut self,
        event_loop: &ActiveEventLoop,
        image: DynamicImage,
        image_data: ImageData,
    ) -> Result<(), Box<dyn Error>> {
        let (width, height) = image_data.meta.dimensions();
        let title = self.title_from_image_path(&image_data);

        let attributes =
            self.initial_window_attributes(title.as_str(), PhysicalSize::new(width, height));

        let (window, display) = glutin::SimpleWindowBuilder::new()
            .set_window_builder(attributes)
            .build(event_loop);

        let texture = self.create_texture(&display, &image)?;
        let window_handle = WindowHandle::new(window, display);

        let image_window = ImageWindow::new(window_handle, image_data, texture);

        self.image_windows.push(image_window);

        Ok(())
    }

    fn get_window(&self, window_id: WindowId) -> Option<&ImageWindow> {
        self.image_windows
            .iter()
            .find(|&image_window| image_window.inner_window().id() == window_id)
    }

    fn close_window_from_id(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId) {
        if self.image_windows.is_empty() {
            return;
        }

        // Quit application if the last image is closed
        // TODO?: Check if window_id matches image_windows[0]...id()
        if self.image_windows.len() == 1 {
            event_loop.exit();
        }

        let close_window_index = self
            .image_windows
            .iter()
            .map(|image_window| image_window.inner_window())
            .position(|inner_window| inner_window.id() == window_id)
            .unwrap();

        self.image_windows.remove(close_window_index);
    }

    fn initial_window_attributes(
        &self,
        title: &str,
        inner_size: PhysicalSize<u32>,
    ) -> WindowAttributes {
        // Set visible to false and wait until image is ready to be rendered:
        //  https://docs.rs/winit/latest/winit/#drawing-on-the-window
        Window::default_attributes()
            .with_transparent(false)
            .with_title(title)
            .with_visible(false)
            .with_inner_size(inner_size)
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

    fn title_from_image_path(&self, image_data: &ImageData) -> String {
        let image_file_name = image_data.path.file_name().unwrap();

        format!("{}: {}", env!("CARGO_PKG_NAME"), image_file_name.display())
    }

    fn apply_config(&mut self) {
        for image_window in &self.image_windows {
            let inner_window = image_window.inner_window();

            inner_window.set_decorations(self.config.show_title);
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // TODO: Make async or run in separate thread...?
        while !self.queued.is_empty() {
            let (image, image_data) = self.queued.pop_front().unwrap();
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
                self.close_window_from_id(event_loop, window_id);
            }

            WindowEvent::RedrawRequested => {
                let _ = self.apply_config();
                let _ = self.render_all();

                for image_window in &self.image_windows {
                    let inner_window = image_window.inner_window();
                    let image_data = image_window.image_data();

                    {
                        // Apply window transforms
                        let (width, height) = image_data.meta.dimensions();
                        let size = PhysicalSize::new(width, height);
                        inner_window.set_min_inner_size(Some(size));
                        inner_window.set_max_inner_size(Some(size));
                    }

                    match inner_window.is_visible() {
                        Some(false) => inner_window.set_visible(true),
                        _ => (),
                    }

                    inner_window.request_redraw();
                }
            }

            WindowEvent::MouseInput { state, .. } => {
                if !state.is_pressed() {
                    return;
                }

                let Some(image_window) = self.get_window(window_id) else {
                    return;
                };

                image_window.inner_window().focus_window();
            }

            WindowEvent::KeyboardInput {
                event,
                is_synthetic: false,
                ..
            } => {
                if !event.state.is_pressed() {
                    return;
                }

                // TODO: Replace with event.logical_key
                let key = event.physical_key;

                let mut updated_config = self.config.clone();

                if key == Tab {
                    updated_config.toggle_show_title();
                }

                self.config = updated_config;
            }

            _ => (),
        }
    }
}
