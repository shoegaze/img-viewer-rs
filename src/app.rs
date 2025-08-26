use glium::Texture2d;
use glium::backend::glutin;
use glium::buffer::BufferCreationError;
use glium::glutin::surface::WindowSurface;
use glium::texture::RawImage2d;
use image::{DynamicImage, GenericImageView, ImageReader};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

use std::env;
use std::error::Error;
use std::path::PathBuf;

use crate::gfx::render::render;

#[derive(Default)]
pub struct App {
    window: Option<Window>,
    display: Option<glutin::Display<WindowSurface>>,
    image_path: Option<PathBuf>,
    decoded_image: Option<DynamicImage>,
    texture: Option<Texture2d>,
}

impl App {
    pub fn open_image(&mut self, image_path: PathBuf) -> Result<(), Box<dyn Error>> {
        let image = ImageReader::open(&image_path)?;
        let decoded_image = image.decode()?;

        self.image_path = Some(image_path);
        self.decoded_image = Some(decoded_image);

        Ok(())
    }

    fn render(&self) -> Result<(), Box<dyn Error>> {
        let Some(display) = self.display.as_ref() else {
            return Err(Box::from("Uninitialized display"));
        };

        let Some(texture) = self.texture.as_ref() else {
            return Err(Box::from("Uninitialized texture"));
        };

        render(display, texture)?;

        Ok(())
    }

    fn create_window(&mut self, event_loop: &ActiveEventLoop) {
        let image = self.decoded_image.as_ref().unwrap();
        let (width, height) = image.dimensions();
        let title = self.title();

        // set visible to false and wait until image is ready to be rendered:
        //  https://docs.rs/winit/latest/winit/#drawing-on-the-window
        let attributes = Window::default_attributes()
            .with_transparent(false)
            .with_title(title)
            .with_visible(false)
            .with_inner_size(PhysicalSize::new(width, height));

        let (window, display) = glutin::SimpleWindowBuilder::new()
            .set_window_builder(attributes)
            .build(event_loop);

        self.window = Some(window);
        self.display = Some(display);
    }

    fn create_texture(&self, image: &DynamicImage) -> Result<Texture2d, BufferCreationError> {
        let image_dim = image.dimensions();
        let image_rgba8 = image.clone().into_rgba8();
        let image_raw = image_rgba8.into_raw();

        let display = self.display.as_ref().unwrap();
        let image = RawImage2d::from_raw_rgba_reversed(&image_raw, image_dim);
        let texture = Texture2d::new(display, image).unwrap();

        Ok(texture)
    }

    fn title(&self) -> String {
        let image_path = self.image_path.as_ref().unwrap();
        let image_path_title = image_path.file_name().unwrap().display();

        format!("{}: {}", env!("CARGO_PKG_NAME"), image_path_title)
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.create_window(event_loop);

        if self.texture.is_none() {
            let create_texture_result = self.create_texture(self.decoded_image.as_ref().unwrap());

            match create_texture_result {
                Ok(texture) => {
                    self.texture = Some(texture);
                }
                _ => (),
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                let _render_result = self.render();

                let window_ref = self.window.as_ref().unwrap();

                let texture = self.texture.as_ref().unwrap();
                let size = PhysicalSize::new(texture.width(), texture.height());

                window_ref.set_min_inner_size(Some(size));
                window_ref.set_max_inner_size(Some(size));

                window_ref.set_visible(true);
                window_ref.request_redraw();
            }

            _ => (),
        }
    }
}
