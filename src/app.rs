use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};


#[derive(Default)]
pub struct App {
    // TODO: `window: Vector<Window>`
    window: Option<Window>
}


impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = Window::default_attributes();
        let window = event_loop.create_window(attributes).unwrap();

        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
              event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                let window_ref = self.window.as_ref().unwrap();

                window_ref.request_redraw();
            }

            _ => ()
        }
    }
}