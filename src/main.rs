mod app;
mod cli;
mod config;
mod gfx;
mod util;
mod window;

use clap::Parser;
use winit::event_loop::{ControlFlow, EventLoop};

use std::env;
use std::error::Error;
use std::path::PathBuf;

use crate::app::App;
use crate::cli::args::Args;
use crate::config::AppConfig;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse().validated()?;

    let config = AppConfig::default();
    let mut app = App::from_config(config);

    for path in args.paths {
        let Err(error) = open_image_from_path(&mut app, path) else {
            continue;
        };

        eprintln!("Couldn't open image: '{}'", error);
    }

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

    app.queue_image_open(canonical_path)?;

    Ok(())
}

fn to_canonical_path(path: &PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    let current_dir = &env::current_dir()?;
    let full_path: PathBuf = [current_dir, path].iter().collect();
    let full_path = full_path.canonicalize()?;

    Ok(full_path)
}
