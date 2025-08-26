mod app;
mod gfx;

use winit::event_loop::{ControlFlow, EventLoop};

use std::env;
use std::path::PathBuf;

use app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::default();
    let event_loop = EventLoop::new().unwrap();

    {
        // DEBUG: Load test image
        let image_path = PathBuf::from("test.png");

        let current_dir = env::current_dir()?;
        let full_path: PathBuf = [current_dir, image_path].iter().collect();
        let full_path = full_path.canonicalize()?;

        app.open_image(full_path)?;
    }

    {
        // Execute app
        event_loop.set_control_flow(ControlFlow::Wait);
        event_loop
            .run_app(&mut app)
            .expect("EventLoop error occurred");
    }

    Ok(())
}
