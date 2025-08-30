use glium::Display;
use glium::glutin::surface::WindowSurface;
use winit::window::Window;

pub struct WindowHandle {
    pub window: Window,
    pub display: Display<WindowSurface>,
}

impl WindowHandle {
    pub fn new(window: Window, display: Display<WindowSurface>) -> Self {
        WindowHandle { window, display }
    }
}
