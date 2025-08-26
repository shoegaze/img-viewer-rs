mod app;
mod cli;
mod gfx;

use clap::Parser;
use winit::event_loop::{ControlFlow, EventLoop};

use std::env;
use std::error::Error;
use std::path::PathBuf;

use app::App;
use cli::args::Args;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse().validated()?;

    let mut app = App::default();
    let event_loop = EventLoop::new().unwrap();

    {
        let image_path = args.path;

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
