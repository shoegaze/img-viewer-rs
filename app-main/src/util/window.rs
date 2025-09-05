use winit::dpi::PhysicalSize;
use winit::window::{Window, WindowAttributes};

use crate::window::image_data::ImageData;

pub fn window_title(image_data: &ImageData) -> String {
    let image_file_name = image_data.path.file_name().unwrap();

    format!("{}: {}", env!("CARGO_PKG_NAME"), image_file_name.display())
}

pub fn initial_window_attributes(title: &str, inner_size: PhysicalSize<u32>) -> WindowAttributes {
    // Set visible to false and wait until image is ready to be rendered:
    //  https://docs.rs/winit/latest/winit/#drawing-on-the-window
    Window::default_attributes()
        .with_transparent(true)
        .with_visible(false)
        .with_resizable(true)
        .with_title(title)
        .with_inner_size(inner_size)
}
