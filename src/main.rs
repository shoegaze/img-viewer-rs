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

    open_image_from_path(&mut app, args.path)?;

    // Execute app
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop
        .run_app(&mut app)
        .expect("EventLoop error occurred");

    Ok(())
}

fn open_image_from_path(app: &mut App, path: PathBuf) -> Result<(), Box<dyn Error>> {
    let canonical_path = to_canonical_path(&path)?;

    app.open_image(canonical_path)?;

    Ok(())
}

fn to_canonical_path(path: &PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    let current_dir = &env::current_dir()?;
    let full_path: PathBuf = [current_dir, path].iter().collect();
    let full_path = full_path.canonicalize()?;

    Ok(full_path)
}
